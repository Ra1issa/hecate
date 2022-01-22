use crate::{
    types::{Envelope, Platform},
};
use ed25519_dalek::{
    Keypair,
    Signer,
    ed25519::signature::Signature,
};

use chrono::Utc;
use rand::{CryptoRng, Rng};

pub fn setup_platform
<R: CryptoRng + Rng>
(
    rng: &mut R
) -> Platform{
    let keypair: Keypair = Keypair::generate(rng);
    Platform
    {
        keypair: keypair.to_bytes().to_vec(),
    }
}

pub fn sign_com
(
    com: Vec<u8>,
    p: Platform,
) -> Envelope{
    // Time stamp
    let dt = Utc::now();
    let time = dt.timestamp().to_le_bytes().to_vec();

    // Concatenate what will be signed
    let s = [com.clone(), time.clone()].concat();

    // Sign
    let pk = Keypair::from_bytes(&p.keypair).unwrap();
    let sig = pk.sign(&s).as_bytes().to_vec();

    Envelope
    {
        com,
        sig,
        time,
    }
}
