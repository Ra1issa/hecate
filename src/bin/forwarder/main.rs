use hecate::{
    receiver,
    forwarder,
    utils,
    types::{Envelope, Mfrank},
};
use ed25519_dalek::PublicKey;
use rand::rngs::OsRng;


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

    let _ = receiver::check_message(mfrank.clone(), envelope.clone(), mod_pk, plat_pk);

    let mut rng = OsRng{};
    let (new_mfrank, new_com) = forwarder::forward(
                                        mfrank.clone(),
                                        envelope.clone(),
                                        &mut rng
                                    );
    utils::write_to_file::<Vec<u8>>(new_com, "forwarded_envelope.txt");
    utils::write_to_file::<Mfrank>(new_mfrank, "forwarded_mfrank.txt");
}
