// #![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::pallet_prelude::{Get, *};
// use frame_system::pallet_prelude::*;
// pub use pallet::*;
// use scale_info::prelude::string::String;


// #[frame_support::pallet]
// pub mod pallet {

// 	use super::*;

// 	/// Configure the pallet by specifying the parameters and types on which it depends.
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config {
// 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

//         #[pallet::constant]
//         type MaxQueryResultLength: Get<u32>;
// 	}

// 	#[pallet::storage]
// 	#[pallet::getter(fn balances)]
// 	pub(super) type Balances<T: Config> =
// 		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

// 	#[pallet::storage]
// 	#[pallet::getter(fn reservations)]
// 	pub(super) type Reservations<T: Config> =
// 		StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

// 	#[pallet::pallet]
// 	pub struct Pallet<T>(_);

// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
// 	pub enum Event<T: Config> {
// 		GeoHashStored(T::AccountId, GeoHash),
// 		GeoHashesStored(T::AccountId),
// 		AccountsRetrieved(BoundedVec<T::AccountId, T::MaxQueryResultLength>),
// 		GeoHashRemoved(T::AccountId, GeoHash),

// 		// Error Events
// 		AlreadyRegistered(T::AccountId, GeoHash),
// 		Other(T::AccountId, String),
// 	}

// 	#[pallet::error]
// 	pub enum Error<T> {
// 		GeoHashOverflow,
// 		GeoHashNotFound,
// 		AlreadyRegistered,
// 		Other,
// 	}


//     #[pallet::call]
//     impl<T: Config> Pallet<T> {
//         #[pallet::weight(10_000)]
//         #[pallet::call_index(0)]
//         pub fn create_tokens(origin: OriginFor<T>, amount: u64) -> DispatchResultWithPostInfo {
//             let who = ensure_signed(origin)?;
    
//             Balances::<T>::mutate(&who, |balance| *balance += amount);
    
//             Self::deposit_event(Event::TokensCreated(who, amount));
//             Ok(().into())
//         }
    
//         #[pallet::weight(10_000)]
//         #[pallet::call_index(1)]
//         pub fn reserve_station(origin: OriginFor<T>, geohash: [u8; 9]) -> DispatchResultWithPostInfo {
//             let who = ensure_signed(origin)?;
    
//             ensure!(Balances::<T>::get(&who) >= 10, Error::<T>::InsufficientBalance);
    
//             Balances::<T>::mutate(&who, |balance| *balance -= 10);
    
//             Reservations::<T>::insert(&who, true);
    
//             Self::deposit_event(Event::StationReserved(who, GeoHash::new(geohash)));
//             Ok(().into())
//         }
    
//         #[pallet::weight(10_000)]
//         #[pallet::call_index(2)]
//         pub fn pay_for_electricity(origin: OriginFor<T>, amount: u64) -> DispatchResultWithPostInfo {
//             let who = ensure_signed(origin)?;
    
//             ensure!(Balances::<T>::get(&who) >= amount, Error::<T>::InsufficientBalance);
    
//             Balances::<T>::mutate(&who, |balance| *balance -= amount);
    
//             Self::deposit_event(Event::ElectricityPaid(who, amount));
//             Ok(().into())
//         }
//     }


// 	impl<T: Config> Pallet<T> {
// 		pub fn get_account_ids(
// 			geo_hash: GeoHash,
// 		) -> BoundedVec<T::AccountId, T::MaxQueryResultLength> {
// 			return GeoHashes::<T>::get(geo_hash)
// 		}
// 	}
// }
