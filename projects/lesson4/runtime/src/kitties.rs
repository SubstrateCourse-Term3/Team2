use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, traits::Randomness, dispatch, Parameter};
use sp_runtime::traits::{SimpleArithmetic, Bounded};
use codec::{Encode, Decode};
use runtime_io::hashing::blake2_128;
use system::ensure_signed;
use rstd::result;
use sp_runtime::traits::Member;

pub trait Trait: system::Trait {
    type KittyIndex: Parameter + Member + SimpleArithmetic + Bounded + Default + Copy + Clone;
}

#[derive(Encode, Decode, Debug)]
pub struct Kitty(pub [u8; 16]);

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub struct LinkedListItem<T: Trait> {
    /// 下一个全局猫 ID
    next: Option<T::KittyIndex>,
    /// 上一个全局猫 ID
    prev: Option<T::KittyIndex>,
}

struct LinkedList<T: Trait>(rstd::marker::PhantomData<T>);

impl<T: Trait> LinkedList<T> {
    fn get_head(sender: T::AccountId) -> Option<LinkedListItem<T>> {
        <OwnedKittiesList<T>>::get((sender, <Option<T::KittyIndex>>::None))
    }

    fn save_head(sender: T::AccountId, head: LinkedListItem<T>) {
        <OwnedKittiesList<T>>::insert((sender, <Option<T::KittyIndex>>::None), head)
    }

    fn save_first_elem(sender: T::AccountId, g_kitty_id: T::KittyIndex) {
        // head的作用是指向第一个元素和最后一个
        Self::save_head(sender.clone(), LinkedListItem {
            next: Some(g_kitty_id.clone()), // 第一个元素
            prev: Some(g_kitty_id.clone()), // 最后一个元素
        });
        // 保存第一个元素, 第一个元素没有上一个和下一个元素.
        <OwnedKittiesList<T>>::insert((sender.clone(), Some(g_kitty_id)), LinkedListItem {
            next: <Option<T::KittyIndex>>::None,
            prev: <Option<T::KittyIndex>>::None,
        });
        // 数量为1
        let one: T::KittyIndex = 1.into();
        <OwnedKittiesCount<T>>::insert(sender, one);
    }

