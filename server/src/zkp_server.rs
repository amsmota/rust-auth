use crate::zkp_structs::ServerCommitment;
use crate::zkp_structs::Challenge;
use crate::zkp_structs::Answer;
use crate::zkp_structs::AuthenticationRequest;
use crate::zkp_structs::Agreement;
use crate::zkp_structs::User;
use std::collections::HashMap;
use rand::Rng;
use num::BigUint;
use num::Zero;
use num::ToPrimitive;

pub struct ZkpServer {
    z: BigUint,
    pub agreement: Agreement,
    pub q: BigUint,
    commitments: HashMap<User, ServerCommitment>,
    challenge: Challenge,
}

impl ZkpServer {
    pub fn new() -> Self {
        Self {
            z: BigUint::zero(),
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            challenge: Challenge { c: 0 },
            q: BigUint::from(10009u128),
        }
    }

    pub fn register(&mut self, user: User, commitment: ServerCommitment) {
        self.commitments.insert(user, commitment);
    }

    pub fn create_authentication_challenge(&mut self, user: User, auth_request: AuthenticationRequest) -> Challenge {
        let mut rng = rand::thread_rng();
        let b = BigUint::from(rng.gen_range(2..100)as u32);
        let x = BigUint::from(rng.gen_range(2..100)as u32);
        let c: u32 = BigUint::from(b).modpow(&x, &self.q).to_u32().unwrap();
        self.challenge = Challenge { c: 2 };
        self.challenge
    }

    pub fn verify_authentication(&self, user: User, answer: Answer) -> String {
        let cc = self.commitments.get(&user);
        let commitment = match cc {
            Some(cc) => cc,
            None => return "uuid not found".to_string(),
        };

        dbg!(answer.s);

        // r1 = g^s * y1^c and r2 = h^s * y2^c
        let agreement = self.agreement.clone();
        dbg!(&self.agreement);
        dbg!(&agreement);
        dbg!(&agreement.g);
        let g = agreement.g;
        let s = answer.s;
        let y1 = agreement.y1;
        let c = self.challenge.c;
        let h = agreement.h;
        let y2 = agreement.y2;

        // let gs = Math::pow2(g, s, self.q);
        // let hs = Math::pow2(h, s, self.q);

        // let yc1 = Math::pow2(y1, c, self.q);
        // let yc2 = Math::pow2(y2, c, self.q);

        let gs = BigUint::from(g).pow(s) % &self.q;

        let hs = BigUint::from(h).pow(s) % &self.q;

        let yc1 = BigUint::from(y1).pow(c) % &self.q;


        let yc2 = BigUint::from(y2).pow(c) % &self.q;

        let rc1 = (gs) % &self.q;
        let rc2 = (yc1*&commitment.r1) ;

        let rr = rc1 == commitment.r1 && rc2 == commitment.r2;
        format!("{}: ({} = {}) and ({} = {})", rr, rc1, commitment.r1, rc2, commitment.r2)
    }
}
