use crate::{
    utils,
    receiver::check_message,
    types::{Moderator, Token, Trace, Report},
};

use signal_crypto::{
    Aes256GcmEncryption,
    Aes256GcmDecryption,
};

use ed25519_dalek::{
    Keypair,
    PublicKey,
    Signer,
    ed25519::signature::Signature
};

use chrono::Utc;
use std::convert::TryInto;
use rand::{CryptoRng, Rng};

pub fn setup_moderator
<R: CryptoRng + Rng>
(
    rng: &mut R
)
-> Moderator {
    let keypair: Keypair = Keypair::generate(rng);
    let mut enc_sk = vec![0 as u8; 32];
    rng.fill_bytes(&mut enc_sk);
    Moderator
    {
        enc_sk,
        keypair: keypair.to_bytes().to_vec(),
    }
}

pub fn generate_token
<R: CryptoRng + Rng>
(
    id: Vec<u8>,
    m: Moderator,
    rng:  &mut R
)
-> Token {
    let nonce = utils::random_block(12, rng);
    let aad = "".as_bytes();

    // Generate x1
    let mut x1 = id.clone();
    let mut gcm_enc = Aes256GcmEncryption::new(&m.enc_sk, &nonce, &aad).unwrap();
    gcm_enc.encrypt(&mut x1).unwrap();
    let _authentication_tag = gcm_enc.compute_tag().unwrap();

    // Time stamp
    let dt = Utc::now();
    let time = dt.timestamp().to_le_bytes().to_vec();

    // Compress RistrettoPt and cast it to the bytes
    let key_eph =  Keypair::generate(rng);
    let pke = key_eph.public.as_bytes().to_vec();
    // Concatenate what will be signed
    let s = [x1.clone(), nonce.clone(), pke, time.clone()].concat();

    // Sign
    let mod_keys = Keypair::from_bytes(&m.keypair).unwrap();
    let mod_sig = mod_keys.sign(&s);

    Token
    {
        x1,
        nonce,
        mod_sig: mod_sig.as_bytes().to_vec(),
        key_eph: key_eph.to_bytes().to_vec(),
        time,
    }
}

pub fn generate_batch
<R: CryptoRng + Rng>
(
    batch_size: usize,
    id: Vec<u8>,
    m: Moderator,
    rng:  &mut R
)-> Vec<Token>{
    let mut batch: Vec<Token> = Vec::new();
    for _i in 0..batch_size{
        batch.push(generate_token(id.clone(), m.clone(), rng));
    }
    batch
}


pub fn inspect(
    report: Report,
    m: Moderator,
    plat_pk: PublicKey,
) -> Trace {
    let aad = "".as_bytes();
    let mod_pk = (Keypair::from_bytes(&m.keypair).unwrap()).public;
    let _ = check_message(report.mfrank.clone(), report.envelope.clone(), mod_pk, plat_pk);

    let mut buf = report.mfrank.x1;
    let mut gcm_dec = Aes256GcmDecryption::new(&m.enc_sk, &report.mfrank.nonce, &aad).unwrap();
    gcm_dec.decrypt(&mut buf).unwrap();
    let time_mod = i64::from_le_bytes(report.mfrank.time.try_into().unwrap());
    let time_plat = i64::from_le_bytes(report.envelope.time.try_into().unwrap());

    Trace{
        id: buf.to_vec(),
        msg: report.mfrank.msg,
        time_diff:time_plat-time_mod,
    }
}
