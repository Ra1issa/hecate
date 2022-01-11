mod test_keys;

use poksho;
use libsignal_protocol::crypto;
use signal_crypto::{
    Aes256GcmEncryption,
    Aes256GcmDecryption};

use curve25519_dalek::{
    scalar::Scalar,
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::RistrettoPoint,
};
use hex;
use rand::Rng;
use rand_core::OsRng;
use chrono;
use std::{str,cmp::max};
use sha2::{Sha256, Digest};

pub struct Moderator{
    enc_sk: Vec<u8>,
    sig_sk: Scalar,
    sig_pk: RistrettoPoint,
}

pub fn random_block(size: u8) -> Vec<u8>{
    let mut block = Vec::new();
    for i in 0..size {
        block.push(rand::thread_rng().gen::<u8>());
    }
    return block;
}

// pub fn add_bytes(a: &[u8], b: &[u8]) -> Vec<u8>{
//     let max_len = max(a.len(), b.len());
//
//
// }

pub fn generate_keys(rng: &mut OsRng) -> (Scalar, RistrettoPoint){
    let sk: Scalar = Scalar::random(rng);
    let pk = sk * RISTRETTO_BASEPOINT_POINT;
    return (sk, pk);
}

pub fn setup_moderator(rng: &mut OsRng) -> Moderator{
    let (sig_sk, sig_pk) = generate_keys(rng);
    let enc_sk = random_block(32);
    Moderator{
        enc_sk,
        sig_sk,
        sig_pk,
    }
}

pub fn generate_token(
    id: String,
    m: Moderator,
    nonce: &[u8],
    aad: &[u8],
    rng: &mut OsRng,
)->(Vec<u8>, Vec<u8>, Scalar, RistrettoPoint){

    let randomness = random_block(32);

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
    let signature = poksho::sign(m.sig_sk, m.sig_pk, &x1, &randomness).unwrap();
    return (x1, signature, ske, pke)
}

fn generate_frank(
    msg: String,
    x1: &[u8],
    mod_sig: &[u8],
    ske: Scalar,
    pke: RistrettoPoint
 )-> (){
     // Hash message
     let mut hasher = Sha256::new();
     hasher.update(msg);
     let hash = hasher.finalize();

     // // Additively split x1 and H(m) into x2
     // let x2 = add_bytes(x1, hash);
}

fn main(){

    let mut rng = OsRng;
    let nonce = random_block(12);
    let aad = random_block(16);
    let msg = "hello".to_string();

    let m = setup_moderator(&mut rng);
    let (x1, mod_sig, ske, pke) = generate_token("01".to_string(), m, &nonce, &aad, &mut rng);
    generate_frank(msg, &x1, &mod_sig, ske, pke);

    // let (sk, pk) = generate_sign_keys(&mut rng);
    // let randomness = random_block(32);
    // let message = "32";
    // let pt = hex::decode(message.clone()).expect("valid hex");
    // let message = message.as_bytes();
    //
    // let signature = poksho::sign(sk, pk, &message, &randomness).unwrap();
    // let a = poksho::verify_signature(&signature, pk, &message).unwrap();
    //
    // let hm = crypto::hmac_sha256(&randomness, &message).unwrap();
    //
    // let (key, nonce, aad) = test_keys::generate_test_enc_keys();
    // let mut buf = pt.clone();
    // println!("original message {:?}", buf);
    // println!("key size {:?}", key.len());
    // println!("nonce size {:?}", &nonce.len());
    // println!("aad size {:?}", aad.len());
    //
    //
    // println!("enc message {:?}", buf);
    //
    // let mut gcm_dec = Aes256GcmDecryption::new(&key, &nonce, &aad).unwrap();
    // gcm_dec.decrypt(&mut buf).unwrap();
    //
    // println!("dec message {:?}", buf);

}
