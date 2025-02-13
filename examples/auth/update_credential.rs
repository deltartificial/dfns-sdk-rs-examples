use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::auth::types::{UpdateCredentialRequest, UpdateCredentialRequestBody},
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

    let credential_id = "example-credential-id";
    let request = UpdateCredentialRequest {
        credential_id: credential_id.to_string(),
        body: UpdateCredentialRequestBody {
            name: "Updated Credential Name".to_string(),
        },
    };

    match client.auth().update_credential(request).await {
        Ok(response) => {
            println!("Credential updated successfully:");
            println!("  UUID: {}", response.uuid);
            println!("  Name: {}", response.name);
            println!("  Kind: {:?}", response.kind);
            println!("  Is Active: {}", response.is_active);
            println!("  Date Created: {}", response.date_created);
            println!("  Public Key: {}", response.public_key);
            println!("  Origin: {}", response.origin);
            println!("  Relying Party ID: {}", response.relying_party_id);
        }
        Err(e) => eprintln!("Error updating credential: {:?}", e),
    }
}
