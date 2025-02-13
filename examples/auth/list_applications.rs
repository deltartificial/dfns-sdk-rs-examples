use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
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

    match client.auth().list_applications().await {
        Ok(response) => {
            println!("Applications listed successfully:");
            for app in response.items {
                println!("\nApplication:");
                println!("  App ID: {}", app.app_id);
                println!("  Name: {}", app.name);
                println!("  Kind: {:?}", app.kind);
                println!("  Is Active: {}", app.is_active);
                println!("  Organization ID: {}", app.org_id);
                
                if !app.permission_assignments.is_empty() {
                    println!("\n  Permission Assignments:");
                    for assignment in app.permission_assignments {
                        println!("    Assignment ID: {}", assignment.assignment_id);
                        println!("    Permission ID: {}", assignment.permission_id);
                        println!("    Permission Name: {}", assignment.permission_name);
                        if let Some(operations) = assignment.operations {
                            println!("    Operations: {:?}", operations);
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error listing applications: {:?}", e),
    }
} 