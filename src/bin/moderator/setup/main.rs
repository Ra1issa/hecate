use hecate::{
    utils,
    moderator,
    types:: Moderator,
};
use curve25519_dalek::ristretto::RistrettoPoint;

fn main(){
    let id = utils::random_block(32);
    let m = moderator::setup_moderator();
    println!("moderator {:?}", m);
    utils::write_to_file::<Vec<u8>>(id, "user_id.txt");
    utils::write_to_file::<RistrettoPoint>(m.sig_pk.clone(), "mod_pk.txt");
    utils::write_to_file::<Moderator>(m, "mod_keys.txt");
}
