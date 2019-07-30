#![feature(test)]
extern crate test;

use test::Bencher;
use blake2b_rs::blake2b;

use glimmer_core::prelude::*;
use glimmer_core::consts::*;
use glimmer_core::util::*;


#[bench]
fn hash_block(b: &mut Bencher) {
    let block =  Block::new(vec![], [0; HASH_LEN]).unwrap();
    let headers = block.encode();
    b.iter(|| {
        Block::calculate_hash(&headers, 0);
    })

}

#[bench]
fn hash_headers(b: &mut Bencher) {
    let mut block =  Block::new(vec![], [0; HASH_LEN]).unwrap();

    // Encode headers
    let mut headers = block.headers();

    // Encode Body
    headers.extend_from_slice(&block.body());
    // Encode random nonce
    headers.extend_from_slice(&convert_u64_to_u8_array(0u64));

    let mut dst = [0; HASH_LEN];

    b.iter(|| {
        // Hash it using blake2b
        blake2b(KEY, &headers, &mut dst);
    })

}
