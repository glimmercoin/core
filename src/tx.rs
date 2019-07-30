//! Glimmer Blockchain Transactions

#[derive(PartialEq, Debug, Clone)]
/// Transaction on the Glimemr Blockchain
pub struct Tx {
    /// Sender Address
    pub sender: String,
    /// Recipient Address
    pub recipient: String,
    /// Amount to send the recipient
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
