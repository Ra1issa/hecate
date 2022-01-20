use hecate::{
    utils,
    types::Token,
};
use std::{
    net::TcpStream,
    io::{BufRead, BufReader},
    // time::SystemTime,
};


pub fn fetch_tokens(stream: &mut TcpStream){
    let mut reader = BufReader::new(stream);
    let tk_bytes: Token =  bincode::deserialize(reader.fill_buf().unwrap()).unwrap();
    utils::write_to_file::<Token>(tk_bytes, "token.txt");
}

fn main(){
    let address = "127.0.0.1:3000";
    let _ = match TcpStream::connect(address){
       Ok(mut stream) => {
           fetch_tokens(&mut stream);
           Ok(())
       },
       Err(e) => {
           println!("Failed to connect: {}", e);
           Err(e)
       }
    };
}
