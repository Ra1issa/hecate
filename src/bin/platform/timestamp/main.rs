use hecate::{
    platform,
    utils,
    types::{Platform, Envelope},
};


fn main(){
    let mut buff_p = Vec::new();
    let p = utils::read_from_file::<Platform>("plat_keys.txt",&mut buff_p);

    let mut buff = Vec::new();
    let com = utils::read_from_file::<Vec<u8>>("commitment.txt",&mut buff);

    let env = platform::sign_com(com.clone(), p.clone());
    utils::write_to_file::<Envelope>(env, "envelope.txt");
}
