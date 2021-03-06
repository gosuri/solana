# Table of contents

* [Introduction](introduction.md)
* [Using Solana from the Command-line](cli/README.md)
  * [Command-line Usage](cli/usage.md)
  * [Remote Wallet](remote-wallet/README.md)
    * [Ledger Hardware Wallet](remote-wallet/ledger.md)
  * [Paper Wallet](paper-wallet/README.md)
    * [Installation](paper-wallet/installation.md)
    * [Paper Wallet Usage](paper-wallet/usage.md)
  * [Offline Signing](offline-signing/README.md)
    * [Durable Transaction Nonces](offline-signing/durable-nonce.md)
* [Developing Applications](apps/README.md)
  * [Example: Web Wallet](apps/webwallet.md)
  * [Example: Tic-Tac-Toe](apps/tictactoe.md)
  * [Drones](apps/drones.md)
  * [Anatomy of a Transaction](transaction.md)
  * [JSON RPC API](apps/jsonrpc-api.md)
  * [JavaScript API](apps/javascript-api.md)
* [Participating in Tour de SOL](tour-de-sol/README.md)
  * [Useful Links & Discussion](tour-de-sol/useful-links.md)
  * [Registration](tour-de-sol/registration/README.md)
    * [How To Register](tour-de-sol/registration/how-to-register.md)
    * [Terms of Participation](tour-de-sol/registration/terms-of-participation.md)
    * [Rewards](tour-de-sol/registration/rewards.md)
    * [Confidentiality](tour-de-sol/registration/confidentiality.md)
    * [Registration FAQ](tour-de-sol/registration/validator-registration-and-rewards-faq.md)
  * [Participation](tour-de-sol/participation/README.md)
    * [Requirements to run a validator](tour-de-sol/participation/validator-technical-requirements.md)
    * [Steps to create a validator](tour-de-sol/participation/steps-to-create-a-validator/README.md)
      * [Install the Solana software](tour-de-sol/participation/steps-to-create-a-validator/install-the-solana-software.md)
      * [Create a validator public key](tour-de-sol/participation/steps-to-create-a-validator/validator-public-key-registration.md)
      * [Create and configure a Solana validator](tour-de-sol/participation/steps-to-create-a-validator/connecting-your-validator.md)
      * [Confirm the Solana network is running](tour-de-sol/participation/steps-to-create-a-validator/confirm-the-solana-network-is-running.md)
      * [Connect to the Solana network](tour-de-sol/participation/steps-to-create-a-validator/connect-to-the-solana-network.md)
      * [Validator catch-up](tour-de-sol/participation/steps-to-create-a-validator/monitoring-your-validator.md)
      * [Staking](tour-de-sol/participation/steps-to-create-a-validator/delegating-stake.md)
      * [Publish information about your validator](tour-de-sol/participation/steps-to-create-a-validator/publishing-information-about-your-validator.md)
    * [Dry Run 6](tour-de-sol/participation/dry-run-6.md)
  * [Submitting Bugs](tour-de-sol/submitting-bugs.md)
* [Running a Validator](running-validator/README.md)
  * [Validator Requirements](running-validator/validator-reqs.md)
  * [Choosing a Testnet](running-validator/validator-testnet.md)
  * [Installing the Validator Software](running-validator/validator-software.md)
  * [Starting a Validator](running-validator/validator-start.md)
  * [Staking](running-validator/validator-stake.md)
  * [Monitoring a Validator](running-validator/validator-monitor.md)
  * [Publishing Validator Info](running-validator/validator-info.md)
  * [Troubleshooting](running-validator/validator-troubleshoot.md)
