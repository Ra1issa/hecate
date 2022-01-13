use hecate::hecate_lib::{
    moderator::{Moderator, inspect},
    utils,
    types::Mfrank,
};

fn main(){
    let mut buff_mfrank = Vec::new();
    let mfrank = utils::read_from_file::<Mfrank>("mfrank.txt",&mut buff_mfrank);

    let mut buff_pk = Vec::new();
    let mod_sig_pk = utils::read_from_file::<Moderator>("mod_keys.txt",&mut buff_pk);

    let report = inspect(mfrank, mod_sig_pk);

    println!("Traced id {:?}", report.id);
}
