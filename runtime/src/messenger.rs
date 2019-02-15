/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/gav-template/srml/example/src/lib.rs

use srml_support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue};
use system::ensure_signed;
use rstd::cmp;
use parity_codec::Encode;
use runtime_primitives::traits::{As, Hash, Zero};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Peer<Hash> {
    id: Hash,
    nickname: Vec<u8>,
}

pub struct Msg<Hash> {
    uuid: Hash,
    msg: Vec<u8>,
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
    // TODO: Add other types and constants required configure this module.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as MessengerModule {
		// Just a dummy storage item. 
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(something): Option<u32>;
		Peers get(peer_id): map T::Hash => Peer<T::Hash>;
		PeerStorage get(peer_by_index): map u64 => T::Hash;

		Messages get(msg): map T::Hash => Msg<T::Hash>;

		MessagesStorage get(msg_index): map u64 => T::Hash;
		MessagesCount get(all_msg_count): u64;
		MessagesIndex: map T::Hash => u64;

		Nonce: u64;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			<Something<T>>::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		pub fn create_message(origin, from: T::Hash, to: T::hash, msg: Vec<u8>) -> Result {
			 let sender = ensure_signed(origin)?;
			 ensure!(<Peers<T>>::exists(from), "This 'from' does not exists");
			 ensure!(<Peers<T>>::exists(to), "This 'to' does not exists");

			 Self::_send_msg(from, to, msg)?;

			 Ok(())
		}
	}
}

decl_event!(
	/// An event in this module.
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		MessegeSended(AccountId, AccountId, Hash),
	}
);

impl<T: Trait> Module<T> {
    fn _send_msg(from: T::AccountId, to: T::AccountId, msg_hash: T::Hash, msg: Msg<T::Hash>) -> Result {

        let new_msg_count = Self::all_msg_count.checked_add(1).ok_or("Overflow adding a new msg");

        <Messages<T>>::insert(msg);

        <MessagesStorage<T>>::insert(Self::all_msg_count, msg_hash); // get(msg_index) //: map u64 => Vec<u8>;
        <MessagesCount<T>>::put(new_msg_count);// get(all_msg_count): u64;
        <MessagesIndex<T>>::insert(msg_hash, Self::all_msg_count); //: map T::Hash => u64;

        Self::deposit_event(RawEvent::MessegeSended(from, to, msg_hash));

        Ok(())
    }
}

/// tests for this module
#[cfg(test)]
mod tests {
    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        BuildStorage,
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
    };
    use support::{assert_ok, impl_outer_origin};

    use super::*;

    impl_outer_origin! {
		pub enum Origin for Test {}
	}

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;

    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Digest = Digest;
        type AccountId = u64;
        type Lookup = IdentityLookup<u64>;
        type Header = Header;
        type Event = ();
        type Log = DigestItem;
    }

    impl Trait for Test {
        type Event = ();
    }

    type MessageModule = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
    }

    #[test]
    fn it_works_for_default_value() {
        with_externalities(&mut new_test_ext(), || {
            // Just a dummy test for the dummy funtion `do_something`
            // calling the `do_something` function with a value 42
            assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
            // asserting that the stored value is equal to what we stored
            assert_eq!(TemplateModule::something(), Some(42));
        });
    }
}
