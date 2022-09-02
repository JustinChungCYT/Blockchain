mod block;
mod utils;
mod hashable;
mod blockchain;
mod transacctions;
mod signature;

use block::Block;
use blockchain::Blockchain;
use signature::KeyPairs;
use transacctions::{ Output, Transaction };
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashSet;
use std::clone::Clone;
use std::vec;
use rsa::{RsaPrivateKey, PaddingScheme};


fn current_time() -> u128 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    time
}

fn main() {
    let alice_keys = KeyPairs::generate();
    let bob_keys = KeyPairs::generate();
    
    let mut blockchain = Blockchain::new();
    let difficulty: u128 = 0x0000ffffffffffffffffffffffffffff;
    
    // inserting blocks
    let mut genesis_block = Block::new(0, vec![Transaction::new(
        vec![],
        vec![Output {
            to: String::from("Alice"), value: 50, pub_key: alice_keys.pub_key.clone()
        }, 
        Output {
            to: String::from("Alice"), value: 10, pub_key: alice_keys.pub_key.clone()
        },
        Output { 
            to: String::from("Bob"), value: 50, pub_key: bob_keys.pub_key.clone()
        }],
        // useless for outputs
        vec![vec![]],
    )], current_time(), Vec::from([0; 32]), difficulty);

    genesis_block.mine();
    let result = blockchain.verify_block(genesis_block);
    match result {
        Result::Ok(_) => {
            println!("Added genesis block successfully!");
        }
        Result::Err(x) => {
            println!("Genesis block failed with: {:?}", x);
        }
    }


    let script_sig_1 = KeyPairs::create_script_sig(&blockchain.blocks[0].transactions[0].outputs[0], alice_keys.priv_key.clone());
    let script_sig_2 = KeyPairs::create_script_sig(&blockchain.blocks[0].transactions[0].outputs[1], alice_keys.priv_key.clone());

    let mut block1 = Block::new(1, vec![
        Transaction::new(
            vec![],
            vec![ Output {
                to: String::from("Alice"),
                value: 1,
                pub_key: alice_keys.pub_key.clone()
            }, Output {
                to: String::from("Bob"),
                value: 1,
                pub_key: bob_keys.pub_key.clone()
            }],
            vec![],
        ),
        Transaction::new(
            vec![ blockchain.blocks[0].transactions[0].outputs[0].clone(), blockchain.blocks[0].transactions[0].outputs[1].clone() ],
            vec![ Output {
                to: String::from("Alice"),
                value: 3,
                pub_key: alice_keys.pub_key.clone()
            }, Output {
                to: String::from("Bob"),
                value: 55,
                pub_key: bob_keys.pub_key.clone()
            }],
            vec![script_sig_1, script_sig_2],
        )
    ], current_time(), blockchain.blocks[0].hash.clone(), difficulty);

    block1.mine();
    let result2 = blockchain.verify_block(block1);
    match result2 {
        Result::Ok(_) => {
            println!("Added block 1 successfully!");
        }
        Result::Err(x) => {
            println!("Block 1 failed with: {x:?}");
        }
    }
}