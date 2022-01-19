use crate::hecate_lib::{
    platform,
    utils,
    types::{Envelope, Platform},
};

use poksho;
use chrono::Utc;

const ENVELOPE_SIZE: usize = 32;

pub fn inject_envelope_com(ctext: &[u8]) -> Vec<u8>{
    let p = platform::setup_platform();

    let mut buff = Vec::new();
    let com = utils::read_from_file::<Vec<u8>>("commitment.txt",&mut buff);
    let env = platform::sign_com(com.clone(), p.clone());

    let e = bincode::serialize(&env).unwrap();
    println!("e size {:?}", e.len());
    [ctext, &e].concat()
}

pub fn remove_envelope_com(ctext: &[u8]) -> Vec<u8>{
    let c_len = ctext.len();
    ctext[0..c_len-ENVELOPE_SIZE].to_vec()
}
