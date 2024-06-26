// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing and working with storage items.

mod storage_address;
mod storage_client;
mod storage_key;
mod storage_type;
mod utils;

pub use storage_client::StorageClient;

pub use storage_type::{Storage, StorageKeyValuePair};

/// Types representing an address which describes where a storage
/// entry lives and how to properly decode it.
pub mod address {
    pub use super::{
        storage_address::{dynamic, Address, DynamicAddress, StorageAddress, Yes},
        storage_key::{StaticStorageKey, StorageKey},
    };
}

pub use storage_key::StorageKey;

// For consistency with other modules, also expose
// the basic address stuff at the root of the module.
pub use storage_address::{dynamic, Address, DynamicAddress, StorageAddress};
