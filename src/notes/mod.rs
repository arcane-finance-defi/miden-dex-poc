use alloc::vec::Vec;

use miden_objects::{
    accounts::AccountId,
    assets::Asset,
    crypto::rand::FeltRng,
    notes::{
        Note, NoteAssets, NoteExecutionHint, NoteExecutionMode, NoteInputs,
        NoteMetadata, NoteRecipient, NoteTag, NoteType,
    },
    Felt, NoteError,
};

pub mod scripts;
pub mod utils;

// STANDARDIZED SCRIPTS
// ================================================================================================

/// Generates a Fund note.
///
/// This script enables the transfer of assets from the `sender` account to the pool's `target` account
/// by specifying the pool's account ID hash.
///
/// The passed-in `rng` is used to generate a serial number for the note. The returned note's tag
/// is set to the pool's account ID.
///
/// # Errors
/// Returns an error if deserialization or compilation of the `Fund` script fails.
pub fn create_fund_note<R: FeltRng>(
    sender: AccountId,
    target: AccountId,
    assets: Vec<Asset>,
    note_type: NoteType,
    aux: Felt,
    rng: &mut R,
) -> Result<Note, NoteError> {
    let note_script = scripts::fund();

    let inputs = NoteInputs::new(vec![target.into()])?; // 1 input
    let tag = NoteTag::from_account_id(target, NoteExecutionMode::Local)?;
    let serial_num = rng.draw_word();

    let metadata = NoteMetadata::new(sender, note_type, tag, NoteExecutionHint::always(), aux)?;
    let vault = NoteAssets::new(assets)?;
    let recipient = NoteRecipient::new(serial_num, note_script, inputs);
    Ok(Note::new(vault, metadata, recipient))
}