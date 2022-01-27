use crate::{
    utils,
    types::{Mfrank, Envelope, Report},
};

use libsignal_protocol::crypto;
use sha2::{Sha256, Digest};
use std::convert::TryInto;
use ed25519_dalek::{Signature, PublicKey, Verifier};


pub fn check_authorship(
    mfrank: Mfrank,
    envelope: Envelope,
)-> Mfrank{
    for i in 0..mfrank.com.len(){
        if mfrank.com[i] != 0 {
            return mfrank;
        }
    }
    return Mfrank::new(mfrank, envelope);
}


pub fn check_message(
    mfrank: Mfrank,
    mod_pk: PublicKey,
    plat_pk: PublicKey,
)-> Report{
    let mf = mfrank.clone();

    let env =  Envelope{
        com: mf.com,
        sig: mf.plat_sig,
        time: mf.plat_time,
    };
    // Concatenate moderator token
    let s = [mf.x1.clone(), mf.nonce.clone(), mf.pke.clone(), mf.mod_time.clone()].concat();
    let e = [env.com.clone(), env.time.clone()].concat();

    // Verify Signatures
    let mod_sig = Signature::from_bytes(&mf.mod_sig).unwrap();
    let send_sig = Signature::from_bytes(&mf.send_sig).unwrap();
    let plat_sig = Signature::from_bytes(&env.sig).unwrap();

    let pke = PublicKey::from_bytes(&mfrank.pke).unwrap();
    pke.verify(&mf.x2, &send_sig).unwrap();
    mod_pk.verify(&s, &mod_sig).unwrap();
    plat_pk.verify( &e, &plat_sig).unwrap();

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
    let _time_mod = i64::from_le_bytes(mf.mod_time.try_into().unwrap());
    let _time_plat = i64::from_le_bytes(env.time.try_into().unwrap());
    // println!("timestamp diff {:?}", time_plat - time_mod);

    Report{
        mfrank,
    }
}
