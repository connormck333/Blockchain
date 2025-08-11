use serde::Serialize;

#[derive(Serialize)]
pub struct TransactionResponse {
    success: bool,
    message: String
}

impl TransactionResponse {
    pub fn new(success: bool, message: String) -> Self {
        TransactionResponse {
            success,
            message
        }
    }
}