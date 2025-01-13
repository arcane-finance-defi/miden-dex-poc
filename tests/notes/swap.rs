use dex_poc::notes::create_swap_note;
use miden_assembly::diagnostics::IntoDiagnostic;
use miden_objects::{
    testing::account_id::{
        ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_1, 
        ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2
    },
    accounts::Account, 
    assets::{
        AssetVault, 
        FungibleAsset
    }, 
    notes::NoteType, 
    transaction::OutputNote, 
    FieldElement
};
use miden_tx::testing::{Auth, MockChain, TransactionContextBuilder};
use vm_processor::{crypto::RpoRandomCoin, Felt};

use crate::common::{
    accounts::create_prefunded_pool_account,
    executor::execute_with_debugger,
    notes::create_p2id_note_from_recipient,
    swap::calculate_swap
};

#[test]
fn swap_should_swap_against_pool() {
    let mut mock_chain = MockChain::new();

    let sender = mock_chain.add_existing_wallet(Auth::BasicAuth, vec![]);

    let pool = create_prefunded_pool_account(
        [1; 32],
        [
            FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_1.try_into().unwrap(), 10_000)
                .unwrap()
                .into(),
            FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), 10_000)
                .unwrap()
                .into()
        ],
    ).into_diagnostic().unwrap();

    let (swap_note, p2id_recipient) = &create_swap_note(
        sender.id(),
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_1.try_into().unwrap(), 100)
                .unwrap()
                .into(),
        ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(),
        NoteType::Public,
        Felt::ZERO,
        &mut RpoRandomCoin::new([Felt::new(1), Felt::new(2), Felt::new(3), Felt::new(4)])
    ).into_diagnostic().unwrap();

    let expected_amount_out: u64 = calculate_swap(10_000, 10_000, 100) as u64;

    let result_note = &create_p2id_note_from_recipient(p2id_recipient.clone(), vec![
        FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), expected_amount_out)
            .unwrap()
            .into()
    ]).into_diagnostic().unwrap();

    mock_chain.add_pending_account(pool.clone());
    mock_chain.add_pending_note(swap_note.clone());

    mock_chain.seal_block(None);


    // CONSTRUCT AND EXECUTE Swap TX (Success)
    // --------------------------------------------------------------------------------------------
    // Execute the transaction and get the witness

    let tx_inputs = mock_chain.get_transaction_inputs(pool.clone(), None, &[
        swap_note.id()
    ], &[]);

    let executed_transaction = execute_with_debugger(
    TransactionContextBuilder::new(pool.clone())
            .tx_inputs(tx_inputs)
            .expected_notes(vec![
                OutputNote::Full(result_note.clone())
            ])
            .build()
        )
        .into_diagnostic()
        .unwrap();

    // vault delta
    let target_pool_account_after: Account = Account::from_parts(
        pool.id(),
        AssetVault::new(&[
            FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_1.try_into().unwrap(), 10_100)
                .unwrap()
                .into(),
            FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), 10_000 - expected_amount_out)
                .unwrap()
                .into()
        ]).unwrap(),
        pool.storage().clone(),
        pool.code().clone(),
        Felt::new(2),
    );

    assert_eq!(executed_transaction.final_account().hash(), target_pool_account_after.hash());

    mock_chain.apply_executed_transaction(&executed_transaction);
    mock_chain.seal_block(None);

    // CONSTRUCT AND EXECUTE Withdraw result TX (Success)
    // --------------------------------------------------------------------------------------------
    // Execute the transaction and get the witness

    let executed_transaction = mock_chain
        .build_tx_context(
            sender.id(), 
            &[result_note.clone().id()], 
            &[])
        .build()
        .execute()
        .into_diagnostic().unwrap();

    // vault delta
    let target_account_after: Account = Account::from_parts(
        sender.id(),
        AssetVault::new(&[
            FungibleAsset::new(ACCOUNT_ID_FUNGIBLE_FAUCET_ON_CHAIN_2.try_into().unwrap(), expected_amount_out)
                .unwrap()
                .into()
        ]).unwrap(),
        sender.storage().clone(),
        sender.code().clone(),
        Felt::new(2),
    );

    assert_eq!(executed_transaction.final_account().hash(), target_account_after.hash());


}
