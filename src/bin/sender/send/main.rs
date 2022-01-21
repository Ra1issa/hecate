use hecate::{
    sender,
    utils,
    types::{Mfrank, Token},
};
use std::fs;

fn main(){
    // let args: Vec<String> = env::args().collect();
    let mut path = utils::get_data_path();
    path.push("msgs/message1.txt");
    println!("path {:?}", path);
    let msg: String= fs::read_to_string(path).unwrap();

    let mut buff = Vec::new();
    let tk = utils::read_from_file::<Token>("token.txt",&mut buff);

    let (mf, com) = sender::generate_frank(msg.to_string(), tk);
    utils::write_to_file::<Mfrank>(mf, "mfrank.txt");
    utils::write_to_file::<Vec<u8>>(com, "commitment.txt");
}