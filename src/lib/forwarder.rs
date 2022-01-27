use crate::{
    utils,
    types::{Mfrank, Envelope},
};
use rand::{CryptoRng, Rng};

pub fn forward
<R: CryptoRng + Rng>
(
    mfrank: Mfrank,
    envelope: Envelope,
    rng: &mut R,
)-> (Mfrank, Vec<u8>){

    let com = utils::random_block(envelope.com.len(), rng);
    let new_mfrank = Mfrank ::new(mfrank, envelope);
    return (new_mfrank, com);
}
