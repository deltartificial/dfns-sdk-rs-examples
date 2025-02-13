use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::auth::types::DeactivatePersonalAccessTokenRequest,
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

    let request = DeactivatePersonalAccessTokenRequest {
        token_id: "example-token-id".to_string(),
    };

    match client
        .auth()
        .deactivate_personal_access_token(request)
        .await
    {
        Ok(response) => {
            println!("Personal access token deactivated successfully:");
            println!("  Token ID: {}", response.token_id);
            println!("  Is Active: {}", response.is_active);
            println!("  Name: {}", response.name);
            println!("  Kind: {:?}", response.kind);
        }
        Err(e) => eprintln!("Error deactivating personal access token: {:?}", e),
    }
}
