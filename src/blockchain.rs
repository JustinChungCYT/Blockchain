use super::*;
use block::compare_difficulty;
use hashable::Hashable;
use signature::LockingScript;


#[derive(Debug)]
pub enum BlockchainErrors {
    IndexError,
    DifficultyError,
    PrevHashError,
    TimestampError,
    InvalidCoinbaseError,
    InvalidCoinbaseOutputError,
    InvalidInputError,
    InvalidInputValueError,
    WrongSignatureError,
    SignatureAmountError
}
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub unspent_outputs: HashSet<Vec<u8>>,
    // new stuff
    pub lock_scripts: Vec<LockingScript>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
            // new stuff
            lock_scripts: Vec::new()
        }
    }

    pub fn verify_block(&mut self, block: Block) -> Result<(), BlockchainErrors> {
        // check block validity
        let i = self.blocks.len();

        if block.index != i as u32{
            println!("Index not matched! Error block: {}", i);
            return Err(BlockchainErrors::IndexError);
        } else if !compare_difficulty(&block.hash(), block.difficulty) {
            println!("Difficulty not matched!! Error block: {}", i);
               return Err(BlockchainErrors::DifficultyError);
        }

        if i == 0 {
            if block.prev_hash != vec![0; 32] {
                println!("Invalid prev_hash for Genesis Block!!");
                return Err(BlockchainErrors::PrevHashError);
            }
        } else if &self.blocks[i-1].timestamp > &block.timestamp {
            println!("Timestamp not matched!! Error block: {}", i);
            return Err(BlockchainErrors::TimestampError);
        }

        // check transactions
        if let Some((coinbase, remaining_tx)) = block.transactions.split_first() {
            if !(coinbase.is_coinbase()) {
                return Err(BlockchainErrors::InvalidCoinbaseError);
            }
            
            let mut outputs_consumed: HashSet<Vec<u8>> = HashSet::new();
            let mut outputs_created: HashSet<Vec<u8>> = HashSet::new();
            // new stuff
            let mut temp_lock_scripts: Vec<LockingScript> = Vec::new();

            let mut block_fee = 0;

            // genesis block special case
            if i == 0 {
                for output in coinbase.outputs.clone() {
                    self.lock_scripts.push( LockingScript {
                        output_hash: output.hash(),
                        pub_key: output.pub_key
                    })
                }
                self.blocks.push(block);
                return Ok(());
            }

            for tx in remaining_tx {
                let input_hashes = tx.input_hash_list();
                let income = tx.input_sum();
                let outcome = tx.output_sum();

                // new stuff
                if tx.signatures.len() != tx.inputs.len() {
                    return Err(BlockchainErrors::SignatureAmountError);
                }

                let mut sign_index = 0;
                for input in tx.inputs.clone() {
                    // potential error
                    let unlock = self.unlock_script(tx.signatures[sign_index].clone(), &input.pub_key);
                    if let Err(error) = unlock {
                        return Err(error);
                    }
                    sign_index += 1;
                }
                // ^^^^

                if !(input_hashes.is_subset(&self.unspent_outputs)) {
                    return Err(BlockchainErrors::InvalidInputError);
                }

                if outcome > income {
                    return Err(BlockchainErrors::InvalidInputValueError);
                }

                block_fee += income - outcome;
                outputs_consumed.extend(input_hashes);
                outputs_created.extend(tx.output_hash_list());

                // new stuff
                for output in tx.outputs.clone() {
                    temp_lock_scripts.push( LockingScript {
                        output_hash: output.hash(),
                        pub_key: output.pub_key
                    })
                }
                // ^^^^
            }

            if block_fee != coinbase.output_sum() {
                return Err(BlockchainErrors::InvalidCoinbaseOutputError);
            } else {
                self.unspent_outputs.extend(coinbase.output_hash_list());
            }
            self.unspent_outputs.retain(|consumed| !(outputs_consumed.contains(consumed)));
            //self.unspent_outputs.extend(outputs_created);
            // new stuff
            self.lock_scripts.extend(temp_lock_scripts);
        }

        self.blocks.push(block);
        Ok(())
    }

    // new stuff
    pub fn unlock_script(&mut self, script_sig: Vec<u8>, pub_key: &RsaPrivateKey) -> Result<(), BlockchainErrors> {
        // potential error
        let hash_from_sig = pub_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &script_sig).unwrap_or(Vec::from([0; 32]));
        if self.lock_scripts.contains( &LockingScript{ output_hash: hash_from_sig.clone(), pub_key: pub_key.clone() }) {
            self.unspent_outputs.insert(hash_from_sig);
            Ok(())
        } else {
            return Err(BlockchainErrors::WrongSignatureError)
        }
    }
}