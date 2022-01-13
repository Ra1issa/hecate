use crate::hecate_lib::{
    utils,
    types::Mfrank,
};
use poksho;
use libsignal_protocol::crypto;
use curve25519_dalek::ristretto::{
    RistrettoPoint,
    CompressedRistretto,
};
use sha2::{Sha256, Digest};
use std::convert::TryInto;

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
    let time = i64::from_le_bytes(mf.time_mod.try_into().unwrap());
    println!("timestamp {:?}", time);

    return true;
}
