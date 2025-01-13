use dex_poc::{
    errors::note_errors::ERR_P2ID_TARGET_ACCT_MISMATCH, 
    notes::create_fund_note,
};
use miden_objects::{
    accounts::Account, assets::{Asset, AssetVault, FungibleAsset}, crypto::rand::RpoRandomCoin, notes::NoteType, testing::account_id::{ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN, ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2}, transaction::OutputNote, Felt
};
use miden_tx::testing::{Auth, MockChain, TransactionContextBuilder};

use crate::common::{accounts::create_prefunded_pool_account, executor::{execute, execute_with_debugger}};

use crate::assert_transaction_executor_error;

/// We test the Fund script with 2 assets to test the loop inside the script.
/// So we create a note containing two assets that can only be consumed by the target account.
#[test]
fn fund_script_multiple_assets() {

    let init_seed: [u8; 32] = [
        95, 113, 209, 94, 84, 105, 250, 242, 223, 203, 216, 124, 22, 159, 14, 132, 215, 85, 183,
        204, 149, 90, 166, 68, 100, 73, 106, 168, 125, 237, 138, 16,
    ];

    let mut mock_chain = MockChain::new();

    let init_asset_1 = FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN.try_into().unwrap(), 10)
        .unwrap()
        .into();
    let init_asset_2 = FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), 10)
        .unwrap()
        .into();

    // Create assets
    let fungible_asset_1: Asset = FungibleAsset::mock(123);
    let fungible_asset_2: Asset =
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), 456)
            .unwrap()
            .into();

    // Create sender and target account
    let sender_account = mock_chain.add_new_wallet(Auth::BasicAuth);
    let target_account = create_prefunded_pool_account(
        init_seed,
        [
            init_asset_1,
            init_asset_2
        ]
    ).unwrap();
    
    let note = create_fund_note(
        sender_account.id(),
        target_account.id(),
        vec![fungible_asset_1, fungible_asset_2],
        NoteType::Public,
        Felt::new(0),
        &mut RpoRandomCoin::new([Felt::new(1), Felt::new(2), Felt::new(3), Felt::new(4)])
    )
    .unwrap();

    // Create the note
    mock_chain.add_pending_account(target_account.clone());
    mock_chain.add_pending_note(note.clone());

    mock_chain.seal_block(None);

    // CONSTRUCT AND EXECUTE TX (Success)
    // --------------------------------------------------------------------------------------------
    // Execute the transaction and get the witness

    let tx_inputs = mock_chain.get_transaction_inputs(target_account.clone(), None, &[
        note.id()
    ], &[]);
    
    let executed_transaction = execute_with_debugger(
        TransactionContextBuilder::new(target_account.clone())
            .tx_inputs(tx_inputs)
            .expected_notes(vec![
                OutputNote::Full(note.clone())
            ])
            .build()
    ).unwrap();

    // vault delta
    let target_account_after: Account = Account::from_parts(
        target_account.id(),
        AssetVault::new(&[
            fungible_asset_1.unwrap_fungible().add(
                init_asset_1
            ).unwrap().into(), 
            fungible_asset_2.unwrap_fungible().add(
                init_asset_2
            ).unwrap().into()
        ]).unwrap(),
        target_account.storage().clone(),
        target_account.code().clone(),
        Felt::new(3),
    );

    assert_eq!(executed_transaction.final_account().hash(), target_account_after.hash());

    // CONSTRUCT AND EXECUTE TX (Failure)
    // --------------------------------------------------------------------------------------------
    // A "malicious" account tries to consume the note, we expect an error (not the correct target)

    let mut init_seed = init_seed.clone();
    init_seed.reverse();

    let malicious_account = create_prefunded_pool_account(
        init_seed,
        [
            init_asset_1,
            init_asset_2
        ]
    ).unwrap();

    mock_chain.add_pending_account(malicious_account.clone());
    mock_chain.seal_block(None);

    let tx_inputs = mock_chain.get_transaction_inputs(malicious_account.clone(), None, &[
        note.id()
    ], &[]);

    // Execute the transaction and get the result
    let executed_transaction_2 = execute(
        TransactionContextBuilder::new(malicious_account.clone())
            .tx_inputs(tx_inputs)
            .expected_notes(vec![
                OutputNote::Full(note.clone())
            ])
            .build()
    );

    // Check that we got the expected result - TransactionExecutorError
    assert_transaction_executor_error!(executed_transaction_2, ERR_P2ID_TARGET_ACCT_MISMATCH)
}
