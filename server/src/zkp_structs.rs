use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use uuid::Uuid;

pub struct Math {

}
impl Math {
    pub fn pow2(base: f32, pow: f32, modl: f32 ) -> f32 {
        let b1 = ((pow as i32) / 2) as f32;
        let b2 = b1 + (pow % 2.0);
        let p1 = f32::powf(base, b1);
        let p2 = f32::powf(base, b2);
        let m1 = ((p1 % modl) * (p2 % modl)) % modl;
        let mut m2 = ((f32::powf(base, b1) % modl) * (f32::powf(base, b2)) % modl) % modl;
        let m3 =  f32::powf(base % modl, pow) % modl;
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
    pub y1: f32,
    pub y2: f32,
    pub g: f32,
    pub h: f32,
    pub x: f32,
}

impl Agreement {
    pub fn new() -> Self {
        Self { y1: 0.0, y2: 0.0, g: 0.0, h: 0.0, x: 0.0 }
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
    pub k: f32,
    pub r1: f32,
    pub r2: f32,
}

impl Commitment {
    pub fn new() -> Self {
        Commitment { k: 0.0, r1: 0.0, r2: 0.0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ServerCommitment {
    pub r1: f32,
    pub r2: f32,
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
    pub s: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Challenge {
    pub c: f32,
}
