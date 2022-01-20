use crate::{
    utils,
    types::{Mfrank, Token},
};
use poksho;
use libsignal_protocol::crypto;
use curve25519_dalek::ristretto::{
    CompressedRistretto,
};
use curve25519_dalek::scalar::Scalar;
use sha2::{Sha256, Digest};
use std::{
    convert::TryInto,
};

pub fn generate_frank(
    msg: String,
    token: Token,
)-> (Mfrank, Vec<u8>){
     // Hash message
     let mut hasher = Sha256::new();
     hasher.update(msg.clone());
     let hash = hasher.finalize();

     // Additively split x1 and H(m) into x2
     let x2 = utils::add_bytes(&token.x1, &hash);

     // Commit x1 and x2
     let x = [token.x1.clone(), x2.clone()].concat();
     let randc = utils::random_block(32);
     let com = crypto::hmac_sha256(&randc, &x).to_vec();

     // Turn pke/ske back to RistrettoPt and scalar
     let r_pke = CompressedRistretto(token.pke.clone().try_into().unwrap());
     let r_pke = r_pke.decompress().unwrap();

     let s_ske = Scalar::from_canonical_bytes(token.ske.try_into().unwrap()).unwrap();

     // Sign x2
     let rands= utils::random_block(32);
     let send_sig = poksho::sign(s_ske, r_pke, &x2, &rands).unwrap();
     let mf = Mfrank{
                 msg,
                 x1: token.x1,
                 x2,
                 nonce: token.nonce,
                 mod_sig: token.mod_sig,
                 send_sig,
                 pke: token.pke,
                 randc,
                 time: token.time,
             };
     return (mf, com);
}
