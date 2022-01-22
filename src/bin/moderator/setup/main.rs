use hecate::{
    utils,
    moderator,
    types:: Moderator,
};
use ed25519_dalek::Keypair;

fn main(){
    let mut rng = rand::thread_rng();

    let id = utils::random_block(32, &mut rng);
    let m = moderator::setup_moderator(&mut rng);
    let k = Keypair::from_bytes(&m.keypair).unwrap();

    println!("moderator {:?}", m);
    utils::write_to_file::<Vec<u8>>(id, "user_id.txt");
    utils::write_to_file::<Vec<u8>>(k.public.as_bytes().to_vec(), "mod_pk.txt");
    utils::write_to_file::<Moderator>(m, "mod_keys.txt");
}
