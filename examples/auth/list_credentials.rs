use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
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

    match client.auth().list_credentials().await {
        Ok(response) => {
            println!("Credentials listed successfully:");
            for credential in response.items {
                println!("\nCredential:");
                println!("  Credential ID: {}", credential.credential_id);
                println!("  Credential UUID: {}", credential.credential_uuid);
                println!("  Name: {}", credential.name);
                println!("  Kind: {:?}", credential.kind);
                println!("  Is Active: {}", credential.is_active);
                println!("  Date Created: {}", credential.date_created);
            }
        }
        Err(e) => eprintln!("Error listing credentials: {:?}", e),
    }
}
