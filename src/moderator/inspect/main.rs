use hecate::hecate_lib::{
    moderator::{Moderator, inspect},
    utils,
    types::{Mfrank, Envelope},
};
use curve25519_dalek::ristretto::RistrettoPoint;
fn main(){
    let mut buff_mfrank = Vec::new();
    let mfrank = utils::read_from_file::<Mfrank>("mfrank.txt",&mut buff_mfrank);

    let mut buff_pk = Vec::new();
    let moderator = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_pk);

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<RistrettoPoint>("plat_pk.txt",&mut buff_pk);

    let mut buff_env = Vec::new();
    let envelope = utils::read_from_file::<Envelope>("envelope.txt",&mut buff_env);

    let report = inspect(mfrank, envelope, moderator, plat_pk);
    println!("Traced id {:?}", report.id);
}
