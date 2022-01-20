use crate::{
    utils,
    types::{Mfrank, Envelope, Report},
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
    mfrank: Mfrank,
    envelope: Envelope,
    mod_pk: RistrettoPoint,
    plat_pk: RistrettoPoint,
)-> Report{
    let mf = mfrank.clone();
    let env = envelope.clone();

    // Concatenate moderator token
    let s = [mf.x1.clone(), mf.nonce.clone(), mf.pke.clone(), mf.time.clone()].concat();
    let e = [env.com.clone(), env.time.clone()].concat();

    // Turn pke back to RistrettoPt
    let r_pke = CompressedRistretto(mf.pke.try_into().unwrap());
    let r_pke = r_pke.decompress().unwrap();

    // Verify Signatures
    poksho::verify_signature(&mf.send_sig, r_pke, &mf.x2).unwrap();
    poksho::verify_signature(&mf.mod_sig, mod_pk, &s).unwrap();
    poksho::verify_signature(&env.sig, plat_pk, &e).unwrap();

    // Verify Commitment
    let x = [mf.x1.clone(), mf.x2.clone()].concat();
    let com = crypto::hmac_sha256(&mf.randc, &x).to_vec();
    assert_eq!(com, env.com);

    // Verify Hash
    let h = utils::sub_bytes(&mf.x1, &mf.x2);
    let mut hasher = Sha256::new();
    hasher.update(mf.msg);
    let hash = hasher.finalize();
    assert_eq!(h, hash.to_vec());

    // Verify Time
    let _time_mod = i64::from_le_bytes(mf.time.try_into().unwrap());
    let _time_plat = i64::from_le_bytes(env.time.try_into().unwrap());
    // println!("timestamp diff {:?}", time_plat - time_mod);

    Report{
        mfrank,
        envelope,
    }
}
