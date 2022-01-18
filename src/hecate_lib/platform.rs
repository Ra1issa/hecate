use crate::hecate_lib::{
    utils,
    types::{Envelope, Platform},
};

use poksho;
use chrono::Utc;

pub fn setup_platform() -> Platform{
    let (sig_sk, sig_pk) = utils::generate_keys();
    Platform
    {
        sig_sk,
        sig_pk,
    }
}

pub fn sign_com
(
    com: Vec<u8>,
    p: Platform,
) -> Envelope{

    let randomness = utils::random_block(32);
    // Time stamp
    let dt = Utc::now();
    let time = dt.timestamp().to_le_bytes().to_vec();

    // Concatenate what will be signed
    let s = [com.clone(), time.clone()].concat();

    // Sign
    let sig = poksho::sign(p.sig_sk, p.sig_pk, &s, &randomness).unwrap();

    Envelope
    {
        com,
        sig,
        time,
    }
}
