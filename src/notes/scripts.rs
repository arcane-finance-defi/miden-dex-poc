use miden_objects::{
    notes::NoteScript,
    utils::{sync::LazyLock, Deserializable},
    vm::Program,
};

// Initialize the P2ID note script only once
static FUND_SCRIPT: LazyLock<NoteScript> = LazyLock::new(|| {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/note_scripts/fund.masb"));
    let program = Program::read_from_bytes(bytes).expect("Shipped Fund script is well-formed");
    NoteScript::new(program)
});

static SWAP_SCRIPT: LazyLock<NoteScript> = LazyLock::new(|| {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/note_scripts/swap.masb"));
    let program = Program::read_from_bytes(bytes).expect("Shipped Swap script is well-formed");
    NoteScript::new(program)
});

/// Returns the Fund note script.
pub fn fund() -> NoteScript {
    FUND_SCRIPT.clone()
}

/// Returns the Swap note script.
pub fn swap() -> NoteScript {
    SWAP_SCRIPT.clone()
}

