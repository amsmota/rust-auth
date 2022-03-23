use crate::zkp_structs::Agreement;
use crate::zkp_structs::Answer;
use crate::zkp_structs::AuthenticationRequest;
use crate::zkp_structs::Challenge;
use crate::zkp_structs::Commitment;
use crate::zkp_structs::ServerCommitment;
use crate::zkp_structs::User;
use num::BigUint;
use num::ToPrimitive;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ZkpClient {
    agreement: Agreement,
    commitments: HashMap<Uuid, Commitment>,
    pub q: BigUint,
}

impl ZkpClient {
    pub fn new() -> Self {
        Self {
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            q: BigUint::from(10009u128),
        }
    }

    pub fn agreement(&mut self) -> Agreement {
        let mut rng = rand::thread_rng();

        // (y1 == g^x1) && (y2 == h^x2)
        let g = BigUint::from(rng.gen_range(1..10)as u32);
        let h = BigUint::from(rng.gen_range(1..10)as u32);

        let x =  BigUint::from(3u32);
        let y1 = BigUint::from(g.clone()).modpow(&x, &self.q);
        let y2 = BigUint::from(h.clone()).modpow(&x, &self.q);
        let aggreement = Agreement { y1, y2, g, h, x };
        self.agreement = aggreement;
        self.agreement.clone()
    }

    pub fn create_register_commits(&mut self, user: User) -> ServerCommitment {
        let mut rng = rand::thread_rng();
        let k =  BigUint::from(9u32); //BigUint::from(rng.gen_range(1..100)as u32) % &self.q;
        let agreement = self.agreement.clone();

        let y1 = BigUint::from(agreement.g).modpow(&k, &self.q);
        let y2 = BigUint::from(agreement.h).modpow(&k, &self.q);
        let commitment = Commitment {
            k: BigUint::from(9u32),
            r1: agreement.y1,
            r2: agreement.y2,
        };
        let serverCommitment = ServerCommitment {
            r1: commitment.r1.clone(),
            r2: commitment.r2.clone(),
        };
        self.commitments.insert(user.uuid, commitment);
        serverCommitment
    }

    pub fn create_authentication_request(&mut self, user: User) -> AuthenticationRequest {
        // let Commitment = self.create_register_commits(user);
        AuthenticationRequest {
            // user,
            // Commitment: ServerCommitment {
            //     r1: Commitment.r1,
            //     r2: Commitment.r2,
            // },
        }
    }

    pub fn prove_authentication(&self, user: User, challenge: Challenge) -> Option<Answer> {
        let cc = self.commitments.get(&user.uuid);
        let commitment = match cc {
            Some(cc) => cc,
            None => return None,
        };

        // s = k - c * x (mod q)
        // z = w * e + r (mod q)
        // z = s, w = x, e = c, r = k
        let x = &self.agreement.x;
        let c = challenge.c;
        let k = &commitment.k;
        let q = &self.q;
        let s = if k >= &(c * x) {
            (k + c * x) % q
        } else {
            (x * c + k) % q
        };
        println!("{} / {} / {}  / {}", k, x, x * c, k >= &(c * x));
        Some(Answer {
            s: s.to_u32().unwrap(),
        })
    }
}
