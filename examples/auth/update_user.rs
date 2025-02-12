use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{UpdateUserRequest, UpdateUserRequestBody},
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

    let user_id = "example-user-id";
    let request = UpdateUserRequest {
        user_id: user_id.to_string(),
        body: UpdateUserRequestBody {
            external_id: Some("updated-user-123".to_string()),
        },
    };

    match client.auth().update_user(request).await {
        Ok(response) => {
            println!("User updated successfully:");
            println!("  ID: {}", response.id);
            println!("  Organization ID: {}", response.org_id);
            println!("  Username: {}", response.username);
            println!("  External ID: {:?}", response.external_id);
            println!("  Is Active: {}", response.is_active);
            println!("  Date Created: {}", response.date_created);
            println!("  Kind: {:?}", response.kind);
            
            if let Some(permissions) = response.permissions {
                println!("\nPermissions:");
                for permission in permissions {
                    println!("  - {}", permission);
                }
            }
        }
        Err(e) => eprintln!("Error updating user: {:?}", e),
    }
} 