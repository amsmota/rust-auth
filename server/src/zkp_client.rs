use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::zkp_server::Challenge;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ZkpClient {
    agreement: Agreement,
    commitments: HashMap<Uuid, Commit>,
    q: f32,
}

impl ZkpClient {
    pub fn new() -> Self {
        Self {
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            q: 7.0,
        }
    }

    pub fn agreement(&mut self) -> Agreement {
        let (aggreement, _) = self.protocol();
        self.agreement = aggreement;
        aggreement
    }

    pub fn create_register_commits(&mut self, user: User) -> Commit {
        let ( _, commitment) = self.protocol();
        self.commitments.insert(user.uuid, commitment);
        commitment
    }

    fn protocol(&mut self) -> (Agreement, Commit) {
        let mut rng = rand::thread_rng();
        // rng.gen::<u128>();

        // (y1 == g^x1) && (y2 == h^x2)
        // y1, y2, x1 = x2, k, r, c, s, g, h, y, q
        // y1  y2              r  C  s  g  B     q

        //  a, b, A, , C,  z
        // (g,g^a, g^b and g^ab) =(",g,A,B,C,")")

        //self.q = rng.gen_range(1..100);

        let g = 3.0; //rng.gen_range(1..100);
        let h = 13.0; //rng.gen_range(1..100);


        let x = rng.gen_range(1..10) as f32;
        let y1 = f32::powf(g, x) % self.q;
        let y2 = f32::powf(h, x) % self.q;

        let agreement = Agreement { y1, y2, g, h, x };
        let commitment = Commit { k: x, r1: y1, r2: y2, };

        ( agreement, commitment )
    }

    pub fn create_authentication_request(&mut self, user: User) -> AuthenticationRequest {
        // let commit = self.create_register_commits(user);
        AuthenticationRequest {
            // user,
            // commit: RandomCommit {
            //     r1: commit.r1,
            //     r2: commit.r2,
            // },
        }
    }

    pub fn prove_authentication(&self, user: User, challenge: Challenge) -> Answer {
        let c = challenge.c;
        let commit = self.commitments.get(&user.uuid);
        let r1: f32 = match commit {
            Some(cc) => cc.r1,
            Error => 0.0,
        };

        let x = self.agreement.x;
        let xx = (c * x) as f32;
        // let s = f32::abs(r1 - xx).try_into().unwrap() as f32 % self.q  as f32;
        let s = r1 - xx;
        let ss = s % self.q;
        // let s = (r1%self.q).abs() - (xx % self.q);
        // let s2 = r1 - xx % self.q;
        // let ss = s.abs() as u32;

        Answer { s: ss.abs() }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AuthenticationRequest {
    // user: User,
    // commit: RandomCommit,
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
    fn new() -> Self {
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
pub struct Commit {
    pub k: f32,
    pub r1: f32,
    pub r2: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct RandomCommit {
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
