use crate::group::{Element, Curve, PairingCurve, Point};
use crate::sig::{Scheme, SignatureScheme};
use rand::prelude::*;
use std::{fmt::Debug};
use algebra::Group;
use thiserror::Error;

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// /// A private share which is part of the threshold signing key
// pub struct Share<S> {
//     /// The share's index in the polynomial
//     pub index: Idx,
//     /// The scalar corresponding to the share's secret
//     pub private: S,
// }

#[derive(Debug, Error)]
pub enum POPRFError {
    #[error("could not hash to curve")]
    HashingError,

    // TODO
}

pub mod poprf {
    use super::*;

    pub trait POPRFScheme {

        fn req(
            public: &Self::Public, // remove?
            domain_tag: &[u8],
            msg: &[u8],
        )  -> Result<((&[u8],)), POPRFError>{ // TODO: return type
            let r = Self::Private::rand(rng); // or ::rand(&mut thread_rng())?
            let c = Self::Private::rand(rng);
            let d = Self::Private::rand(rng);

            let mut h = Self::Signature::new(); //TODO: replace Signature: H2
            h.map(msg).map_err(|_| POPRFError::HashingError)?;

            let a = h.mul(r);
            let b = h.mul(c).add(Self::Signature::one().mul(d)); // b = h^c * g2^d

            ((domain_tag, m, r, c, d), (a, b))
        }

        // Prove(a, b, c/r, d)
        fn prove(
            a,
            b,
            x,
            y,
        ) {
            let v1 = Self::Private::rand(rng);
            let v2 = Self::Private::rand(rng);
            let g2 = self::Self::Signature::one();
            let v =  g2.mul(v1).add(a.mul(v2));
            let concatenate = ();//  TODO: g2 ∣∣V ∣∣a∣∣b
            let z = Self::Signature::new();
            z.map(concatenate).map_err(|_| POPRFError::HashingError)?;
            let u1 = v1.sub(y.add(z)); // TODO:v1 −y⋅z ?
            let u1 = v2.sub(x.add(z));

            (z, u1, u2)
        }

        fn verify(
            a,
            b,
            pi,
        ) {
            z = ();// TODO?
            let g2 = self::Self::Signature::one();
            let v =  g2.mul(u1).add(a.mul(u2)).add(b.mul(z));
            let concatenate = ();//  TODO: g2 ∣∣V ∣∣a∣∣b
            let h = Self::Signature::new();
            h.map(concatenate).map_err(|_| POPRFError::HashingError)?;

            (z == h)
        }

        fn blind_ev() {

        }

        fn aggregate() {

        }

        fn finalize() {

        }
    }
}

#[derive(Clone, Debug)]
pub struct G2Scheme<C: PairingCurve> {
    m: PhantomData<C>,
}

impl<C> POPRFScheme for G2Scheme<C>
    where
        C: PairingCurve,
{
    // TODO: rename
    type Private = C::Scalar;
    type Public = C::G2;
    type Signature = C::G1;
}


