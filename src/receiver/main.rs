use hecate::hecate_lib::{
    receiver,
    utils,
    types::{Envelope, Mfrank},
};
use curve25519_dalek::ristretto::RistrettoPoint;

fn main(){
    let mut buff_mfrank = Vec::new();
    let mfrank = utils::read_from_file::<Mfrank>("mfrank.txt",&mut buff_mfrank);

    let mut buff_pk = Vec::new();
    let mod_pk = utils::read_from_file::<RistrettoPoint>("mod_pk.txt",&mut buff_pk);

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<RistrettoPoint>("plat_pk.txt",&mut buff_pk);

    let mut buff_env = Vec::new();
    let envelope = utils::read_from_file::<Envelope>("envelope.txt",&mut buff_env);

    let _b = receiver::check_message(mfrank, envelope, mod_pk, plat_pk);
}
