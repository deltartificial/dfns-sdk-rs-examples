use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreatePersonalAccessTokenRequest, CreatePersonalAccessTokenRequestBody},
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

    let request = CreatePersonalAccessTokenRequest {
        body: CreatePersonalAccessTokenRequestBody {
            name: "Example Token".to_string(),
            permission_id: Some("example-permission-id".to_string()),
            days_valid: Some(30.0),
            external_id: Some("example-external-id".to_string()),
            public_key: "example-public-key".to_string(),
            seconds_valid: None,
        },
    };

    match client.auth().create_personal_access_token(request).await {
        Ok(response) => {
            println!("Personal Access Token created successfully:");
            println!("  Token ID: {}", response.token_id);
            println!("  Name: {:?}", response.name);
            println!("  Kind: {:?}", response.kind);
            println!("  Is Active: {}", response.is_active);
            println!("  Organization ID: {}", response.org_id);
            println!("  Date Created: {}", response.date_created);
            println!("  Access Token: {}", response.access_token);
            println!("  Credential ID: {}", response.cred_id);
        }
        Err(e) => eprintln!("Error creating personal access token: {:?}", e),
    }
} 