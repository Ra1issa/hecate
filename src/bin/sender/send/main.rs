use hecate::{
    sender,
    utils,
    types::{Mfrank, Token},
};
use std::fs;
use rand::rngs::OsRng;

fn main(){
    let mut path = utils::get_data_path();
    path.push("msgs/msg11.txt");
    let msg: String= fs::read_to_string(path).unwrap();

    let mut buff = Vec::new();
    let tk = utils::read_from_file::<Token>("token.txt",&mut buff);

    let mut rng = OsRng{};
    let (mf, com) = sender::generate_frank(msg.clone(), tk.clone(), &mut rng);

    utils::write_to_file::<Mfrank>(mf, "mfrank.txt");
    utils::write_to_file::<Vec<u8>>(com, "commitment.txt");
}
