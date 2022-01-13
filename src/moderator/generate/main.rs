use hecate::hecate_lib::{
    moderator,
    utils,
    types::Token,
};
use curve25519_dalek::ristretto::RistrettoPoint;

fn main(){

    let id = utils::random_block(32);
    let m = moderator::setup_moderator();

    println!("User id {:?}", id);
    utils::write_to_file::<moderator::Moderator>(m.clone(), "mod_keys.txt");
    utils::write_to_file::<RistrettoPoint>(m.sig_pk, "mod_sig_pk.txt");
    let tk = moderator::generate_token(id.clone(), m.clone());

    utils::write_to_file::<Token>(tk, "token.txt");
}
