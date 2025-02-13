use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::permissions::types::{ListPermissionsRequest, Query},
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

    let request = Some(ListPermissionsRequest {
        query: Some(Query {
            limit: Some("10".to_string()),
            pagination_token: None,
        }),
    });

    match client.permissions().list_permissions(request).await {
        Ok(response) => {
            println!("Successfully retrieved permissions:");
            for item in response.items {
                println!("\nPermission:");
                println!("  ID: {}", item.id);
                println!("  Name: {}", item.name);
                println!("  Status: {:?}", item.status);
                println!("  Is Archived: {}", item.is_archived);
                println!("  Is Immutable: {}", item.is_immutable);
                println!("  Operations: {:?}", item.operations);
                println!("  Date Created: {}", item.date_created);
                println!("  Date Updated: {}", item.date_updated);

                if let Some(pending_change) = item.pending_change_request {
                    println!("  Pending Change Request:");
                    println!("    ID: {}", pending_change.id);
                    println!("    Status: {:?}", pending_change.status);
                    println!("    Operation Kind: {:?}", pending_change.operation_kind);
                    println!("    Date Created: {}", pending_change.date_created);
                    if let Some(date) = pending_change.date_resolved {
                        println!("    Date Resolved: {}", date);
                    }
                }
            }

            if let Some(token) = response.next_page_token {
                println!("\nNext page token: {}", token);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
