use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateCredentialWithCodeRequest, CreateCredentialWithCodeBody, CreateCredentialWithCodeBodyCredentialInfo, CredentialKindElement},
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

    let request = CreateCredentialWithCodeRequest {
        body: CreateCredentialWithCodeBody {
            challenge_identifier: "example-challenge-id".to_string(),
            credential_info: CreateCredentialWithCodeBodyCredentialInfo {
                attestation_data: None,
                client_data: None,
                cred_id: None,
                password: Some("example-password".to_string()),
                otp_code: None,
            },
            credential_kind: CredentialKindElement::Password,
            credential_name: "My Password Credential".to_string(),
            encrypted_private_key: None,
        },
    };

    match client.auth().create_credential_with_code(request).await {
        Ok(response) => {
            println!("Credential created successfully:");
            println!("  Credential ID: {}", response.credential_id);
            println!("  Credential UUID: {}", response.credential_uuid);
            println!("  Kind: {:?}", response.kind);
            println!("  Name: {}", response.name);
            println!("  Origin: {}", response.origin);
            println!("  Public Key: {}", response.public_key);
            println!("  Relying Party ID: {}", response.relying_party_id);
            println!("  Date Created: {}", response.date_created);
            println!("  Is Active: {}", response.is_active);
        }
        Err(e) => eprintln!("Error creating credential with code: {:?}", e),
    }
} 