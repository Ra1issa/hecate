use crate::hecate_lib::{
    utils,
    moderator::Token,

};
use poksho;
use libsignal_protocol::crypto;
use curve25519_dalek::ristretto::{
    RistrettoPoint,
    CompressedRistretto,
};
use curve25519_dalek::scalar::Scalar;
use sha2::{Sha256, Digest};
use std::{
    convert::TryInto,
    str::from_utf8,
};
use chrono::DateTime;

#[derive(Clone)]
pub struct Mfrank{
    pub msg: String,
    pub x1: Vec<u8>,
    pub x2: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mod_sig: Vec<u8>,
    pub send_sig: Vec<u8>,
    pub pke: Vec<u8>,
    pub com: Vec<u8>,
    pub randc: Vec<u8>,
    pub time_mod: Vec<u8>,
}

pub fn generate_frank(
    msg: String,
    token: Token,
)-> Mfrank{
     // Hash message
     let mut hasher = Sha256::new();
     hasher.update(msg.clone());
     let hash = hasher.finalize();

     // Additively split x1 and H(m) into x2
     let x2 = utils::add_bytes(&token.x1, &hash);

     // Commit x1 and x2
     let x = [token.x1.clone(), x2.clone()].concat();
     let randc = utils::random_block(32);
     let com = crypto::hmac_sha256(&randc, &x).unwrap().to_vec();

     // Turn pke/ske back to RistrettoPt and scalar
     let r_pke = CompressedRistretto(token.pke.clone().try_into().unwrap());
     let r_pke = r_pke.decompress().unwrap();

     let s_ske = Scalar::from_canonical_bytes(token.ske.try_into().unwrap()).unwrap();

     // Sign x2
     let rands= utils::random_block(32);
     let send_sig = poksho::sign(s_ske, r_pke, &x2, &rands).unwrap();
     Mfrank
     {
         msg,
         x1: token.x1,
         x2,
         nonce: token.nonce,
         mod_sig: token.mod_sig,
         send_sig,
         pke: token.pke,
         com,
         randc,
         time_mod: token.time,
     }
}

pub fn check_message(
    mf: Mfrank,
    mod_pk: RistrettoPoint
)-> bool{

    // Concatenate moderator token
    let s = [mf.x1.clone(), mf.nonce.clone(), mf.pke.clone(), mf.time_mod.clone()].concat();

    // Turn pke back to RistrettoPt
    let r_pke = CompressedRistretto(mf.pke.try_into().unwrap());
    let r_pke = r_pke.decompress().unwrap();

    // Verify Signatures
    poksho::verify_signature(&mf.send_sig, r_pke, &mf.x2).unwrap();
    poksho::verify_signature(&mf.mod_sig, mod_pk, &s).unwrap();

    // Verify Commitment
    let x = [mf.x1.clone(), mf.x2.clone()].concat();
    let com = crypto::hmac_sha256(&mf.randc, &x).unwrap().to_vec();
    assert_eq!(com, mf.com);

    // Verify Hash
    let h = utils::sub_bytes(&mf.x1, &mf.x2);
    let mut hasher = Sha256::new();
    hasher.update(mf.msg);
    let hash = hasher.finalize();
    assert_eq!(h, hash.to_vec());

    // Verify Time
    let time = from_utf8(&mf.time_mod).unwrap();
    let time_mod = DateTime::parse_from_rfc2822(time).unwrap();
    println!("time {:?}", time_mod);

    return true;
}
