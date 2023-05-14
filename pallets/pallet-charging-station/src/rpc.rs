use jsonrpc_core::Result as RpcResult;
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use your_pallet::GeoHash;

#[rpc]
pub trait GeoHashApi<BlockHash> {
    #[rpc(name = "geohash_getAccountsForGeohash")]
    fn get_accounts_for_geohash(&self, geohash: GeoHash, at: Option<BlockHash>) -> RpcResult<Vec<AccountId>>;
}