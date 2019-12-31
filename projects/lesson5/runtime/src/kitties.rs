use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, traits::Randomness, dispatch, Parameter};
use sp_runtime::traits::{SimpleArithmetic, Bounded, Member};
use codec::{Encode, Decode};
use runtime_io::hashing::blake2_128;
use system::ensure_signed;
use rstd::result;
use support::traits::Currency;
use support::traits::ExistenceRequirement;

pub trait Trait: system::Trait {
    type KittyIndex: Parameter + Member + SimpleArithmetic + Bounded + Default + Copy;
    type Currency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct KittyLinkedItem<T: Trait> {
    pub prev: Option<T::KittyIndex>,
    pub next: Option<T::KittyIndex>,
}

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        /// Stores all the kitties, key is the kitty id / index
        pub Kitties get(fn kitties): map T::KittyIndex => Option<Kitty>;
        /// Stores the total number of kitties. i.e. the next kitty index
        pub KittiesCount get(fn kitties_count): T::KittyIndex;

        pub OwnedKitties get(fn owned_kitties): map (T::AccountId, Option<T::KittyIndex>) => Option<KittyLinkedItem<T>>;

        /// T::KittyIndex 从0开始, 方便遍历. 作为数组使用
        /// `Arr[i]->(谁的猫, 全局猫ID, 多少钱)`
        pub Prices get(fn prices): map T::KittyIndex => (T::AccountId, T::KittyIndex, BalanceOf<T>);
        pub PricesCount get(fn prices_count): T::KittyIndex;
        pub OnSale get(fn on_sale): map T::KittyIndex => bool;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Create a new kitty
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;

            // Generate a random 128bit value
            let dna = Self::random_value(&sender);

            // Create and store kitty
            let kitty = Kitty(dna);
            Self::insert_kitty(&sender, kitty_id, kitty);
        }

        /// Breed kitties
        pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
            let sender = ensure_signed(origin)?;
            Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;
        }

        // 作业：实现 transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex)
        // 使用 ensure! 来保证只有主人才有权限调用 transfer
        // 使用 OwnedKitties::append 和 OwnedKitties::remove 来修改小猫的主人
        pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) {
            let sender = ensure_signed(origin)?;
            Self::transfer_kitty(sender, to, kitty_id)?;
        }

        pub fn ask(origin, kitty_id: T::KittyIndex, price: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;
            Self::ask_kitty(sender, kitty_id, price)?;
        }

        pub fn delete_price(origin, index: T::KittyIndex, kitty_index: T::KittyIndex) {
            let sender = ensure_signed(origin)?;
            Self::delete_price_kitty(sender, index, kitty_index)?;
        }

        pub fn buy(origin, index: T::KittyIndex) {
            let sender = ensure_signed(origin)?;
            Self::buy_kitty(sender, index)?;
        }
    }
}

impl<T: Trait> OwnedKitties<T> {
    fn read_head(account: &T::AccountId) -> KittyLinkedItem<T> {
        Self::read(account, None)
    }

    fn write_head(account: &T::AccountId, item: KittyLinkedItem<T>) {
        Self::write(account, None, item);
    }

    fn read(account: &T::AccountId, key: Option<T::KittyIndex>) -> KittyLinkedItem<T> {
        <OwnedKitties<T>>::get((&account, key)).unwrap_or_else(|| KittyLinkedItem {
            prev: None,
            next: None,
        })
    }

    fn write(account: &T::AccountId, key: Option<T::KittyIndex>, item: KittyLinkedItem<T>) {
        <OwnedKitties<T>>::insert((&account, key), item);
    }

    pub fn append(account: &T::AccountId, kitty_id: T::KittyIndex) {
        let head = Self::read_head(account);
        let new_head = KittyLinkedItem {
            prev: Some(kitty_id.clone()),
            next: head.next,
        };

        Self::write_head(account, new_head);

        let prev = Self::read(account, head.prev);
        let new_prev = KittyLinkedItem {
            prev: prev.prev,
            next: Some(kitty_id.clone()),
        };
        Self::write(account, head.prev, new_prev);

        let item = KittyLinkedItem {
            prev: head.prev,
            next: None,
        };
        Self::write(account, Some(kitty_id), item);
    }

    pub fn remove(account: &T::AccountId, kitty_id: T::KittyIndex) {
        if let Some(item) = <OwnedKitties<T>>::take((&account, Some(kitty_id))) {
            let prev = Self::read(account, item.prev);
            let new_prev = KittyLinkedItem {
                prev: prev.prev,
                next: item.next,
            };

            Self::write(account, item.prev, new_prev);

            let next = Self::read(account, item.next);
            let new_next = KittyLinkedItem {
                prev: item.prev,
                next: next.next,
            };

            Self::write(account, item.next, new_next);
        }
    }
}

impl<T: Trait> Prices<T> {
    fn append(sender: T::AccountId, kitty_id: T::KittyIndex, price: BalanceOf<T>) {
        let count = <PricesCount<T>>::get();
        <Prices<T>>::insert(count, (sender, kitty_id.clone(), price));
        <PricesCount<T>>::put(count + 1.into());
        <OnSale<T>>::insert(kitty_id, true);
    }

