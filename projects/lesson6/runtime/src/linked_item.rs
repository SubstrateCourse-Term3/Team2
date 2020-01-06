use support::{StorageMap, Parameter};
use sp_runtime::traits::Member;
use codec::{Encode, Decode};

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct LinkedItem<Value> {
	pub prev: Option<Value>,
	pub next: Option<Value>,
}

pub struct LinkedList<Storage, Key, Value>(rstd::marker::PhantomData<(Storage, Key, Value)>);

impl<Storage, Key, Value> LinkedList<Storage, Key, Value> where
	Value: Parameter + Member + Copy,
	Key: Parameter,
	Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query=Option<LinkedItem<Value>>>,
{
	fn read_head(key: &Key) -> LinkedItem<Value> {
		Self::read(key, None)
	}

	fn write_head(key: &Key, item: LinkedItem<Value>) {
		Self::write(key, None, item);
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
		// 作业： 实现 append
		let head = Self::read_head(key);
		Self::write_head(key, LinkedItem {
			prev: Some(value.clone()),
			next: head.next,
		});

		let prev = Self::read(key, head.prev);
		Self::write(key, head.prev, LinkedItem {
			prev: prev.prev,
			next: Some(value.clone()),
		});

		Self::write(key, Some(value), LinkedItem {
			prev: head.prev,
			next: None,
		});
	}

	pub fn remove(key: &Key, value: Value) {
		// 作业： 实现 remove
		if let Some(item) = Storage::take((key, Some(value))) {
			let prev = Self::read(key, item.prev);
			Self::write(key, item.prev, LinkedItem {
				prev: prev.prev,
				next: item.next,
			});

			let next = Self::read(key, item.next);
			Self::write(key, item.next, LinkedItem {
				prev: item.prev,
				next: next.next,
			});
		}
	}
}