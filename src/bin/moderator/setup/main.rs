use hecate::{
    utils,
    moderator,
    types:: Moderator,
};

fn main(){
    let id = utils::random_block(32);
    let m = moderator::setup_moderator();

    utils::write_to_file::<Vec<u8>>(id, "user_id.txt");
    utils::write_to_file::<Moderator>(m.clone(), "mod_keys.txt");
}
