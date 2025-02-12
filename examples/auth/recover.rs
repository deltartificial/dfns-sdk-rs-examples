use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{RecoverRequest, RecoverRequestBody, RecoveryCredential, RecoveryCredentialInfo},
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

    let request = RecoverRequest {
        body: RecoverRequestBody {
            challenge_identifier: "example-challenge-id".to_string(),
            recovery_credential: RecoveryCredential {
                credential_info: RecoveryCredentialInfo {
                    attestation_data: None,
                    client_data: None,
                    cred_id: None,
                    password: Some("example-password".to_string()),
                },
                credential_name: "Recovery Credential".to_string(),
                encrypted_private_key: None,
            },
        },
    };

    match client.auth().recover(request).await {
        Ok(response) => {
            println!("Recovery successful:");
            println!("\nUser Info:");
            println!("  ID: {}", response.user.id);
            println!("  Organization ID: {}", response.user.org_id);
            println!("  Username: {}", response.user.username);
            println!("  External ID: {:?}", response.user.external_id);
            println!("  Is Active: {}", response.user.is_active);
            println!("  Date Created: {}", response.user.date_created);
            
            println!("\nCredential Info:");
            println!("  UUID: {}", response.credential.uuid);
            println!("  Name: {}", response.credential.name);
            println!("  Kind: {:?}", response.credential.kind);
            println!("  Is Active: {}", response.credential.is_active);
            println!("  Date Created: {}", response.credential.date_created);
        }
        Err(e) => eprintln!("Error during recovery: {:?}", e),
    }
} 