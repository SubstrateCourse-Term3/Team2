# 创建节点 (10min)
* curl https://raw.githubusercontent.com/paritytech/substrate-up/9c966504b4a60c2e6b9187b118926a12d2da9448/substrate-node-new -sSf|bash -s substrate-kitties tingalin

# 启动节点 (47min)
* WASM_BUILD_TYPE=release cargo run -- --dev -d target/substrate --execution=NativeElseWasm

# 配置 explorer
* https://polkadot.js.org/apps/#/settings
![i](img/local_node.png)

# 链信息
![i](img/blocks_info.png)

# 转帐
![i](img/transacion_making.png)
![i](img/queued.png)
![i](img/finalized.png)
![i](img/extrinsics.png)