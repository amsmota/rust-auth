use crate::zkp_client::Answer;
use crate::zkp_client::AuthenticationRequest;
use crate::zkp_client::Agreement;
use crate::zkp_client::Commit;
use crate::zkp_client::User;
use rand::Rng;
use std::collections::HashMap;

pub struct ZkpServer {
    pub agreement: Agreement,
    commitments: HashMap<User, Commit>,
    challenge: Challenge
}

impl ZkpServer {
    pub fn new() -> Self {
        Self {
            agreement: Agreement { y1: 0.0, y2: 0.0, g: 0.0, h: 0.0, x: 0.0 },
            commitments: HashMap::new(),
            challenge: Challenge { c: 0.0 },
        }
    }

    pub fn register(&mut self, user: User, commit: Commit) {
        self.commitments.insert(user, commit);
    }

    pub fn create_authentication_challenge(&mut self, user: User, auth_request: AuthenticationRequest) -> Challenge {
        let mut rng = rand::thread_rng();
        let c = rng.gen_range(0..10) as f32;
        self.challenge = Challenge { c };

        self.challenge        
    }

    pub fn verify_authentication(&self, user: User, answer: Answer) -> bool {
        let commit = &self.commitments.get(&user);
        let (cr1, cr2) = match commit {
            Some(cc) => (cc.r1, cc.r2),
            Error => (0.0, 0.0),
        };


        let s = answer.s;
        let c = self.challenge.c;

        let g1 = f32::powf(self.agreement.g, s);
        let y1 = f32::powf(self.agreement.y1, c);
        let h1 = f32::powf(self.agreement.h, s);
        let y2 = f32::powf(self.agreement.y2, c);

        let r1 = g1 * y1;
        let r2 = h1 * y2;

        dbg!(r1);
        dbg!(r2);

        r1 == cr1 && r2 == cr2
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Challenge {
    pub c: f32,
}
