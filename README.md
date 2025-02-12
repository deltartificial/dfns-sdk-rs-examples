# dfns-sdk-rs-examples

This is a collection of examples demonstrating how to use the dfns-sdk-rs library.

- [Dfns Website](https://www.dfns.co)
- [Dfns API Docs](https://docs.dfns.co)
- [Dfns SDK Rust](https://github.com/deltartificial/dfns-sdk-rs)

## Installation

```bash
cargo add dfns-sdk-rs
```

## Usage

To run any example, use:

```bash
cargo run --example <example_name>
```

For instance:

```bash
cargo run --example list_wallets
```

## Overview

This repository contains the following examples:

##### Wallets

- [x] [List Wallets](examples/wallets/list_wallets.rs) - Demonstrates how to retrieve a list of wallets
- [x] [Create Wallet](examples/wallets/create_wallet.rs) - Shows how to create a new wallet
- [x] [Get Wallet](examples/wallets/get_wallet.rs) - Retrieves details of a specific wallet
- [x] [Update Wallet](examples/wallets/update_wallet.rs) - Updates wallet information
- [x] [Tag Wallet](examples/wallets/tag_wallet.rs) - Adds tags to a wallet
- [x] [Untag Wallet](examples/wallets/untag_wallet.rs) - Removes tags from a wallet
- [x] [Get Wallet Assets](examples/wallets/get_wallet_assets.rs) - Lists assets in a wallet
- [x] [Get Wallet History](examples/wallets/get_wallet_history.rs) - Shows transaction history
- [x] [Get Wallet NFTs](examples/wallets/get_wallet_nfts.rs) - Lists NFTs in a wallet
- [x] [Transfer Asset](examples/wallets/transfer_asset.rs) - Transfers assets between wallets
- [x] [Broadcast Transaction](examples/wallets/broadcast_transaction.rs) - Broadcasts a transaction
- [x] [Generate Signature](examples/wallets/generate_signature.rs) - Generates a signature
- [x] [Get Signature](examples/wallets/get_signature.rs) - Retrieves a specific signature
- [x] [List Signatures](examples/wallets/list_signatures.rs) - Lists all signatures
- [x] [Get Transaction](examples/wallets/get_transaction.rs) - Gets transaction details
- [x] [List Transactions](examples/wallets/list_transactions.rs) - Lists all transactions
- [x] [Get Transfer](examples/wallets/get_transfer.rs) - Gets transfer details
- [x] [List Transfers](examples/wallets/list_transfers.rs) - Lists all transfers
- [x] [Export Wallet](examples/wallets/export_wallet.rs) - Exports a wallet
- [x] [Import Wallet](examples/wallets/import_wallet.rs) - Imports a wallet
- [x] [Delegate Wallet](examples/wallets/delegate_wallet.rs) - Delegates wallet control

##### Webhooks

- [x] [Create Webhook](examples/webhooks/create_webhook.rs) - Creates a new webhook
- [x] [Get Webhook](examples/webhooks/get_webhook.rs) - Gets webhook details
- [x] [Update Webhook](examples/webhooks/update_webhook.rs) - Updates webhook configuration
- [x] [Delete Webhook](examples/webhooks/delete_webhook.rs) - Deletes a webhook
- [x] [List Webhooks](examples/webhooks/list_webhooks.rs) - Lists all webhooks
- [x] [Ping Webhook](examples/webhooks/ping_webhook.rs) - Tests webhook connectivity
- [x] [Get Webhook Event](examples/webhooks/get_webhook_event.rs) - Gets webhook event details
- [x] [List Webhook Events](examples/webhooks/list_webhook_events.rs) - Lists webhook events

##### Staking

- [x] [Create Stake](examples/staking/create_stake.rs) - Creates a new stake
- [x] [Create Stake Action](examples/staking/create_stake_action.rs) - Creates a stake action
- [x] [Get Stake Rewards](examples/staking/get_stake_rewards.rs) - Gets stake rewards
- [x] [List Stake Actions](examples/staking/list_stake_actions.rs) - Lists stake actions
- [x] [List Stakes](examples/staking/list_stakes.rs) - Lists all stakes

##### Signers

- [x] [List Signers](examples/signers/list_signers.rs) - Lists all signers in clusters

##### Policies

- [x] [Archive Policy](examples/policies/archive_policy.rs) - Archives a policy
- [x] [Create Approval Decision](examples/policies/create_approval_decision.rs) - Creates an approval decision
- [x] [Create Policy](examples/policies/create_policy.rs) - Creates a new policy
- [x] [Get Approval](examples/policies/get_approval.rs) - Gets approval details
- [x] [Get Policy](examples/policies/get_policy.rs) - Gets policy details
- [x] [List Approvals](examples/policies/list_approvals.rs) - Lists all approvals
- [x] [List Policies](examples/policies/list_policies.rs) - Lists all policies
- [x] [Update Policy](examples/policies/update_policy.rs) - Updates policy configuration
