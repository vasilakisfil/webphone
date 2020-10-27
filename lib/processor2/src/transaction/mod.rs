use models::transaction::TransactionMsg;

#[derive(Clone)]
pub struct TransactionLayer;

impl TransactionLayer {
    pub fn new() -> Self {
        Self
    }

    pub async fn process(&self, msg: TransactionMsg) {
        println!("{:?}", msg);
    }
}
