extern crate poksho;

use curve25519_dalek::{
    scalar::Scalar,
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::RistrettoPoint,
};

use rand_core::OsRng;

pub fn random_block(size: u8) -> Vec<u8>{
    let mut block = Vec::new();
    for i in 0..size {
        block.push(i as u8);
    }
    return block;
}

pub fn generate_sign_keys(rng: &mut OsRng) -> (Scalar, RistrettoPoint){
    let sk: Scalar = Scalar::random(rng);
    let pk = sk * RISTRETTO_BASEPOINT_POINT;
    return (sk, pk);
}

fn main(){

    let mut rng = OsRng;
    let (sk, pk) = generate_sign_keys(&mut rng);
    let randomness = random_block(32);
    let message = random_block(100);

    let signature = poksho::sign(sk, pk, &message, &randomness).unwrap();
    poksho::verify_signature(&signature, pk, &message).unwrap();
    println!("hello");
}
