use super::*;
use std::fmt::{ Debug, Formatter, Result };
use crate::utils::{ u32_to_u8_slice, u128_to_u8_slice, u64_to_u8_slice, hash_to_difficulty };
use crate::hashable::Hashable;
use transacctions::Transaction;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Vec<u8>,
    pub nonce: u64,
    pub prev_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, timestamp: u128, prev_hash: Vec<u8>, difficulty: u128) -> Self {
        Block {
            index: index,
            timestamp: timestamp,
            hash: Vec::new(),
            nonce: 0,
            prev_hash: prev_hash,
            transactions: transactions,
            difficulty: difficulty
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 1..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let new_hash = self.hash();
            if compare_difficulty(&new_hash, self.difficulty) {
                self.hash = new_hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn payload(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.append(&mut u32_to_u8_slice(&self.index));
        vec.append(&mut u128_to_u8_slice(&self.timestamp));
        vec.append(&mut u64_to_u8_slice(&self.nonce));
        vec.append(&mut self.prev_hash.clone());
        vec
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        f.debug_struct("Block")
            .field("index", &self.index)
            .field("timestamp", &self.timestamp)
            .field("hash", &self.hash)
            .field("nonce", &self.nonce)
            .field("prev_hash", &self.prev_hash)
            .field("difficulty", &self.difficulty)
            .finish()
    }
}

pub fn compare_difficulty(hash: &Vec<u8>, difficulty: u128) -> bool {
    hash_to_difficulty(&hash) < difficulty
}
