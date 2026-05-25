use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier, Signature};
use rand::rngs::OsRng; 

pub struct NodeIdentity {
    signing_key: SigningKey,
}
