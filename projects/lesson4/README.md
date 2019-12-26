# 第四课
    1. Substrate Kitties V2 实现
    2. 链上升级
    3. pallet-sudo 代码分析
    4. pallet-member 与 pallet-collective 
    
# 第四课作业

## 设计加密猫模块V3
* 转移猫, 复杂度必须优于 O(n)

### V3 设计

#### 方案1 使用数组 O(1)
```text
pub OwnedKitties get(fn owned_kitties): map (T::AccountId, T::KittyIndex) => T::KittyIndex;
pub OwnedKittiesCount get(fn owned_kitties_count): map T::AccountId => T::KittyIndex;

为了让转让猫操作复杂度为O(1)
假设一种情况, 数组元素为5个. [1,2,3,4,5]
现在要删除元素2. 那么可以让OwnedKittiesCount=4, 并且OwnedKitties=[1,5,3,4]. 把最后一个元素覆盖要删除的元素.

这种方法效率高, 缺点是顺序打乱.
```
		
#### 方案2 双向链表O(1), 其实数组的方法更好, 但是为了练习Rust, 所以实现了这个方案
* 存储所有的猫用的map: 全局猫ID=>猫
* 存储个人的猫用的map: (用户ID, 全局猫ID) => KittyListItem{next:全局猫ID, prev:全局猫ID}
* 以上2个定义, 相当于每个用户都有一个链表, 在所有的猫的map上'链'出自己的猫集合.
* 转移猫的时候, 只需要修改个人的猫的链表即可. 
* 链表定义: runtime/src/kitties.rs:24
* 链表测试: runtime/src/kitties.rs:424
* 转猫测试: runtime/src/kitties.rs:494

## 完成 `combine_dna`
* 实现: runtime/src/kitties.rs:177
* 测试: runtime/src/kitties.rs:374

## 重构 `create` 使用新的帮助函数
* 实现: runtime/src/kitties.rs:196
* 测试: runtime/src/kitties.rs:353

# 额外作业
## 创建新的 polkadot apps 项目
## 创建树形存储
既然上面基于KV存储设计了链表, 那么照猫画虎即可设计二叉树.




