# 第五课作业
```text
1. 设计加密猫模块V4
   需求：
   1）交易所
   2）给自己的小猫设定价钱
   3） 购买其他人的小猫
2. 完成 `transfer`
3. 完成 `insert_owned_kitty`
```

# 内容
```text

  set_price(kitty_id, new_price)  {
      check_kitty_exit(kitty_id)//确保猫的存在 
      check_owner(sender,kitty_id);//确保主人是sender 
      kitty_price(new_price);//设价
  }

  buy_kitty( kitty_id, max_price:)  {
      check_kitty_exit(kitty_id)//确保猫的存在 
      check_owner(sender,kitty_id);//确保主人是sender 
      ensure!(owner != sender)
      check_if_kitty_for_sale(kitty_price)；猫的价格不是零
      check_enough_money(kitty_price);猫的价格不大于你的blance
      transfer(to, kitty_id);
  }
        
```