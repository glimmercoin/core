//! Glimmer Blockchain Transactions
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Transaction on the Glimemr Blockchain
pub struct Tx {
    pub sender: String,
    pub recipient: String,
    pub amount: f64
}

impl Tx {
    /// Create a new transaction
    pub fn new(sender: &str, recipient: &str, amount: f64) -> Self {
        Tx {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount
        }
    }
}
