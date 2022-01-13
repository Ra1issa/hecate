use hecate::hecate_lib::{
    receiver,
    utils,
    types::Mfrank,
};
use curve25519_dalek::ristretto::RistrettoPoint;

fn main(){
    let mut buff_mfrank = Vec::new();
    let mfrank = utils::read_from_file::<Mfrank>("mfrank.txt",&mut buff_mfrank);

    let mut buff_pk = Vec::new();
    let mod_sig_pk = utils::read_from_file::<RistrettoPoint>("mod_sig_pk.txt",&mut buff_pk);

    let _b = receiver::check_message(mfrank, mod_sig_pk);
}
