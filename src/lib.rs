// #![deny(warnings)]

use std::collections::HashSet;
use std::error::Error;

// Define modules
pub mod error;
pub mod chain;
pub mod proto;
pub mod tx;
pub mod util;
pub mod consts;
pub mod block;

use chain::*;

pub mod prelude {
    pub use super::GlimmerNode;
    pub use super::chain::*;
    pub use super::tx::*;
    pub use super::block::*;
}

/// Glimmer Node
/// This contains the networking logic for the glimmer blockchain
pub struct GlimmerNode {
    pub chain: Glimmer,
    pub nodes: HashSet<String>,
}

impl GlimmerNode {
    /// Create a new Glimmer Blockchain node
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut genesis = GlimmerNode {
            nodes: HashSet::new(),
            chain: Glimmer::new()?
        };

        // TODO: Add offical glimmer node
        genesis.nodes.insert("localhost:5000".to_string());

        Ok(genesis)
    }

    /// Returns a refrence to a glimmer blockchain
    pub fn chain(&self) -> &Blockchain {
        &self.chain.chain
    }

    /// Returns a mutable refrence to a glimmer blockchain
    pub fn chain_mut(&mut self) -> &mut Blockchain {
        &mut self.chain.chain
    }

    /// Add nodes to local HashSet
    pub fn register_nodes(&mut self, nodes: Vec<String>) {
        for node in nodes {
            self.nodes.insert(node);
            // TODO: Download nodes list from peers.
        }
    }

    /// Verify if a glimmer blockchain is valid
    pub fn verify_chain(chain: Blockchain) -> bool {
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

            // Verify the POW nonces are valid
            if !Glimmer::verify_block(block, block.nonce) {
                return false;
            }

            // TODO: Verify individual txs
            tmp_last_block = block;
            cur_idx += 1
        }

        true

    }

    /// Find longest chain from all nodes
    pub fn resolve_conflicts(&mut self) -> Result<bool, Box<dyn Error>> {
        // let new_chain: Option<Blockchain> = None;
        // let min_len = self.chain().len();

        Ok(false)
    } 
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
