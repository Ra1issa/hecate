use hecate::hooks::{
    inject_mfrank,
    remove_mfrank,
};

use bincode;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jstring, jcharArray};

#[no_mangle]
pub fn Java_org_Hecate_inject_1mfrank_1jni(
    env: JNIEnv,
    class: JClass,
    ptext: JString
) -> jcharArray{
    let input: String =
        env.get_string(ptext).expect("Couldn't get java string!").into();
    let result = inject_mfrank(input);
    let len = result.len() as usize;

    let mut res_encoded: Vec<u16> = Vec::new();
    for i in 0..len{
        res_encoded.push(result[i] as u16);
    }

    let output = env.new_char_array(len as i32).unwrap();
    env.set_char_array_region(output, 0, &res_encoded).unwrap();
    output
}

#[no_mangle]
pub fn Java_org_Hecate_remove_1mfrank_1jni(
    env: JNIEnv,
    class: JClass,
    ptext: jcharArray
) -> jstring {

    let len = env.get_array_length(ptext).unwrap() as usize;
    let mut buff = vec![0 as u16; len];
    env.get_char_array_region(ptext, 0, &mut buff[..]).expect("Couldn't get java string!");

    let mut mfrank = vec![0 as u8; len];
    for i in 0..len{
        mfrank[i] = buff[i] as u8;
    }
    let result = remove_mfrank(&mfrank);
    let output = env.new_string(result)
        .expect("Couldn't create java string!");
    output.into_inner()
}
