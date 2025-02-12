use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateDelegatedRecoveryChallengeRequest, CreateDelegatedRecoveryChallengeRequestBody},
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

    let request = CreateDelegatedRecoveryChallengeRequest {
        body: CreateDelegatedRecoveryChallengeRequestBody {
            credential_id: "example-credential-id".to_string(),
            username: "example.user@domain.com".to_string(),
        },
    };

    match client.auth().create_delegated_recovery_challenge(request).await {
        Ok(response) => {
            println!("Delegated recovery challenge created successfully:");
            println!("  Temporary Auth Token: {}", response.temporary_authentication_token);
            println!("  Challenge: {}", response.challenge);
            println!("  OTP URL: {}", response.otp_url);
            println!("  Attestation: {:?}", response.attestation);
            
            if let Some(rp) = response.rp {
                println!("\nRelying Party Info:");
                println!("  ID: {}", rp.id);
                println!("  Name: {}", rp.name);
            }

            println!("\nSupported Credential Kinds:");
            println!("  First Factor: {:?}", response.supported_credential_kinds.first_factor);
            println!("  Second Factor: {:?}", response.supported_credential_kinds.second_factor);

            if !response.allowed_recovery_credentials.is_empty() {
                println!("\nAllowed Recovery Credentials:");
                for cred in response.allowed_recovery_credentials {
                    println!("  ID: {}", cred.id);
                    println!("  Encrypted Recovery Key: {}", cred.encrypted_recovery_key);
                }
            }
        }
        Err(e) => eprintln!("Error creating delegated recovery challenge: {:?}", e),
    }
} 