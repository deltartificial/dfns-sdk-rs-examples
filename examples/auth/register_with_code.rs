use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{RegisterWithCodeRequest, RegisterWithCodeRequestBody, FirstFactorCredential, CredentialInfo, FirstFactorKind},
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

    let request = RegisterWithCodeRequest {
        body: RegisterWithCodeRequestBody {
            first_factor_credential: FirstFactorCredential {
                credential_info: CredentialInfo {
                    attestation_data: None,
                    client_data: None,
                    cred_id: None,
                    password: Some("example-password".to_string()),
                },
                credential_kind: FirstFactorKind::Password,
                credential_name: "My Password Credential".to_string(),
                encrypted_private_key: None,
            },
            registration_code: "123456".to_string(),
        },
    };

    match client.auth().register_with_code(request).await {
        Ok(response) => {
            println!("Registration with code successful:");
            println!("\nUser Info:");
            println!("  ID: {}", response.user.id);
            println!("  Organization ID: {}", response.user.org_id);
            println!("  Username: {}", response.user.username);
            println!("  External ID: {:?}", response.user.external_id);
            println!("  Is Active: {}", response.user.is_active);
            println!("  Date Created: {}", response.user.date_created);
            
            println!("\nCredential Info:");
            println!("  UUID: {}", response.credential.uuid);
            println!("  Name: {}", response.credential.name);
            println!("  Kind: {:?}", response.credential.kind);
            println!("  Is Active: {}", response.credential.is_active);
            println!("  Date Created: {}", response.credential.date_created);
        }
        Err(e) => eprintln!("Error during registration with code: {:?}", e),
    }
} 