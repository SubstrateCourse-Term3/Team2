# 第五课
    1. 链表数据结构实现
    2. 单元测试
    3. map / linked_map / double_map 对比
    4. pallet-balances 代码分析

# 第五课作业
## 设计加密猫模块V4
    需求：
    1）交易所
    2）给自己的小猫设定价钱
    3）购买其他人的小猫
* 给小猫定价格:     runtime/src/kitties.rs:73
* 给小猫定价格测试:  runtime/src/kitties.rs:381
* 购买小猫:        runtime/src/kitties.rs:83
* 购买小猫测试:     runtime/src/kitties.rs:381(定价&购买在一起)

## 完成 `transfer`
* 转移: runtime/src/kitties.rs:68
* 测试: runtime/src/kitties.rs:415

## 完成 `insert_owned_kitty`
* runtime/src/kitties.rs:245

# 额外作业
    1. 利用 polkadot.js 开发一个命令行软件
       1）创建小猫
       2） 赠予小猫

