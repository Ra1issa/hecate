use hecate::hecate_lib::{
    sender,
    utils,
    types::{Mfrank, Token},
};
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let msg = &args[1];

    let mut buff = Vec::new();
    let tk = utils::read_from_file::<Token>("token.txt",&mut buff);

    let mf = sender::generate_frank(msg.to_string(), tk);
    utils::write_to_file_json::<Mfrank>(mf, "mfrank.txt");

    // let msg = "hello".to_string();
    // let id = utils::random_block(32);
    //
    // let m = moderator::setup_moderator();
    // let tk = moderator::generate_token(id.clone(), m.clone());
    //
    // let encoded: Vec<u8> = bincode::serialize(&mf).unwrap();
    // let decoded: sender::Mfrank = bincode::deserialize(&encoded).unwrap();
    // println!("deserialize {:?}", decoded);
    //
    // let _b = sender::check_message(mf.clone(), m.sig_pk);
    // let r = moderator::inspect(mf, m);
    // assert_eq!(id, r.id)
}
