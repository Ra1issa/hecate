use hecate::{
    platform,
    utils,
    types::Platform,
};
use ed25519_dalek::Keypair;

fn main(){
    let mut rng = rand::thread_rng();
    let p = platform::setup_platform(&mut rng);
    let k = Keypair::from_bytes(&p.keypair).unwrap();
    utils::write_to_file::<Platform>(p.clone(), "plat_keys.txt");
    utils::write_to_file::<Vec<u8>>(k.public.as_bytes().to_vec(), "plat_pk.txt");
}
