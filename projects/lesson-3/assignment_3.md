
1. 防止溢出方法：
   使用rust的诸如checked_add()之类的安全方法

2. 繁殖方法：
//伪代码
//关键就是要随机生成dna
breed_kity(father_dna, mother_dna)
{
    //把father/mother_dna看成一个bitmap位/字节图(每位/字节代表一个属性)
    //for(i=0; i<sizeof(father/mother_dna); i++) {
	//aa = get_random(); 随机值，如果随机值末尾为1，选取father dna对应的位
	if (aa & 1)
		//chaid->dna[i] = father_dna[i];
	else
		//chaid->dna[i] = mother_dna[i];
    }
}

3. 随机数生成：
   原则就是各种源混合，多选择几种来源.
   如可以把链上每个block的出块时间当做一个随机来源

