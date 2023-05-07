use sodiumoxide::crypto::box_;
// use sodiumoxide::crypto::secretbox;

pub struct KeyPair {
    pub public_key: box_::PublicKey,
    pub private_key: box_::SecretKey,
}

pub fn generate_keypair() -> KeyPair {
    let (public_key, private_key) = box_::gen_keypair();
    KeyPair {
        public_key,
        private_key,
    }
}
