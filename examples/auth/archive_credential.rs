use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::{
        types::ArchiveCredentialRequest,
        client::AuthClient,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

struct ExampleSigner {
    cred_id: String,
    signature: String,
}

impl ExampleSigner {
    fn new(cred_id: String, signature: String) -> Self {
        Self { cred_id, signature }
    }
}

#[async_trait]
impl CredentialSigner for ExampleSigner {
    async fn sign(&self, _challenge: UserActionChallenge) -> Result<FirstFactorAssertion, DfnsError> {
        Ok(FirstFactorAssertion {
            credential_assertion: None,
            kind: FirstFactorAssertionKind::Key,
            password: Some(self.signature.clone()),
        })
    }
}

#[tokio::main]
async fn main() {
    let signer = Arc::new(ExampleSigner::new(
        "example-cred-id".to_string(),
        "example-signature".to_string(),
    ));

    let base_options = DfnsBaseApiOptions {
        app_id: "your-app-id".to_string(),
        auth_token: Some("your-auth-token".to_string()),
        base_url: Some("https://api.dfns.ninja".to_string()),
        app_secret: None,
    };

    let client = DfnsApiClient::new(base_options, Some(signer));

    let request = ArchiveCredentialRequest {
        credential_id: "example-credential-uuid".to_string(),
    };

    match client.auth().archive_credential(request).await {
        Ok(response) => {
            println!("Credential archived successfully:");
            println!("  Credential ID: {}", response.credential_id);
            println!("  Name: {:?}", response.name);
            println!("  Kind: {:?}", response.kind);
            println!("  Is Active: {}", response.is_active);
            println!("  User ID: {}", response.user_id);
            
            if let Some(webauthn_info) = response.webauthn_info {
                println!("\nWebAuthn Info:");
                println!("  Credential ID: {}", webauthn_info.credential_id);
                println!("  Public Key: {}", webauthn_info.public_key);
                println!("  AAGUID: {}", webauthn_info.aaguid);
                println!("  Sign Count: {}", webauthn_info.sign_count);
                println!("  Attestation Type: {}", webauthn_info.attestation_type);
                println!("  Backup State: {}", webauthn_info.backup_state);
                println!("  Backup Eligible: {}", webauthn_info.backup_eligible);
            }
        }
        Err(e) => eprintln!("Error archiving credential: {:?}", e),
    }
} 