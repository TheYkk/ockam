use super::CredentialSchema;
use ockam_core::lib::*;

/// A list of the accepted schemas, public keys, and required to be revealed
/// attributes from a verifier
#[derive(Debug, Clone)]
pub struct PresentationManifest {
    /// The credential schema associated with the public key
    pub credential_schema: CredentialSchema,
    /// The public key of the issuer
    pub public_key: [u8; 96],
    /// The attributes required to be revealed
    pub revealed: Vec<usize>,
}
