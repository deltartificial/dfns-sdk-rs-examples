use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient, DfnsBaseApiOptions, DfnsError,
    api::exchanges::types::{CreateDepositBodyKind, CreateWithdrawalBody, CreateWithdrawalRequest},
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

    let request = CreateWithdrawalRequest {
        account_id: "example-account-id".to_string(),
        exchange_id: "example-exchange-id".to_string(),
        body: CreateWithdrawalBody {
            amount: "0.1".to_string(),
            kind: CreateDepositBodyKind::Native,
            wallet_id: "example-wallet-id".to_string(),
            create_destination_account: None,
            external_id: None,
            otp: None,
            priority: None,
            contract: None,
            token_id: None,
            asset_id: None,
            asset_code: None,
            issuer: None,
            mint: None,
            master: None,
        },
    };

    match client.exchanges().create_withdrawal(request).await {
        Ok(response) => {
            println!("Withdrawal created successfully:");
            println!("ID: {}", response.id);
            println!("Account ID: {}", response.account_id);
            println!("Exchange ID: {}", response.exchange_id);
            println!("Date Created: {}", response.date_created);
        }
        Err(e) => println!("Error creating withdrawal: {:?}", e),
    }
}
