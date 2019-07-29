// External Dependencies
use std::error::Error;
use serde::{Serialize, Deserialize};
use crypto_hash::{Algorithm, hex_digest};
use blake2b_rs::blake2b;

use crate::util::*;
use crate::consts::*;
use crate::tx::*;

pub type Blockchain = Vec<Block>;

/// Block in the Glimmer blockchain
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: usize,
    timestamp: u128,
    pub txs: Vec<Tx>,
    pub proof: u64,
    pub prev_hash: Vec<u8>
}

impl Block {
    /// Creates a new block
    pub fn new(index: usize, txs: Vec<Tx>, proof: u64, prev_hash: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Block {
            index,
            timestamp: time()?,
            txs,
            proof,
            prev_hash
        })
    }

    pub fn hash(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = serde_json::to_string(&self)?;
        let mut dst = [0; HASH_LEN];

        blake2b(KEY, data.as_bytes(), &mut dst);

        Ok(dst.to_vec())
    }
}

/// Glimmer blockchain
/// This contains the blockchain logic 
pub struct Glimmer {
    pub chain: Blockchain,
    current_txs: Vec<Tx>
}

impl Glimmer {
    /// Create a new instance of the Glimmer blockchain
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut genesis = Glimmer {
            chain: Vec::new(),
            current_txs: Vec::new()
        };

        // Add genesis block
        genesis.add_block(100, Some([0; 64].to_vec()))?;

        Ok(genesis)
    }

    /// Add a block to the blockchain 
    pub fn add_block(&mut self, proof: u64, prev_hash: Option<Vec<u8>>) -> Result<Option<&Block>, Box<dyn Error>>{
        // Get the hash of the last block
        let hash = match prev_hash {
            Some(hash) => hash,
            None => self.chain.last().unwrap().hash()?
        };

        let block = Block::new(
            self.chain.len() + 1,
            self.current_txs.clone(),
            proof,
            hash 
        )?;

        self.current_txs = Vec::new();

        self.chain.push(block);

        Ok(self.last_block())
    }


    pub fn add_tx(&mut self, tx: Tx) -> usize {
        self.current_txs.push(tx);
        match self.last_block() {
            Some(block) => block.index + 1,
            None => 0
        }
    }

    /// Returns the last block of the chain
    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }


    /// Basic POW algo
    /// TODO: Make this more optimized
    pub fn pow(last_proof: u64) -> u64 {
        let mut proof = 0;

        while !Glimmer::verify_proof(last_proof, proof) {
            proof += 1;
        }

        proof
    }


    /// Verify if a proof is valid
    pub fn verify_proof(last_proof: u64, proof: u64) -> bool {
        let guess = serde_json::to_string(&(format!("{}{}", last_proof, proof))).unwrap();
        let digest = hex_digest(Algorithm::SHA256, guess.as_bytes());
        let guess_sub = &digest[0..POW_DIFFICULTLY];
        let valid = guess_sub == POW_GOAL;
        if DEBUG {
            println!("Valid: {}, {} == {}...", valid, digest, repeat_char('0', POW_DIFFICULTLY));
        }
        valid
    }


    pub fn mine(&mut self, wallet: &String) -> &Block {
        let last_block= self.last_block().unwrap();
        let prev_hash = last_block.hash().unwrap();

        let proof = Glimmer::pow(last_block.proof);

        self.add_tx(Tx::new(String::from("0"), wallet.to_string(), 1.0));

        let block = self.add_block(proof, Some(prev_hash)).unwrap().unwrap();
        &block

    }


}
