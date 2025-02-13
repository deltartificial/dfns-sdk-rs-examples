use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::permissions::types::{CreateAssignmentRequest, CreateAssignmentRequestBody},
    error::DfnsError,
    models::generic::DfnsBaseApiOptions,
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

    let permission_id = "example-permission-id";
    let request = CreateAssignmentRequest {
        permission_id: permission_id.to_string(),
        body: CreateAssignmentRequestBody {
            identity_id: "example-identity-id".to_string(),
        },
    };

    match client.permissions().create_assignment(request).await {
        Ok(response) => {
            println!("Successfully created assignment:");
            println!("  ID: {}", response.id);
            println!("  Permission ID: {}", response.permission_id);
            println!("  Identity ID: {}", response.identity_id);
            println!("  Is Immutable: {}", response.is_immutable);
            println!("  Date Created: {}", response.date_created);
            println!("  Date Updated: {}", response.date_updated);
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
