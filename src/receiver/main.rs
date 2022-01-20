use hecate::{
    receiver,
    utils,
    types::{Envelope, Mfrank, Report},
};
use curve25519_dalek::ristretto::RistrettoPoint;
use std::{
    net::{
        TcpListener,
    },
    io::Write
};

pub fn verify_message()-> Report {
    let mut buff_mfrank = Vec::new();
    let mfrank = utils::read_from_file::<Mfrank>("mfrank.txt",&mut buff_mfrank);

    let mut buff_pk = Vec::new();
    let mod_pk = utils::read_from_file::<RistrettoPoint>("mod_pk.txt",&mut buff_pk);

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<RistrettoPoint>("plat_pk.txt",&mut buff_pk);

    let mut buff_env = Vec::new();
    let envelope = utils::read_from_file::<Envelope>("envelope.txt",&mut buff_env);

    let report = receiver::check_message(mfrank.clone(), envelope.clone(), mod_pk, plat_pk);
    report
}

pub fn report(report: Report){
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on port 3000");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let report_bytes = bincode::serialize(&report).unwrap();
                let _ = stream.write_all(&report_bytes);
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
    let rep = verify_message();
    report(rep);
}
