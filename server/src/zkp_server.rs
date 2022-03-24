use crate::zkp_structs::ServerCommitment;
use crate::zkp_structs::Math;
use crate::zkp_structs::Challenge;
use crate::zkp_structs::Answer;
use crate::zkp_structs::AuthenticationRequest;
use crate::zkp_structs::Agreement;
use crate::zkp_structs::User;
use std::collections::HashMap;
use rand::Rng;

pub struct ZkpServer {
    pub agreement: Agreement,
    commitments: HashMap<User, ServerCommitment>,
    challenge: Challenge,
    pub q: u128,
}

impl ZkpServer {
    pub fn new() -> Self {
        Self {
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            challenge: Challenge { c: 0 },
            q: 19,
        }
    }

    pub fn register(&mut self, user: User, commitment: ServerCommitment) {
        self.commitments.insert(user, commitment);
    }

    pub fn create_authentication_challenge(&mut self, user: User, auth_request: AuthenticationRequest) -> Challenge {
        let mut rng = rand::thread_rng();
        // let b = rng.gen_range(2..100);
        // let x = rng.gen_range(2..10);
        // let c = Math::modpow(b, x, self.q);
        let c = rng.gen_range(1..10);
        self.challenge = Challenge { c };
        self.challenge
    }

    pub fn verify_authentication(&self, user: User, answer: Answer) -> String {
        let cc = self.commitments.get(&user);
        let commitment = match cc {
            Some(cc) => cc,
            None => &ServerCommitment {r1: 0, r2: 0 },
        };

        dbg!(answer.s);

        // r1 = g^s * y1^c and r2 = h^s * y2^c
        let g = self.agreement.g;
        let s = answer.s;
        let y1 = self.agreement.y1;
        let c = self.challenge.c;
        let h = self.agreement.h;
        let y2 = self.agreement.y2;

        let gs = Math::imodpow(g, s, self.q);
        let hs = Math::imodpow(h, s, self.q);

        let yc1 = Math::umodpow(y1, c, self.q);
        let yc2 = Math::umodpow(y2, c, self.q);

        let rc1 = Math::imul(gs, yc1);
        let rc2 = Math::imul(hs, yc2);

        let rr = rc1 == commitment.r1 as i128 && rc2 == commitment.r2 as i128;
        format!("{}: ({} = {}) and ({} = {})", rr, rc1, commitment.r1, rc2, commitment.r2)
    }
}
