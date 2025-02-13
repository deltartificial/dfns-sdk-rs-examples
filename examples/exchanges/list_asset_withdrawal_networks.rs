use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::exchanges::types::ListAssetWithdrawalNetworksRequest,
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

    let request = ListAssetWithdrawalNetworksRequest {
        account_id: "example-account-id".to_string(),
        exchange_id: "example-exchange-id".to_string(),
        asset: "BTC".to_string(),
    };

    match client
        .exchanges()
        .list_asset_withdrawal_networks(request)
        .await
    {
        Ok(response) => {
            println!("Asset withdrawal networks:");
            for network in response {
                println!("\nNetwork: {:?}", network.network);
                println!("Kind: {:?}", network.kind);
                println!("Decimals: {}", network.decimals);
                if let Some(metadata) = network.metadata {
                    println!("Metadata: {}", metadata);
                }
            }
        }
        Err(e) => println!("Error listing asset withdrawal networks: {:?}", e),
    }
}
