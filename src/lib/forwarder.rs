use crate::{
    utils,
    types::{FMfrank, Mfrank, Envelope},
};
use rand::{CryptoRng, Rng};

pub fn forward
<R: CryptoRng + Rng>
(
    mfrank: Option<Mfrank>,
    fmfrank: Option<FMfrank>,
    envelope: Envelope,
    rng: &mut R,
)-> Result<(FMfrank, Envelope), &'static str>{

    let new_envelope = Envelope{
        com: utils::random_block(envelope.com.len(), rng),
        sig: utils::random_block(envelope.sig.len(), rng),
        time: utils::random_block(envelope.time.len(), rng),
    };
    match mfrank{
            Some(mf) => {
                let new_mfrank = FMfrank::new(mf, envelope);
                Ok((new_mfrank, new_envelope))
            },
            None =>{
                match fmfrank{
                    Some(fmf) => Ok((fmf, new_envelope)),
                    None => Err("invalid franked message"),
                }
            },
    }
}
