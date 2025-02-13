use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateRegistrationChallengeRequest, CreateRegistrationChallengeRequestBody, UserInfoKind},
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

    let request = CreateRegistrationChallengeRequest {
        body: CreateRegistrationChallengeRequestBody {
            username: "example.user@domain.com".to_string(),
            org_id: "example-org-id".to_string(),
            registration_code: "123456".to_string(),
        },
    };

    match client.auth().create_registration_challenge(request).await {
        Ok(response) => {
            println!("Registration challenge created successfully:");
            println!("  Challenge: {}", response.challenge);
            println!("  Attestation: {:?}", response.attestation);
            println!("  OTP URL: {}", response.otp_url);
            println!("  Temporary Auth Token: {}", response.temporary_authentication_token);
            
            if let Some(rp) = response.rp {
                println!("\nRelying Party Info:");
                println!("  ID: {}", rp.id);
                println!("  Name: {}", rp.name);
            }

            println!("\nExcluded Credentials:");
            for cred in response.exclude_credentials {
                println!("  ID: {}", cred.id);
                println!("  Type: {:?}", cred.exclude_credential_type);
            }

            println!("\nSupported Credential Kinds:");
            println!("  First Factor: {:?}", response.supported_credential_kinds.first_factor);
            println!("  Second Factor: {:?}", response.supported_credential_kinds.second_factor);

            println!("\nUser Info:");
            println!("  ID: {}", response.user.id);
            println!("  Name: {}", response.user.name);
            println!("  Display Name: {}", response.user.display_name);
        }
        Err(e) => eprintln!("Error creating registration challenge: {:?}", e),
    }
} 