use hecate::{
    sender,
    utils,
    types::{Mfrank, Token},
};
use std::fs;

fn main(){
    let mut path = utils::get_data_path();
    path.push("msgs/msg1.txt");
    println!("path {:?}", path);
    let msg: String= fs::read_to_string(path).unwrap();

    let mut buff = Vec::new();
    let tk = utils::read_from_file::<Token>("token.txt",&mut buff);

    let mut rng = rand::thread_rng();
    let (mf, com) = sender::generate_frank(msg, tk, &mut rng);
    utils::write_to_file::<Mfrank>(mf, "mfrank.txt");
    utils::write_to_file::<Vec<u8>>(com, "commitment.txt");
}
