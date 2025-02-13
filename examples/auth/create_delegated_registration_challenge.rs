use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::auth::types::{
        CreateDelegatedRegistrationChallengeRequest,
        CreateDelegatedRegistrationChallengeRequestBody, UserInfoKind,
    },
    signer::{
        CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge,
    },
};
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
    async fn sign(
        &self,
        _challenge: UserActionChallenge,
    ) -> Result<FirstFactorAssertion, DfnsError> {
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

    let request = CreateDelegatedRegistrationChallengeRequest {
        body: CreateDelegatedRegistrationChallengeRequestBody {
            email: "example.user@domain.com".to_string(),
            external_id: Some("user123".to_string()),
            kind: UserInfoKind::EndUser,
        },
    };

    match client
        .auth()
        .create_delegated_registration_challenge(request)
        .await
    {
        Ok(response) => {
            println!("Delegated registration challenge created successfully:");
            println!(
                "  Temporary Auth Token: {}",
                response.temporary_authentication_token
            );
            println!("  Challenge: {}", response.challenge);
            println!("  OTP URL: {}", response.otp_url);
            println!("  Attestation: {:?}", response.attestation);

            if let Some(rp) = response.rp {
                println!("\nRelying Party Info:");
                println!("  ID: {}", rp.id);
                println!("  Name: {}", rp.name);
            }

            println!("\nSupported Credential Kinds:");
            println!(
                "  First Factor: {:?}",
                response.supported_credential_kinds.first_factor
            );
            println!(
                "  Second Factor: {:?}",
                response.supported_credential_kinds.second_factor
            );

            if !response.exclude_credentials.is_empty() {
                println!("\nExcluded Credentials:");
                for cred in response.exclude_credentials {
                    println!("  ID: {}", cred.id);
                    println!("  Type: {:?}", cred.exclude_credential_type);
                }
            }

            println!("\nUser Info:");
            println!("  ID: {}", response.user.id);
            println!("  Name: {}", response.user.name);
            println!("  Display Name: {}", response.user.display_name);
        }
        Err(e) => eprintln!("Error creating delegated registration challenge: {:?}", e),
    }
}
