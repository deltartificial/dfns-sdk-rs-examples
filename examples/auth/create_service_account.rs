use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateServiceAccountRequest, CreateServiceAccountRequestBody},
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

    let request = CreateServiceAccountRequest {
        body: CreateServiceAccountRequestBody {
            name: "Example Service Account".to_string(),
            days_valid: Some(30.0),
            external_id: Some("example-external-id".to_string()),
            permission_id: Some("example-permission-id".to_string()),
            public_key: "example-public-key".to_string(),
        },
    };

    match client.auth().create_service_account(request).await {
        Ok(response) => {
            println!("Service Account created successfully:");
            println!("  User ID: {}", response.user_info.user_id);
            println!("  Username: {}", response.user_info.username);
            println!("  Name: {}", response.user_info.name);
            println!("  Kind: {:?}", response.user_info.kind);
            println!("  Is Active: {}", response.user_info.is_active);
            println!("  Is Registered: {}", response.user_info.is_registered);
            println!("  Is Service Account: {}", response.user_info.is_service_account);
            println!("  Organization ID: {}", response.user_info.org_id);
            
            println!("\nAccess Tokens:");
            for token in response.access_tokens {
                println!("  Token ID: {}", token.token_id);
                println!("  Name: {}", token.name);
                println!("  Kind: {:?}", token.kind);
                println!("  Is Active: {}", token.is_active);
                println!("  Organization ID: {}", token.org_id);
                println!("  Linked User ID: {}", token.linked_user_id);
                println!("  Linked App ID: {}", token.linked_app_id);
                println!("  Date Created: {}", token.date_created);
            }
        }
        Err(e) => eprintln!("Error creating service account: {:?}", e),
    }
} 