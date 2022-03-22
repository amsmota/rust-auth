use uuid::Uuid;
use crate::zkp_server::ZkpServer;
use crate::zkp_client::ZkpClient;
use crate::zkp_structs::User;
use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};


pub struct AuthHandler {
    client: ZkpClient,
    server: ZkpServer,
}

impl AuthHandler {
    pub fn new() -> Self {
        Self { client: ZkpClient::new(), server: ZkpServer::new() }
    }
}

impl Handler for AuthHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {

                "/client/agreement" => {
                    let aggreement = self.client.agreement();
                    self.server.agreement = aggreement;
                    Response::new(StatusCode::Ok, Some(aggreement.to_string()))
                },
                
                "/client/register" => {
                    let user = User::new();
                    let commitment = self.client.create_register_commits(user);
                    let resp = format!("uuid: {}\n\r (r1, r2) = ({}, {})", user.uuid, &commitment.r1, &commitment.r2);
                    self.server.register(user, commitment);
                    Response::new(StatusCode::Ok, Some(resp))
                },

                "/server/auth" => {
                    let uuid = request.query_string().unwrap().get_as_text("uuid");
                    let user = User{uuid: Uuid::parse_str(uuid).unwrap()};
                    dbg!(user);
                    let auth_request = self.client.create_authentication_request(user);
                    dbg!(auth_request);
                    let challenge = self.server.create_authentication_challenge(user, auth_request);
                    dbg!(challenge);
                    let answer = self.client.prove_authentication(user, challenge);
                    dbg!(answer);
                    if let None = answer {
                        return Response::new(StatusCode::NotFound, Some("uuid not found".to_string()));
                    }
                    let authenticated = self.server.verify_authentication(user, answer.unwrap());
                    dbg!(&authenticated);
                    Response::new(StatusCode::Ok, Some(authenticated))
                },
                "/server/q" => {
                    let qq = request.query_string().unwrap().get_as_text("q").to_string();
                    self.client.q = qq.parse::<u128>().unwrap();
                    self.server.q = qq.parse::<u128>().unwrap();
                    Response::new(StatusCode::Ok, None)
                },

                _ => Response::new(StatusCode::NotFound, None)
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
