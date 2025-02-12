use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::DeactivateApplicationRequest,
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

    let request = DeactivateApplicationRequest {
        app_id: "example-app-id".to_string(),
    };

    match client.auth().deactivate_application(request).await {
        Ok(response) => {
            println!("Application deactivated successfully:");
            println!("  App ID: {}", response.app_id);
            println!("  Name: {}", response.name);
            println!("  Kind: {:?}", response.kind);
            println!("  Is Active: {}", response.is_active);
            println!("  Organization ID: {}", response.org_id);
            println!("  Date Created: {}", response.date_created);
            
            if !response.permission_assignments.is_empty() {
                println!("\nPermission Assignments:");
                for assignment in response.permission_assignments {
                    println!("  Assignment ID: {}", assignment.assignment_id);
                    println!("  Permission ID: {}", assignment.permission_id);
                    println!("  Permission Name: {}", assignment.permission_name);
                    if let Some(operations) = assignment.operations {
                        println!("  Operations: {:?}", operations);
                    }
                }
            }

            if let Some(permissions) = response.permissions {
                println!("\nPermissions: {:?}", permissions);
            }
        }
        Err(e) => eprintln!("Error deactivating application: {:?}", e),
    }
} 