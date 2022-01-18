use hecate::hecate_lib::{
    moderator,
    utils,
    types::{Moderator, Token},
};
use curve25519_dalek::ristretto::RistrettoPoint;
use std::{
    net::{
        TcpListener,
        TcpStream,
    },
    io::Write,
    time::SystemTime,
};

fn send_tokens(id: Vec<u8>, m: Moderator, stream: &mut TcpStream){
    let start = SystemTime::now();
    let tk = moderator::generate_token(id.clone(), m.clone());
    let _t = start.elapsed().unwrap();
    let tk_bytes = bincode::serialize(&tk).unwrap();

    let _ = stream.write_all(&tk_bytes);
    let _ = stream.flush();
    utils::write_to_file::<Token>(tk, "token2.txt");
}

fn main(){
    let mut buff_id = Vec::new();
    let id = utils::read_from_file::<Vec<u8>>("user_id.txt",&mut buff_id);

    let mut buff_m = Vec::new();
    let m = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_m);

    println!("User id {:?}", id);
    utils::write_to_file::<Moderator>(m.clone(), "mod_keys.txt");
    utils::write_to_file::<RistrettoPoint>(m.sig_pk, "mod_pk.txt");

    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on port 3000");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                send_tokens(id.clone(), m.clone(), &mut stream);
                return;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
