use curve25519_dalek::{
    scalar::Scalar,
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::RistrettoPoint,
};
use rand::Rng;
use rand_core::OsRng;

use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use serde::{Serialize, Deserialize};


pub fn random_block(size: u8) -> Vec<u8>{
    let mut block = Vec::new();
    for _i in 0..size {
        block.push(rand::thread_rng().gen::<u8>());
    }
    return block;
}

pub fn add_bytes(a: &[u8], b: &[u8]) -> Vec<u8>{
    let mut c = Vec::new();
    for i in 0..a.len(){
        let r = ((a[i] as u16 + b[i] as u16) % 256) as u8;
        c.push(r as u8);
    }
    return c;
}

pub fn sub_bytes(b: &[u8], c: &[u8]) -> Vec<u8>{
    let mut a = Vec::new();
    for i in 0..b.len(){
        a.push(((c[i] as i16 - b[i] as i16) % 256) as u8);
    }
    return a;
}

pub fn generate_keys() -> (Scalar, RistrettoPoint){
    let mut rng = OsRng;
    let sk: Scalar = Scalar::random(&mut rng);
    let pk = sk * RISTRETTO_BASEPOINT_POINT;
    return (sk, pk);
}

pub fn write_to_file<'a, T: Serialize>
(
    data: T,
    file_name: &str
){
    let data = bincode::serialize(&data).unwrap();

    let mut path = get_project_path();
    path.push("data");
    path.push(file_name);
    let path_str = path.clone().into_os_string().into_string().unwrap();

    let mut file = File::create(path_str).unwrap();
    file.write(&data).unwrap();
}

pub fn read_from_file<'a, T>
(
    file_name: &str,
    buff: &'a mut Vec<u8>,
) -> T
where
    T:  Deserialize<'a>
{
    let mut path = get_project_path();
    path.push("data");
    path.push(file_name);
    let path_str = path.clone().into_os_string().into_string().unwrap();
    let mut file = File::open(path_str).unwrap();
    file.read_to_end(buff).unwrap();
    let msg: T = bincode::deserialize(buff).unwrap();
    return msg;
}

pub fn get_project_path() -> PathBuf{
    let path = env::current_exe().unwrap();
    let path_str = path.clone().into_os_string().into_string().unwrap();
    let path_split = path_str.split_inclusive("hecate").collect::<Vec<&str>>();
    let path = PathBuf::from(path_split[0]);
    return path;
}
