use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::wallets::types::{
        BroadcastTransactionBody, BroadcastTransactionBodyKind, BroadcastTransactionRequest,
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

    let request = BroadcastTransactionRequest {
        wallet_id: "example-wallet-id".to_string(),
        body: BroadcastTransactionBody {
            kind: BroadcastTransactionBodyKind::Eip1559,
            transaction: None,
            data: Some("0x".to_string()),
            gas_limit: Some("21000".to_string()),
            nonce: None,
            to: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
            value: Some("1000000000000000000".to_string()),
            max_fee_per_gas: Some("50000000000".to_string()),
            max_priority_fee_per_gas: Some("1500000000".to_string()),
            gas_price: None,
            psbt: None,
            external_id: Some("tx-123".to_string()),
        },
    };

    match client.wallets().broadcast_transaction(request).await {
        Ok(result) => println!("Transaction broadcasted: {:?}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
