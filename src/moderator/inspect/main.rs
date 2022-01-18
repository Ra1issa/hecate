use hecate::hecate_lib::{
    moderator,
    utils,
    types::{Moderator, Report, Trace},
};
use curve25519_dalek::ristretto::RistrettoPoint;
use std::{
    net::TcpStream,
    io::{BufRead, BufReader},
    // time::SystemTime,
};
pub fn receive_report() -> Report{
    let address = "127.0.0.1:3000";
    let report = match TcpStream::connect(address){
       Ok(stream) => {
           let mut reader = BufReader::new(stream);
           Ok(bincode::deserialize(reader.fill_buf().unwrap()).unwrap())
       },
       Err(e) => {
           println!("Failed to connect: {}", e);
           Err(e)
       }
    };
    report.unwrap()
}

pub fn verify_message(report: Report) -> Trace{
    let mut buff_pk = Vec::new();
    let moderator = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_pk);

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<RistrettoPoint>("plat_pk.txt",&mut buff_pk);

    moderator::inspect(report, moderator, plat_pk)
}

fn main(){

    let report = receive_report();
    let trace = verify_message(report);
    println!("Reported id {:?}", trace.id);
    println!("Time diff {:?}", trace.time_diff);
    println!("Message {:?}", trace.msg);
}
