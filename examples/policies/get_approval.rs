use dfns_sdk_rs::{
    DfnsApiClient,
    error::DfnsError,
    models::generic::DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::policies::types::GetApprovalRequest,
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

    let approval_id = "example-approval-id";
    let request = GetApprovalRequest {
        approval_id: approval_id.to_string(),
    };

    match client.policies().get_approval(request).await {
        Ok(response) => {
            println!("Successfully retrieved approval:");
            println!("  ID: {}", response.id);
            println!("  Status: {:?}", response.status);
            println!("  Date Created: {}", response.date_created.unwrap_or_default());
            println!("  Initiator ID: {}", response.initiator_id);
            
            if !response.decisions.is_empty() {
                println!("Decisions:");
                for decision in response.decisions {
                    println!("  - User: {}", decision.user_id);
                    println!("    Value: {:?}", decision.value);
                    println!("    Date: {}", decision.date);
                }
            }

            if !response.policy_evaluations.is_empty() {
                println!("Policy Evaluations:");
                for evaluation in response.policy_evaluations {
                    println!("  - Policy ID: {}", evaluation.policy_id);
                    println!("    Triggered: {}", evaluation.triggered);
                    println!("    Reason: {}", evaluation.reason);
                }
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
} 