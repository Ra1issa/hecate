use hecate::hecate_lib::{
    platform,
    utils,
    types::Envelope,
};
use curve25519_dalek::ristretto::RistrettoPoint;

fn main(){

    let p = platform::setup_platform();
    utils::write_to_file::<platform::Platform>(p.clone(), "plat_keys.txt");
    utils::write_to_file::<RistrettoPoint>(p.sig_pk, "plat_pk.txt");

    let mut buff = Vec::new();
    let com = utils::read_from_file::<Vec<u8>>("commitment.txt",&mut buff);

    let env = platform::sign_com(com.clone(), p.clone());
    utils::write_to_file::<Envelope>(env, "envelope.txt");
}
