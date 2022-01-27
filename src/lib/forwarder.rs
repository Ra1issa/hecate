use crate::{
    utils,
    types::Mfrank,
};
use rand::{CryptoRng, Rng};

pub fn forward
<R: CryptoRng + Rng>
(
    mfrank: Mfrank,
    rng: &mut R,
)-> Vec<u8>{

    let com = utils::random_block(mfrank.com.len(), rng);
    return com;
}
