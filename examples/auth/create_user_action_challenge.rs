use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateUserActionChallengeRequest, CreateUserActionChallengeRequestBody, UserActionServerKind},
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

    let request = CreateUserActionChallengeRequest {
        body: CreateUserActionChallengeRequestBody {
            user_action_http_method: "POST".to_string(),
            user_action_http_path: "/api/auth/action".to_string(),
            user_action_payload: r#"{"action": "example_action"}"#.to_string(),
            user_action_server_kind: Some(UserActionServerKind::Api),
        },
    };

    match client.auth().create_user_action_challenge(request).await {
        Ok(response) => {
            println!("User action challenge created successfully:");
            println!("  Challenge Identifier: {}", response.challenge_identifier);
            println!("  Challenge: {}", response.challenge);
            println!("  User Verification: {:?}", response.user_verification);
            println!("  External Auth URL: {}", response.external_authentication_url);
            
            if let Some(rp) = response.rp {
                println!("\nRelying Party Info:");
                println!("  ID: {}", rp.id);
                println!("  Name: {}", rp.name);
            }

            println!("\nSupported Credential Kinds:");
            for kind in response.supported_credential_kinds {
                println!("  Factor: {:?}", kind.factor);
                println!("  Kind: {:?}", kind.kind);
                println!("  Requires Second Factor: {}", kind.requires_second_factor);
            }

            if let Some(allow_credentials) = &response.allow_credentials {
                println!("\nAllowed Credentials:");
                if !allow_credentials.key.is_empty() {
                    println!("  Keys:");
                    for key in &allow_credentials.key {
                        println!("    ID: {}", key.id);
                        println!("    Type: {:?}", key.key_type);
                    }
                }
                if let Some(webauthn) = &allow_credentials.webauthn {
                    println!("  WebAuthn:");
                    for cred in webauthn {
                        println!("    ID: {}", cred.id);
                        println!("    Type: {:?}", cred.webauthn_type);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error creating user action challenge: {:?}", e),
    }
} 