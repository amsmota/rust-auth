use crate::zkp_structs::Agreement;
use crate::zkp_structs::Commitment;
use crate::zkp_structs::Answer;
use crate::zkp_structs::AuthenticationRequest;
use crate::zkp_structs::Challenge;
use crate::zkp_structs::Math;
use crate::zkp_structs::User;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ZkpClient {
    agreement: Agreement,
    commitments: HashMap<Uuid, Commitment>,
    q: f32,
}

impl ZkpClient {
    pub fn new() -> Self {
        Self {
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            q: 107.0,
        }
    }

    pub fn agreement(&mut self) -> Agreement {
        let (aggreement, _) = self.protocol();
        self.agreement = aggreement;
        aggreement
    }

    pub fn create_register_commits(&mut self, user: User) -> Commitment {
        let mut rng = rand::thread_rng();
        let k = (rng.gen_range(1..10) as u32) as f32;
        let agr = self.agreement;
        let y1 = Math::pow2(agr.g, k, self.q);
        let y2 = Math::pow2(agr.h, k, self.q);
        let commitment = Commitment { k: k, r1: y1, r2: y2};
        self.commitments.insert(user.uuid, commitment);
        commitment
    }

    fn protocol(&mut self) -> (Agreement, Commitment) {
        let mut rng = rand::thread_rng();
        // rng.gen::<u128>();

        // (y1 == g^x1) && (y2 == h^x2)
        // y1, y2, x1 = x2, k, r, c, s, g, h, y, q
        // y1  y2              r  C  s  g  B     q

        //  a, b, A, , C,  z
        // (g,g^a, g^b and g^ab) =(",g,A,B,C,")")

        //self.q = rng.gen_range(1..100);

        let g = 5.0; //(rng.gen_range(1..100) as u32) as f32;
        let h = 13.0; //(rng.gen_range(1..100) as u32) as f32;

        let x = 3.0; //(rng.gen_range(1..10) as u32) as f32;
        let y1 = Math::pow2(g, x, self.q);
        let y2 = Math::pow2(h, x, self.q);

        let agreement = Agreement { y1, y2, g, h, x, };
        let commitment = Commitment {
            k: x,
            r1: y1,
            r2: y2,
        };

        (agreement, commitment)
    }

    pub fn create_authentication_request(&mut self, user: User) -> AuthenticationRequest {
        // let Commitment = self.create_register_commits(user);
        AuthenticationRequest {
            // user,
            // Commitment: RandomCommit {
            //     r1: Commitment.r1,
            //     r2: Commitment.r2,
            // },
        }
    }

    pub fn prove_authentication(&self, user: User, challenge: Challenge) -> Answer {
        let c = challenge.c;
        let commitment = self.commitments.get(&user.uuid);
        let k: f32 = match commitment {
            Some(cc) => cc.k,
            None => 0.0,
        };
        if k == 0.0 {
            return Answer { s: 0.0 };
        }

        // s = k - c * x (mod q)
        let x = self.agreement.x;
        let q = self.q;
        //let s = (k - c%q * x%q).abs();

        let s = ((k - ((c * x) % q)) % q).abs();

        // let x = self.agreement.x;
        // let xx = (c * x) as f32;
        // // let s = f32::abs(r1 - xx).try_into().unwrap() as f32 % self.q  as f32;
        // let s = k - xx;
        // let ss = s % self.q;
        // // let s = (r1%self.q).abs() - (xx % self.q);
        // // let s2 = r1 - xx % self.q;
        // // let ss = s.abs() as u32;

        Answer { s }
    }
}
