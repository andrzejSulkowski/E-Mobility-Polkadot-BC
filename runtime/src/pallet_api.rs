#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::Vec;
use codec::Codec;



//     pub trait GeoRpcRuntimeApi<Block: BlockT, AccountId: Codec>  
sp_api::decl_runtime_apis! {
    pub trait GeoRpcRuntimeApi<AccountId> where
    AccountId: Codec,
    {
        fn get_account_ids(geo_hash: [u8; 9]) -> Vec<AccountId>;
    }
}
