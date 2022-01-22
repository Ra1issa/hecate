use hecate::{
    receiver,
    utils,
    types::{Envelope, Mfrank, Report},
};
use std::{
    net::{
        TcpListener,
    },
    io::Write
};
use ed25519_dalek::PublicKey;

pub fn verify_message()-> Report {
    let mut buff_mfrank = Vec::new();
    let mfrank = utils::read_from_file::<Mfrank>("mfrank.txt",&mut buff_mfrank);

    let mut buff_pk = Vec::new();
    let mod_pk = utils::read_from_file::<Vec<u8>>("mod_pk.txt",&mut buff_pk);
    let mod_pk = PublicKey::from_bytes(&mod_pk).unwrap();

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<Vec<u8>>("plat_pk.txt",&mut buff_pk);
    let plat_pk = PublicKey::from_bytes(&plat_pk).unwrap();

    let mut buff_env = Vec::new();
    let envelope = utils::read_from_file::<Envelope>("envelope.txt",&mut buff_env);

    let report = receiver::check_message(mfrank.clone(), envelope.clone(), mod_pk, plat_pk);

    report
}

pub fn connect_report(report: Report){
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
    let report = verify_message();
    utils::write_to_file::<Report>(report, "report.txt");
    // connect_report(rep);
}
