use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::auth::types::{
        CreateUserActionSignatureRequest, CreateUserActionSignatureRequestBody, FirstFactorKind,
        FluffySecondFactor as SecondFactor, PurpleFirstFactor as FirstFactor, SecondFactorKind,
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

    let request = CreateUserActionSignatureRequest {
        body: CreateUserActionSignatureRequestBody {
            challenge_identifier: "example-challenge-id".to_string(),
            first_factor: FirstFactor {
                credential_assertion: None,
                kind: FirstFactorKind::Password,
                password: Some("example-password".to_string()),
            },
            second_factor: Some(SecondFactor {
                credential_assertion: None,
                kind: SecondFactorKind::Totp,
                otp_code: Some("123456".to_string()),
            }),
        },
    };

    match client.auth().create_user_action_signature(request).await {
        Ok(response) => {
            println!("User action signature created successfully:");
            println!("  User Action: {:?}", response.user_action);
        }
        Err(e) => eprintln!("Error creating user action signature: {:?}", e),
    }
}
