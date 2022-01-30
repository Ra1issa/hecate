use hecate::{
    receiver,
    utils,
    types::{Envelope, Mfrank, Report},
};

use ed25519_dalek::PublicKey;

fn main(){
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


    let new_mfrank = receiver::check_authorship(mfrank.clone(), envelope.clone());
    let report = receiver::check_message(new_mfrank, mod_pk, plat_pk);
    utils::write_to_file::<Report>(report, "report.txt");
}
