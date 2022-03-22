use crate::zkp_structs::ServerCommitment;
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
    pub q: u128,
}

impl ZkpClient {
    pub fn new() -> Self {
        Self {
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            q: 19,
        }
    }

    pub fn agreement(&mut self) -> Agreement {
        let mut rng = rand::thread_rng();

        // (y1 == g^x1) && (y2 == h^x2)
        let g = rng.gen_range(1..100);
        let h = rng.gen_range(1..100);

        let x = rng.gen_range(1..16);
        let y1 = Math::pow2(g, x, self.q);
        let y2 = Math::pow2(h, x, self.q);

        let aggreement = Agreement { y1, y2, g, h, x, };
        self.agreement = aggreement;
        aggreement
    }

    pub fn create_register_commits(&mut self, user: User) -> ServerCommitment {
        let mut rng = rand::thread_rng();
        let k = rng.gen_range(1..16);
        let agr = self.agreement;
        let y1 = Math::pow2(agr.g, k, self.q);
        let y2 = Math::pow2(agr.h, k, self.q);
        let commitment = Commitment { k: k, r1: y1, r2: y2};
        self.commitments.insert(user.uuid, commitment);
        ServerCommitment { r1: commitment.r1, r2: commitment.r2 }
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
        // s = z, x = w, c = e, k = r
        let x = self.agreement.x;
        let c = challenge.c;
        let k = commitment.k;
        let q = self.q;
        let s = (x * c + k) %q;

        // let x = self.agreement.x;
        // let q = self.q;
        // let mul = (c  % q * x  % q) % q;
        // let sub = k % q;
        // println!("{} - {}", k, sub);
        // let (mut s, m) = sub.overflowing_sub(mul);
        // dbg!(s, m);
        // if m { s = mul };
        //let s = ((k - ((c * x) % q)) % q).abs();
        Some(Answer { s })
    }
}
