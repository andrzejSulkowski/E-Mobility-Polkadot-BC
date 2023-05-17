// node/src/geo_rpc.rs

use std::sync::Arc;
use jsonrpsee::core::RpcResult;
use node_template_runtime::MaxQueryResultLength;
use node_template_runtime::pallet_charging_station::GeoHash;
use sp_blockchain::HeaderBackend;
use sp_api::ProvideRuntimeApi;

use node_template_runtime::{opaque::Block, Hash, AccountId };
use node_template_runtime::pallet_api::GeoRpcRuntimeApi;

use thiserror::Error;
use jsonrpsee::{
	proc_macros::rpc,
};
use jsonrpsee::types::error::{CallError, ErrorObject};
use sp_runtime::traits::{Block as BlockT};

#[derive(Error, Debug)]
pub enum CustomApiError {
    #[error("API Error: {0}")]
    ApiError(#[from] sp_api::ApiError),
}


pub struct GeoRpc<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}
impl<C, P> GeoRpc<C, P> {
    pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
    }
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

#[rpc(client, server)]
pub trait GeoRpcApi<BlockHash> {

    #[rpc(name = "georpc_getGeohash")]
    #[method(name = "get_account_ids")]
    fn get_account_ids_rpc(&self, geo_hash: [u8; 9]) -> RpcResult< Vec<AccountId> >;
}

impl<C> 
    GeoRpcApiServer<
    <Block as BlockT>::Hash

    > for GeoRpc<C, Block>
where
    Block: BlockT,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: GeoRpcRuntimeApi<Block, AccountId>,
{
    fn get_account_ids_rpc(&self, geo_hash: [u8; 9]) -> RpcResult< Vec<AccountId> > {
        let api = self.client.runtime_api();
        let block_hash = self.client.info().best_hash;
        let geo_hash_obj = GeoHash::new(geo_hash);

        let account_ids = api.get_account_ids(block_hash, geo_hash_obj)
            .map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query fee details.",
				Some(e.to_string()),
			))
		})?;
       Ok(account_ids)
    }
}
