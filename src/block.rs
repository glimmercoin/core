use std::error::Error;
use crate::util::*;
use serde::{Serialize, Deserialize};
use blake2b_rs::blake2b;

use crate::consts::*;
use crate::tx::Tx;

/// Block in the Glimmer blockchain
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    timestamp: u128,
    pub txs: Vec<Tx>,
    pub nonce: u64,
    pub prev_hash: Vec<u8>
}

impl Block {
    /// Creates a new block
    pub fn new(nonce: u64, prev_hash: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Block {
            timestamp: time()?,
            txs: Vec::new(),
            nonce,
            prev_hash
        })
    }

    /// Append the txs to a block
    pub fn append_tx(&mut self, mut txs: &mut Vec<Tx>) {
        self.txs.append(&mut txs);
    }

    pub fn push_tx(&mut self,  tx: Tx) {
        self.txs.push(tx);
    }

    /// Returns a hash of the block
    pub fn hash(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = serde_json::to_string(&self)?;
        let mut dst = [0; HASH_LEN];

        blake2b(KEY, data.as_bytes(), &mut dst);

        Ok(dst.to_vec())
    }

    pub fn genesis() -> Self {
        Block {
            timestamp: time().unwrap(),
            txs: Vec::new(),
            nonce: 100,
            prev_hash: [0; HASH_LEN].to_vec()
        }
    }
}
