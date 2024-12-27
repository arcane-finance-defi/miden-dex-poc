use miden_assembly::{
    Library,
    utils::{sync::LazyLock, Deserializable}
};

static POOL_ACCOUNT_CODE: LazyLock<Library> = LazyLock::new(|| {
    let bytes = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/assets/contracts/pool.masl"
    ));
    Library::read_from_bytes(bytes).expect("Shipped Pool pair library is well-formed")
});


pub fn pool_account_library() -> Library {
    POOL_ACCOUNT_CODE.clone()
}