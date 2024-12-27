#![no_std]
use alloc::sync::Arc;

#[macro_use]
extern crate alloc;

#[cfg(feature = "testing")]
pub mod testing;

pub mod notes;
pub mod errors;
pub mod accounts;