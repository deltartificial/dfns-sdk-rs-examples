use dfns_sdk_rs::{
    DfnsApiClient, DfnsError, DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::auth::types::{CreateCredentialChallengeRequest, CreateCredentialChallengeRequestBody, CredentialKindElement},
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

    let request = CreateCredentialChallengeRequest {
        body: CreateCredentialChallengeRequestBody {
            kind: CredentialKindElement::Fido2,
        },
    };

    match client.auth().create_credential_challenge(request).await {
        Ok(response) => {
            println!("Credential challenge created successfully:");
            println!("  Challenge Identifier: {}", response.challenge_identifier);
            println!("  Kind: {:?}", response.kind);
            println!("  Temporary Auth Token: {}", response.temporary_authentication_token);
            
            if let Some(rp) = response.rp {
                println!("\nRelying Party Info:");
                println!("  ID: {}", rp.id);
                println!("  Name: {}", rp.name);
            }

            if let Some(attestation) = response.attestation {
                println!("\nAttestation: {:?}", attestation);
            }

            if let Some(challenge) = response.challenge {
                println!("Challenge: {}", challenge);
            }

            if let Some(pub_key_cred_params) = response.pub_key_cred_params {
                println!("\nPublic Key Credential Parameters:");
                for param in pub_key_cred_params {
                    println!("  Algorithm: {}", param.alg);
                    println!("  Type: {:?}", param.pub_key_cred_param_type);
                }
            }
        }
        Err(e) => eprintln!("Error creating credential challenge: {:?}", e),
    }
} 