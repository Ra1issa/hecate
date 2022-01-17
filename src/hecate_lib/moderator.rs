use crate::hecate_lib::{
    utils,
    receiver::check_message,
    types::{Token, Trace, Report},
};

use poksho;

use signal_crypto::{
    Aes256GcmEncryption,
    Aes256GcmDecryption,
};

use curve25519_dalek::{
    scalar::Scalar,
    ristretto::RistrettoPoint,
};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Moderator{
    enc_sk: Vec<u8>,
    sig_sk: Scalar,
    pub sig_pk: RistrettoPoint,
}

pub fn setup_moderator() -> Moderator{
    let (sig_sk, sig_pk) = utils::generate_keys();
    let enc_sk = utils::random_block(32);
    Moderator
    {
        enc_sk,
        sig_sk,
        sig_pk,
    }
}

pub fn generate_token
(
    id: Vec<u8>,
    m: Moderator,
) -> Token {

    let randomness = utils::random_block(32);
    let nonce = utils::random_block(12);
    let aad = "".as_bytes();

    // Generate x1
    let mut x1 = id.clone();
    let mut gcm_enc = Aes256GcmEncryption::new(&m.enc_sk, &nonce, &aad).unwrap();
    gcm_enc.encrypt(&mut x1).unwrap();
    let _authentication_tag = gcm_enc.compute_tag().unwrap();

    // Time stamp
    let dt = Utc::now();
    let time = dt.timestamp().to_le_bytes().to_vec();

    // Generate ephemeral keys
    let (ske, pke) = utils::generate_keys();

    // Compress RistrettoPt and cast it to the bytes
    let pke = pke.compress();
    let pke = pke.as_bytes().to_vec();

    // Concatenate what will be signed
    let s = [x1.clone(), nonce.clone(), pke.clone(), time.clone()].concat();

    // Sign
    let mod_sig = poksho::sign(m.sig_sk, m.sig_pk, &s, &randomness).unwrap();

    Token
    {
        x1,
        nonce,
        mod_sig,
        ske: ske.to_bytes().to_vec(),
        pke,
        time,
    }
}


pub fn inspect(
    report: Report,
    m: Moderator,
    plat_pk: RistrettoPoint,
) -> Trace {
    let aad = "".as_bytes();

    let b = check_message(report.mfrank.clone(), report.envelope.clone(), m.sig_pk, plat_pk);
    assert_eq!(b, true);

    let mut buf = report.mfrank.x1.clone();
    let mut gcm_dec = Aes256GcmDecryption::new(&m.enc_sk, &report.mfrank.nonce, &aad).unwrap();
    gcm_dec.decrypt(&mut buf).unwrap();

    Trace{
        id: buf.to_vec(),
        msg: report.mfrank.msg,
        time_diff: 0 as i64,
    }
}
