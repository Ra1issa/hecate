use hecate::{
    platform,
    utils,
    types::Platform,
};
use curve25519_dalek::ristretto::RistrettoPoint;

fn main(){
    let p = platform::setup_platform();
    utils::write_to_file::<Platform>(p.clone(), "plat_keys.txt");
    utils::write_to_file::<RistrettoPoint>(p.sig_pk, "plat_pk.txt");
}
