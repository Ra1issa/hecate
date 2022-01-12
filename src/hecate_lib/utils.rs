use curve25519_dalek::{
    scalar::Scalar,
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::RistrettoPoint,
};
use rand::Rng;
use rand_core::OsRng;

pub fn random_block(size: u8) -> Vec<u8>{
    let mut block = Vec::new();
    for _i in 0..size {
        block.push(rand::thread_rng().gen::<u8>());
    }
    return block;
}

pub fn add_bytes(a: &[u8], b: &[u8]) -> Vec<u8>{
    let mut c = Vec::new();
    for i in 0..a.len(){
        let r = ((a[i] as u16 + b[i] as u16) % 256) as u8;
        c.push(r as u8);
    }
    return c;
}

pub fn sub_bytes(b: &[u8], c: &[u8]) -> Vec<u8>{
    let mut a = Vec::new();
    for i in 0..b.len(){
        a.push(((c[i] as i16 - b[i] as i16) % 256) as u8);
    }
    return a;
}

pub fn generate_keys(rng: &mut OsRng) -> (Scalar, RistrettoPoint){
    let sk: Scalar = Scalar::random(rng);
    let pk = sk * RISTRETTO_BASEPOINT_POINT;
    return (sk, pk);
}
