use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::ArchiveServiceAccountRequest,
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

    let request = ArchiveServiceAccountRequest {
        service_account_id: "example-service-account-id".to_string(),
    };

    match client.auth().archive_service_account(request).await {
        Ok(response) => {
            println!("Service Account archived successfully:");
            println!("User Info:");
            println!("  User ID: {}", response.user_info.user_id);
            println!("  Name: {}", response.user_info.name);
            println!("  Kind: {:?}", response.user_info.kind);
            println!("  Is Active: {}", response.user_info.is_active);
            println!("  Organization ID: {}", response.user_info.org_id);

            println!("\nAccess Tokens:");
            for token in response.access_tokens {
                println!("  Token ID: {}", token.token_id);
                println!("  Name: {}", token.name);
                println!("  Kind: {:?}", token.kind);
                println!("  Is Active: {}", token.is_active);
                println!();
            }
        }
        Err(e) => eprintln!("Error archiving service account: {:?}", e),
    }
} 