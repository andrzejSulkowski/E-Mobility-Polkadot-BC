#![cfg_attr(not(feature = "std"), no_std)]

use codec;
use pallet_charging_station::GeoHash;
use crate::Vec;

sp_api::decl_runtime_apis! {
    pub trait GeoRpcRuntimeApi<AccountId>
    where
        AccountId: codec::Codec,
    {
        fn get_account_ids(geo_hash: GeoHash) -> Vec<AccountId>;
    }
}
