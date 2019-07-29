//! Glimmer Blockchain Transactions
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tx {
    sender: String,
    recipient: String,
    amount: f64
}

impl Tx {
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        Tx {
            sender,
            recipient,
            amount
        }
    }
}
