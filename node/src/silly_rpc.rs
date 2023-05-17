// use jsonrpsee_derive::rpc;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::RpcResult;
//use jsonrpsee_core::Result;

#[rpc(client, server)]
pub trait SillyRpc {
    #[rpc(name = "hello_five")]
    #[method(name = "get_silly_5")]
    fn silly_5(&self) -> RpcResult<u64>;

    #[rpc(name = "hello_seven")]
    #[method(name = "get_silly_7")]
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