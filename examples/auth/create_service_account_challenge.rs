use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateServiceAccountChallengeRequest, CreateServiceAccountChallengeRequestBody},
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

    let request = CreateServiceAccountChallengeRequest {
        body: CreateServiceAccountChallengeRequestBody {
            name: "Example Service Account".to_string(),
            external_id: Some("service123".to_string()),
        },
    };

    match client.auth().create_service_account_challenge(request).await {
        Ok(response) => {
            println!("Service account challenge created successfully:");
            println!("  Challenge Identifier: {}", response.challenge_identifier);
            println!("  Challenge: {}", response.challenge);
            println!("  External Auth URL: {}", response.external_authentication_url);
            println!("  User Verification: {:?}", response.user_verification);
            println!("  Attestation: {:?}", response.attestation);
            
            if let Some(rp) = response.rp {
                println!("\nRelying Party Info:");
                println!("  ID: {}", rp.id);
                println!("  Name: {}", rp.name);
            }

            println!("\nSupported Credential Kinds:");
            for kind in response.supported_credential_kinds {
                println!("  Kind: {:?}", kind.kind);
                println!("  Factor: {:?}", kind.factor);
                println!("  Requires Second Factor: {}", kind.requires_second_factor);
            }

            println!("\nService Account Info:");
            println!("  ID: {}", response.service_account.id);
            println!("  Name: {}", response.service_account.name);
            println!("  External ID: {:?}", response.service_account.external_id);
            println!("  Date Created: {}", response.service_account.date_created);
            println!("  Is Active: {}", response.service_account.is_active);
        }
        Err(e) => eprintln!("Error creating service account challenge: {:?}", e),
    }
} 