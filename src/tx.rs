//! Glimmer Blockchain Transactions
use ed25519_dalek::{PublicKey, Signature, SignatureError};
use crate::util::convert_u64_to_u8_array;

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
    pub mining_fee: f64,

    /// Sender public_key
    pub public_key: PublicKey,
    /// Signature of sender
    pub sig: Signature,
}

impl std::fmt::Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f," Tx:")?;
        writeln!(f," Sender: {}", self.sender)?;
        writeln!(f," Recipient: {}", self.recipient)?;
        writeln!(f," Amount: {}", self.amount)?;
        writeln!(f," Mining Fee: {}", self.mining_fee)?;
        writeln!(f,"")?;
        Ok(())
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

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend_from_slice(self.sender.as_bytes());
        vec.extend_from_slice(self.recipient.as_bytes());
        vec.extend_from_slice(&self.amount.to_bits().to_be_bytes());
        vec.extend_from_slice(&self.mining_fee.to_bits().to_be_bytes());
    }

    pub fn verify_sig(&self) -> bool {
        match self.public_key.verify(&self.to_bytes(), &self.sig) {
            Ok() => true,
            Err(_) => false
        }
    }

    /// Returns the total cost of this transaction
    pub fn cost(&self) -> f64 {
        self.amount + self.mining_fee
    }
}
