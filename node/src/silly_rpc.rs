// use jsonrpsee_derive::rpc;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::RpcResult;
//use jsonrpsee_core::Result;

#[rpc(client, server)]
pub trait SillyRpc {
    #[method(name = "hello_five")]
    fn silly_5(&self) -> RpcResult<u64>;

    #[method(name = "hello_seven")]
    fn silly_7(&self) -> RpcResult<u64>;
}


pub struct Silly;

impl Silly {
    pub fn new() -> Self {
		Self
    }
}

impl SillyRpcServer for Silly {
    fn silly_5(&self) -> RpcResult<u64> {
        Ok(5)
    }
    fn silly_7(&self) -> RpcResult<u64> {
        Ok(7)
    }
}