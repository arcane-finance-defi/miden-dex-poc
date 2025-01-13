use alloc::vec::Vec;
use miden_objects::{accounts::AccountId, assets::Asset, notes::{Note, NoteAssets, NoteExecutionHint, NoteMetadata, NoteRecipient, NoteTag}, Felt, FieldElement, NoteError};

pub fn create_p2id_note_from_recipient(recipient: NoteRecipient, assets: Vec<Asset>) -> Result<Note, NoteError> {
    Ok(Note::new(NoteAssets::new(assets)?, NoteMetadata::new(
        AccountId::new_unchecked([Felt::ZERO, Felt::ZERO]),
        miden_objects::notes::NoteType::Public,
        NoteTag::from(0), 
        NoteExecutionHint::always(),
        Felt::ZERO
    )?, recipient))
}
