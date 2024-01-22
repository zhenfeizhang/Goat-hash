use ark_std::rand::RngCore;

use goldilocks::Goldilocks;
use param::M;
use ring::{RingElement, RingElementNTTRepr};

#[cfg(test)]
mod bench;
mod param;
mod ring;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoatHash {
    param: [RingElementNTTRepr<Goldilocks>; M],
}

impl GoatHash {
    pub fn init(mut rng: impl RngCore) -> Self {
        Self {
            param: (0..M)
                .map(|_| RingElementNTTRepr::random(&mut rng))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn hash(&self, inputs: &[RingElement<Goldilocks>; M]) -> RingElement<Goldilocks> {
        (&self
            .param
            .iter()
            .zip(inputs.iter())
            .map(|(x, y)| x * &RingElementNTTRepr::from(y))
            .sum::<RingElementNTTRepr<_>>())
            .into()
    }
}
