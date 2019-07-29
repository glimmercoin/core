// External Dependencies
use std::error::Error;
use serde::{Serialize, Deserialize};
use blake2b_rs::blake2b;

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
    pub fn add_block(&mut self, nonce: u64, prev_hash: Option<Vec<u8>>) -> Result<Option<&Block>, Box<dyn Error>>{
        // Get the hash of the last block
        let hash = match prev_hash {
            Some(hash) => hash,
            None => self.chain.last().unwrap().hash()?
        };

        let mut block = Block::new(nonce, hash)?;

        block.append_tx(&mut self.current_txs);

        self.current_txs = Vec::new();

        self.chain.push(block);

        Ok(self.last_block())
    }


    pub fn add_tx(&mut self, tx: Tx) -> usize {
        self.current_txs.push(tx);
        self.chain.len() + 1
    }

    /// Returns the last block of the chain
    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }


    /// Basic POW algo
    /// TODO: Make this more optimized
    pub fn pow(last_nonce: u64) -> Option<u64> {

        for nonce in 0..MAX_NONCE {
            if Glimmer::verify_nonce(last_nonce, nonce) {
                return Some(nonce);
            }
        }

        None

    }


    /// Verify if a nonce is valid
    pub fn verify_nonce(last_nonce: u64, nonce: u64) -> bool {
        let guess = serde_json::to_string(&(format!("{}{}", last_nonce, nonce))).unwrap();
        let mut dst = [0; HASH_LEN];

        blake2b(KEY, guess.as_bytes(), &mut dst);

        let guess_sub = &dst[0..POW_DIFFICULTLY];
        let valid = guess_sub == POW_GOAL;
        if DEBUG {
            println!("Valid: {}, {:?} == {}...", valid, dst.to_vec(), repeat_char('0', POW_DIFFICULTLY));
        }
        valid
    }


    pub fn mine(&mut self, wallet: &String) -> &Block {
        let last_block= self.last_block().unwrap();
        let prev_hash = last_block.hash().unwrap();

        // TODO: Make sure we can fail mining
        let nonce = Glimmer::pow(last_block.nonce).unwrap();

        self.add_tx(Tx::new(String::from("0"), wallet.to_string(), 1.0));

        let block = self.add_block(nonce, Some(prev_hash)).unwrap().unwrap();
        &block

    }


}
