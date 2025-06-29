use serde::{Deserialize, Serialize};
use sha256;
use std::time::SystemTime;
use num_bigint::BigUint;

use crate::db_utils;

const DIFFICULTY: usize = 8; // number of zeros needed to prefix hash (bits)

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {

    pub index: u32,
    pub timestamp: u64, // timestamp in seconds since 1970-01-01 00:00 UTC (epoch)
    pub data: String, // data encoded in blocks
    pub previous: String, // hash for the previous block
    pub nonce: u32,
    pub hash: String,
    pub diff_bits: u32,
    pub acc_diff: u64

}

impl Block {

    pub fn from_json(json: &str) -> Result<Block, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Error serializing block JSON")
    }
    
    pub fn generate_genesis_block() -> Block {
        let mut genesis = Block {
            index: 0,
            timestamp: 0,
            data: String::from(""),
            previous: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            nonce: 0,
            hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),        
            diff_bits: 1,
            acc_diff: 2
        };
        genesis.nonce = crate::block_utils::Block::generate_work(&mut genesis).expect("Genesis work generation failed");
        genesis.hash = crate::block_utils::Block::generate_hash(&genesis);
        genesis
    }

    pub fn generate_hash(block: &Block) -> String {
        sha256::digest(format!(
            "{}{}{}{}{}",
            block.index,
            block.timestamp,
            sha256::digest(&block.data),
            block.previous,
            block.nonce
        ))
    }

    pub fn generate_work(block: &mut Block) -> Option<u32> {
        for nonce in 0.. {
            block.nonce = nonce;
            if Block::is_work_valid(block) { return Some(nonce) };
        };
        None
    }

    pub fn is_work_valid(block: &Block) -> bool {
        let hash = Block::generate_hash(block);
        match BigUint::parse_bytes(hash.as_bytes(), 16) {
            Some(parsed_bytes) => {
                let hash_bits = parsed_bytes.to_radix_be(2);
                hash_bits.len() <= 256-(block.diff_bits as usize)
            },
            None => false
        }
    }

    pub fn verify_block_is_valid(block: &Block) -> Result<bool, &'static str> {
        // check if the previous block exists and is a valid block
        let is_genesis = block.index == 0;
        let prev_block_option: Option<Block> = db_utils::get_block(&block.previous);
        match prev_block_option {
            Some(_) => (),
            None => { if !is_genesis { return Err("No previous block."); } }
        };

        // check that the timestamp of the block is greater than that of the previous block and less than 10 minutes into the future
        let secs_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        match prev_block_option {
            Some(prev_block) => {
                // not genesis
                if block.index-prev_block.index != 1 { return Err("Invalid block index."); };
                if !(block.timestamp > prev_block.timestamp) { return Err("Timestamp is less than or equal to previous block."); };
            },
            None => ()
        };
        if !(block.timestamp < (secs_since_epoch as u64 + 600)) { return Err("Block is too far ahead in the future."); };

        // [TODO] verify that data is valid

        // verify that work (nonce) is valid
        if !(Block::is_work_valid(block)) { return Err("Nonce is not valid."); };

        // verify that hash is valid
        if Block::generate_hash(block) != block.hash { return Err("Hash is not valid."); };

        Ok(true)
    }

    pub fn add_acc_diff_to_block(mut block: Block) -> Block {
        let is_genesis = block.index == 0;
        let prev_block_option: Option<Block> = db_utils::get_block(&block.previous);

        match prev_block_option {
            Some(prev_block) => {
                println!("{} | {}", prev_block.acc_diff, block.diff_bits);
                block.acc_diff = prev_block.acc_diff + (2 as u64).pow(block.diff_bits);
            },
            None => {
                block.acc_diff = (2 as u64).pow(block.diff_bits);
            }
        };

        block
    }

}