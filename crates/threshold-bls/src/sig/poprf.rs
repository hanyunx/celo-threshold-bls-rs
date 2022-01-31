use crate::group::{Element, Curve, PairingCurve, Point, Scalar};
use crate::sig::{Scheme, SignatureScheme};
use rand::prelude::*;
use std::{fmt::Debug, marker::PhantomData};
use algebra::Group;
use serde::{de::DeserializeOwned, Serialize};
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
        /// `...` represents the field over which private keys are represented.
        type Scalar: Scalar<RHS = Self::Scalar>;
        /// `...` represents the group over which the public keys are
        /// represented.
        type G2: Point<RHS = Self::Scalar> + Serialize + DeserializeOwned;
        /// `...` represents the group over which the signatures are reresented.
        type G1: Point<RHS = Self::Scalar> + Serialize + DeserializeOwned;


        fn req(
            public: &Self::Public, // remove?
            domain_tag: &[u8],
            msg: &[u8],
        )  -> Result<((&[u8], &[u8], Self::Scalar, Self::Scalar, Self::Scalar), (Self::G2, Self::G2)), POPRFError>{ // TODO: return type
            let rng = &mut rand::thread_rng();
            let r = Self::Scalar::rand(rng); // TODO: move to preprocessing?
            let c = Self::Scalar::rand(rng);
            let d = Self::Scalar::rand(rng);

            let mut h = Self::G2::new(); //TODO: replace Signature: H2
            h.map(msg).map_err(|_| POPRFError::HashingError)?;

            let a = h.mul(r);
            let b = h.mul(c).add(Self::G2::one().mul(d)); // b = h^c * g2^d

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

impl<C> poprf::POPRFScheme for G2Scheme<C>
    where
        C: PairingCurve,
{
    type Scalar = C::Scalar;
    type G2 = C::G2;
    type G1 = C::G1;
}


