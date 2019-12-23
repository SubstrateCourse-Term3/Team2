
一 单个结构：
单个猫的基本结构
struct Kitty {
 id: hash,
 dna: hash,
 price: u64,
 generation: u64,
}

二 复合结构：
猫的id=>所有者的隐射关系存储结构

map id:hash => Kitty;

所有者id=>猫的列表隐射关系存储结构

map id::Hash => Kitties[];

所有猫的存储结构
Kitty_Array: map index:u64 => Kitty;

三方法：
遍历所有猫的方法
fn list_all_kitties() {
	//遍历Kitty_Array这个数组
}

遍历某个用户所有猫的方法
fn list_all_kitties(id) -> kitties[] {
	//函数参数是用户id
	//return map[id]; 就行
}

dna合成的方法
fn generate_new_kitty_dna(dna: father, dna: mother) -> {
	//new_dna = random(father, mother);
	//new_kitty.dna = new_dna;
}


