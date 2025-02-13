use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::wallets::types::{
        GenerateSignatureBody, GenerateSignatureBodyKind, GenerateSignatureRequest, Message,
    },
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

    let request = GenerateSignatureRequest {
        wallet_id: "example-wallet-id".to_string(),
        body: GenerateSignatureBody {
            kind: GenerateSignatureBodyKind::Message,
            message: Some(Message::String("Hello, World!".to_string())),
            external_id: Some("sig-123".to_string()),
            sign_doc: None,
            hash: None,
            taproot_merkle_root: None,
            transaction: None,
            domain: None,
            types: None,
            psbt: None,
            format: None,
        },
    };

    match client.wallets().generate_signature(request).await {
        Ok(result) => println!("Signature generated: {:?}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
