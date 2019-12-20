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

# 加密猫设计稿

## 数据结构定义
```text
AccountID 为 substrate 上的用户账户ID

KittyIndex : u32 记录下一个新产生的Kitty的序号. 每取一次加1.  
    nextID(c : Store_KittyCount)->KittyIndex{c++}

KittyDNA : [u8;16]  记录猫的DNA. 
    DNAMerge(a:DNA,b:DNA)->(new:DNA) {
        处理a,b的前10个字节
        {
            如果对应位一样, new的对应位为1, 否则为0
        }
        随机交换a,b
        [a,b]=shuffle([a,b])
        处理a,b的后6个字节
        {
            new[10:13]=a[10:13]//覆盖新DNA对应位置
            new[13:16]=b[13:16]//覆盖新DNA对应位置
        }
    }

Kitty: 猫
    DNA : KittyDNA 它的DNA
    OwnerID : AccountID 它的主人的ID
    // 如要遍历, 则根据当前的 KittyCount, 挨个读取即可
    GetKitty(i: KittyIndex){根据index到存储Store_KittyMap里面获取kitty} 
    // 支持大量读取 Kitty
    GetKittiesByRange(begin:KittyIndex,end:KittyIndex){循环调用GetKitty} 
    
KittyListItem: 链表项
    Prev:KittyIndex
    Next:KittyIndex

KittyList: 链表, 基于KV存储
    getHead(Store_KittyOwnerMap,AccountId)->KittyListItem { Store_KittyOwnerMap::get(AccountId,None) }
    // 伪代码, 读取某人下一个猫ID
    // 返回全局猫ID以及链表项
    NextKitty(Store_KittyOwnerMap,KittyListItem,AccountId)->(KittyIndex,KittyListItem) { 
        if KittyListItem.is_none(){
            let head = getHead(Store_KittyOwnerMap,AccountId);
            return (head.next,Store_KittyOwnerMap::get((AccountId,head.next)));
        } else {
            return (KittyListItem.next,Store_KittyOwnerMap::get((AccountId,KittyListItem.next)));
        }
    }
    PrevKitty(){}//类似NextKitty
    // 保存猫到链表
    Save(Store_KittyOwnerMap,newId,sender) {
        let lastOwnerKittyId = Store_UserKittyCountMap::get(sender);
        Store_UserKittyCountMap::put(sender, lastOwnerKittyId+1);//递增个人计数器
        let item = Store_KittyOwnerMap::get((sender,lastOwnerKittyId));//读取链表项
        item.next = newId;//item.prev不变
        Store_KittyOwnerMap::put((sender,lastOwnerKittyId),item);//更新链表项, 使之指向此人的下一个猫ID
        let newItem = KittyListItem{Prev:lastOwnerKittyId,Next:None};
        Store_KittyOwnerMap::put((sender,newId),newItem);//插入新链表项
    }
```

## 全局过程, 可被外部调用
```text
CreateNewCat(sender, a:KittyIndex, b:KittyIndex){
    let (a,b) = (Store_KittyMap::get(a),Store_KittyMap::get(b));//读取父母
    let newDNA = KittyDNA::DNAMerge(a.DNA, b.DNA);//创建新DNA
    let newId = KittyIndex::nextID(Store_KittyCount)//创建新ID
    let newKitty = Kitty {DNA: newDNA, OwnerID:sender};//生成实体
    Store_KittyMap::put(newId,newKitty);//保存实体
    Store_KittyCount::inc();//递增全局计数器
    KittyList::Save(Store_KittyOwnerMap,newId,sender);//维护sender的猫链表
}
GetKittyCount() {Store_KittyCount::get()}
GetKittyCountByOwner(){Store_UserKittyCountMap::get(sender)}
GetKittyByID(id){Kitty::GetKitty(id)}
GetKittiesByRange(id1,id2){Kitty::GetKittiesByRange(id1,id2)}
// 返回猫实体, 以及 KittyListItem. 下次用户要查询自己的下一个猫,
// 需要带上 KittyListItem 实体, 如果没有, 要传None, 将查询第一个
GetNextKittyByOwner(KittyListItem)->(Kitty,KittyListItem){
    let (id,KittyListItem) = KittyList::NextKitty(Store_KittyOwnerMap,KittyListItem,sender);
    return (Kitty::GetKitty(id),KittyListItem);
}
GetPrevKittyByOwner(KittyListItem)->(Kitty,KittyListItem){
    let (id,KittyListItem) = KittyList::PrevKitty(Store_KittyOwnerMap,KittyListItem,sender);
    return (Kitty::GetKitty(id),KittyListItem);
}
```

## 全局存储
```text
Store_KittyCount u32 全局猫数量
Store_UserKittyCountMap(AccountId, u32) 每个账户的猫数量
Store_KittyMap(KittyIndex, Kitty) 猫ID和猫实例的对应
Store_KittyOwnerMap((AccountId,KittyIndex), KittyListItem) // 每个人的猫链表
```

