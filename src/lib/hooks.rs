use crate::{
    platform,
    moderator,
    receiver,
    sender,
    utils,
    types::{Platform, Moderator, Mfrank, Envelope},
};
use curve25519_dalek::ristretto::RistrettoPoint;

const ENVELOPE_SIZE: usize = 128;

pub fn inject_mfrank(ptext: String) -> Vec<u8>{
    println!("Adding Mfrank");

    let id = utils::random_block(32);
    let mut buff_m = Vec::new();
    let m = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_m);

    let tk = moderator::generate_token(id.clone(), m.clone());
    let (mf, com) = sender::generate_frank(ptext, tk);
    let mfrank = bincode::serialize(&mf).unwrap().to_vec();
    utils::write_to_file::<Vec<u8>>(com, "commitment.txt");

    mfrank
}

pub fn inject_envelope_com(ctext: &[u8]) -> Vec<u8>{
    println!("Adding envelope");
    let mut buff_p = Vec::new();
    let p = utils::read_from_file::<Platform>("plat_keys.txt",&mut buff_p);

    let mut buff = Vec::new();
    let com = utils::read_from_file::<Vec<u8>>("commitment.txt",&mut buff);
    let env = platform::sign_com(com.clone(), p.clone());

    let e = bincode::serialize(&env).unwrap().to_vec();
    println!("e size {:?}", e.len());
    println!("ctext size {:?}", ctext.len());
    [ctext, &e].concat()
}

pub fn remove_envelope_com(ctext: &[u8]) -> Vec<u8>{
    println!("Removing envelope");
    println!("ctext {:?}", ctext.len());
    let c_len = ctext.len();
    let mut env_bytes = ctext[c_len-ENVELOPE_SIZE..c_len].to_vec();
    let envelope: Envelope = bincode::deserialize(&env_bytes).unwrap();
    utils::write_to_file::<Envelope>(envelope, "envelope.txt");
    ctext[0..c_len-ENVELOPE_SIZE].to_vec()
}

pub fn remove_mfrank(mfrank_bytes: &[u8]) -> String{
    println!("Removing Mfrank");
    println!("Mfrank size {:?}", mfrank_bytes.len());
    let mut buff_pk = Vec::new();
    let mod_pk = utils::read_from_file::<RistrettoPoint>("mod_pk.txt",&mut buff_pk);

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<RistrettoPoint>("plat_pk.txt",&mut buff_pk);

    let mut buff_env = Vec::new();
    let envelope = utils::read_from_file::<Envelope>("envelope.txt",&mut buff_env);

    let mfrank : Mfrank = bincode::deserialize(mfrank_bytes).unwrap();
    let m = mfrank.msg.clone();
    let _ = receiver::check_message(mfrank, envelope, mod_pk, plat_pk);
    m
}
