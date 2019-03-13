
use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Wallet<Hash, Balance> {
    id: Hash,
    balance: Balance,
}