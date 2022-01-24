use hecate::{
    receiver,
    utils,
    types::{Envelope, FMfrank, Mfrank, Report},
};
use ed25519_dalek::PublicKey;

fn main(){
    let mut buff_mfrank = Vec::new();
    let fmfrank = utils::read_from_file::<FMfrank>("forwarded_mfrank.txt",&mut buff_mfrank);

    let mut buff_pk = Vec::new();
    let mod_pk = utils::read_from_file::<Vec<u8>>("mod_pk.txt",&mut buff_pk);
    let mod_pk = PublicKey::from_bytes(&mod_pk).unwrap();

    let mut buff_pk = Vec::new();
    let plat_pk = utils::read_from_file::<Vec<u8>>("plat_pk.txt",&mut buff_pk);
    let plat_pk = PublicKey::from_bytes(&plat_pk).unwrap();

    let mut buff_env = Vec::new();
    let _fenvelope = utils::read_from_file::<Envelope>("forwarded_envelope.txt",&mut buff_env);

    let mfrank = Mfrank{
        msg: fmfrank.msg,
        x1: fmfrank.x1,
        x2: fmfrank.x2,
        nonce: fmfrank.nonce,
        mod_sig: fmfrank.mod_sig,
        send_sig: fmfrank.send_sig,
        pke: fmfrank.pke,
        randc: fmfrank.randc,
        time: fmfrank.mod_time,
    };

    let envelope = Envelope{
        com: fmfrank.com,
        sig: fmfrank.plat_sig,
        time: fmfrank.plat_time,
    };
    let report = receiver::check_message(mfrank.clone(), envelope.clone(), mod_pk, plat_pk);
    utils::write_to_file::<Report>(report, "report.txt");
    // connect_report(rep);
}
