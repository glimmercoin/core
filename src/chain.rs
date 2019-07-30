// External Dependencies
use std::error::Error;
use serde::{Serialize, Deserialize};
use blake2b_rs::blake2b;
use num_bigint::BigUint;
use num_traits::identities::One;

use crate::error::MiningError;
use crate::util::*;
use crate::consts::*;
use crate::tx::*;
use crate::block::*;

pub type Blockchain = Vec<Block>;

/// Glimmer blockchain
/// This contains the blockchain logic 
pub struct Glimmer {
    pub chain: Blockchain,
    current_txs: Vec<Tx>
}

impl Glimmer {
    /// Create a new instance of the Glimmer blockchain
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Glimmer {
            chain: vec![Block::genesis()],
            current_txs: Vec::new()
        })
    }

    /// Add a block to the blockchain 
    pub fn add_block(&mut self) -> Result<(), MiningError>{
        let block: Block;
        {
            match self.chain.last() {
                Some(prev) => {
                    block = Block::new(self.current_txs.clone(), prev.hash())?;
                    self.current_txs = vec![Tx::new(RESERVE_WALLET, MINER_WALLET, REWARD)];
                }
                // Adding a block to an empty blockchain is an error, a genesis block needs to be
                // created first.
                None => {
                    return Err(MiningError::NoParent)
                }
            }
        }

        self.chain.push(block);

        Ok(())
    }


    pub fn add_tx(&mut self, tx: Tx) -> usize {
        self.current_txs.push(tx);
        self.chain.len() + 1
    }

    /// Returns the last block of the chain
    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }


    /// Verify if a nonce is valid
    pub fn verify_block(block: &Block, nonce: u64) -> bool {
        // let guess = serde_json::to_string(&(format!("{}{}", last_nonce, nonce))).unwrap();
      let target = BigUint::one() << (HASH_BITS - POW_DIFFICULTLY);

      let hash = Block::calculate_hash(&block, nonce);
      let hash_int = BigUint::from_bytes_be(&hash);

      if hash_int < target {
          return true;
      }
      else {
          return false;
      }
    }


    // pub fn mine(&mut self, wallet: &String) -> &Block {
    //     let last_block= self.last_block().unwrap();
    //     let prev_hash = last_block.hash();

    //     // TODO: Make sure we can fail mining
    //     let nonce = pow(last_block.nonce).unwrap();

    //     self.add_tx(Tx::new(String::from("0"), wallet.to_string(), 1.0));

    //     let block = self.add_block(nonce, Some(prev_hash)).unwrap().unwrap();
    //     &block

    // }


}
