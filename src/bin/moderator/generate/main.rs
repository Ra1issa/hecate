use hecate::{
    moderator,
    utils,
    types::{Moderator, Token},
};

use std::{
    net::{
        TcpListener,
    },
    io::Write,
};
use rand::rngs::OsRng;

pub fn connect_send(id: Vec<u8>, m: Moderator){
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on port 3000");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let mut rng = rand::thread_rng();
                let tk = moderator::generate_token(id.clone(), m.clone(),&mut rng);
                let tk_bytes = bincode::serialize(&tk).unwrap();
                let _ = stream.write_all(&tk_bytes);
                let _ = stream.flush();
                return;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn main(){
    let mut buff_id = Vec::new();
    let id = utils::read_from_file::<Vec<u8>>("user_id.txt",&mut buff_id);
    let mut buff_m = Vec::new();
    let m = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_m);

    // let _ = connect_send(id, m);
    let mut rng = OsRng{};
    let tk = moderator::generate_token(id.clone(), m.clone(),&mut rng);
    utils::write_to_file::<Token>(tk, "token.txt");
}
