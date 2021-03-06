use hecate::{
    moderator,
    utils,
    types::{Moderator, Report, Trace},
};

use ed25519_dalek::PublicKey;

pub fn verify_message(report: Report) -> Trace{
    let mut buff_pk = Vec::new();
    let moderator = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_pk);

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<Vec<u8>>("plat_pk.txt",&mut buff_pk);
    let plat_pk = PublicKey::from_bytes(&plat_pk).unwrap();

    moderator::inspect(report, moderator, plat_pk)
}

fn main(){

    let mut buff_report = Vec::new();
    let report = utils::read_from_file::<Report>("report.txt",&mut buff_report);
    // let report = receive_report();
    let trace = verify_message(report);
    println!("Reported id {:?}", trace.id);
    println!("Time diff {:?}", trace.time_diff);
    println!("Message {:?}", trace.msg);
}
