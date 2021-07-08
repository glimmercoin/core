//! Glimmer Blockchain Transactions

#[derive(PartialEq, Debug, Clone)]
/// Transaction on the Glimemr Blockchain
pub struct Tx {
    /// Sender Address
    pub sender: String,
    /// Recipient Address
    pub recipient: String,
    /// Amount to send the recipient
    pub amount: f64,
    /// Fee sent to miner
    pub mining_fee: f64
}

impl std::fmt::Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f," Tx:");
        writeln!(f," Sender: {}", self.sender);
        writeln!(f," Recipient: {}", self.recipient);
        writeln!(f," Amount: {}", self.amount);
        writeln!(f," Mining Fee: {}", self.mining_fee);
        writeln!(f,"")
    }
}

impl Tx {
    /// Create a new transaction
    pub fn new(sender: &str, recipient: &str, amount: f64, mining_fee: f64) -> Self {
        Tx {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
            mining_fee
        }
    }

    /// Returns the total cost of this transaction
    pub fn cost(&self) -> f64 {
        self.amount + self.mining_fee
    }
}
