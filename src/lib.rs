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


    // /// Find longest chain from all nodes
    // pub fn resolve_conflicts(&mut self) -> Result<bool, Box<dyn Error>> {
    //     // let new_chain: Option<Blockchain> = None;
    //     // let min_len = self.chain().len();

    //     Ok(false)
    // } 
}
