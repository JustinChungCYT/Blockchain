use crypto_hash::{ Algorithm, digest };

pub trait Hashable {
    fn payload(&self) -> Vec<u8>;
    fn hash(&self) -> Vec<u8> {
        digest(Algorithm::SHA256, &(self.payload()))
    }
}