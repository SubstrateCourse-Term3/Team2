use support::{StorageMap, Parameter};
use sp_runtime::traits::Member;
use codec::{Encode, Decode};

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
pub struct LinkedItem<Value> {
	pub prev: Option<Value>,
	pub next: Option<Value>,
}

impl<Value> Encode for LinkedItem<Value> where Option<Value>: Encode {
	fn encode_to<EncOut: codec::Output>(&self, dest: &mut EncOut) {
		dest.push(&self.prev);
		dest.push(&self.next);
	}
}

impl<Value> Decode for LinkedItem<Value> where Option<Value>: Decode {
	fn decode<DecIn: codec::Input>(
		input: &mut DecIn,
	) -> core::result::Result<Self, codec::Error> {
		Ok(LinkedItem {
			prev: {
				let res = Decode::decode(input);
				match res {
					Err(_) => return Err("Error decoding field LinkedItem.prev".into()),
					Ok(a) => a,
				}
			},
			next: {
				let res = Decode::decode(input);
				match res {
					Err(_) => return Err("Error decoding field LinkedItem.next".into()),
					Ok(a) => a,
				}
			},
		})
	}
}

impl<Value> codec::EncodeLike for LinkedItem<Value> where Option<Value>: Encode {}

pub struct LinkedList<Storage, Key, Value>(rstd::marker::PhantomData<(Storage, Key, Value)>);

impl<Storage, Key, Value> LinkedList<Storage, Key, Value> where
	Value: Parameter + Member + Copy,
	Key: Parameter,
	Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query=Option<LinkedItem<Value>>>,
{
	fn read_head(key: &Key) -> LinkedItem<Value> {
		Self::read(key, None)
	}

	fn write_head(account: &Key, item: LinkedItem<Value>) {
		Self::write(account, None, item);
	}

	fn read(key: &Key, value: Option<Value>) -> LinkedItem<Value> {
		Storage::get((&key, value)).unwrap_or_else(|| LinkedItem {
			prev: None,
			next: None,
		})
	}

	fn write(key: &Key, value: Option<Value>, item: LinkedItem<Value>) {
		Storage::insert((&key, value), item);
	}

	pub fn append(key: &Key, value: Value) {
		let head = Self::read_head(key);
		let new_head = LinkedItem {
			prev: Some(value),
			next: head.next,
		};

		Self::write_head(key, new_head);

		let prev = Self::read(key, head.prev);
		let new_prev = LinkedItem {
			prev: prev.prev,
			next: Some(value),
		};
		Self::write(key, head.prev, new_prev);

		let item = LinkedItem {
			prev: head.prev,
			next: None,
		};
		Self::write(key, Some(value), item);
	}

	pub fn remove(key: &Key, value: Value) {
		if let Some(item) = Storage::take((&key, Some(value))) {
			let prev = Self::read(key, item.prev);
			let new_prev = LinkedItem {
				prev: prev.prev,
				next: item.next,
			};

			Self::write(key, item.prev, new_prev);

			let next = Self::read(key, item.next);
			let new_next = LinkedItem {
				prev: item.prev,
				next: next.next,
			};

			Self::write(key, item.next, new_next);
		}
	}
}