use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token{
    pub x1: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mod_sig: Vec<u8>,
    pub ske: Vec<u8>,
    pub pke: Vec<u8>,
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
