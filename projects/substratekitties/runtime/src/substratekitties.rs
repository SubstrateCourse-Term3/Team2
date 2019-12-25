use support::{decl_storage, decl_module, StorageMap, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{As, Hash};



pub trait Trait: balances::Trait {}

#[derive(Encode,Decode,Default,Clone,PartialEq)]
#[cfg_attr(feature="std",derive(Debug))]
pub struct Kitty<Balance,Hash>{
    id:Hash,
    dna:Hash,
    price:Balance,
    gen:u64,
}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
        //Test value
        //Value get(value_getter):u64;
        //Value: map T::AccountId=>u64; 
        MyKitty: map T::AccountId=>Kitty<T::Balance,T::Hash>;



    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here

        fn create_kitty(origin, price:T::Balance, id:T::Hash, dna:T::Hash, gen:u64) -> Result{
            let sender=ensure_signed(origin)?;
           //<Value<T>>::put(v);
            //<Value<T>>::insert(sender,v);

            let mut new_kitty=Kitty{
                id:<T as system::Trait>::Hashing::hash_of(&0),
                dna:<T as system::Trait>::Hashing::hash_of(&0),
                price:<T::Balance as As<u64>>::sa(0),
                gen:0,
            };
            new_kitty.id=id;
            new_kitty.dna=dna;
            new_kitty.price=price;
            new_kitty.gen=gen;

            <MyKitty<T>>::insert(&sender, new_kitty);


            Ok(())
        }

       

    }
}
