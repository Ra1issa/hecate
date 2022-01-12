use hecate::hecate_lib::{
    user,
    moderator,
    utils,
};
use rand_core::OsRng;


fn main(){

    let mut rng = OsRng;
    let msg = "hello".to_string();
    let id = utils::random_block(32);

    let m = moderator::setup_moderator(&mut rng);
    let tk = moderator::generate_token(id.clone(), m.clone(), &mut rng);
    let mf = user::generate_frank(msg, tk);

    let _b = user::check_message(mf.clone(), m.sig_pk);
    let r = moderator::inspect(mf, m);
    assert_eq!(id, r.id)
}
