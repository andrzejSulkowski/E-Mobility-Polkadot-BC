
use frame_support::dispatch::Vec;
use codec::Codec;



sp_api::decl_runtime_apis! {
    pub trait GeoRpcRuntimeApi<AccountId> 
    where 
        AccountId: Codec
    {
        fn get_account_ids(geo_hash: [u8; 9]) -> Vec<AccountId>;
    }
}
