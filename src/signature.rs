use super::*;
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use transacctions::Output;
use hashable::Hashable;

pub struct KeyPairs {
    pub pub_key: RsaPrivateKey,
    pub priv_key: RsaPublicKey
}

//#[derive(Hash, Eq, PartialEq, Debug)]
#[derive(PartialEq, Debug)]
pub struct LockingScript {
    pub output_hash: Vec<u8>,
    pub pub_key: RsaPrivateKey
}

impl KeyPairs {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        KeyPairs {
            pub_key: priv_key,
            priv_key: pub_key
        }
    }

    // new stuff
    pub fn create_script_sig(utxo: &Output, priv_key: RsaPublicKey) -> Vec<u8> {
        let data = utxo.hash();
        let mut rng = rand::thread_rng();
        let script_sig = priv_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..]).expect("failed to encrypt");
        script_sig
    }
}