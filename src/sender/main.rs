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

    let (mf, com) = sender::generate_frank(msg.to_string(), tk);
    utils::write_to_file::<Mfrank>(mf, "mfrank.txt");
    utils::write_to_file::<Vec<u8>>(com, "commitment.txt");
}
