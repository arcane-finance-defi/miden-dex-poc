use crate::accounts::components::pool_account_library;

use miden_objects::{
    accounts::{Account, AccountBuilder, AccountComponent, AccountId, AccountIdAnchor, AccountStorageMode, AccountType, StorageSlot},
    AccountError, Word,
};

pub struct PoolAccount {
    asset_faucets: [AccountId; 2],
}

impl PoolAccount {
    pub fn new(asset_faucets: [AccountId; 2]) -> Self {
        Self { asset_faucets }
    }
}


impl From<PoolAccount> for AccountComponent {
    fn from(pool: PoolAccount) -> Self {
        AccountComponent::new(
            pool_account_library(), 
            vec![
                StorageSlot::Value([
                    pool.asset_faucets[0].first_felt(), 
                    pool.asset_faucets[0].second_felt(), 
                    pool.asset_faucets[1].first_felt(), 
                    pool.asset_faucets[1].second_felt(), 
                ])
            ]
        )
          .expect("pool account component should satisfy the requirements of a valid account component")
          .with_supported_type(AccountType::RegularAccountImmutableCode)
    }
}

fn account_builder(
    init_seed: [u8; 32],
    asset_faucets: [AccountId; 2],
    account_type: AccountType,
    account_storage_mode: AccountStorageMode
) -> AccountBuilder {
    AccountBuilder::new()
        .init_seed(init_seed)
        .account_type(account_type)
        .storage_mode(account_storage_mode)
        .with_component(PoolAccount::new(asset_faucets))
}

pub fn create_pool_account(
    init_seed: [u8; 32],
    asset_faucets: [AccountId; 2],
    account_type: AccountType,
    account_storage_mode: AccountStorageMode,
    anchor: AccountIdAnchor,
) -> Result<(Account, Word), AccountError> {
    let (account, account_seed) = account_builder(
        init_seed, 
        asset_faucets,
        account_type, 
        account_storage_mode
    ).anchor(anchor).build()?;
    Ok((account, account_seed))
}