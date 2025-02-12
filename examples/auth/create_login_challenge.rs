use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateLoginChallengeRequest, CreateLoginChallengeRequestBody},
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

    let request = CreateLoginChallengeRequest {
        body: CreateLoginChallengeRequestBody {
            login_code: Some("123456".to_string()),
            org_id: "example-org-id".to_string(),
            username: "example.user@domain.com".to_string(),
        },
    };

    match client.auth().create_login_challenge(request).await {
        Ok(response) => {
            println!("Login challenge created successfully:");
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

            println!("\nAllowed Credentials:");
            let allow_creds = response.allow_credentials;
            
            if !allow_creds.key.is_empty() {
                println!("  Keys:");
                for key in allow_creds.key {
                    println!("    ID: {}", key.id);
                    println!("    Type: {:?}", key.key_type);
                }
            }

            if let Some(password_keys) = allow_creds.password_protected_key {
                println!("  Password Protected Keys:");
                for key in password_keys {
                    println!("    ID: {}", key.id);
                    println!("    Type: {:?}", key.password_protected_key_type);
                }
            }

            if !allow_creds.webauthn.is_empty() {
                println!("  WebAuthn:");
                for key in allow_creds.webauthn {
                    println!("    ID: {}", key.id);
                    println!("    Type: {:?}", key.webauthn_type);
                }
            }

            println!("\nSupported Credential Kinds:");
            for kind in response.supported_credential_kinds {
                println!("  Kind: {:?}", kind.kind);
                println!("  Factor: {:?}", kind.factor);
                println!("  Requires Second Factor: {}", kind.requires_second_factor);
            }
        }
        Err(e) => eprintln!("Error creating login challenge: {:?}", e),
    }
} 