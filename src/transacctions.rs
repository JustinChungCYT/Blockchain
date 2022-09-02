use super::*;
use hashable::Hashable;
use utils::u64_to_u8_slice;
use std::clone::Clone;
use rsa::RsaPrivateKey;

#[derive(Clone)]
pub struct Output {
    pub to: String,
    pub value: u64,
    pub pub_key: RsaPrivateKey
}

impl Hashable for Output {
    fn payload(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(self.to.as_bytes());
        vec.extend(u64_to_u8_slice(&self.value));
        vec
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
    pub signatures: Vec<Vec<u8>>
}

impl Transaction {
    // new stuff
    pub fn new(inputs: Vec<Output>, outputs: Vec<Output>, script_sigs: Vec<Vec<u8>>) -> Self {
        Transaction {
            inputs,
            outputs,
            signatures: script_sigs
        }
    }
    // new stuff

    pub fn input_sum(&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| input.value)
            .sum()
    }

    pub fn output_sum(&self) -> u64 {
        self.outputs
            .iter()
            .map(|output| output.value)
            .sum()
    }

    pub fn input_hash_list(&self) -> HashSet<Vec<u8>> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Vec<u8>>>()
    }

    pub fn output_hash_list(&self) -> HashSet<Vec<u8>> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Vec<u8>>>()
    }

    pub fn is_coinbase(&self) -> bool{
        self.inputs.is_empty()
    }
}