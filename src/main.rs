mod test_keys;

use poksho;
use libsignal_protocol::crypto;
use signal_crypto::{
    Aes256GcmEncryption,
    Aes256GcmDecryption,
};

use curve25519_dalek::{
    scalar::Scalar,
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::RistrettoPoint,
};
use hex;
use rand::Rng;
use rand_core::OsRng;
use chrono;
use std::{
    str,
    cmp::max,
    collections::HashMap,
};
use sha2::{Sha256, Digest};

pub struct Moderator{
    enc_sk: Vec<u8>,
    sig_sk: Scalar,
    sig_pk: RistrettoPoint,
    state: HashMap<String, Vec<u8>>,
}
#[derive(Clone)]
pub struct Mfrank{
    msg: String,
    x1: Vec<u8>,
    x2: Vec<u8>,
    mod_sig: Vec<u8>,
    send_sig: Vec<u8>,
    pke: RistrettoPoint,
    com: Vec<u8>,
    randc: Vec<u8>,
}

pub struct Token{
    x1: Vec<u8>,
    mod_sig: Vec<u8>,
    ske: Scalar,
    pke: RistrettoPoint,
}

pub struct Report{
    id: String,
    msg: String,
    time: String,
}

pub fn random_block(size: u8) -> Vec<u8>{
    let mut block = Vec::new();
    for i in 0..size {
        block.push(rand::thread_rng().gen::<u8>());
    }
    return block;
}

pub fn add_bytes(a: &[u8], b: &[u8]) -> Vec<u8>{
    let mut c = Vec::new();
    for i in 0..a.len(){
        c.push(a[i] + b[i])
    }
    return c;
}

pub fn sub_bytes(b: &[u8], c: &[u8]) -> Vec<u8>{
    let mut a = Vec::new();
    for i in 0..a.len(){
        a.push(b[i] - c[i])
    }
    return a;
}

pub fn generate_keys(rng: &mut OsRng) -> (Scalar, RistrettoPoint){
    let sk: Scalar = Scalar::random(rng);
    let pk = sk * RISTRETTO_BASEPOINT_POINT;
    return (sk, pk);
}

pub fn setup_moderator(rng: &mut OsRng) -> Moderator{
    let (sig_sk, sig_pk) = generate_keys(rng);
    let enc_sk = random_block(32);
    Moderator
    {
        enc_sk,
        sig_sk,
        sig_pk,
        state: HashMap::new(),
    }
}

pub fn generate_token
(
    id: String,
    mut m: Moderator,
    rng: &mut OsRng,
) -> Token {

    let randomness = random_block(32);
    let nonce = random_block(12);
    let aad = "".as_bytes();

    // Generate x1
    let pt = hex::decode(id.clone()).expect("valid hex");
    let mut x1 = pt.clone();
    let mut gcm_enc = Aes256GcmEncryption::new(&m.enc_sk, &nonce, &aad).unwrap();
    gcm_enc.encrypt(&mut x1).unwrap();
    let authentication_tag = gcm_enc.compute_tag().unwrap();

    // Time stamp
    let time = chrono::offset::Utc::now();
    println!("time {:?}", time);

    // Generate ephemeral keys
    let (ske, pke) = generate_keys(rng);

    // Sign
    let mod_sig = poksho::sign(m.sig_sk, m.sig_pk, &x1, &randomness).unwrap();

    // Store nonce
    let str_x1 = format!("{:?}", &x1);
    m.state.insert(str_x1, nonce);

    Token
    {
        x1,
        mod_sig,
        ske,
        pke,
    }
}

fn generate_frank(
    msg: String,
    x1: Vec<u8>,
    mod_sig: Vec<u8>,
    ske: Scalar,
    pke: RistrettoPoint
)-> Mfrank{
     // Hash message
     let mut hasher = Sha256::new();
     hasher.update(msg.clone());
     let hash = hasher.finalize();

     // Additively split x1 and H(m) into x2
     let x2 = add_bytes(&x1, &hash);

     // Commit x1 and x2
     let x = [x1.clone(), x2.clone()].concat();
     let randc = random_block(32);
     let com = crypto::hmac_sha256(&randc, &x).unwrap().to_vec();

     // Sign x2
     let rands= random_block(32);
     let send_sig = poksho::sign(ske, pke, &x2, &rands).unwrap();
     Mfrank
     {
         msg,
         x1,
         x2,
         mod_sig,
         send_sig,
         pke,
         com,
         randc,
     }
}

fn check_message(mf: Mfrank, mod_pk: RistrettoPoint)-> bool{
    // Verify Signatures
    let ver_send = poksho::verify_signature(&mf.send_sig, mf.pke, &mf.x2).unwrap();
    let ver_mod = poksho::verify_signature(&mf.mod_sig, mod_pk, &mf.x1).unwrap();
    // assert_eq!(ver_send, ());
    // assert_eq!(ver_mod, ());

    // Verify Commitment
    let x = [mf.x1.clone(), mf.x2.clone()].concat();
    let com = crypto::hmac_sha256(&mf.randc, &x).unwrap().to_vec();
    assert_eq!(com, mf.com);

    // Verify Hash
    let h = sub_bytes(&mf.x1, &mf.x2);
    let mut hasher = Sha256::new();
    hasher.update(mf.msg);
    let hash = hasher.finalize();
    assert_eq!(h, hash.to_vec());

    // Verify Time

    return true;
}

fn inspect(mf: Mfrank, m: Moderator) -> Report{
    let aad = "".as_bytes();
    let str_x1 = format!("{:?}", &mf.x1);
    let nonce = m.state.get(&str_x1).unwrap();

    let b = check_message(mf.clone(), m.sig_pk);
    assert_eq!(b, true);

    let mut buf = mf.x1.clone();
    let mut gcm_dec = Aes256GcmDecryption::new(&m.enc_sk, &nonce, &aad).unwrap();
    gcm_dec.decrypt(&mut buf).unwrap();

    let id = str::from_utf8(&buf).unwrap();

    Report{
        id: id.to_string(),
        msg: mf.msg,
        time: "".to_string(),
    }
}


fn main(){

    let mut rng = OsRng;
    let msg = "hello".to_string();
    let id = "01".to_string();

    let m = setup_moderator(&mut rng);
    let tk = generate_token(id, m, &mut rng);
    let mf = generate_frank(msg, tk.x1, tk.mod_sig, tk.ske, tk.pke);



}
