use crate::{core::CoreLayer, transaction::TransactionLayer};
use models::server::{UdpTuple, ServerHandle};

pub struct TransportLayer {
    server_handle: ServerHandle

impl TransportLayer {
    pub fn new() -> Self {
        Self
    }

    pub async fn process_message(&self, server_handle: ServerHandle, msg: UdpTuple) {
        println!("{:?}", msg);
    }
}
