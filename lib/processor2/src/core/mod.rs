use models::core::CoreMsg;

pub struct CoreLayer {
    transaction_layer: crate::transaction::TransactionLayer
}

impl CoreLayer {
    pub fn new(transaction_layer: crate::transaction::TransactionLayer) -> Self {
        Self { transaction_layer }
    }

    pub async fn process(&self, msg: CoreMsg) {
        println!("{:?}", msg);
    }
}
