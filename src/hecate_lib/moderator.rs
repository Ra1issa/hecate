use crate::hecate_lib::{
    utils,
    receiver::check_message,
    types::{Mfrank, Token, Report},
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

#[derive(Clone, Debug)]
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
    let time = Utc::now().to_rfc2822();
    let time = time.to_string();
    let time = time.as_bytes().to_vec();

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


pub fn inspect(mf: Mfrank, m: Moderator) -> Report{
    let aad = "".as_bytes();

    let b = check_message(mf.clone(), m.sig_pk);
    assert_eq!(b, true);

    let mut buf = mf.x1.clone();
    let mut gcm_dec = Aes256GcmDecryption::new(&m.enc_sk, &mf.nonce, &aad).unwrap();
    gcm_dec.decrypt(&mut buf).unwrap();

    Report{
        id: buf.to_vec(),
        msg: mf.msg,
        time: "".to_string(),
    }
}
