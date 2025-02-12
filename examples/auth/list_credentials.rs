use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::ListCredentialsRequest,
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

    let request = ListCredentialsRequest {
        limit: Some(10),
        paging_token: None,
        user_id: Some("example-user-id".to_string()),
    };

    match client.auth().list_credentials(request).await {
        Ok(response) => {
            println!("Credentials listed successfully:");
            for credential in response.items {
                println!("\nCredential:");
                println!("  Credential UUID: {}", credential.credential_uuid);
                println!("  Name: {}", credential.name);
                println!("  Kind: {:?}", credential.kind);
                println!("  Is Active: {}", credential.is_active);
                println!("  Organization ID: {}", credential.org_id);
                println!("  User ID: {}", credential.user_id);
                println!("  Date Created: {}", credential.date_created);
                
                if let Some(last_used) = credential.last_used {
                    println!("  Last Used: {}", last_used);
                }
                
                if let Some(webauthn_info) = credential.webauthn_info {
                    println!("\n  WebAuthn Info:");
                    println!("    Credential ID: {}", webauthn_info.credential_id);
                    println!("    Public Key: {}", webauthn_info.public_key);
                    println!("    AAGUID: {}", webauthn_info.aaguid);
                    println!("    Sign Count: {}", webauthn_info.sign_count);
                    println!("    Attestation Type: {}", webauthn_info.attestation_type);
                    println!("    Backup State: {}", webauthn_info.backup_state);
                    println!("    Backup Eligible: {}", webauthn_info.backup_eligible);
                }
            }

            if let Some(paging) = response.paging {
                println!("\nPaging:");
                println!("  Previous Token: {:?}", paging.previous_token);
                println!("  Next Token: {:?}", paging.next_token);
                println!("  Total Count: {:?}", paging.total_count);
            }
        }
        Err(e) => eprintln!("Error listing credentials: {:?}", e),
    }
} 