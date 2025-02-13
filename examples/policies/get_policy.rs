use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::policies::types::GetPolicyRequest,
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

    let policy_id = "example-policy-id";
    let request = GetPolicyRequest {
        policy_id: policy_id.to_string(),
    };

    match client.policies().get_policy(request).await {
        Ok(response) => {
            println!("Successfully retrieved policy:");
            println!("  ID: {}", response.id);
            println!("  Name: {}", response.name);
            println!("  Status: {}", response.status);
            println!("  Activity Kind: {:?}", response.activity_kind);
            println!("  Action Kind: {:?}", response.action.kind);
            println!("  Rule Kind: {:?}", response.rule.kind);

            if let Some(pending_change) = response.pending_change_request {
                println!("Pending Change Request:");
                println!("  ID: {}", pending_change.id);
                println!("  Status: {:?}", pending_change.status);
                println!("  Operation: {:?}", pending_change.operation_kind);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
