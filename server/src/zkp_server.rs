use crate::zkp_structs::Math;
use crate::zkp_structs::Challenge;
use crate::zkp_structs::Answer;
use crate::zkp_structs::AuthenticationRequest;
use crate::zkp_structs::Agreement;
use crate::zkp_structs::Commitment;
use crate::zkp_structs::User;
use rand::Rng;
use std::collections::HashMap;

pub struct ZkpServer {
    pub agreement: Agreement,
    commitments: HashMap<User, Commitment>,
    challenge: Challenge
}

impl ZkpServer {
    pub fn new() -> Self {
        Self {
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            challenge: Challenge { c: 0.0 },
        }
    }

    pub fn register(&mut self, user: User, Commitment: Commitment) {
        self.commitments.insert(user, Commitment);
    }

    pub fn create_authentication_challenge(&mut self, user: User, auth_request: AuthenticationRequest) -> Challenge {
        let mut rng = rand::thread_rng();
        let c = 23.0; //rng.gen_range(1..10) as f32;
        self.challenge = Challenge { c };

        self.challenge
    }

    pub fn verify_authentication(&self, user: User, answer: Answer) -> bool {
        let cc = self.commitments.get(&user);
        let commitment = match cc {
            Some(cc) => cc,
            None => &Commitment {k: 0.0, r1: 0.0, r2: 0.0 },
        };

        // r1 = g^s * y1^c and r2 = h^s * y2^c
        let g = commitment.r1;
        let s = answer.s;
        let y1 = self.agreement.y1;
        let c = self.challenge.c;
        let h = commitment.r2;
        let y2 = self.agreement.y2;

        let gs = Math::pow2(g, s, 107.0);
        let hs = Math::pow2(h, s, 107.0);

        let yc1 = Math::pow2(y1, c, 107.0);
        let yc2 = Math::pow2(y2, c, 107.0);

        let rc1 = gs * yc1;
        let rc2 = hs * yc2;

        dbg!(rc1);
        dbg!(rc2);

        println!("({} = {} and {} = {}", rc1, commitment.r1, rc2, commitment.r2);
        rc1 == commitment.r1 && rc2 == commitment.r2

    }
}
