use crate::{
    platform,
    moderator,
    sender,
    utils,
    types::{Mfrank, EnvelopeExtension},
};

const ENVELOPE_SIZE: usize = 112;


pub fn inject_envelope_com(ctext: &[u8]) -> Vec<u8>{
    println!("Adding envelope");
    let p = platform::setup_platform();

    let mut buff = Vec::new();
    let com = utils::read_from_file::<Vec<u8>>("commitment.txt",&mut buff);
    let env = platform::sign_com(com.clone(), p.clone());
    let env_test = EnvelopeExtension{com: env.com.clone(), sig: env.sig.clone()};
    let e = bincode::serialize(&env_test).unwrap().to_vec();
    [ctext, &e].concat()
}


pub fn remove_envelope_com(ctext: &[u8]) -> Vec<u8>{
    println!("Removing envelope");

    let c_len = ctext.len();
    let pad_len = ENVELOPE_SIZE % 16;
    ctext[0..c_len-(ENVELOPE_SIZE+pad_len)].to_vec()
}

pub fn inject_mfrank(ptext: String) -> Vec<u8>{
    println!("Adding Mfrank");

    let id = utils::random_block(32);
    let m = moderator::setup_moderator();

    let tk = moderator::generate_token(id.clone(), m.clone());
    let (mf, com) = sender::generate_frank(ptext, tk);
    let mfrank = bincode::serialize(&mf).unwrap().to_vec();
    utils::write_to_file::<Vec<u8>>(com, "commitment.txt");

    mfrank
}

pub fn remove_mfrank(mfrank_bytes: &[u8]) -> String{
    println!("Removing Mfrank");

    let mfrank : Mfrank = bincode::deserialize(mfrank_bytes).unwrap();
    mfrank.msg
}
