use dex_poc::{accounts::pool::create_pool_account, errors::account_errors::ERR_POOL_ASSET_IS_NOT_IN_PAIR, notes::create_fund_note };
use miden_assembly::diagnostics::IntoDiagnostic;
use miden_objects::{
    accounts::{
        Account, AccountStorageMode, AccountType
    }, assets::{Asset, AssetVault, FungibleAsset}, notes::NoteType, testing::account_id::{
        ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN, ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_1, ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2
    }
};
use miden_tx::testing::{Auth, MockChain, TransactionContextBuilder};
use vm_processor::{crypto::RpoRandomCoin, Felt};

use crate::common::executor::execute;
use crate::assert_transaction_executor_error;

#[test]
fn test_fund_pool_without_authentication() {
    let mut mock_chain = MockChain::new();

    let (pool, pool_seed) = create_pool_account(
        [1; 32],
        [
            ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN.try_into().unwrap(), 
            ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap()
            ],
        AccountType::RegularAccountImmutableCode,
        AccountStorageMode::Public,
        (&mock_chain.block_header(0)).try_into().unwrap()
    ).into_diagnostic().unwrap();


    // Create assets
    let fungible_asset_1: Asset = 
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN.try_into().unwrap(), 123)
            .unwrap()
            .into();
    let fungible_asset_2: Asset =
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), 123)
            .unwrap()
            .into();

    let sender_account = mock_chain.add_new_wallet(Auth::BasicAuth);

    let funding_note = create_fund_note(
        sender_account.id(), 
        pool.id(), 
        vec![fungible_asset_1, fungible_asset_2], 
        NoteType::Public, 
        Felt::new(0),
        &mut RpoRandomCoin::new([Felt::new(1), Felt::new(2), Felt::new(3), Felt::new(4)])
    ).unwrap();

    mock_chain.add_pending_note(funding_note.clone());
    mock_chain.add_pending_account(pool.clone());

    mock_chain.seal_block(None);

    // CONSTRUCT AND EXECUTE TX (Success)
    // --------------------------------------------------------------------------------------------
    // Execute the transaction and get the witness

    let tx_inputs = mock_chain.get_transaction_inputs(pool.clone(), pool_seed.into(), &[
        funding_note.id()
    ], &[]);

    let executed_transaction = TransactionContextBuilder::new(pool.clone())
        .tx_inputs(tx_inputs)
        .build()
        .execute().into_diagnostic()
        .unwrap();

    // vault delta
    let target_account_after: Account = Account::from_parts(
        pool.id(),
        AssetVault::new(&[fungible_asset_1, fungible_asset_2]).unwrap(),
        pool.storage().clone(),
        pool.code().clone(),
        Felt::new(2),
    );

    assert_eq!(executed_transaction.final_account().hash(), target_account_after.hash());

}

#[test]
fn test_fund_pool_with_inappropriate_asset_faucet() {
    let mut mock_chain = MockChain::new();

    let (pool, pool_seed) = create_pool_account(
        [1; 32],
        [
                ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_1.try_into().unwrap(), 
                ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap()
            ],
        AccountType::RegularAccountImmutableCode,
        AccountStorageMode::Public,
        (&mock_chain.block_header(0)).try_into().unwrap()
    ).into_diagnostic().unwrap();


    // Create assets
    let fungible_asset_1: Asset = 
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN.try_into().unwrap(), 123)
            .unwrap()
            .into();
    let fungible_asset_2: Asset =
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), 123)
            .unwrap()
            .into();

    let sender_account = mock_chain.add_new_wallet(Auth::BasicAuth);

    let funding_note = create_fund_note(
        sender_account.id(), 
        pool.id(), 
        vec![fungible_asset_1, fungible_asset_2], 
        NoteType::Public, 
        Felt::new(0),
        &mut RpoRandomCoin::new([Felt::new(1), Felt::new(2), Felt::new(3), Felt::new(4)])
    ).unwrap();

    mock_chain.add_pending_note(funding_note.clone());
    mock_chain.add_pending_account(pool.clone());

    mock_chain.seal_block(None);

    // CONSTRUCT AND EXECUTE TX (Failed)
    // --------------------------------------------------------------------------------------------
    // Execute the transaction and get the witness

    let tx_inputs = mock_chain.get_transaction_inputs(pool.clone(), pool_seed.into(), &[
        funding_note.id()
    ], &[]);

    let executed_transaction = execute(TransactionContextBuilder::new(pool.clone())
        .tx_inputs(tx_inputs)
        .build());

    assert_transaction_executor_error!(executed_transaction, ERR_POOL_ASSET_IS_NOT_IN_PAIR);

}