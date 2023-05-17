#![cfg_attr(not(feature = "std"), no_std)]

//use frame_support::dispatch::Vec;
//use sp_core::H256 as Hash;
//use crate::AccountId;
use codec;
use pallet_charging_station::GeoHash;
use crate::Vec;

//     pub trait GeoRpcRuntimeApi<Block: BlockT, AccountId: Codec>  
sp_api::decl_runtime_apis! {
    pub trait GeoRpcRuntimeApi<AccountId>
    where
        AccountId: codec::Codec,
    {
        fn get_account_ids(geo_hash: GeoHash) -> Vec<AccountId>;
    }
}
