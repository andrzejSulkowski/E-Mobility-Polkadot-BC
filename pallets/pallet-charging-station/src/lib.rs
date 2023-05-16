#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub mod api;
pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_support::pallet_prelude::Get;
use frame_system::pallet_prelude::*;
//pub use pallet_charging_station::api::GeoRpcRuntimeApi;
pub use api::GeoRpcRuntimeApi;

#[cfg(feature = "std")]
extern crate geohash;

#[frame_support::pallet]
pub mod pallet {

use super::*;


    	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type MaxQueryResultLength: Get<u32>;
	}

    #[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo, MaxEncodedLen, Debug)]
    #[scale_info(skip_type_params(T))]
    pub struct GeoHash([u8; 9]);  // Adjust the size to fit your needs

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
    pub(super) type GeoHashes<T: Config> = StorageMap<_, Blake2_128Concat, GeoHash, BoundedVec<T::AccountId, T::MaxQueryResultLength>, ValueQuery>;


	#[pallet::pallet]
	pub struct Pallet<T>(_);


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        GeoHashStored(T::AccountId, GeoHash),
        GeoHashesStored(T::AccountId),
        AccountsRetrieved(BoundedVec<T::AccountId, T::MaxQueryResultLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
        GeoHashOverflow,
        GeoHashNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn submit_geohash(origin: OriginFor<T>, geohash: [u8; 9]) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            
            let geo = GeoHash::new(geohash);

            GeoHashes::<T>::try_mutate(&geo, |accounts| {
                accounts.try_push(who.clone()).map_err(|_| Error::<T>::GeoHashOverflow)
            })?;


            Self::deposit_event(Event::GeoHashStored(who, geo));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn submit_multiple_geohashes(origin: OriginFor<T>, geohashes: BoundedVec<[u8; 9], T::MaxQueryResultLength>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            
            for geohash in geohashes {
                let geo = GeoHash::new(geohash);
        
                GeoHashes::<T>::try_mutate(&geo, |accounts| {
                    accounts.try_push(who.clone()).map_err(|_| Error::<T>::GeoHashOverflow)
                })?;
            }

            Self::deposit_event(Event::GeoHashesStored(who));
            Ok(().into())
        }
	}

    impl<T: Config> Pallet<T> {
        pub fn get_account_ids(geo_hash: GeoHash) -> BoundedVec<T::AccountId, T::MaxQueryResultLength> {
            GeoHashes::<T>::get(geo_hash)
        }
    }
}
