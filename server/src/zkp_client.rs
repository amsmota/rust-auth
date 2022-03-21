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
    q: u32,
}

impl ZkpClient {
    pub fn new() -> Self {
        Self {
            agreement: Agreement::new(),
            commitments: HashMap::new(),
            q: 107,
        }
    }

    pub fn agreement(&mut self) -> Agreement {
        let mut rng = rand::thread_rng();
        // rng.gen::<u128>();

        // (y1 == g^x1) && (y2 == h^x2)
        // y1, y2, x1 = x2, k, r, c, s, g, h, y, q
        // y1  y2              r  C  s  g  B     q

        //  a, b, A, , C,  z
        // (g,g^a, g^b and g^ab) =(",g,A,B,C,")")

        //self.q = rng.gen_range(1..100);

        let g = (rng.gen_range(1..100) as u32) as u32;
        let h = (rng.gen_range(1..100) as u32) as u32;

        let x = (rng.gen_range(1..10) as u32) as u32;
        let y1 = Math::pow2(g, x, self.q);
        let y2 = Math::pow2(h, x, self.q);

        let aggreement = Agreement { y1, y2, g, h, x, };
        self.agreement = aggreement;
        aggreement
    }

    pub fn create_register_commits(&mut self, user: User) -> ServerCommitment {
        let mut rng = rand::thread_rng();
        let k = (rng.gen_range(1..10) as u32) as u32;
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

    pub fn prove_authentication(&self, user: User, challenge: Challenge) -> Answer {
        let c = challenge.c;
        let commitment = self.commitments.get(&user.uuid);
        let k: u32 = match commitment {
            Some(cc) => cc.k,
            None => 0,
        };
        if k == 0 {
            return Answer { s: 0 };
        }

        // s = k - c * x (mod q)
        let x = self.agreement.x;
        let q = self.q;
        let m = (c * x) % q;
        let s = (k - m) % q;
        //let s = ((k - ((c * x) % q)) % q).abs();

        Answer { s }
    }












    
}
