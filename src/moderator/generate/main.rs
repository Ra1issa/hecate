use hecate::hecate_lib::{
    moderator,
    utils,
    types::Token,
};
use curve25519_dalek::ristretto::RistrettoPoint;
use std::{
    net::{
        TcpListener,
        TcpStream,
    },
    io::Write
};

fn send_tokens(id: Vec<u8>, m: moderator::Moderator, stream: &mut TcpStream){
    let tk = moderator::generate_token(id.clone(), m.clone());
    let tk_bytes = bincode::serialize(&tk).unwrap();

    let _ = stream.write_all(&tk_bytes);
    let _ = stream.flush();
    utils::write_to_file::<Token>(tk, "token2.txt");
}

fn main(){
    let id = utils::random_block(32);
    let m = moderator::setup_moderator();

    println!("User id {:?}", id);
    utils::write_to_file::<moderator::Moderator>(m.clone(), "mod_keys.txt");
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
