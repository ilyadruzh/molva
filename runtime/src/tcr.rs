use parity_codec::Encode;
use rstd::cmp;
use runtime_primitives::traits::{As, Hash, Zero};
use srml_support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue};
use system::ensure_signed;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct CuratedList<Hash> {
    id: Hash,
    name: Vec<u8>,
}

pub struct CuratedListItem<Hash> {
    id: Hash,
    name: Vec<u8>,
    list: Hash,
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
    <T as system::Trait>::AccountId,
    <T as system::Trait>::Hash,
    <T as balances::Trait>::Balance
    {
        CuratedListCreated(Hash),
        CuratedListItemCreated(Hash),
    }
);

decl_storage! {

    trait Store for Module<T: Trait> as TcrStorage {

          Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
          KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

          AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
          AllKittiesCount get(all_kitties_count): u64;
          AllKittiesIndex: map T::Hash => u64;

          OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
          OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
          OwnedKittiesIndex: map T::Hash => u64;

          Nonce: u64;
  }
}

decl_module! {

    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;


            // Create CuratedList
        fn create_curated_list(origin) -> Result {
             let sender = ensure_signed(origin)?;
            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }

        // Create curated list item and push to list
        fn create_curated_list_item(origin) -> Reslut {

        }

        // Choose
        fn vote(origin) -> Result {

        }


//        fn create_kitty(origin) -> Result {
//            let sender = ensure_signed(origin)?;
//            let nonce = <Nonce<T>>::get();
//            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
//                .using_encoded(<T as system::Trait>::Hashing::hash);
//
//            let new_kitty = Kitty {
//                    id: random_hash,
//                    dna: random_hash,
//                    price: <T::Balance as As<u64>>::sa(0),
//                    gen: 0,
//            };
//
//            Self::_mint(sender, random_hash, new_kitty)?;
//
//            <Nonce<T>>::mutate(|n| *n += 1);
//
//            Ok(())
//        }

//        fn set_price(origin, kitty_id: T::Hash, new_price: T::Balance) -> Result {
//            let sender = ensure_signed(origin)?;
//
//            ensure!(<Kitties<T>>::exists(kitty_id), "This cat does not exist");
//
//            let owner = Self::owner_of(kitty_id).ok_or("No owner for this object")?;
//            ensure!(owner == sender, "You are not the owner");
//
//            let mut kitty = Self::kitty(kitty_id);
//            kitty.price = new_price;
//
//            <Kitties<T>>::insert(kitty_id, kitty);
//
//            Self::deposit_event(RawEvent::PriceSet(sender, kitty_id, new_price));
//
//            Ok(())
//        }
//
//        fn transfer(origin, to: T::AccountId, kitty_id: T::Hash) -> Result {
//            let sender = ensure_signed(origin)?;
//
//            let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
//            ensure!(owner == sender, "You do not own this kitty");
//
//            Self::_transfer_from(sender, to, kitty_id)?;
//
//            Ok(())
//        }
//
//        fn buy_kitty(origin, kitty_id: T::Hash, max_price: T::Balance) -> Result {
//            let sender = ensure_signed(origin)?;
//
//            ensure!(<Kitties<T>>::exists(kitty_id), "This cat does not exist");
//
//            let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
//            ensure!(owner != sender, "You can't buy your own cat");
//
//            let mut kitty = Self::kitty(kitty_id);
//
//            let kitty_price = kitty.price;
//            ensure!(!kitty_price.is_zero(), "The cat you want to buy is not for sale");
//            ensure!(kitty_price <= max_price, "The cat you want to buy costs more than your max price");
//
//            <balances::Module<T>>::make_transfer(&sender, &owner, kitty_price)?;
//
//            Self::_transfer_from(owner.clone(), sender.clone(), kitty_id)?;
//
//            kitty.price = <T::Balance as As<u64>>::sa(0);
//            <Kitties<T>>::insert(kitty_id, kitty);
//
//            Self::deposit_event(RawEvent::Bought(sender, owner, kitty_id, kitty_price));
//
//            Ok(())
//        }
//
//        fn breed_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> Result{
//            let sender = ensure_signed(origin)?;
//
//            ensure!(<Kitties<T>>::exists(kitty_id_1), "This cat 1 does not exist");
//            ensure!(<Kitties<T>>::exists(kitty_id_2), "This cat 2 does not exist");
//
//            let nonce = <Nonce<T>>::get();
//            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
//                .using_encoded(<T as system::Trait>::Hashing::hash);
//
//            let kitty_1 = Self::kitty(kitty_id_1);
//            let kitty_2 = Self::kitty(kitty_id_2);
//
//            let mut final_dna = kitty_1.dna;
//
//            for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate() {
//                if r % 2 == 0 {
//                    final_dna.as_mut()[i] = *dna_2_element;
//                }
//            }
//
//            let new_kitty = Kitty {
//                id: random_hash,
//                dna: final_dna,
//                price: <T::Balance as As<u64>>::sa(0),
//                gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
//            };
//
//            Self::_mint(sender, random_hash, new_kitty)?;
//
//            <Nonce<T>>::mutate(|n| *n += 1);
//
//            Ok(())
//        }
    }

}

impl<T: Trait> Module<T> {

    fn _vote() -> Result {

    }

    fn _mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
        ensure!(!<KittyOwner<T>>::exists(kitty_id), "Kitty already exists");

        let owned_kitty_count = Self::owned_kitty_count(&to);

        let new_owned_kitty_count = owned_kitty_count.checked_add(1)
            .ok_or("Overflow adding a new owned kitty")?;

        let all_kitties_count = Self::all_kitties_count();

        let new_all_kitties_count = all_kitties_count.checked_add(1)
            .ok_or("Overflow adding a new kitty")?;

//        ensure!(!<Kitties<T>>::exists(kitty_id), "Kitty already exists");

        <Kitties<T>>::insert(kitty_id, new_kitty);
        <KittyOwner<T>>::insert(kitty_id, &to);

        <AllKittiesArray<T>>::insert(all_kitties_count, kitty_id);
        <AllKittiesCount<T>>::put(new_all_kitties_count);
        <AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);

        <OwnedKittiesArray<T>>::insert((to.clone(), new_owned_kitty_count), kitty_id);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
        <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);

        Self::deposit_event(RawEvent::Created(to, kitty_id));

        Ok(())
    }

    fn _transfer_from(from: T::AccountId, to: T::AccountId, kitty_id: T::Hash) -> Result {
        let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;

        ensure!(owner == from, "'from' account does not own this kitty");

        let owned_kitty_count_from = Self::owned_kitty_count(&from);
        let owned_kitty_count_to = Self::owned_kitty_count(&to);

        let new_owned_kitty_count_to = owned_kitty_count_to.checked_add(1)
            .ok_or("Transfer causes overflow of 'to' kitty balance")?;

        let new_owned_kitty_count_from = owned_kitty_count_from.checked_sub(1)
            .ok_or("Transfer causes underflow of 'from' kitty balance")?;

        // "Swap and pop"
        let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);
        if kitty_index != new_owned_kitty_count_from {
            let last_kitty_id = <OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
            <OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
            <OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
        }

        // Now we can remove this item by removing the last element

        <KittyOwner<T>>::insert(&kitty_id, &to);
        <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count_to);

        <OwnedKittiesArray<T>>::remove((from.clone(), new_owned_kitty_count_from));
        <OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count_to), kitty_id);

        <OwnedKittiesCount<T>>::insert(&from, new_owned_kitty_count_from);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count_to);

        Self::deposit_event(RawEvent::Transferred(from, to, kitty_id));

        Ok(())
    }
}