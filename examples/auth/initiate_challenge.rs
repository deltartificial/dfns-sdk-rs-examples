use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{InitiateChallengeRequest, UserAction},
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

    let request = InitiateChallengeRequest {
        action: UserAction::SignIn,
        credential_uuid: Some("example-credential-uuid".to_string()),
    };

    match client.auth().initiate_challenge(request).await {
        Ok(response) => {
            println!("Challenge initiated successfully:");
            println!("  Challenge ID: {}", response.challenge_id);
            println!("  Action: {:?}", response.action);
            println!("  Status: {:?}", response.status);
            println!("  Expiration: {}", response.expiration);
            println!("  Organization ID: {}", response.org_id);
            println!("  User ID: {}", response.user_id);
            
            if let Some(credential_uuid) = response.credential_uuid {
                println!("  Credential UUID: {}", credential_uuid);
            }
            
            if let Some(challenge_data) = response.challenge_data {
                println!("\nChallenge Data:");
                println!("  Challenge: {}", challenge_data.challenge);
                println!("  RP ID: {}", challenge_data.rp_id);
                println!("  Origin: {}", challenge_data.origin);
                if let Some(user_verification) = challenge_data.user_verification {
                    println!("  User Verification: {}", user_verification);
                }
            }
        }
        Err(e) => eprintln!("Error initiating challenge: {:?}", e),
    }
} 