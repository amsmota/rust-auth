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
                    let Commitment = self.client.create_register_commits(user);
                    let resp = format!("uuid: {}\n\r (r1, r2) = ({}, {})", user.uuid, &Commitment.r1, &Commitment.r2);
                    self.server.register(user, Commitment);
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
                    if answer.s == 0.0 {
                        return Response::new(StatusCode::NotFound, Some("uuid not found".to_string()));
                    }
                    let authenticated = self.server.verify_authentication(user, answer);
                    dbg!(authenticated);
                    Response::new(StatusCode::Ok, Some(format!("{}", authenticated)))
                }
                _ => Response::new(StatusCode::NotFound, None)
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
