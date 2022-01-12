use hecate::hecate_lib::{
    user,
    moderator,
    utils,
};
use bincode;

fn main(){

    let msg = "hello".to_string();
    let id = utils::random_block(32);

    let m = moderator::setup_moderator();
    let tk = moderator::generate_token(id.clone(), m.clone());
    let mf = user::generate_frank(msg, tk);

    let encoded: Vec<u8> = bincode::serialize(&mf).unwrap();
    let decoded: user::Mfrank = bincode::deserialize(&encoded).unwrap();
    println!("deserialize {:?}", decoded);

    let _b = user::check_message(mf.clone(), m.sig_pk);
    let r = moderator::inspect(mf, m);
    assert_eq!(id, r.id)
}
