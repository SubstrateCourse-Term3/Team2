# 第三课
    1. 如何创建 Substrate Module
    2. macro 宏使用
        1. cargo expand
        2. decl_module
        3. decl_storage
    3. Substrate Kitties 简单实现
        https://polkadot.js.org/apps/#/settings/developer
        { "Kitty": "[u8; 16]" }
    4. pallet-assets 代码分析
    
# 第三课作业

## create 这里面的kitties_count有溢出的可能性，修复这个问题
`修复处: runtime/src/kitties.rs:42`  
`测试溢出: runtime/src/kitties.rs:132`

## 设计加密猫模块 V2

### 需求
    1. 繁殖小猫
    2. 选择两只现有的猫作为父母
    3. 小猫必须继承父母的基因
    4. 同样的父母生出来的小猫不能相同
    
### V2 设计稿 (隐藏了 V1 中不变的部分)

#### 数据结构定义
```text
KittyDNA : [u8;16]  记录猫的DNA. 
    DNAMerge(a:DNA,b:DNA,mutation_factor:DNA)->(new:DNA) {
        // 处理a,b的前12个字节, 对应位进行XOR操作. 下面用的字节slice来表示
        new[0:12] =  a[0:12] xor b[0:12] xor mutation_factor[0:12];
        
        // 下面保留一些父母的特征给后代
        随机交换a,b
        [a,b]=shuffle([a,b])
        处理a,b的后4个字节
        {
            new[12:14]=a[12:14]//覆盖新DNA对应位置
            new[14:16]=b[14:16]//覆盖新DNA对应位置
        }
    }
```

#### 全局过程, 可被外部调用
```text
BreedKitty(sender, a:KittyIndex, b:KittyIndex){
    check_owner(sender,a,b);//确保a,b的主人是sender 
    check_kitties_count_overflow();//确保不会溢出
    
    // 生成随机dna, 充当变异因子
    let payload: (T::Hash, T::AccountId, Option<u32>, T::BlockNumber) = (
        <randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
        sender,
        <system::Module<T>>::extrinsic_index(),
        <system::Module<T>>::block_number(),
    );
    let mutation_factor: [u8; 16] = payload.using_encoded(blake2_128);

    let (a,b) = (Store_KittyMap::get(a),Store_KittyMap::get(b));//读取父母
    let newDNA = KittyDNA::DNAMerge(a.DNA, b.DNA, mutation_factor);//创建新DNA
    let newId = KittyIndex::nextID(Store_KittyCount)//创建新ID
    let newKitty = Kitty {DNA: newDNA, OwnerID:sender};//生成实体
    Store_KittyMap::put(newId,newKitty);//保存实体
    Store_KittyCount::inc();//递增全局计数器
    KittyList::Save(Store_KittyOwnerMap,newId,sender);//维护sender的猫链表
}
```

# 额外作业
    1. 解释如何在链上实现（伪）随机数？
        1. 对比不同方案的优缺点

## 真随机数
```
以太坊的真随机发生器Oracle就是引入了random.org提供的随机数.
据说是通过可验证的不可篡改的通道引入区块链系统内部. 
random.org声称使用什么大气中的随机因素. 但是random.org是个
中心化的服务.这对于主打去中心化的区块链系统来说,无疑是如鲠在喉.
```

## 伪随机数

### Substrate Randomness::random_seed()
```
根据函数的文档描述, 其算法是使用了前81个块的块Hash, 作为输入
来生成随机数的. 显然, 安全性不是很好. 因为区块信息的公开的, 
块生成者可以影响生成的区块的Hash, 特别的多个块生产这合谋的情况.
但这样的安全性对于涉及的利益不大的应用来说, 已经足够. 
```

### commit & reveal 的方式
```
这种方式使用起来有点麻烦. 例如用在竞猜游戏里面. 
庄家先在数据库里面放置一个数reveal, 然后将commit发布到网站, 
其中hash(reveal)=commit. 玩家可以在规定时间内对某个commit
进行相关下注操作(区块链交易). 最后的开奖环节, 
庄家发送一笔区块链交易, 使用reveal和玩家下注时的区块
Hash来作为随机发生器的输入.
这样可以防止玩家, 或者庄家出老千.
```

### VRF (Verifiable Random Function, 可验证随机函数)
```
VRF单独使用没有办法防止女巫攻击.
应该可以结合其他方式使用. 
```



