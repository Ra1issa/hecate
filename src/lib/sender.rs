use crate::{
    utils,
    types::{Mfrank, Token},
};

use libsignal_protocol::crypto;
use sha2::{Sha256, Digest};
use ed25519_dalek::{Keypair, Signer};
use rand::{CryptoRng, Rng};

pub fn generate_frank
<R: CryptoRng + Rng>
(
    msg: String,
    token: Token,
    rng: &mut R,
)-> (Mfrank, Vec<u8>){
     // Hash message
     let mut hasher = Sha256::new();
     hasher.update(msg.clone());
     let hash = hasher.finalize();

     // Additively split x1 and H(m) into x2
     let x2 = utils::add_bytes(&token.x1, &hash);

     // Commit x1 and x2
     let x = [token.x1.clone(), x2.clone()].concat();
     let randc = utils::random_block(32, rng);
     let com = crypto::hmac_sha256(&randc, &x).to_vec();

     // Sign x2
     let k = Keypair::from_bytes(&token.key_eph).unwrap();
     let send_sig = k.sign(&x2).to_bytes().to_vec();

     let mf = Mfrank{
                 msg,
                 x1: token.x1,
                 x2,
                 nonce: token.nonce,
                 mod_sig: token.mod_sig,
                 send_sig: send_sig,
                 pke: k.public.as_bytes().to_vec(),
                 randc,
                 time: token.time,
             };
     return (mf, com);
}
