#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Get, *};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::prelude::string::String;

#[cfg(feature = "std")]
extern crate geohash;

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaxQueryResultLength: Get<u32>;
	}

	#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo, MaxEncodedLen, Debug)]
	#[scale_info(skip_type_params(T))]
	pub struct GeoHash([u8; 9]); // Adjust the size to fit your needs

	impl GeoHash {
		// Create a new GeoHash
		pub fn new(hash: [u8; 9]) -> Self {
			Self(hash)
		}
		// Access the underlying byte array
		pub fn as_bytes(&self) -> &[u8; 9] {
			&self.0
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn geohashes)]
	pub(super) type GeoHashes<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		GeoHash,
		BoundedVec<T::AccountId, T::MaxQueryResultLength>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn balances)]
	pub(super) type Balances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn reservations)]
    pub(super) type Reservations<T: Config> =
        StorageMap<_, Blake2_128Concat, (GeoHash, u64), T::AccountId, OptionQuery>;
        

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		pub fn get_account_ids(
			geo_hash: GeoHash,
		) -> BoundedVec<T::AccountId, T::MaxQueryResultLength> {
			return GeoHashes::<T>::get(geo_hash)
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		GeoHashStored(T::AccountId, GeoHash),
		GeoHashesStored(T::AccountId),
		AccountsRetrieved(BoundedVec<T::AccountId, T::MaxQueryResultLength>),
		GeoHashRemoved(T::AccountId, GeoHash),

		// Error Events
		AlreadyRegistered(T::AccountId, GeoHash),
		Other(T::AccountId, String),

		// Token Events
		TokensCreated(T::AccountId, u64),
		StationReserved(T::AccountId, GeoHash, u64),
		ElectricityPaid(T::AccountId, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		GeoHashOverflow,
		GeoHashNotFound,
		AlreadyRegistered,
		InsufficientBalance,
		Other,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		#[pallet::call_index(0)]
		pub fn submit_geohash(
			origin: OriginFor<T>,
			geohash: [u8; 9],
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let geo = GeoHash::new(geohash);

			GeoHashes::<T>::try_mutate(&geo, |accounts| {
				if !accounts.iter().any(|account| account == &who) {
					accounts.try_push(who.clone()).map_err(|_| Error::<T>::GeoHashOverflow)
				} else {
					Self::deposit_event(Event::AlreadyRegistered(who.clone(), geo.clone()));
					Err(Error::<T>::AlreadyRegistered.into())
				}
			})?;

			Self::deposit_event(Event::GeoHashStored(who, geo));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		#[pallet::call_index(1)]
		pub fn submit_multiple_geohashes(
			origin: OriginFor<T>,
			geohashes: BoundedVec<[u8; 9], T::MaxQueryResultLength>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			for geohash in geohashes {
				let geo = GeoHash::new(geohash);

				let res = GeoHashes::<T>::try_mutate(&geo, |accounts| {
					if !accounts.iter().any(|account| account == &who) {
						accounts.try_push(who.clone()).map_err(|_| Error::<T>::GeoHashOverflow)?;
						Self::deposit_event(Event::GeoHashesStored(who.clone()));
						Ok(())
					} else {
						Self::deposit_event(Event::AlreadyRegistered(who.clone(), geo.clone()));
						Err(Error::<T>::AlreadyRegistered)
					}
				});

				// Handle the result of try_mutate
				match res {
					Ok(_) => {},
					Err(e) => return Err(e.into()),
				};
			}

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		#[pallet::call_index(2)]
		pub fn delete_charging_station(
			origin: OriginFor<T>,
			geohash: [u8; 9],
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let geo = GeoHash::new(geohash);

			GeoHashes::<T>::try_mutate(&geo, |accounts| {
				if let Some(pos) = accounts.iter().position(|account| account == &who) {
					accounts.remove(pos);
					Self::deposit_event(Event::GeoHashRemoved(who.clone(), geo.clone()));
					Ok(())
				} else {
					Self::deposit_event(Event::Other(
						who.clone(),
						String::from("Not Authorized to delete this GeoHash"),
					));
					Err(Error::<T>::Other)
				}
			})
			.map_err(|e| Into::<DispatchError>::into(e))?;

			Ok(().into())
		}
		// ---------------------------------------------------
		#[pallet::weight(10_000)]
		#[pallet::call_index(3)]
		pub fn create_tokens(origin: OriginFor<T>, amount: u64) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Balances::<T>::mutate(&who, |balance| *balance += amount);

			Self::deposit_event(Event::TokensCreated(who, amount));
			Ok(().into())
		}

        #[pallet::weight(10_000)]
        #[pallet::call_index(4)]
        pub fn reserve_station(origin: OriginFor<T>, geohash: [u8; 9], time: u64) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
        
            ensure!(Balances::<T>::get(&who) >= 10, Error::<T>::InsufficientBalance);
        
            Balances::<T>::mutate(&who, |balance| *balance -= 10);
            
            let geo = GeoHash::new(geohash);
            Reservations::<T>::insert((geo, time), who.clone());
        
            Self::deposit_event(Event::StationReserved(who, GeoHash::new(geohash), time));
            Ok(().into())
        }
        

        #[pallet::weight(10_000)]
        #[pallet::call_index(5)]
        pub fn pay_for_electricity(
            origin: OriginFor<T>, 
            amount: u64, 
            geohash: [u8; 9], 
            time: u64
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
        
            let geo = GeoHash::new(geohash);
        
            let total_amount = match Reservations::<T>::get((geo, time)) {
                Some(account) if account == who => amount.saturating_sub(time),
                _ => amount,
            };
        
            ensure!(Balances::<T>::get(&who) >= total_amount, Error::<T>::InsufficientBalance);
        
            Balances::<T>::mutate(&who, |balance| *balance -= total_amount);
        
            Self::deposit_event(Event::ElectricityPaid(who, total_amount));
            Ok(().into())
        }
	}
}
