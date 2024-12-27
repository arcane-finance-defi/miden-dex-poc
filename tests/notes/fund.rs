use dex_poc::{
    errors::note_errors::ERR_P2ID_TARGET_ACCT_MISMATCH, notes::create_fund_note,
};
use miden_objects::{
    accounts::{
        account_id::testing::ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2,
        Account,
    },
    assets::{Asset, AssetVault, FungibleAsset},
    crypto::rand::RpoRandomCoin,
    notes::NoteType,
    Felt,
};
use miden_tx::testing::{Auth, MockChain};

use crate::assert_transaction_executor_error;

/// We test the Fund script with 2 assets to test the loop inside the script.
/// So we create a note containing two assets that can only be consumed by the target account.
#[test]
fn fund_script_multiple_assets() {
    let mut mock_chain = MockChain::new();

    // Create assets
    let fungible_asset_1: Asset = FungibleAsset::mock(123);
    let fungible_asset_2: Asset =
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), 456)
            .unwrap()
            .into();

    // Create sender and target account
    let sender_account = mock_chain.add_new_wallet(Auth::BasicAuth);
    let target_account = mock_chain.add_existing_wallet(Auth::BasicAuth, vec![]);
    
    let note = create_fund_note(
        sender_account.id(),
        target_account.id(),
        vec![fungible_asset_1, fungible_asset_2],
        NoteType::Public,
        Felt::new(0),
        &mut RpoRandomCoin::new([Felt::new(1), Felt::new(2), Felt::new(3), Felt::new(4)])
    )
    .unwrap();

    println!("target_account.id: {:?}", target_account.id());
    println!("note.inputs: {:?}", note.inputs());

    // Create the note
    mock_chain.add_pending_note(note.clone());

    mock_chain.seal_block(None);

    // CONSTRUCT AND EXECUTE TX (Success)
    // --------------------------------------------------------------------------------------------
    // Execute the transaction and get the witness
    let executed_transaction = mock_chain
        .build_tx_context(target_account.id(), &[note.id()], &[])
        .build()
        .execute()
        .unwrap();

    // vault delta
    let target_account_after: Account = Account::from_parts(
        target_account.id(),
        AssetVault::new(&[fungible_asset_1, fungible_asset_2]).unwrap(),
        target_account.storage().clone(),
        target_account.code().clone(),
        Felt::new(2),
    );

    assert_eq!(executed_transaction.final_account().hash(), target_account_after.hash());

    // CONSTRUCT AND EXECUTE TX (Failure)
    // --------------------------------------------------------------------------------------------
    // A "malicious" account tries to consume the note, we expect an error (not the correct target)

    let malicious_account = mock_chain.add_existing_wallet(Auth::BasicAuth, vec![]);
    mock_chain.seal_block(None);

    // Execute the transaction and get the result
    let executed_transaction_2 = mock_chain
        .build_tx_context(malicious_account.id(), &[], &[note])
        .build()
        .execute();

    // Check that we got the expected result - TransactionExecutorError
    assert_transaction_executor_error!(executed_transaction_2, ERR_P2ID_TARGET_ACCT_MISMATCH)
}