    fn delete(p: (T::AccountId, T::KittyIndex, BalanceOf<T>), arr_index: T::KittyIndex, len: T::KittyIndex) {
        if arr_index < len.clone() - 1.into() {
            let last = <Prices<T>>::get(len.clone() - 1.into());
            <Prices<T>>::insert(arr_index, last);
        }
        <PricesCount<T>>::put(len - 1.into());
        // <Prices<T>>::remove(len.clone() - 1.into()); 这行可以打开. 最好不打开这样减少后续买卖的开销
        <OnSale<T>>::remove(p.1);
    }
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    ((selector & dna1) | (!selector & dna2))
}

impl<T: Trait> Module<T> {
    fn ask_kitty(sender: T::AccountId, kitty_id: T::KittyIndex, price: BalanceOf<T>) -> dispatch::Result {
        if !<OnSale<T>>::get(kitty_id.clone()) {
            ensure!(<OwnedKitties<T>>::exists((sender.clone(), Some(kitty_id))), "no permission to ask");
            <Prices<T>>::append(sender, kitty_id, price);
        }
        Ok(())
    }

    fn delete_price_kitty(sender: T::AccountId, arr_index: T::KittyIndex, kitty_index: T::KittyIndex) -> dispatch::Result {
        let len = Self::prices_count();
        ensure!(len > arr_index.clone(), "invalid index");
        let p = Self::prices(arr_index.clone());
        let owner = p.0.clone();
        let kitty_id = p.1.clone();
        ensure!(sender == owner && kitty_id == kitty_index, "permission deny");
        <Prices<T>>::delete(p, arr_index, len);
        Ok(())
    }

    fn buy_kitty(sender: T::AccountId, arr_index: T::KittyIndex) -> dispatch::Result {
        let len = Self::prices_count();
        ensure!(len > arr_index.clone(), "invalid index");
        let p = Self::prices(arr_index.clone());
        // 检查猫主人是否还拥有猫, 防止挂单后转移猫
        let owner = p.0.clone();
        let kitty_id = p.1.clone();
        let price = p.2.clone();

        // 挂单的人都已经不拥有这只猫了, 直接删除价格即可
        if !<OwnedKitties<T>>::exists((owner.clone(), Some(kitty_id.clone()))) {
            Self::delete_price_kitty(owner, arr_index, kitty_id)?;
        } else {
            ensure!(sender != owner, "permission deny");
            T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::AllowDeath)?;
            // 下架商品
            <Prices<T>>::delete(p, arr_index, len);
            // 转移猫
            Self::transfer_kitty(owner, sender, kitty_id)?;
        }
        Ok(())
    }

    fn transfer_kitty(sender: T::AccountId, receiver: T::AccountId, kitty_id: T::KittyIndex) -> dispatch::Result {
        ensure!(<OwnedKitties<T>>::exists((sender.clone(), Some(kitty_id))), "no permission to transfer kitty");
        <OwnedKitties<T>>::remove(&sender, kitty_id.clone());
        <OwnedKitties<T>>::append(&receiver, kitty_id);
        Ok(())
    }

    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let payload = (
            <randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
            &sender,
            <system::Module<T>>::extrinsic_index(),
            <system::Module<T>>::block_number(),
        );
        payload.using_encoded(blake2_128)
    }

    fn next_kitty_id() -> result::Result<T::KittyIndex, &'static str> {
        let kitty_id = Self::kitties_count();
        if kitty_id == T::KittyIndex::max_value() {
            return Err("Kitties count overflow");
        }
        Ok(kitty_id)
    }

    fn insert_owned_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex) {
        // 作业：调用 OwnedKitties::append 完成实现
        <OwnedKitties<T>>::append(owner, kitty_id);
    }

    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
        // Create and store kitty
        <Kitties<T>>::insert(kitty_id.clone(), kitty);
        <KittiesCount<T>>::put(kitty_id.clone() + 1.into());

        Self::insert_owned_kitty(owner, kitty_id);
    }

    fn do_breed(sender: &T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> dispatch::Result {
        let kitty1 = Self::kitties(kitty_id_1);
        let kitty2 = Self::kitties(kitty_id_2);

        ensure!(kitty1.is_some(), "Invalid kitty_id_1");
        ensure!(kitty2.is_some(), "Invalid kitty_id_2");
        ensure!(kitty_id_1 != kitty_id_2, "Needs different parent");

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty1.unwrap().0;
        let kitty2_dna = kitty2.unwrap().0;

        // Generate a random 128bit value
        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        // Combine parents and selector to create new kitty
        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

        Ok(())
    }
}

