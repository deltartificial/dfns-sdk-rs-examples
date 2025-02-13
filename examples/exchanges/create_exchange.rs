use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::exchanges::types::{
        Body, BodyReadConfiguration, BodyWriteConfiguration, CreateExchangeBodyKind,
        CreateExchangeRequest,
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

    let request = CreateExchangeRequest {
        body: Body {
            kind: CreateExchangeBodyKind::Binance,
            name: Some("Example Exchange".to_string()),
            read_configuration: BodyReadConfiguration {
                private_api_key: "example-private-key".to_string(),
                public_api_key: "example-public-key".to_string(),
                otp: None,
                password: None,
            },
            write_configuration: BodyWriteConfiguration {
                private_api_key: "example-private-key".to_string(),
                public_api_key: "example-public-key".to_string(),
                otp: None,
                password: None,
            },
        },
    };

    match client.exchanges().create_exchange(request).await {
        Ok(response) => {
            println!("Exchange created successfully:");
            println!("ID: {}", response.id);
            println!("Name: {:?}", response.name);
            println!("Kind: {:?}", response.kind);
            println!("Created At: {}", response.date_created);
        }
        Err(e) => println!("Error creating exchange: {:?}", e),
    }
}
