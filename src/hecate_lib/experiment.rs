use crate::hecate_lib::{
    platform,
    utils,
};

const ENVELOPE_SIZE: usize = 128;

pub fn inject_envelope_com(ctext: &[u8]) -> Vec<u8>{
    let p = platform::setup_platform();

    let mut buff = Vec::new();
    let com = utils::read_from_file::<Vec<u8>>("commitment.txt",&mut buff);
    let env = platform::sign_com(com.clone(), p.clone());

    let mut e = bincode::serialize(&env).unwrap().to_vec();
    let pad_len = ENVELOPE_SIZE % 16;
    for _ in 0.. pad_len{
        e.push(0 as u8);
    }
    println!("Adding envelope");
    [ctext, &e].concat()
}

pub fn remove_envelope_com(ctext: &[u8]) -> Vec<u8>{
    println!("Removing envelope");
    let c_len = ctext.len();
    let pad_len = ENVELOPE_SIZE % 16;
    ctext[0..c_len-(ENVELOPE_SIZE+pad_len)].to_vec()
}
