use std::error::Error;
use glimmer_core::prelude::*;
use glimmer_core::consts::*;
use glimmer_core::block::*;

#[test]
fn invalidate_chain() {
    
    let mut node = GlimmerNode::new().unwrap();

    let chain = &mut node.chain;
    
    eprintln!("Reserve Bal: {}", chain.get_bal(RESERVE_WALLET));

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    assert!(GlimmerNode::verify_chain(chain));

    chain.chain.get_mut(0).unwrap().txs.get_mut(0).unwrap().amount = 1.0;

    assert!(!GlimmerNode::verify_chain(chain));
}

#[test]
fn validate_tx() {
    let mut node = GlimmerNode::new().unwrap();

    let chain = &mut node.chain;

    assert_eq!(GENESIS_RESERVE, chain.get_bal(RESERVE_WALLET));

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    assert_eq!(GENESIS_RESERVE - 65.0, chain.get_bal(RESERVE_WALLET));

    assert_eq!(65.0, chain.get_bal("grandma"));

}

