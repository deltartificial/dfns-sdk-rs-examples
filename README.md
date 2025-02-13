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
cargo run --example activate_application
```

## Overview

This repository contains the following examples:

##### Authentication

- [x] [Activate Application](examples/auth/activate_application.rs) - Activates an application
- [x] [Activate Credential](examples/auth/activate_credential.rs) - Activates a credential
- [x] [Activate Personal Access Token](examples/auth/activate_personal_access_token.rs) - Activates a personal access token
- [x] [Activate Service Account](examples/auth/activate_service_account.rs) - Activates a service account
- [x] [Activate User](examples/auth/activate_user.rs) - Activates a user
- [x] [Deactivate Application](examples/auth/deactivate_application.rs) - Deactivates an application
- [x] [Deactivate Credential](examples/auth/deactivate_credential.rs) - Deactivates a credential
- [x] [Deactivate Personal Access Token](examples/auth/deactivate_personal_access_token.rs) - Deactivates a personal access token
- [x] [Deactivate Service Account](examples/auth/deactivate_service_account.rs) - Deactivates a service account
- [x] [Deactivate User](examples/auth/deactivate_user.rs) - Deactivates a user
- [x] [Archive Application](examples/auth/archive_application.rs) - Archives an application
- [x] [Archive Personal Access Token](examples/auth/archive_personal_access_token.rs) - Archives a personal access token
- [x] [Archive Service Account](examples/auth/archive_service_account.rs) - Archives a service account
- [x] [Archive User](examples/auth/archive_user.rs) - Archives a user
- [x] [Create Credential Challenge](examples/auth/create_credential_challenge.rs) - Creates a credential challenge
- [x] [Create Credential Code](examples/auth/create_credential_code.rs) - Creates a credential code
- [x] [Create Credential With Code](examples/auth/create_credential_with_code.rs) - Creates a credential using a code
- [x] [Create Delegated Recovery Challenge](examples/auth/create_delegated_recovery_challenge.rs) - Creates a delegated recovery challenge
- [x] [Create Delegated Registration Challenge](examples/auth/create_delegated_registration_challenge.rs) - Creates a delegated registration challenge
- [x] [Create Login Challenge](examples/auth/create_login_challenge.rs) - Creates a login challenge
- [x] [Create Personal Access Token](examples/auth/create_personal_access_token.rs) - Creates a personal access token
- [x] [Create Recovery Challenge](examples/auth/create_recovery_challenge.rs) - Creates a recovery challenge
- [x] [Create Registration Challenge](examples/auth/create_registration_challenge.rs) - Creates a registration challenge
- [x] [Create Service Account](examples/auth/create_service_account.rs) - Creates a service account
- [x] [Create User](examples/auth/create_user.rs) - Creates a new user
- [x] [Create User Action Challenge](examples/auth/create_user_action_challenge.rs) - Creates a user action challenge
- [x] [Create User Action Signature](examples/auth/create_user_action_signature.rs) - Creates a user action signature
- [x] [Get Application](examples/auth/get_application.rs) - Gets application details
- [x] [Get Personal Access Token](examples/auth/get_personal_access_token.rs) - Gets personal access token details
- [x] [Get Service Account](examples/auth/get_service_account.rs) - Gets service account details
- [x] [Get User](examples/auth/get_user.rs) - Gets user details
- [x] [List Applications](examples/auth/list_applications.rs) - Lists all applications
- [x] [List Credentials](examples/auth/list_credentials.rs) - Lists all credentials
- [x] [List Personal Access Tokens](examples/auth/list_personal_access_tokens.rs) - Lists all personal access tokens
- [x] [Login](examples/auth/login.rs) - Performs a login
- [x] [Recover](examples/auth/recover.rs) - Recovers an account
- [x] [Recreate Delegated Registration Challenge](examples/auth/recreate_delegated_registration_challenge.rs) - Recreates a delegated registration challenge
- [x] [Register](examples/auth/register.rs) - Registers a new user
- [x] [Register End User](examples/auth/register_end_user.rs) - Registers an end user
- [x] [Register With Recovery](examples/auth/register_with_recovery.rs) - Registers using recovery
- [x] [Update Application](examples/auth/update_application.rs) - Updates an application
- [x] [Update Credential](examples/auth/update_credential.rs) - Updates a credential
- [x] [Update Personal Access Token](examples/auth/update_personal_access_token.rs) - Updates a personal access token
- [x] [Update Service Account](examples/auth/update_service_account.rs) - Updates a service account
- [x] [Verify Challenge](examples/auth/verify_challenge.rs) - Verifies a challenge
- [x] [Verify Recovery Challenge](examples/auth/verify_recovery_challenge.rs) - Verifies a recovery challenge
- [x] [Verify Registration Challenge](examples/auth/verify_registration_challenge.rs) - Verifies a registration challenge

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

##### Permissions

- [x] [Archive Permission](examples/permissions/archive_permission.rs) - Archives a permission
- [x] [Create Assignment](examples/permissions/create_assignment.rs) - Creates a permission assignment
- [x] [Create Permission](examples/permissions/create_permission.rs) - Creates a new permission
- [x] [Delete Assignment](examples/permissions/delete_assignment.rs) - Deletes a permission assignment
- [x] [Get Permission](examples/permissions/get_permission.rs) - Gets permission details
- [x] [List Assignments](examples/permissions/list_assignments.rs) - Lists all assignments
- [x] [List Permissions](examples/permissions/list_permissions.rs) - Lists all permissions
- [x] [Update Permission](examples/permissions/update_permission.rs) - Updates permission configuration

##### Networks

- [x] [Get Fees](examples/networks/get_fees.rs) - Gets network fees for various blockchains
- [x] [Read Contract](examples/networks/read_contract.rs) - Reads data from a smart contract

##### Exchanges

- [x] [Create Deposit](examples/exchanges/create_deposit.rs) - Creates a new deposit
- [x] [Create Exchange](examples/exchanges/create_exchange.rs) - Creates a new exchange
- [x] [Create Withdrawal](examples/exchanges/create_withdrawal.rs) - Creates a new withdrawal
- [x] [Delete Exchange](examples/exchanges/delete_exchange.rs) - Deletes an exchange
- [x] [Get Exchange](examples/exchanges/get_exchange.rs) - Gets exchange details
- [x] [List Account Assets](examples/exchanges/list_account_assets.rs) - Lists assets in an account
- [x] [List Accounts](examples/exchanges/list_accounts.rs) - Lists all accounts
- [x] [List Asset Withdrawal Networks](examples/exchanges/list_asset_withdrawal_networks.rs) - Lists withdrawal networks for an asset
- [x] [List Exchanges](examples/exchanges/list_exchanges.rs) - Lists all exchanges

##### Policies

- [x] [Archive Policy](examples/policies/archive_policy.rs) - Archives a policy
- [x] [Create Approval Decision](examples/policies/create_approval_decision.rs) - Creates an approval decision
- [x] [Create Policy](examples/policies/create_policy.rs) - Creates a new policy
- [x] [Get Approval](examples/policies/get_approval.rs) - Gets approval details
- [x] [Get Policy](examples/policies/get_policy.rs) - Gets policy details
- [x] [List Approvals](examples/policies/list_approvals.rs) - Lists all approvals
- [x] [List Policies](examples/policies/list_policies.rs) - Lists all policies
- [x] [Update Policy](examples/policies/update_policy.rs) - Updates policy configuration
