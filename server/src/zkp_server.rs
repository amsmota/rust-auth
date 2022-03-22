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
        let b = rng.gen_range(1..10);
        let x = rng.gen_range(1..10);
        let c = Math::pow2(b, x, self.q);
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
        let g = commitment.r1;
        let s = answer.s;
        let y1 = self.agreement.y1;
        let c = self.challenge.c;
        let h = commitment.r2;
        let y2 = self.agreement.y2;

        let gs = Math::pow2(g, s, self.q);
        let hs = Math::pow2(h, s, self.q);

        let yc1 = Math::pow2(y1, c, self.q);
        let yc2 = Math::pow2(y2, c, self.q);

        let rc1 = (gs * yc1) % self.q ;
        let rc2 = (hs * yc2) % self.q;

        dbg!(rc1);
        dbg!(rc2);

        let rr = rc1 == commitment.r1 && rc2 == commitment.r2;
        format!("{}: ({} = {}) and ({} = {})", rr, rc1, commitment.r1, rc2, commitment.r2)

    }
}
