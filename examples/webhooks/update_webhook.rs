use dfns_sdk_rs::{
    DfnsApiClient,
    error::DfnsError,
    models::generic::DfnsBaseApiOptions,
    signer::{CredentialSigner, FirstFactorAssertion, FirstFactorAssertionKind, UserActionChallenge},
    api::webhooks::types::{UpdateWebhookRequest, UpdateWebhookRequestBody, Event, Status},
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

    let request = UpdateWebhookRequest {
        webhook_id: "example-webhook-id".to_string(),
        body: UpdateWebhookRequestBody {
            description: Some("Updated webhook description".to_string()),
            url: Some("https://example.com/webhook".to_string()),
            events: Some(vec![Event::WalletCreated]),
            status: Some(Status::Enabled),
        },
    };

    match client.webhooks().update_webhook(request).await {
        Ok(webhook) => println!("Updated webhook: {:?}", webhook),
        Err(e) => eprintln!("Error: {:?}", e),
    }
} 