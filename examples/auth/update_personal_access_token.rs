use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{UpdatePersonalAccessTokenRequest, UpdatePersonalAccessTokenRequestBody},
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

    let token_id = "example-token-id";
    let request = UpdatePersonalAccessTokenRequest {
        token_id: token_id.to_string(),
        body: UpdatePersonalAccessTokenRequestBody {
            name: Some("Updated Token Name".to_string()),
            external_id: None,
        },
    };

    match client.auth().update_personal_access_token(request).await {
        Ok(response) => {
            println!("Personal access token updated successfully:");
            println!("  Name: {}", response.name);
            println!("  Is Active: {}", response.is_active);
            println!("  Kind: {:?}", response.kind);
            println!("  Date Created: {}", response.date_created);
            println!("  Access Token: {:?}", response.access_token);
        }
        Err(e) => eprintln!("Error updating personal access token: {:?}", e),
    }
} 