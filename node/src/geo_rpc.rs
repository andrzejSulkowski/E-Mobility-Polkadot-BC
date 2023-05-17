// node/src/geo_rpc.rs

use std::sync::Arc;
use sp_blockchain::HeaderBackend;
use sp_api::ProvideRuntimeApi;

use node_template_runtime::{opaque::Block, Hash, AccountId };
use node_template_runtime::pallet_api::GeoRpcRuntimeApi;
use thiserror::Error;
use jsonrpsee::{
	core::{Error as JsonRpseeError},
	proc_macros::rpc,
};
use sp_runtime::traits::{Block as BlockT};


#[derive(Error, Debug)]
pub enum CustomApiError {
    #[error("API Error: {0}")]
    ApiError(#[from] sp_api::ApiError),
}



#[rpc(client, server)]
pub trait GeoRpcApi<BlockHash> {
    #[method(name = "get_account_ids")]
    fn get_account_ids(&self, at: Option<BlockHash>, geo_hash: [u8; 9]) -> Result<Vec<AccountId>, JsonRpseeError>;
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

impl<C> 
    GeoRpcApiServer<
    <Block as BlockT>::Hash

    > for GeoRpc<C, Block>
where
    Block: BlockT,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: GeoRpcRuntimeApi<Block, AccountId>,
{
    fn get_account_ids(&self, at: Option<Hash>, geo_hash: [u8; 9]) -> Result<Vec<AccountId>, JsonRpseeError> {
        let api = self.client.runtime_api();
        let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

        api.get_account_ids(at_hash, geo_hash).map_err(|e| {
            JsonRpseeError::Custom(String::from("No AccountIds for this geoHash")) 
        })
    }
}
