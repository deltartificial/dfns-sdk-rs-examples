use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::wallets::types::{
        Priority, TransferAssetBody, TransferAssetBodyKind, TransferAssetRequest,
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

    let request = TransferAssetRequest {
        wallet_id: "example-wallet-id".to_string(),
        body: TransferAssetBody {
            kind: TransferAssetBodyKind::Native,
            to: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            amount: Some("1000000000000000000".to_string()),
            priority: Some(Priority::Standard),
            memo: Some("Payment for services".to_string()),
            external_id: Some("transfer-123".to_string()),
            create_destination_account: None,
            asset_id: None,
            metadata: None,
            contract: None,
            token_id: None,
            asset_code: None,
            issuer: None,
            mint: None,
            master: None,
        },
    };

    match client.wallets().transfer_asset(request).await {
        Ok(result) => println!("Transfer initiated: {:?}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
