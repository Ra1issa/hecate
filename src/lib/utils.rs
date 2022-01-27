use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{PathBuf},
};
use serde::{Serialize, Deserialize};
use shellexpand;
use rand::{CryptoRng, Rng};

pub fn random_block
<R: CryptoRng + Rng>
(
    size: usize,
    rng: &mut R,
) -> Vec<u8> {
    let mut block = vec![0 as u8; size];
    rng.fill_bytes(&mut block);
    return block;
}

pub fn add_bytes(a: &[u8], b: &[u8]) -> Vec<u8>{
    let c_len = a.len();
    let mut c = vec![0 as u8; c_len];
    for i in 0..c_len{
        let r = ((a[i] as u16 + b[i] as u16) % 256) as u8;
        c[i] = r as u8;
    }
    return c;
}

pub fn sub_bytes(b: &[u8], c: &[u8]) -> Vec<u8>{
    let a_len = c.len();
    let mut a = vec![0 as u8; a_len];
    for i in 0..a_len{
        a[i] = ((c[i] as i16 - b[i] as i16) % 256) as u8;
    }
    return a;
}

pub fn write_to_file<'a, T: Serialize>
(
    data: T,
    file_name: &str
){
    let data = bincode::serialize(&data).unwrap();
    let mut path = get_data_path();
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
    let mut path = get_data_path();
    path.push(file_name);
    let path_str = path.clone().into_os_string().into_string().unwrap();
    let mut file = File::open(path_str).unwrap();
    file.read_to_end(buff).unwrap();
    // println!("****************************");
    // println!("{:?}", file_name);
    // println!("{:?}", buff);
    // println!("****************************");
    let msg: T = bincode::deserialize(buff).unwrap();
    return msg;
}

pub fn get_data_path() -> PathBuf{
    let root_dir = shellexpand::tilde("~/Documents/hecate/data").to_string();
    let path = PathBuf::from(root_dir);
    create_dir_all(path.clone()).unwrap();
    path
}
