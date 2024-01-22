use ark_std::rand::RngCore;
use ff::Field;
use goldilocks::Goldilocks;
use param::{M, N};
use ring::{RingELementNTTRepr, RingElement};

mod param;
mod ring;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoatHash {
    param: [RingELementNTTRepr<Goldilocks>; M],
}

impl GoatHash {
    pub fn init<R>(mut rng: impl RngCore) -> Self {
        Self {
            param: (0..M)
                .map(|_| RingELementNTTRepr {
                    elements: (0..N).map(|_| Goldilocks::random(&mut rng)).collect(),
                })
                .collect::<Vec<RingELementNTTRepr<_>>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn hash(&self, inputs: &[RingElement<Goldilocks>; M]) -> RingElement<Goldilocks> {
        (&self
            .param
            .iter()
            .zip(inputs.iter())
            .map(|(x, y)| x * &RingELementNTTRepr::from(y))
            .sum::<RingELementNTTRepr<_>>())
            .into()
    }
}
