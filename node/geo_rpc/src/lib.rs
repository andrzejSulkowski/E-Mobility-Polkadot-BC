#![allow(unused)]
#![warn(missing_docs)]

//! Geo RPC module.

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};


#[rpc(client, server, namespace = "geo")]
pub trait GeoRpcApi<BlockHash> {
    /// Get AccountIds with specific geoHashes.
    #[method(name = "geo_getAccountIds")]
    fn get_account_ids(&self, geo_hash: String, at: Option<BlockHash>) -> RpcResult<Vec<u8>>;
}

/// An implementation of the GeoRpcApi.
pub struct GeoRpc<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> GeoRpc<C, B> {
    /// Create a new GeoRpc handler.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl<C, Block> GeoRpcApi<<Block as BlockT>::Hash> for GeoRpc<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi,
    C: HeaderBackend<Block>,
    C::Api: GeoRpcRuntimeApi<Block>,
{
    fn get_account_ids(
        &self,
        geo_hash: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        let runtime_api_result = api.get_account_ids_with_geo_hash(&at, geo_hash);
        runtime_api_result.map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query fee details.",
				Some(e.to_string()),
			))
		})

    }
}
