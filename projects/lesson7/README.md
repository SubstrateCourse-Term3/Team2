# 第七课
    1. Weights 系统
    2. Metadata 元数据详细介绍
    3. SCALE 编码
    4. Offchain worker
    5. pallet-im-online 代码分析

# 第七课作业
    手动实现 Kitty 和 LinkedItem 的 Encode 和 Decode
    fn encode_to<T: Output>(&self, dest: &mut T)
        runtime/src/kitties.rs:23
    fn decode<I: Input>(input: &mut I) -> Option<Self>
        runtime/src/linked_item.rs:11

# 额外作业
    讨论 SCALE 编码和其他编码的相比的优缺点
    其他编码例子：protobuf, JSON, cbor
    如果由你来选择，你会选择什么编码，为什么？

## 讨论
```text
SCALE主要是根据字段的相对位置来确定字段顺序, 每个字段大小根据编码可以知道. 
这样这种编码占用空间最少. 缺点是升级麻烦. 但是适合寸土寸金的区块链存储. 
用于网络传输也很好, 毕竟这是最小的序列化方式了. 

JSON则需要保存字段名字, 并且是文本编码. 所以占用空间大. 
好处是可以按照随意的顺序保存字段. 

protobuf为了支持可以升级, 也需要保存字段相关信息. 
```

