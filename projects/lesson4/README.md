# 第四课作业
```text
1. 设计加密猫模块V3
   需求：
   1. 转移猫
   2. 复杂度必须优于 O(n)
2. 完成 `combine_dna`
3. 重构 `create` 使用新的帮助函数
```

# 内容
```text
fn transfer(to, kitty_id) {
  check_owner(sender,a);//确保a的主人是sender 
  ensure!(owner == sender)；
  checked_add(owned_kitty_count_to)；
  checked_sub(owned_kitty_count_from)；
            
}
```

# 可被外部调用
```text
Transferred(from,to,kitty_id);//查看转移信息
```
