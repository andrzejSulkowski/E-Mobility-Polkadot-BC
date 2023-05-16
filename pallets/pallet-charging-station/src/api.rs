use sp_api::decl_runtime_apis;
use codec::Codec;

decl_runtime_apis! {
    pub trait GeoRpcRuntimeApi<AccountId> where AccountId: Codec {
        fn get_account_ids(geo_hash: [u8; 9]) -> Vec<AccountId>;
    }
}