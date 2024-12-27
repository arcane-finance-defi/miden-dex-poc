
use miden_assembly::Assembler;
use miden_lib::transaction::TransactionKernel;

use crate::accounts::components::pool_account_library;

pub fn test_assembler() -> Assembler {
    let assembler = TransactionKernel::testing_assembler();

    assembler.with_library(pool_account_library()).unwrap()
}