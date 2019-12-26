# 第三课作业
```text
设计加密猫模块 V2
需求
1. 繁殖小猫
2. 选择两只现有的猫作为父母
3. 小猫必须继承父母的基因
4. 同样的父母生出来的小猫不能相同

create 这里面的kitties_count有溢出的可能性，修复这个问题

# 内容
```text
//修复kitties_count有溢出的可能性
 let kitties_count = Self::kitties_count();

            let new_all_kitties_count = kitties_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to total supply")?;

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

  BreedKitty(sender, a:KittyIndex, b:KittyIndex){
    check_kitty_exit(a)//确保猫的存在 
    check_kitty_exit(b)//确保猫的存在 
    check_owner(sender,a,b);//确保a,b的主人是sender 
    check_kitties_count_overflow();//确保不会溢出

    // 生成随机dna
    let payload: (T::Hash, T::AccountId, Option<u32>, T::BlockNumber) = (
        <randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
        sender,
        <system::Module<T>>::extrinsic_index(),
        <system::Module<T>>::block_number(),
    );
    let dna: [u8; 16] = payload.using_encoded(blake2_128);

    
    let (a,b) = (Store_KittyMap::get(a),Store_KittyMap::get(b));//读取父母
    let newDNA = KittyDNA::DNAMerge(a.DNA, b.DNA, dna);//创建新DNA
    let newId = KittyIndex::nextID(Store_KittyCount)//创建新ID
    let newKitty = Kitty {DNA: newDNA, OwnerID:sender};//生成实体
    Store_KittyMap::put(newId,newKitty);//保存实体
    Store_KittyCount::inc();//递增全局计数器