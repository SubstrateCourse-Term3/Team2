use support::{decl_module, decl_storage, StorageValue, StorageMap, traits::Randomness, dispatch};
use codec::{Encode, Decode};
use runtime_io::hashing::blake2_128;
use system::ensure_signed;
use log::info;

pub trait Trait: system::Trait {}

#[derive(Encode, Decode, Default, Debug)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
            Self::create_kitty(origin)?;
		}
	}
}

impl<T: Trait> Module<T> {
    /*
        $ subkey inspect //Alice
        Secret Key URI `//Alice` is account:
          Secret seed:      0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
          Public key (hex): 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
          Account ID:       0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
          SS58 Address:     5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    */
    pub(crate) fn create_kitty(origin: T::Origin) -> dispatch::DispatchResult<&'static str> {
        // checking overflow
        let count = Self::kitties_count();
        if count == u32::max_value() {
            return Err("Kitties count overflow");
        }

        // pub struct AccountId32([u8; 32]);
        // ensure_signed -> AccountId;
        let sender: T::AccountId = ensure_signed(origin)?;
        //info!("---sender info---: {:?} ", sender);// d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
        let payload: (T::Hash, T::AccountId, Option<u32>, T::BlockNumber) = (
            <randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
            sender,
            <system::Module<T>>::extrinsic_index(),
            <system::Module<T>>::block_number(),
        );
        let dna: [u8; 16] = payload.using_encoded(blake2_128);
        let kitty = Kitty(dna);

        // saving new kitty
        Kitties::insert(count, kitty);
        KittiesCount::put(count + 1);
        Ok(())
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
    #[derive(Clone, Eq, PartialEq)]
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

    impl Trait for Test {}

    type KittyModule = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
    }

    #[test]
    fn create_kitty() {
        new_test_ext().execute_with(|| {
            KittyModule::create_kitty(Origin::signed(1));
            assert_eq!(1, KittyModule::kitties_count());
            // KittyModule::kitties(0).0
            let v: Vec<u8> = (&Kitties::get(0).0[..]).into();
            let b = v.iter().fold(0u128, |sum, &x| { sum + x as u128 });
            assert!(b > 0);
        });
    }

    #[test]
    fn create_kitty_overflow() {
        new_test_ext().execute_with(|| {
            KittiesCount::put(u32::max_value());
            let r = KittyModule::create_kitty(Origin::signed(1));
            assert_eq!(Err("Kitties count overflow"), r);
        });
    }
}
