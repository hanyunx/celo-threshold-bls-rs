use crate::group::{Element, Curve, PairingCurve, Point, Scalar};
use crate::sig::{Scheme, SignatureScheme, Share};
use rand::prelude::*;
use std::{fmt::Debug, marker::PhantomData};
use algebra::Group;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum POPRFError {
    #[error("could not hash to curve")]
    HashingError,

    #[error("could not serialize")]
    SerializationError,
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

            let mut h = Self::G2::new();
            h.map(msg).map_err(|_| POPRFError::HashingError)?;

            let a = h.mul(r);
            let b = h.mul(c).add(Self::G2::one().mul(d)); // b = h^c * g2^d

            Ok((domain_tag, m, r, c, d), (a, b))
        }

        // Prove(a, b, c/r, d)
        fn prove(
            a: Self::G2,
            b: self::G2,
            x: Self::Scalar,
            y: Self::Scalar,
        ) -> Result<(Self::Scalar, Self::Scalar, Self::Scalar), POPRFError>{
            let rng = &mut rand::thread_rng();
            let v1 = Self::Scalar::rand(rng);
            let v2 = Self::Scalar::rand(rng);
            let g2 = self::Self::G2::one();
            let v =  g2.mul(v1).add(a.mul(v2));

            // Concatenate (g2 || v || a || b)
            let g2_ser = bincode::serialize(&g2).map_err(|_| POPRFError::SerializationError)?;
            let v_ser = bincode::serialize(&v).map_err(|_| POPRFError::SerializationError)?;
            let a_ser = bincode::serialize(&a).map_err(|_| POPRFError::SerializationError)?;
            let b_ser = bincode::serialize(&b).map_err(|_| POPRFError::SerializationError)?;
            let mut concatenate:Vec<u8> = [g2_ser, v_ser, a_ser, b_ser].concat();

            // TODO: implement hash to scalar field
            let mut z = Self::G2::new();
            z.map(concatenate).map_err(|_| POPRFError::HashingError)?;
            
            let s1 = v1.sub(y.mul(z));
            let s2 = v2.sub(x.mul(z));

            Ok(z, s1, s2)
        }

        fn verify(
            a: Self::G2,
            b: Self::G2,
            z: Self::Scalar,
            s1: Self::Scalar,
            s2: Self::Scalar,
        ) -> Result<bool, POPRFError> {
            let g2 = self::Self::G2::one();
            let v =  g2.mul(s1).add(a.mul(s2)).add(b.mul(z));

            // Concatenate (g2 || v || a || b)
            let g2_ser = bincode::serialize(&g2).map_err(|_| POPRFError::SerializationError)?;
            let v_ser = bincode::serialize(&v).map_err(|_| POPRFError::SerializationError)?;
            let a_ser = bincode::serialize(&a).map_err(|_| POPRFError::SerializationError)?;
            let b_ser = bincode::serialize(&b).map_err(|_| POPRFError::SerializationError)?;
            let mut concatenate:Vec<u8> = [g2_ser, v_ser, a_ser, b_ser].concat();

            // TODO: implement hash to scalar field
            // let h = 

            Ok(z == h)
        }

        fn blind_ev(k: Self::Scalar, t, (a, b)) -> Result<(Self::GT, Self::GT), POPRFError>; 

        fn aggregate(
            threshold:usize,
            shares: &[(Share<(Self::GT, Self::GT)>, Share<(Self::GT, Self::GT)>)]
        ) -> Result<(Self::GT, Self::GT), POPRFError>{
            
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
    fn blind_ev(k, t, (a, b)) -> Result<(Self::GT, Self::GT), POPRFError> {
        let mut h = Self::G1::new();
        h.map(t).map_err(|_| POPRFError::HashingError);
        let hk = h.mul(k);
        // A <- e(H1(t)^k, a)
        let A = C::pair(&hk, &a);
        // B <- e(H1(t)^k, b)
        let B = C::pair(&hk, &b);

        (A, B)

    }

    type Scalar = C::Scalar;
    type G2 = C::G2;
    type G1 = C::G1;
    type GT = C::GT;
}


