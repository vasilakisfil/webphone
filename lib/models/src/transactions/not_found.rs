#[derive(Debug, Clone)]
pub struct TransactionData {
    pub id: i64,
    pub branch_id: String,
    pub dialog_id: i64,
}

#[derive(Debug, Clone)]
pub enum NotFound {
    Default(TransactionData),
}
