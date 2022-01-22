use crate::{
    platform,
    moderator,
    receiver,
    sender,
    // utils,
    tests,
    types::{Platform, Moderator, Mfrank, Envelope},
};
use ed25519_dalek::PublicKey;

const ENVELOPE_SIZE: usize = 128;

pub fn inject_mfrank(ptext: String) -> Vec<u8>{
    println!("Adding Mfrank");
    // let id = utils::random_block(32);
    // let mut buff_m = Vec::new();
    // let m = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_m);

    let id = tests::ID.to_vec();
    let m: Moderator = bincode::deserialize(tests::MOD).unwrap();

    let mut rng = rand::thread_rng();
    let tk = moderator::generate_token(id, m, &mut rng);
    let (_mf, _com) = sender::generate_frank(ptext, tk, &mut rng);

    let mfrank = tests::MFRANK.to_vec();
    mfrank
}

pub fn inject_envelope_com(ctext: &[u8]) -> Vec<u8>{
    println!("Adding envelope");
    // let mut buff_p = Vec::new();
    // let p = utils::read_from_file::<Platform>("plat_keys.txt",&mut buff_p);
    let p: Platform = bincode::deserialize(tests::PLAT).unwrap();
    let com: Vec<u8> = bincode::deserialize(tests::COM).unwrap();
    // let mut buff = Vec::new();
    // let com = utils::read_from_file::<Vec<u8>>("commitment.txt",&mut buff);
    let _env = platform::sign_com(com.clone(), p.clone());
    let e = tests::ENVELOPE;
    //
    // let e = bincode::serialize(&env).unwrap().to_vec();

    [ctext, &e].concat()
}

pub fn remove_envelope_com(ctext: &[u8]) -> Vec<u8>{
    println!("Removing envelope");
    let c_len = ctext.len();
    let _env_bytes = ctext[c_len-ENVELOPE_SIZE..c_len].to_vec();
    // Temporary: When the cipher is small, its not the actual message
    // Its the ack
    // if c_len > 300 {
    //     let envelope: Envelope = bincode::deserialize(&env_bytes).unwrap();
    //     utils::write_to_file::<Envelope>(envelope, "envelope.txt");
    // }
    ctext[0..c_len-ENVELOPE_SIZE].to_vec()
}

pub fn remove_mfrank(mfrank_bytes: &[u8]) -> String{
    println!("Removing Mfrank");
    // let mut buff_pk = Vec::new();
    // let mod_pk = utils::read_from_file::<RistrettoPoint>("mod_pk.txt",&mut buff_pk);
    //
    // let mut buff_pk = Vec::new();
    // let plat_pk = utils::read_from_file::<RistrettoPoint>("plat_pk.txt",&mut buff_pk);
    //
    // let mut buff_env = Vec::new();
    // let envelope = utils::read_from_file::<Envelope>("envelope.txt",&mut buff_env);
    let mod_pk_bytes: Vec<u8> = bincode::deserialize(tests::MOD_PK).unwrap();
    let plat_pk_bytes: Vec<u8> = bincode::deserialize(tests::PLAT_PK).unwrap();
    let mod_pk = PublicKey::from_bytes(&mod_pk_bytes).unwrap();
    let plat_pk = PublicKey::from_bytes(&plat_pk_bytes).unwrap();
    let envelope : Envelope = bincode::deserialize(tests::ENVELOPE).unwrap();
    let mfrank : Mfrank = bincode::deserialize(mfrank_bytes).unwrap();
    let m = mfrank.msg.clone();
    let _ = receiver::check_message(mfrank, envelope, mod_pk, plat_pk);
    m
}
