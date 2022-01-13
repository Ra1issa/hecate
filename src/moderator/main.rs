use hecate::hecate_lib::{
    moderator,
    utils,
    types::Token,
};
fn main(){

    let id = utils::random_block(32);
    let m = moderator::setup_moderator();
    let tk = moderator::generate_token(id.clone(), m.clone());

    utils::write_to_file::<Token>(tk, "token.txt");
}
