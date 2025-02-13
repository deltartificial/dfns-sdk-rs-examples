use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::auth::types::{
        CredentialInfo3, FirstFactorKind, RegisterEndUserRequest, RegisterEndUserRequestBody,
        StickyFirstFactorCredential, StickyRecoveryCredential, StickySecondFactorCredential,
    },
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

    let request = RegisterEndUserRequest {
        body: RegisterEndUserRequestBody {
            first_factor_credential: StickyFirstFactorCredential {
                credential_info: CredentialInfo3 {
                    attestation_data: None,
                    client_data: None,
                    cred_id: None,
                    password: Some("example-password".to_string()),
                },
                credential_name: None,
                credential_kind: FirstFactorKind::Password,
                encrypted_private_key: None,
            },
            recovery_credential: None,
            second_factor_credential: None,
            wallets: vec![],
        },
    };

    match client.auth().register_end_user(request).await {
        Ok(response) => {
            println!("End user registered successfully:");
            println!("\nUser Info:");
            println!("  ID: {}", response.user.id);
            println!("  Organization ID: {}", response.user.org_id);
            println!("  Username: {}", response.user.username);

            println!("\nCredential Info:");
            println!("  UUID: {}", response.credential.uuid);
            println!("  Name: {}", response.credential.name);
            println!("  Kind: {:?}", response.credential.kind);
        }
        Err(e) => eprintln!("Error registering end user: {:?}", e),
    }
}
