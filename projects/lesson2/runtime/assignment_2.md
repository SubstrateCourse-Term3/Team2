# 第二课作业
```text
一.设计加密码猫模块

   1. 数据结构

   2. 存储定义

   3. 可调用函数

   4. 算法伪代码

二. 需求

   1. 链上存储加密猫数据

   2. 遍历所有加密猫

   3. 每只猫都有自己的dna，为128bit的数据

   4. 设计如何生成dna (伪代码算法）

   5. 每个用户可以拥有零到多只猫

   6. 每只猫只有一个主人

   7. 遍历用户拥有的所有猫
```

# 设计加密码猫模块
```text
遍历所有加密猫
AllKittiesIndex:: u64 每产生一个猫就有一个新的index ，从零开始


每只猫都有自己的dna，为128bit的数据
设计如何生成dna (伪代码算法）
KittyDna: 
let random_hash = math.random
let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
                gen: 0,
            };
breeded_kittyDna :[u8;16]
fn DnaMerge(new_kittyA, new_kittyB) -> new_kitty {
	if new_kittyA前3个元素的值都是高于100{
		clone new_kittyA前3个元素
	} else{
		clone new_kittyB前3个元素
		}
 其余13个元素AB随机交换
             [a,b]=shuffle([a,b])

   }


每个用户可以拥有零到多只猫
遍历用户拥有的所有猫
If AccountId && kittyId {
    OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;
	}

每只猫只有一个主人
KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

链上存储加密猫数据
   AllKittiesCount get(all_kitties_count): u64;  全部猫数量
   AllKittiesIndex: map T::Hash => u64; 全部猫index
   OwnedKittiesIndex: map T::Hash => u64;每个人的猫index
   OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;猫的主人
   KittiesMap(KittyIndex, Kitty)猫ID和实例猫的对应