* [Running an Archiver](running-archiver.md)
* [Understanding Solana's Architecture](cluster/README.md)
  * [Synchronization](cluster/synchronization.md)
  * [Leader Rotation](cluster/leader-rotation.md)
  * [Fork Generation](cluster/fork-generation.md)
  * [Managing Forks](cluster/managing-forks.md)
  * [Turbine Block Propagation](cluster/turbine-block-propagation.md)
  * [Ledger Replication](cluster/ledger-replication.md)
  * [Secure Vote Signing](cluster/vote-signing.md)
  * [Stake Delegation and Rewards](cluster/stake-delegation-and-rewards.md)
  * [Performance Metrics](cluster/performance-metrics.md)
* [Anatomy of a Validator](validator/README.md)
  * [TPU](validator/tpu.md)
  * [TVU](validator/tvu.md)
  * [Blockstore](validator/blockstore.md)
  * [Gossip Service](validator/gossip.md)
  * [The Runtime](validator/runtime.md)
* [Building from Source](building-from-source.md)
* [Terminology](terminology.md)
* [Implemented Design Proposals](implemented-proposals/README.md)
  * [Cluster Software Installation and Updates](implemented-proposals/installer.md)
  * [Cluster Economics](implemented-proposals/ed_overview/README.md)
    * [Validation-client Economics](implemented-proposals/ed_overview/ed_validation_client_economics/README.md)
      * [State-validation Protocol-based Rewards](implemented-proposals/ed_overview/ed_validation_client_economics/ed_vce_state_validation_protocol_based_rewards.md)
      * [State-validation Transaction Fees](implemented-proposals/ed_overview/ed_validation_client_economics/ed_vce_state_validation_transaction_fees.md)
      * [Replication-validation Transaction Fees](implemented-proposals/ed_overview/ed_validation_client_economics/ed_vce_replication_validation_transaction_fees.md)
      * [Validation Stake Delegation](implemented-proposals/ed_overview/ed_validation_client_economics/ed_vce_validation_stake_delegation.md)
    * [Replication-client Economics](implemented-proposals/ed_overview/ed_replication_client_economics/README.md)
      * [Storage-replication Rewards](implemented-proposals/ed_overview/ed_replication_client_economics/ed_rce_storage_replication_rewards.md)
      * [Replication-client Reward Auto-delegation](implemented-proposals/ed_overview/ed_replication_client_economics/ed_rce_replication_client_reward_auto_delegation.md)
    * [Storage Rent Economics](implemented-proposals/ed_overview/ed_storage_rent_economics.md)
    * [Economic Sustainability](implemented-proposals/ed_overview/ed_economic_sustainability.md)
    * [Attack Vectors](implemented-proposals/ed_overview/ed_attack_vectors.md)
    * [Economic Design MVP](implemented-proposals/ed_overview/ed_mvp.md)
    * [References](implemented-proposals/ed_overview/ed_references.md)
  * [Deterministic Transaction Fees](implemented-proposals/transaction-fees.md)
  * [Tower BFT](implemented-proposals/tower-bft.md)
  * [Leader-to-Leader Transition](implemented-proposals/leader-leader-transition.md)
  * [Leader-to-Validator Transition](implemented-proposals/leader-validator-transition.md)
  * [Persistent Account Storage](implemented-proposals/persistent-account-storage.md)
  * [Reliable Vote Transmission](implemented-proposals/reliable-vote-transmission.md)
  * [Repair Service](implemented-proposals/repair-service.md)
  * [Testing Programs](implemented-proposals/testing-programs.md)
  * [Credit-only Accounts](implemented-proposals/readonly-accounts.md)
  * [Embedding the Move Langauge](implemented-proposals/embedding-move.md)
  * [Staking Rewards](implemented-proposals/staking-rewards.md)
  * [Rent](implemented-proposals/rent.md)
  * [Durable Transaction Nonces](implemented-proposals/durable-tx-nonces.md)
  * [Validator Timestamp Oracle](implemented-proposals/validator-timestamp-oracle.md)
  * [Commitment](implemented-proposals/commitment.md)
  * [Snapshot Verification](implemented-proposals/snapshot-verification.md)
* [Accepted Design Proposals](proposals/README.md)
  * [Ledger Replication](proposals/ledger-replication-to-implement.md)
  * [Secure Vote Signing](proposals/vote-signing-to-implement.md)
  * [Cluster Test Framework](proposals/cluster-test-framework.md)
  * [Validator](proposals/validator-proposal.md)
  * [Simple Payment and State Verification](proposals/simple-payment-and-state-verification.md)
  * [Cross-Program Invocation](proposals/cross-program-invocation.md)
  * [Inter-chain Transaction Verification](proposals/interchain-transaction-verification.md)
  * [Snapshot Verification](proposals/snapshot-verification.md)
  * [Bankless Leader](proposals/bankless-leader.md)
  * [Slashing](proposals/slashing.md)
  * [Tick Verification](proposals/tick-verification.md)
  * [Block Confirmation](proposals/block-confirmation.md)
  * [ABI Management](proposals/abi-management.md)
