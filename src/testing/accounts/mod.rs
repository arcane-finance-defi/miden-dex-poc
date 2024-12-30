use miden_objects::{accounts::{Account, AccountBuilder, AccountStorageMode, AccountType}, assets::Asset, AccountError};

use crate::accounts::pool::PoolAccount;


pub fn create_prefunded_pool_account(
    init_seed: [u8; 32],
    assets: [Asset; 2],
) -> Result<Account, AccountError> {

    AccountBuilder::new()
        .init_seed(init_seed)
        .account_type(AccountType::RegularAccountImmutableCode)
        .storage_mode(AccountStorageMode::Public)
        .with_component(PoolAccount::new(assets.map(|asset| asset.faucet_id())))
        .with_assets(assets)
        .build_existing()
}