/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use primitives::H256;
    use support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
    };
    use support::traits::OnFreeBalanceZero;

    impl_outer_origin! {
        pub enum Origin for Test {}
    }


    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Test;

    pub type Balance = u64;

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: Weight = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
        pub const TransferFee: Balance = 0;
        pub const CreationFee: Balance = 0;
    }

    impl system::Trait for Test {
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
    }

    impl<T: Trait> OnFreeBalanceZero<T::AccountId> for Module<T> {
        fn on_free_balance_zero(_stash: &T::AccountId) {
            unimplemented!()
        }
    }

    impl balances::Trait for Test {
        type Balance = Balance;
        type OnFreeBalanceZero = Module<Test>;
        type OnNewAccount = ();
        type TransferPayment = ();
        type DustRemoval = ();
        type Event = ();
        type ExistentialDeposit = ();
        type TransferFee = TransferFee;
        type CreationFee = CreationFee;
    }

    impl Trait for Test {
        type KittyIndex = u32;
        type Currency = balances::Module<Self>;
    }

    type OwnedKittiesTest = OwnedKitties<Test>;
    type KittiesTest = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
        balances::GenesisConfig::<Test> {
            balances: vec![
                (1, 10000),
                (2, 10000),
                (3, 10000),
                (4, 10000),
            ],
            vesting: vec![],
        }.assimilate_storage(&mut t).unwrap();
        t.into()
    }

    #[test]
    fn test_ask_buy() {
        new_test_ext().execute_with(|| {
            let _ = <Module<Test>>::create(Origin::signed(1));
            assert_eq!(KittiesTest::prices_count(), 0);
            assert_ok!(<Module<Test>>::ask_kitty(1, 0, 1000));
            assert_eq!(KittiesTest::prices_count(), 1);
            let p = KittiesTest::prices(0);
            assert_eq!(p.0, 1);
            assert_eq!(p.1, 0);
            assert_eq!(p.2, 1000);

            assert_ok!(<Module<Test>>::buy_kitty(2, 0));
            assert_eq!(KittiesTest::prices_count(), 0);
        });
    }

    #[test]
    fn test_delete_price() {
        new_test_ext().execute_with(|| {
            let _ = <Module<Test>>::create(Origin::signed(1));
            assert_eq!(KittiesTest::prices_count(), 0);
            assert_ok!(<Module<Test>>::ask_kitty(1, 0, 1000));
            assert_eq!(KittiesTest::prices_count(), 1);
            let p = KittiesTest::prices(0);
            assert_eq!(p.0, 1);
            assert_eq!(p.1, 0);
            assert_eq!(p.2, 1000);

            assert_ok!(<Module<Test>>::delete_price_kitty(1, 0, 0));
            assert_eq!(KittiesTest::prices_count(), 0);
        });
    }

    #[test]
    fn test_transfer_kitty() {
        new_test_ext().execute_with(|| {
            let _ = <Module<Test>>::create(Origin::signed(1));
            let _ = <Module<Test>>::create(Origin::signed(2));
            assert_eq!(<KittiesCount<Test>>::get(), 2);
            let _ = <Module<Test>>::transfer(Origin::signed(1), 2, 0);
            assert_eq!(<KittiesCount<Test>>::get(), 2);
            assert_eq!(OwnedKittiesTest::read(&1, Some(0)), KittyLinkedItem { prev: None, next: None });
            assert_eq!(OwnedKittiesTest::read(&2, Some(0)), KittyLinkedItem { prev: Some(1), next: None });
            assert_eq!(OwnedKittiesTest::read(&2, Some(1)), KittyLinkedItem { prev: None, next: Some(0) });
        });
    }

    #[test]
    fn owned_kitties_can_append_values() {
        new_test_ext().execute_with(|| {
            OwnedKittiesTest::append(&0, 1);

            assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
                prev: Some(1),
                next: Some(1),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
                prev: None,
                next: None,
            }));

            OwnedKittiesTest::append(&0, 2);

            assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
                prev: Some(2),
                next: Some(1),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
                prev: None,
                next: Some(2),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
                prev: Some(1),
                next: None,
            }));

            OwnedKittiesTest::append(&0, 3);

            assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
                prev: Some(3),
                next: Some(1),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
                prev: None,
                next: Some(2),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
                prev: Some(1),
                next: Some(3),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
                prev: Some(2),
                next: None,
            }));
        });
    }

    #[test]
    fn owned_kitties_can_remove_values() {
        new_test_ext().execute_with(|| {
            OwnedKittiesTest::append(&0, 1);
            OwnedKittiesTest::append(&0, 2);
            OwnedKittiesTest::append(&0, 3);

            OwnedKittiesTest::remove(&0, 2);

            assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
                prev: Some(3),
                next: Some(1),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
                prev: None,
                next: Some(3),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);

            assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
                prev: Some(1),
                next: None,
            }));

            OwnedKittiesTest::remove(&0, 1);

            assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
                prev: Some(3),
                next: Some(3),
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), None);

            assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);

            assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
                prev: None,
                next: None,
            }));

            OwnedKittiesTest::remove(&0, 3);

            assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
                prev: None,
                next: None,
            }));

            assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), None);

            assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);

            assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);
        });
    }
}
