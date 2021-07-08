use std::error::Error;
use glimmer_core::prelude::*;
use glimmer_core::consts::*;
use glimmer_core::block::*;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Mining genesis block");
    
    let mut node = GlimmerNode::new()?;

    println!("Mined");

    let chain = &mut node.chain;

    chain.add_tx(Tx::new(RESERVE_WALLET, "bjldsfkljlkj8u8h", 65.0, 0.15 * 65.0));
    chain.add_tx(Tx::new(RESERVE_WALLET, "hi", 65.0, 0.25 * 65.0));
    chain.add_tx(Tx::new(RESERVE_WALLET, "hfksdlj", 65.0, 0.35 * 65.0));
    chain.add_tx(Tx::new(RESERVE_WALLET, "jfkdslfjk", 65.0, 0.15 * 65.0));

    println!("Mining block");

    chain.add_block();

    println!("Mined");

    chain.add_tx(Tx::new(RESERVE_WALLET, "bjldsfkljlkj8u8h", 65.0, 0.15 * 65.0));

    println!("Mining block");

    chain.add_block();

    println!("Mined");

    chain.add_tx(Tx::new(RESERVE_WALLET, "bjldsfkljlkj8u8h", 65.0, 0.15 * 65.0));

    println!("Mining block");

    chain.add_block();

    println!("Mined");

    print_chain(chain.chain.clone());

    println!("Reserve Balance: {}", chain.get_bal(RESERVE_WALLET));
    println!("Miner's Balance: {}", chain.get_bal("miner"));
    println!("bjldsfkljlkj8u8h's Balance: {}", chain.get_bal("bjldsfkljlkj8u8h"));

    Ok(())
}

fn print_chain(chain: Vec<Block>) {

    for block in chain {
        println!("Block: {}", pretty_hash(&block.hash().to_vec()));
        println!("Nonce: {}", block.nonce);
        println!("Parent: {}", pretty_hash(&block.prev_hash.to_vec()));
        println!("Txs: {}", block.txs.len());
        for tx in block.txs {
            println!(" Tx:");
            println!(" Sender: {}", tx.sender);
            println!(" Recipient: {}", tx.recipient);
            println!(" Amount: {}", tx.amount);
            println!(" Mining Fee: {}", tx.mining_fee);
            println!("");
        }
        println!("");
    }
}

fn pretty_hash(hash: &Vec<u8>) -> String {
    let mut string = String::with_capacity(2*hash.len());
    for n in hash {
        string.push_str(&format!("{:x}", n));
    }
    string
}
