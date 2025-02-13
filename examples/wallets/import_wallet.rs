use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::wallets::types::{
        BodyEncryptedKeyShare, CreateWalletBodyNetwork, Curve, ImportWalletRequest,
        ImportWalletRequestBody, Protocol,
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

    let request = ImportWalletRequest {
        body: ImportWalletRequestBody {
            curve: Curve::Secp256K1,
            network: CreateWalletBodyNetwork::Ethereum,
            protocol: Protocol::Cggmp21,
            min_signers: 2.0,
            name: Some("Imported Wallet".to_string()),
            external_id: Some("imported-123".to_string()),
            encrypted_key_shares: vec![
                BodyEncryptedKeyShare {
                    encrypted_key_share: "encrypted-share-1".to_string(),
                    signer_id: "signer-1".to_string(),
                },
                BodyEncryptedKeyShare {
                    encrypted_key_share: "encrypted-share-2".to_string(),
                    signer_id: "signer-2".to_string(),
                },
            ],
        },
    };

    match client.wallets().import_wallet(request).await {
        Ok(wallet) => println!("Imported wallet: {:?}", wallet),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
