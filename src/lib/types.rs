use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Moderator{
    pub enc_sk: Vec<u8>,
    pub keypair: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Platform{
    pub keypair: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token{
    pub x1: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mod_sig: Vec<u8>,
    pub key_eph: Vec<u8>,
    pub time: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mfrank{
    pub msg: String,
    pub x1: Vec<u8>,
    pub x2: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mod_sig: Vec<u8>,
    pub send_sig: Vec<u8>,
    pub pke: Vec<u8>,
    pub randc: Vec<u8>,
    pub time: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FMfrank{
    pub msg: String,
    pub x1: Vec<u8>,
    pub x2: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mod_sig: Vec<u8>,
    pub send_sig: Vec<u8>,
    pub pke: Vec<u8>,
    pub randc: Vec<u8>,
    pub mod_time: Vec<u8>,
    pub com: Vec<u8>,
    pub plat_sig: Vec<u8>,
    pub plat_time: Vec<u8>,
}

impl FMfrank{
    pub fn new(mfrank: Mfrank, envelope: Envelope) -> FMfrank{
        FMfrank {
            msg: mfrank.msg,
            x1: mfrank.x1,
            x2: mfrank.x2,
            nonce: mfrank.nonce,
            mod_sig: mfrank.mod_sig,
            send_sig: mfrank.send_sig,
            pke: mfrank.pke,
            randc: mfrank.randc,
            mod_time: mfrank.time,
            com: envelope.com,
            plat_sig: envelope.sig,
            plat_time: envelope.time,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Envelope{
    pub com: Vec<u8>,
    pub sig: Vec<u8>,
    pub time: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Report{
    pub mfrank: Mfrank,
    pub envelope: Envelope,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trace{
    pub id: Vec<u8>,
    pub msg: String,
    pub time_diff: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Test{
    pub id: Vec<u8>,
    pub msg: Vec<String>,
    pub token: Token,
    pub mfrank: Vec<Mfrank>,
    pub mod_pk: Vec<u8>,
    pub envelope: Vec<Envelope>,
    pub report: Vec<Report>,
    pub moderator: Moderator,
    pub platform: Platform,
    pub plat_pk: Vec<u8>,
    pub msg_sizes: Vec<usize>,
}
