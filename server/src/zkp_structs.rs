use num::BigUint;
use num::Zero;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use uuid::Uuid;

pub struct Math {}
impl Math {
    pub fn zero() -> BigUint {
        BigUint::zero()
    }
    pub fn pow2(base: BigUint, pow: BigUint, modl: BigUint) -> BigUint {
        // let b1 = ((pow as i32) / 2) as u32;
        // let b2 = b1 + (pow % 2) as u32;
        // let p1 = BigUint::pow(base % modl, b1) % modl;
        // let p2 = BigUint::pow(base % modl, b2) % modl;
        // ((p1 % modl) * (p2 % modl)) % modl
        BigUint::zero()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AuthenticationRequest {
    // user: User,
// Commitment: ServerCommitment,
}

#[derive(Clone, Debug)]
pub struct Agreement {
    pub y1: BigUint,
    pub y2: BigUint,
    pub g: BigUint,
    pub h: BigUint,
    pub x: BigUint,
}

impl Agreement {
    pub fn new() -> Self {
        Self {
            y1: BigUint::zero(),
            y2: BigUint::zero(),
            g: BigUint::zero(),
            h: BigUint::zero(),
            x: BigUint::zero(),
        }
    }
}

impl Display for Agreement {
    // (y1 == g^x1) && (y2 == h^x2)
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(y1 == g^x1) && (y2 == h^x2)\r\n");
        write!(
            f,
            "({} == {}^{}) && ({} == {}^{})",
            self.y1, self.g, self.x, self.y2, self.h, self.x
        )
    }
}

#[derive(Clone, Debug)]
pub struct Commitment {
    pub k: BigUint,
    pub r1: BigUint,
    pub r2: BigUint,
}

impl Commitment {
    pub fn new() -> Self {
        Commitment {
            k: BigUint::zero(),
            r1: BigUint::zero(),
            r2: BigUint::zero(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerCommitment {
    pub r1: BigUint,
    pub r2: BigUint,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct User {
    pub uuid: Uuid,
}

impl User {
    pub fn new() -> Self {
        User {
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Answer {
    pub s: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Challenge {
    pub c: u32,
}
