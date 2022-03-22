use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use uuid::Uuid;

pub struct Math {

}
impl Math {
    pub fn pow2(base: u128, pow: u128, modl: u128 ) -> u128 {
        let b1 = ((pow as i32) / 2) as u32;
        let b2 = b1 + (pow % 2) as u32;
        let p1 = u128::pow(base % modl, b1) % modl;
        let p2 = u128::pow(base % modl, b2) % modl;
        ((p1 % modl) * (p2 % modl)) % modl
    } 
} 

#[derive(Copy, Clone, Debug)]
pub struct AuthenticationRequest {
    // user: User,
    // Commitment: ServerCommitment,
}

#[derive(Copy, Clone, Debug)]
pub struct Agreement {
    pub y1: u128,
    pub y2: u128,
    pub g: u128,
    pub h: u128,
    pub x: u128,
}

impl Agreement {
    pub fn new() -> Self {
        Self { y1: 0, y2: 0, g: 0, h: 0, x: 0 }
    }
}

impl Display for Agreement {
    // (y1 == g^x1) && (y2 == h^x2)
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(y1 == g^x1) && (y2 == h^x2)\r\n");
        write!(f, "({} == {}^{}) && ({} == {}^{})", self.y1, self.g, self.x, self.y2, self.h, self.x)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Commitment {
    pub k: u128,
    pub r1: u128,
    pub r2: u128,
}

impl Commitment {
    pub fn new() -> Self {
        Commitment { k: 0, r1: 0, r2: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ServerCommitment {
    pub r1: u128,
    pub r2: u128,
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

#[derive(Copy, Clone, Debug)]
pub struct Answer {
    pub s: u128,
}

#[derive(Copy, Clone, Debug)]
pub struct Challenge {
    pub c: u128,
}
