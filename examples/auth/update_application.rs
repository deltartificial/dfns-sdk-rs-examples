use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{UpdateApplicationRequest, UpdateApplicationRequestBody},
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

    let app_id = "example-app-id";
    let request = UpdateApplicationRequest {
        app_id: app_id.to_string(),
        body: UpdateApplicationRequestBody {
            name: Some("Updated Application Name".to_string()),
            external_id: Some("updated-app-123".to_string()),
        },
    };

    match client.auth().update_application(request).await {
        Ok(response) => {
            println!("Application updated successfully:");
            println!("  App ID: {}", response.app_id);
            println!("  Expected Origin: {:?}", response.expected_origin);
            println!("  Expected RP ID: {:?}", response.expected_rp_id);
            println!("  Is Active: {}", response.is_active);
            println!("  Name: {:?}", response.name);
            println!("  Access Tokens: {:?}", response.access_tokens);
            println!("  Permissions: {:?}", response.permissions);
            println!("  Signing Keys: {:?}", response.signing_keys);
            println!("  Webhooks: {:?}", response.webhooks);
        }
        Err(e) => eprintln!("Error updating application: {:?}", e),
    }
} 