    fn append(sender: T::AccountId, g_kitty_id: T::KittyIndex) {
        match Self::get_head(sender.clone()) {
            None => {
                Self::save_first_elem(sender.clone(), g_kitty_id);
            }
            Some(mut head) => {
                if head.prev == None {
                    assert_eq!(head.next, <Option<T::KittyIndex>>::None);
                    Self::save_first_elem(sender.clone(), g_kitty_id);
                } else {
                    match <OwnedKittiesList<T>>::get((sender.clone(), Some(g_kitty_id))) {
                        None => {
                            // 从head读取最后一个元素
                            <OwnedKittiesList<T>>::mutate((sender.clone(), head.prev.clone()), |v| {
                                if let Some(el) = v {
                                    *v = Some(LinkedListItem { next: Some(g_kitty_id.clone()), prev: el.prev.clone() });
                                }
                            });

                            // 保存新元素
                            <OwnedKittiesList<T>>::insert((sender.clone(), Some(g_kitty_id)), LinkedListItem {
                                next: <Option<T::KittyIndex>>::None,
                                prev: head.prev.clone(),//原来的最后一个元素
                            });

                            // 更新head
                            head.prev = Some(g_kitty_id.clone()); // 新的最后一个元素
                            Self::save_head(sender.clone(), head);

                            <OwnedKittiesCount<T>>::mutate(sender.clone(), |v| {
                                *v += 1.into();
                                *v
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn delete(sender: T::AccountId, g_kitty_id: T::KittyIndex) {
        match Self::get_head(sender.clone()) {
            None => {}
            Some(mut head) => {
                match <OwnedKittiesList<T>>::take((sender.clone(), Some(g_kitty_id))) {
                    None => {}
                    Some(rm_elem) => {
                        if let Some(prev) = &rm_elem.prev {
                            <OwnedKittiesList<T>>::mutate((sender.clone(), Some(prev.clone())), |v| {
                                if let Some(el) = v {
                                    *v = Some(LinkedListItem { next: rm_elem.next.clone(), prev: el.prev.clone() });
                                }
                            });
                        }
                        if let Some(next) = &rm_elem.next {
                            <OwnedKittiesList<T>>::mutate((sender.clone(), Some(next.clone())), |v| {
                                if let Some(el) = v {
                                    *v = Some(LinkedListItem { next: el.next.clone(), prev: rm_elem.prev.clone() });
                                }
                            });
                        }
                        // 处理 head
                        let mut b = false;
                        if let Some(next) = &head.next {
                            if *next == g_kitty_id {
                                head.next = rm_elem.next.clone();
                                b = true;
                            }
                        }
                        if let Some(prev) = &head.prev {
                            if *prev == g_kitty_id {
                                head.prev = rm_elem.prev.clone();
                                b = true;
                            }
                        }
                        if b {
                            Self::save_head(sender.clone(), head);
                        }
                        <OwnedKittiesCount<T>>::mutate(sender.clone(), |v| {
                            *v -= 1.into();
                            *v
                        });
                    }
                }
            }
        }
    }
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map T::KittyIndex => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): T::KittyIndex;

		/// 保存链表的map
		pub OwnedKittiesList get(fn owned_kitties_list): map (T::AccountId,Option<T::KittyIndex>) => Option<LinkedListItem<T>>;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(fn owned_kitties_count): map T::AccountId => T::KittyIndex;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			Self::create_kitty(sender)?;
		}

		/// Breed kitties
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;
			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

        /// Transfer kitty
		pub fn transfer(origin, kitty_id: T::KittyIndex, receiver: T::AccountId) {
			let sender = ensure_signed(origin)?;
			Self::transfer_kitty(sender, kitty_id, receiver)?;
		}
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    // 作业：实现combine_dna
    // 伪代码：
    // selector.map_bits(|bit, index| if (bit == 1) { dna1 & (1 << index) } else { dna2 & (1 << index) })
    // 注意 map_bits这个方法不存在。只要能达到同样效果，不局限算法
    // 测试数据：dna1 = 0b11110000, dna2 = 0b11001100, selector = 0b10101010, 返回值 0b11100100
    let mut new = 0u8;
    for i in 0..8 {
        let m: u8 = 1 << i;
        if selector & m == 0 {
            new |= dna2 & m;
        } else {
            new |= dna1 & m;
        }
    }
    return new;
}

impl<T: Trait> Module<T> {
    fn create_kitty(sender: T::AccountId) -> dispatch::Result {
        // 作业：重构create方法，避免重复代码
        let kitty_id = Self::next_kitty_id()?;
        let dna = Self::random_value(&sender);
        let kitty = Kitty(dna);
        Self::insert_kitty(sender, kitty_id, kitty);
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

    //noinspection ALL
    fn insert_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
        // Create and store kitty
        <Kitties<T>>::insert(kitty_id, kitty);
        <KittiesCount<T>>::put(kitty_id + 1.into());

        // Store the ownership information
        <LinkedList<T>>::append(owner, kitty_id);
    }

    fn transfer_kitty(sender: T::AccountId, kitty_id: T::KittyIndex, receiver: T::AccountId) -> dispatch::Result {
        if let Some(kitty) = <OwnedKittiesList<T>>::get((sender.clone(), Some(kitty_id.clone()))) {
            <LinkedList<T>>::delete(sender.clone(), kitty_id.clone());
            <LinkedList<T>>::append(receiver.clone(), kitty_id.clone());
        }
        Ok(())
    }

    fn do_breed(sender: T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> dispatch::Result {
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

    fn set_kitties_count(c: T::KittyIndex) {
        <KittiesCount<T>>::put(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use primitives::H256;
    use support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
    };

    impl_outer_origin! {
		pub enum Origin for Test {}
	}

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Test;
    parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
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

    impl Trait for Test {
        type KittyIndex = u32;
    }

    type KittyModule = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
    }

    #[test]
    fn create_kitty() {
        new_test_ext().execute_with(|| {
            let _ = KittyModule::create_kitty(1);
            assert_eq!(1, KittyModule::kitties_count());
            if let Some(kitty) = KittyModule::kitties(0) {
                let v: Vec<u8> = (&kitty.0[..]).into();
                let b = v.iter().fold(0u128, |sum, &x| { sum + x as u128 });
                assert!(b > 0);
            } else {
                panic!("error")
            }
        });
    }

    #[test]
    fn create_kitty_overflow() {
        new_test_ext().execute_with(|| {
            KittyModule::set_kitties_count(Bounded::max_value());
            let r = KittyModule::create_kitty(1);
            assert_eq!(Err("Kitties count overflow"), r);
        });
    }

    #[test]
    fn breed_kitty() {
        new_test_ext().execute_with(|| {
            <system::Module<Test>>::set_extrinsic_index(0);
            let _ = KittyModule::create_kitty(1);

            <system::Module<Test>>::set_extrinsic_index(1);
            let _ = KittyModule::create_kitty(1);

            assert_eq!(2, KittyModule::kitties_count());
            assert_ok!(KittyModule::do_breed(1, 0, 1));
            assert_eq!(3, KittyModule::kitties_count());
            let dna1 = KittyModule::kitties(0).unwrap().0;
            let dna2 = KittyModule::kitties(1).unwrap().0;
            let dna3 = KittyModule::kitties(2).unwrap().0;
            assert_ne!(dna1, dna2);
            assert_ne!(dna1, dna3);
            assert_ne!(dna2, dna3);
        });
    }

    #[test]
    fn test_combine_dna() {
        new_test_ext().execute_with(|| {
            let dna1 = 0b11110000;
            let dna2 = 0b11001100;
            let sele = 0b10101010u8;
            let dna3 = combine_dna(dna1, dna2, sele);
            // println!("{:b}", dna3);
            assert_eq!(dna3, 0b11100100);

            let dna1 = 0b00010000;
            let dna2 = 0b10001000;
            let sele = 0b00010000;
            let dna3 = combine_dna(dna1, dna2, sele);
            // println!("{:b}", dna3);
            assert_eq!(dna3, 0b10011000);
        });
    }

    fn fill() {
        let head = <LinkedList<Test>>::get_head(1);
        assert_eq!(head, None);
        assert_eq!(KittyModule::owned_kitties_count(1), 0u32);

        <LinkedList<Test>>::append(1, 22u32);
        assert_eq!(KittyModule::owned_kitties_count(1), 1u32);
        <LinkedList<Test>>::append(1, 22u32);
        assert_eq!(KittyModule::owned_kitties_count(1), 1u32);
        <LinkedList<Test>>::append(1, 23u32);
        assert_eq!(KittyModule::owned_kitties_count(1), 2u32);
        <LinkedList<Test>>::append(1, 24u32);
        assert_eq!(KittyModule::owned_kitties_count(1), 3u32);

        // 遍历刚才加入的元素
        let head = <LinkedList<Test>>::get_head(1);
        assert_eq!(head, Some(LinkedListItem { next: Some(22u32), prev: Some(24u32) }));
        assert_eq!(
            KittyModule::owned_kitties_list((1, Some(22u32))),
            Some(LinkedListItem { next: Some(23u32), prev: None }),
        );
        assert_eq!(
            KittyModule::owned_kitties_list((1, Some(23u32))),
            Some(LinkedListItem { next: Some(24u32), prev: Some(22u32) }),
        );
        assert_eq!(
            KittyModule::owned_kitties_list((1, Some(24u32))),
            Some(LinkedListItem { next: None, prev: Some(23u32) }),
        );
    }

    #[test]
    fn test_add_delete() {
        new_test_ext().execute_with(|| {
            fill(); // head - 22 <-> 23 <-> 24
            // 删除中间的
            <LinkedList<Test>>::delete(1, 23u32);
            assert_eq!(KittyModule::owned_kitties_count(1), 2u32);
            // 遍历刚才加入的元素
            assert_eq!(
                <LinkedList<Test>>::get_head(1),
                Some(LinkedListItem { next: Some(22u32), prev: Some(24u32) }),
            );
            assert_eq!(
                KittyModule::owned_kitties_list((1, Some(22u32))),
                Some(LinkedListItem { next: Some(24u32), prev: None }),
            );
            assert_eq!(
                KittyModule::owned_kitties_list((1, Some(24u32))),
                Some(LinkedListItem { next: None, prev: Some(22u32) }),
            );
            // 目前的链表为: head - 22 <-> 24
            // 下面删除 24
            <LinkedList<Test>>::delete(1, 24u32);
            assert_eq!(KittyModule::owned_kitties_count(1), 1u32);
            // 遍历刚才加入的元素
            assert_eq!(
                <LinkedList<Test>>::get_head(1),
                Some(LinkedListItem { next: Some(22u32), prev: Some(22u32) }),
            );
            assert_eq!(
                KittyModule::owned_kitties_list((1, Some(22u32))),
                Some(LinkedListItem { next: None, prev: None }),
            );
            // 目前的链表为: head - 22
            // 下面删除 22
            <LinkedList<Test>>::delete(1, 22u32);
            assert_eq!(KittyModule::owned_kitties_count(1), 0u32);
            // 遍历刚才加入的元素
            assert_eq!(
                <LinkedList<Test>>::get_head(1),
                Some(LinkedListItem { next: None, prev: None }),
            );

            // 再加回去
            <LinkedList<Test>>::append(1, 22u32);
            assert_eq!(KittyModule::owned_kitties_count(1), 1u32);
            <LinkedList<Test>>::append(1, 22u32);
            assert_eq!(KittyModule::owned_kitties_count(1), 1u32);
            <LinkedList<Test>>::append(1, 23u32);
            assert_eq!(KittyModule::owned_kitties_count(1), 2u32);
            <LinkedList<Test>>::append(1, 24u32);
            assert_eq!(KittyModule::owned_kitties_count(1), 3u32);
            // 遍历刚才加入的元素
            let head = <LinkedList<Test>>::get_head(1);
            assert_eq!(head, Some(LinkedListItem { next: Some(22u32), prev: Some(24u32) }));
            assert_eq!(
                KittyModule::owned_kitties_list((1, Some(22u32))),
                Some(LinkedListItem { next: Some(23u32), prev: None }),
            );
            assert_eq!(
                KittyModule::owned_kitties_list((1, Some(23u32))),
                Some(LinkedListItem { next: Some(24u32), prev: Some(22u32) }),
            );
            assert_eq!(
                KittyModule::owned_kitties_list((1, Some(24u32))),
                Some(LinkedListItem { next: None, prev: Some(23u32) }),
            );
        });
    }

    #[test]
    fn test_transfer() {
        new_test_ext().execute_with(|| {
            let _ = KittyModule::create_kitty(1);
            assert_eq!(1, KittyModule::kitties_count());

            KittyModule::transfer_kitty(1, 0, 2);
            assert_eq!(1, KittyModule::kitties_count());
            assert_eq!(0, KittyModule::owned_kitties_count(1));
            assert_eq!(1, KittyModule::owned_kitties_count(2));
        });
    }
}
