use async_trait::async_trait;
use dfns_sdk_rs::{
    DfnsApiClient,
    api::networks::types::{GetFeesQueryNetwork, GetFeesRequest, Query},
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

    let request = Some(GetFeesRequest {
        query: Some(Query {
            network: GetFeesQueryNetwork::Ethereum,
        }),
    });

    match client.networks().get_fees(request).await {
        Ok(response) => {
            println!("Successfully retrieved network fees:");
            println!("  Network: {:?}", response.network);
            println!("  Block Number: {}", response.block_number);
            println!("  Kind: {:?}", response.kind);

            println!("\nFast Transaction:");
            println!("  Block Horizon: {:?}", response.fast.block_horizon);
            println!("  Fee Rate: {:?}", response.fast.fee_rate);
            println!("  Max Fee Per Gas: {:?}", response.fast.max_fee_per_gas);
            println!(
                "  Max Priority Fee Per Gas: {:?}",
                response.fast.max_priority_fee_per_gas
            );

            println!("\nStandard Transaction:");
            println!("  Block Horizon: {:?}", response.standard.block_horizon);
            println!("  Fee Rate: {:?}", response.standard.fee_rate);
            println!("  Max Fee Per Gas: {:?}", response.standard.max_fee_per_gas);
            println!(
                "  Max Priority Fee Per Gas: {:?}",
                response.standard.max_priority_fee_per_gas
            );

            println!("\nSlow Transaction:");
            println!("  Block Horizon: {:?}", response.slow.block_horizon);
            println!("  Fee Rate: {:?}", response.slow.fee_rate);
            println!("  Max Fee Per Gas: {:?}", response.slow.max_fee_per_gas);
            println!(
                "  Max Priority Fee Per Gas: {:?}",
                response.slow.max_priority_fee_per_gas
            );

            if let Some(base_fee) = response.estimated_base_fee {
                println!("\nEstimated Base Fee: {}", base_fee);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
