use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::policies::types::{ListPoliciesRequest, ListPoliciesRequestQuery},
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

    let request = ListPoliciesRequest {
        query: Some(ListPoliciesRequestQuery {
            limit: Some("10".to_string()),
            status: None,
            pagination_token: None,
        }),
    };

    match client.policies().list_policies(Some(request)).await {
        Ok(response) => {
            println!("Successfully retrieved policies:");
            for item in response.items {
                println!("\nPolicy:");
                println!("  ID: {}", item.id);
                println!("  Name: {}", item.name);
                println!("  Status: {}", item.status);
                println!("  Activity Kind: {:?}", item.activity_kind);
                println!("  Action Kind: {:?}", item.action.kind);
                println!("  Rule Kind: {:?}", item.rule.kind);

                if let Some(pending_change) = item.pending_change_request {
                    println!("  Pending Change Request:");
                    println!("    ID: {}", pending_change.id);
                    println!("    Status: {:?}", pending_change.status);
                    println!("    Operation: {:?}", pending_change.operation_kind);
                }
            }

            if let Some(token) = response.next_page_token {
                println!("\nNext page token: {}", token);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
