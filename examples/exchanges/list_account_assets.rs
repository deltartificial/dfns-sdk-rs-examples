use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::exchanges::types::{ListAccountAssetsRequest, ListAccountAssetsRequestQuery},
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

    let request = ListAccountAssetsRequest {
        account_id: "example-account-id".to_string(),
        exchange_id: "example-exchange-id".to_string(),
        query: Some(ListAccountAssetsRequestQuery {
            limit: Some(10.0),
            pagination_token: None,
        }),
    };

    match client.exchanges().list_account_assets(request).await {
        Ok(response) => {
            println!("Account assets:");
            for asset in response.items {
                println!("\nSymbol: {}", asset.symbol);
                println!("Balance: {}", asset.balance);
            }
            if let Some(token) = response.next_page_token {
                println!("\nNext page token: {}", token);
            }
        }
        Err(e) => println!("Error listing account assets: {:?}", e),
    }
}
