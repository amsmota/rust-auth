# rust-auth

The code in this project is based on the Learn Rust by Building Real Applications on Udemy, and then adapted to include a ZKP Authentication protocol based on the Chaum–Pedersen Protocol.

It has 3 endpoints spread to 2 modulos, **client** (Prover) and **server** (Verifier):

GET http://localhost:8080/client/agreement to create an aggrement betweem Prover and Verifier on the form (y1 = g^x) and (y2 = h^x)

GET http://localhost:8080/client/register to register a user along a Commitment in the form (r1, r2) = (g^k, h^k)

GET http://localhost:8080/server/auth?uuid=%uuid% from the ouput of the register call
  
This authentication step does the following operations

- creates a Authentication Request that is passed from the client to the server
- the server responds with a Authentication Challenge
- the client computes a Authentication Proof that is passed to the server as a Answer to the challenge
- the server then verifies the authentication and returns the result

Literature:

https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf#389

https://link.springer.com/article/10.1007/s10817-020-09581-w/figures/16
