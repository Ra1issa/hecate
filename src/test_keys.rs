use std::collections::HashMap;
use serde::Deserialize;
use hex;

#[derive(Deserialize, Debug)]
struct WycheproofTest {
    // #[serde(rename = "tcId")]
    // tc_id: usize,
    key: String,
    #[serde(rename = "iv")]
    nonce: String,
    aad: String,
    // #[serde(rename = "msg")]
    // pt: String,
    // ct: String,
    // tag: String,
    // result: String,
    // flags: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct WycheproofTestGroup {
    // #[serde(rename = "ivSize")]
    // iv_size: usize,
    // #[serde(rename = "keySize")]
    // key_size: usize,
    // #[serde(rename = "tagSize")]
    // tag_size: usize,
    // #[serde(rename = "type")]
    // typ: String,
    tests: Vec<WycheproofTest>,
}

#[derive(Deserialize, Debug)]
struct WycheproofTestSet {
    // algorithm: String,
    // #[serde(rename = "generatorVersion")]
    // generator_version: String,
    // #[serde(rename = "numberOfTests")]
    // number_of_tests: usize,
    // header: Vec<String>,
    // notes: HashMap<String, String>,
    // schema: String,
    // #[serde(rename = "testGroups")]
    test_groups: Vec<WycheproofTestGroup>,
}

pub fn generate_test_enc_keys() -> (Vec<u8>, Vec<u8>, Vec<u8>){
    let kat_data = include_bytes!("../data/aes_gcm_test.json");
    let kats: WycheproofTestSet = serde_json::from_slice(kat_data).expect("Valid JSON");
    let test = &kats.test_groups[3].tests[1];

    let key = hex::decode(test.key.clone()).expect("valid hex");
    let nonce = hex::decode(test.nonce.clone()).expect("valid hex");
    let aad = hex::decode(test.aad.clone()).expect("valid hex");

    return (key, nonce, aad)
}
