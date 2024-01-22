use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use ark_std::rand::RngCore;
use goldilocks::{Goldilocks, SmallField};

use crate::param::{INV_NTT_TABLE, N, NTT_TABLE, ONE_OVER_N};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RingElement<F: SmallField> {
    elements: Vec<F>,
}

impl Mul for &RingElement<Goldilocks> {
    type Output = RingElement<Goldilocks>;

    fn mul(self, rhs: &RingElement<Goldilocks>) -> Self::Output {
        (&(&RingElementNTTRepr::from(self) * &RingElementNTTRepr::from(rhs))).into()
    }
}

impl Mul for RingElement<Goldilocks> {
    type Output = RingElement<Goldilocks>;

    fn mul(self, rhs: RingElement<Goldilocks>) -> Self::Output {
        &self * &rhs
    }
}

impl<F: SmallField> RingElement<F> {
    pub fn random(mut rng: impl RngCore) -> Self {
        Self {
            elements: (0..N).map(|_| F::random(&mut rng)).collect(),
        }
    }

    pub fn random_message(mut rng: impl RngCore) -> Self {
        Self {
            elements: (0..N)
                .map(|_| F::from((rng.next_u32() % 16) as u64))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RingElementNTTRepr<F: SmallField> {
    pub(crate) elements: Vec<F>,
}

impl<F: SmallField> Default for RingElementNTTRepr<F> {
    fn default() -> Self {
        Self {
            elements: vec![F::default(); N],
        }
    }
}

impl<F: SmallField> Add for &RingElementNTTRepr<F> {
    type Output = RingElementNTTRepr<F>;

    // Pairwise addition
    fn add(self, rhs: &RingElementNTTRepr<F>) -> Self::Output {
        let mut res = self.clone();
        res.elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(res, r)| *res += r);
        res
    }
}

impl<F: SmallField> Add for RingElementNTTRepr<F> {
    type Output = RingElementNTTRepr<F>;

    fn add(self, rhs: RingElementNTTRepr<F>) -> Self::Output {
        &self + &rhs
    }
}

impl<F: SmallField> Sum for RingElementNTTRepr<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, item| acc + item)
    }
}

impl<F: SmallField> Mul for &RingElementNTTRepr<F> {
    type Output = RingElementNTTRepr<F>;

    // Pairwise multiplication
    fn mul(self, rhs: &RingElementNTTRepr<F>) -> Self::Output {
        let mut res = self.clone();
        res.elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(res, r)| *res *= r);
        res
    }
}

impl<F: SmallField> Mul for RingElementNTTRepr<F> {
    type Output = RingElementNTTRepr<F>;

    fn mul(self, rhs: RingElementNTTRepr<F>) -> Self::Output {
        &self * &rhs
    }
}

impl<F: SmallField> RingElementNTTRepr<F> {
    pub fn random(mut rng: impl RngCore) -> Self {
        Self {
            elements: (0..N).map(|_| F::random(&mut rng)).collect(),
        }
    }
}

impl From<&RingElement<Goldilocks>> for RingElementNTTRepr<Goldilocks> {
    // Forward NTT transform
    fn from(r: &RingElement<Goldilocks>) -> Self {
        let mut p = r.elements.clone();
        let mut t = N;
        for l in 0..8 {
            let m = 1 << l;
            let ht = t >> 1;
            let mut i = 0;
            let mut j1 = 0;
            while i < m {
                let s = NTT_TABLE[m + i];
                let j2 = j1 + ht;
                let mut j = j1;
                while j < j2 {
                    let u = p[j];
                    let v = (p[j + ht]) * (s);
                    p[j] = u + v;
                    p[j + ht] = u - v;
                    j += 1;
                }
                i += 1;
                j1 += t;
            }
            t = ht;
        }
        Self { elements: p }
    }
}

impl From<&RingElementNTTRepr<Goldilocks>> for RingElement<Goldilocks> {
    // Reverse NTT transform
    fn from(r: &RingElementNTTRepr<Goldilocks>) -> Self {
        let mut p = r.elements.clone();
        let mut t = 1;
        let mut m = N;

        while m > 1 {
            let hm = m >> 1;
            let dt = t << 1;
            let mut i = 0usize;
            let mut j1 = 0;
            while i < hm {
                let j2 = j1 + t;
                let s = INV_NTT_TABLE[hm + i];
                let mut j = j1;
                while j < j2 {
                    let u = p[j];
                    let v = p[j + t];
                    p[j] = u + v;
                    p[j + t] = (u - v) * s;
                    j += 1;
                }
                i += 1;
                j1 += dt;
            }
            t = dt;
            m = hm;
        }
        for e in p.iter_mut() {
            *e = *e * ONE_OVER_N;
        }
        Self { elements: p }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::test_rng;
    use ff::Field;

    // school book multiplication
    // slow. only used for correctness checking
    fn schoolbook(
        a: &RingElement<Goldilocks>,
        b: &RingElement<Goldilocks>,
    ) -> RingElement<Goldilocks> {
        let mut buf = [Goldilocks::ZERO; N * 2];
        let mut c = [Goldilocks::ZERO; N];
        for i in 0..N {
            for j in 0..N {
                buf[i + j] += a.elements[i] * b.elements[j];
            }
        }
        for i in 0..N {
            c[i] = buf[i] - buf[i + N as usize];
        }
        RingElement {
            elements: c.to_vec(),
        }
    }

    #[test]
    fn test_ring_mul() {
        let mut rng = test_rng();
        let a = RingElement::random(&mut rng);
        let b = RingElement::random(&mut rng);
        let c = RingElement::random(&mut rng);

        let ab_then_c = &(&a * &b) * &c;
        let ac_then_b = &(&a * &c) * &b;
        let bc_then_a = &(&b * &c) * &a;

        assert_eq!(ab_then_c, ac_then_b);
        assert_eq!(ab_then_c, bc_then_a);

        let ab_then_c_schoolbook = schoolbook(&schoolbook(&a, &b), &c);
        assert_eq!(ab_then_c, ab_then_c_schoolbook);
    }
}
