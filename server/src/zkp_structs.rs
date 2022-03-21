use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use uuid::Uuid;

pub struct Math {

}
impl Math {
    pub fn pow2(base: u32, pow: u32, modl: u32 ) -> u32 {
        let b1 = ((pow as i32) / 2) as u32;
        let b2 = b1 + (pow % 2);
        let p1 = u32::pow(base, b1);
        let p2 = u32::pow(base, b2);
        let m1 = ((p1 % modl) * (p2 % modl)) % modl;
        let mut m2 = ((u32::pow(base, b1) % modl) * (u32::pow(base, b2)) % modl) % modl;
        let m3 =  u32::pow(base % modl, pow) % modl;
        m2 = m2;
        m1
    } 
}

#[derive(Copy, Clone, Debug)]
pub struct AuthenticationRequest {
    // user: User,
    // Commitment: ServerCommitment,
}

#[derive(Copy, Clone, Debug)]
pub struct Agreement {
    pub y1: u32,
    pub y2: u32,
    pub g: u32,
    pub h: u32,
    pub x: u32,
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
    pub k: u32,
    pub r1: u32,
    pub r2: u32,
}

impl Commitment {
    pub fn new() -> Self {
        Commitment { k: 0, r1: 0, r2: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ServerCommitment {
    pub r1: u32,
    pub r2: u32,
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
    pub s: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Challenge {
    pub c: u32,
}
