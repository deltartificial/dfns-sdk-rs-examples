use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::exchanges::types::{ListExchangesRequest, ListExchangesRequestQuery},
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

    let request = Some(ListExchangesRequest {
        query: Some(ListExchangesRequestQuery {
            limit: Some(10.0),
            pagination_token: None,
        }),
    });

    match client.exchanges().list_exchanges(request).await {
        Ok(response) => {
            println!("Exchanges:");
            for exchange in response.items {
                println!("\nID: {}", exchange.id);
                println!("Name: {:?}", exchange.name);
                println!("Kind: {:?}", exchange.kind);
                println!("Created At: {}", exchange.date_created);
            }
            if let Some(token) = response.next_page_token {
                println!("\nNext page token: {}", token);
            }
        }
        Err(e) => println!("Error listing exchanges: {:?}", e),
    }
}
