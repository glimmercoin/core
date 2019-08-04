// External Dependencies
use std::error::Error;
use num_bigint::BigUint;
use num_traits::identities::One;

use crate::error::MiningError;
use crate::consts::*;
use crate::tx::*;
use crate::block::*;
use crate::util::*;

pub type Blockchain = Vec<Block>;

/// Glimmer blockchain
/// This contains the blockchain logic 
pub struct Glimmer {
    pub chain: Blockchain,
    current_txs: Vec<Tx>
}

impl std::fmt::Display for Glimmer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for block in &self.chain {
            writeln!(f,"Block: {}", pretty_hash(&block.hash().to_vec()))?;
            writeln!(f,"Nonce: {}", block.nonce)?;
            writeln!(f,"Parent: {}", pretty_hash(&block.prev_hash.to_vec()))?;
            writeln!(f,"Txs: {}", block.txs.len())?;
            for tx in &block.txs {
                writeln!(f, "{}", tx)?;
            }
            write!(f,"")?;
        }

        Ok(())
    }

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
            // let txs = if self.current_txs.

            match self.chain.last() {
                Some(prev) => {
                    block = Block::new(&mut self.current_txs, prev.hash())?;
                    // self.current_txs = vec![Tx::new(RESERVE_WALLET, MINER_WALLET, REWARD,0.0)];
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


    /// Verify if a block is valid
    pub fn verify_block(block: &Block, nonce: u64) -> bool {
        let target = get_target();

        let hash = Block::calculate_hash(&block.encode(), nonce);
        let hash_int = BigUint::from_bytes_be(&hash);

        if hash_int <= target && {
            return true;
        }
        else {
            return false;
        }
    }

    /// Verify if a glimmer blockchain is valid
    pub fn verify_chain(glim: &Glimmer) -> bool {
        let chain = &glim.chain;
        let mut tmp_last_block = chain.get(0).unwrap();
        let mut cur_idx = 1; 

        // Iterate over all blocks in the chain
        while cur_idx < chain.len() {
            let block = &chain[cur_idx];

            // Verify that the prev_hash of the current 
            // block equals the hash of the last block
            if block.prev_hash != tmp_last_block.hash() {
                return false;
            }
            
            // Verify txs
            for tx in &block.txs {
                if !glim.verify_tx(tx) {
                    return false
                };
            }

            // Verify the POW nonces are valid
            if !Glimmer::verify_block(block, block.nonce) {
                return false;
            }


            tmp_last_block = block;
            cur_idx += 1
        }

        true

    }

    /// Returns the balance of a provide address
    pub fn get_bal(&self, addr: &str) -> f64 {
        // Sender balance
        let mut balance: f64 = 0.0;

        // Find balance of tx
        for block in &self.chain {
            for tx in &block.txs {
                // Withdrawls
                if tx.sender == addr {
                    balance -= tx.amount;
                }

                // Deposits
                if tx.recipient == addr {
                    balance += tx.amount;
                }
            }
        }
        balance
    }

    /// Verify whether a sender has enough to create a tx
    pub fn verify_tx(&self, new_tx: &Tx) -> bool {

        if !new_tx.verify_sig(){
            return false
        }

        let balance = self.get_bal(&new_tx.sender);

        if new_tx.sender == "" {
            return true;
        }

        if new_tx.recipient == "" {
            return false;
        }

        if new_tx.cost() > balance {
            return false;
        }

        true
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
