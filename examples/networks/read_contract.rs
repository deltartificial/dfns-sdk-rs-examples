use dfns_sdk_rs::{
    DfnsApiClient,
    error::DfnsError,
    models::generic::DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::networks::types::{ReadContractRequest, Body, ReadContractBodyKind, ReadContractBodyNetwork},
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

    let request = ReadContractRequest {
        body: Body {
            contract: "0x1234567890123456789012345678901234567890".to_string(),
            data: "0x70a08231000000000000000000000000e16c1623c1aa7d919cd2241d8b36d9e79c1be2a2".to_string(),
            kind: ReadContractBodyKind::Evm,
            network: ReadContractBodyNetwork::Ethereum,
        },
    };

    match client.networks().read_contract(request).await {
        Ok(response) => {
            println!("Successfully read contract data:");
            println!("  Data: {}", response.data);
            println!("  Kind: {:?}", response.kind);
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
} 