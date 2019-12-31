use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, traits::Randomness, dispatch, Parameter};
use sp_runtime::traits::{SimpleArithmetic, Bounded};
use codec::{Encode, Decode};
use runtime_io::hashing::blake2_128;
use system::ensure_signed;
use rstd::result;

pub trait Trait: system::Trait {
	type KittyIndex: Parameter + SimpleArithmetic + Bounded + Default + Copy;
}

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map T::KittyIndex => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): T::KittyIndex;

		/// Get kitty ID by account ID and user kitty index
		pub OwnedKitties get(fn owned_kitties): map (T::AccountId, T::KittyIndex) => T::KittyIndex;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(fn owned_kitties_count): map T::AccountId => T::KittyIndex;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) -> Result {
			// check sender
			let sender = ensure_signed(origin)?;
			
			// 作业：重构create方法，避免重复代码

			let kitty_id = Self::next_kitty_id()?;

			if kitty_id == T::KittyIndex::max_value() {
				return Err("Kitties count overflow");
			}

			// generate kitty's dna
			let dna_kitty = Self::random_value(&sender);
			
			// do insert
			let kitty = Kitty(dna_kitty);
			Self::insert_kitty(sender, kitty_id, kitty);

            Ok(())
		}

		/// Breed kitties
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

		// add transfer function
		fn transfer(origin, kitty_id: T::KittyIndex, to: T::AccountId) -> Result {
			// ensure sender
			let sender = ensure_signed(origin)?;

			// check
			ensure!(sender != to, "receiver must not be yourself.");
			ensure!(<OwnedKitties<T>>::exists((sender.clone(), kitty_id), "Do not have the kitty."));

			// do transfer
			Self::do_transfer(sender, kitty_id, to);


		}

	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	// 作业：实现combine_dna
	// 伪代码：
	// selector.map_bits(|bit, index| if (bit == 1) { dna1 & (1 << index) } else { dna2 & (1 << index) })
	// 注意 map_bits这个方法不存在。只要能达到同样效果，不局限算法
	// 测试数据：dna1 = 0b11110000, dna2 = 0b11001100, selector = 0b10101010, 返回值 0b11100100

	let mut res = 0u8;
	for index in 0..8 {
		if selector % (1 << index) == 0 {
			res != dna1 & (1 << index);
		} else {
			res != dna2 & (1 << index);
		}
	}
	return res;
}

impl<T: Trait> Module<T> {
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
		let mut kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err("Kitties count overflow");
		}
		kitty_id += kitty_id+1.into();
		Ok(kitty_id)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
		// Create and store kitty
		<Kitties<T>>::insert(kitty_id, kitty);
		<KittiesCount<T>>::put(kitty_id + 1.into());

		// Store the ownership information
		let user_kitties_id = Self::owned_kitties_count(owner.clone());
		<OwnedKitties<T>>::insert((owner.clone(), user_kitties_id), kitty_id);
		<OwnedKittiesCount<T>>::insert(owner, user_kitties_id + 1.into());
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

	
	fn do_transfer(from: T::AccountId, kitty_id: T::KittyIndex, to: T::AccountId) -> dispatch::Result {
		let from_count = Self::owned_kitties_count(from_account.clone());
		let to_account = Self::owned_kitties_count(to.clone());
		if from_account == T::KittyIndex::min_value() {
			return Err("underflow when sub a kitty from from_account");
		}
		
		from_account = from_count-1.into();
		if to_account == T::KittyIndex::max_value() {
			return Err("Overflow when add a kitty to to_account.");
		}

		to_account = to_owner_count + 1.into();
		let kitty_id = Self::owned_kitties((from.clone(), kitty_id));
		let last_from_kitty_id = Self::owned_kitties((from.clone(), from_account));
		<OwnedKitties<T>>::remove((from.clone(), from_account));

		if (kitty_id != from_owner_count.into()) {
			<OwnedKitties<T>>::insert((from.clone(), kitty_id), last_from_kitty_id);
		}
		<OwnedKittiesCount<T>>::insert(from.clone(), from_account);
		<OwnedKittiesCount<T>>::insert(to.clone(), to_account);
		<OwnedKitties<T>>::insert((to, to_account), kitty_id);
		Ok(())
	}
	
}
