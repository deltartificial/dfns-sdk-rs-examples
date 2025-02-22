use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::auth::types::ArchiveApplicationRequest,
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

    let request = ArchiveApplicationRequest {
        app_id: "example-app-id".to_string(),
    };

    match client.auth().archive_application(request).await {
        Ok(response) => {
            println!("Application archived successfully:");
            println!("  App ID: {}", response.app_id);
            println!("  Name: {:?}", response.name);
            println!("  Kind: {:?}", response.kind);
            println!("  Is Active: {}", response.is_active);
            println!("  Expected Origin: {:?}", response.expected_origin);
            println!("  Expected RP ID: {:?}", response.expected_rp_id);

            if !response.access_tokens.is_empty() {
                println!("\nAccess Tokens:");
                for token in &response.access_tokens {
                    println!("  Token ID: {}", token.token_id);
                    println!("  Name: {}", token.name);
                    println!("  Kind: {:?}", token.kind);
                    println!("  Is Active: {}", token.is_active);
                }
            }
        }
        Err(e) => eprintln!("Error archiving application: {:?}", e),
    }
}
