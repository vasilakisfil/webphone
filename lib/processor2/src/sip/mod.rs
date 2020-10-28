use crate::{core::CoreLayer, transaction::TransactionLayer};
use models::server::{UdpTuple, ServerHandle};

pub struct TransportLayer {
    transaction_layer: crate::transaction::TransactionLayer,
    core_layer: crate::core::CoreLayer,
}

impl TransportLayer {
    pub fn new() -> Self {
        let transaction_layer = TransactionLayer::new();
        let core_layer = CoreLayer::new(transaction_layer.clone());
        let transaction = Self {
            core_layer: CoreLayer::new(transaction_layer.clone()),
            transaction_layer,
        }
    }

    pub async fn process(&self, server_handle: ServerHandle, msg: UdpTuple) {
        println!("{:?}", msg);
    }
}
