use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::ListPersonalAccessTokensRequest,
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

    let request = ListPersonalAccessTokensRequest {
        limit: Some(10),
        paging_token: None,
    };

    match client.auth().list_personal_access_tokens(request).await {
        Ok(response) => {
            println!("Personal Access Tokens listed successfully:");
            for token in response.items {
                println!("\nPersonal Access Token:");
                println!("  Token ID: {}", token.token_id);
                println!("  Name: {}", token.name);
                println!("  Kind: {:?}", token.kind);
                println!("  Is Active: {}", token.is_active);
                println!("  Organization ID: {}", token.org_id);
                if let Some(linked_user_id) = &token.linked_user_id {
                    println!("  Linked User ID: {}", linked_user_id);
                }
                if let Some(linked_app_id) = &token.linked_app_id {
                    println!("  Linked App ID: {}", linked_app_id);
                }
                println!("  Date Created: {}", token.date_created);
                
                if !token.permission_assignments.is_empty() {
                    println!("\n  Permission Assignments:");
                    for assignment in token.permission_assignments {
                        println!("    Assignment ID: {}", assignment.assignment_id);
                        println!("    Permission ID: {}", assignment.permission_id);
                        println!("    Permission Name: {}", assignment.permission_name);
                        if let Some(operations) = assignment.operations {
                            println!("    Operations: {:?}", operations);
                        }
                    }
                }
            }

            if let Some(paging) = response.paging {
                println!("\nPaging:");
                println!("  Previous Token: {:?}", paging.previous_token);
                println!("  Next Token: {:?}", paging.next_token);
                println!("  Total Count: {:?}", paging.total_count);
            }
        }
        Err(e) => eprintln!("Error listing personal access tokens: {:?}", e),
    }
} 