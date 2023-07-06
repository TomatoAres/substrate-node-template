## 第六节课作业

### 作业
* 为 proof of existence (poe) 模块的可调用函数 create_claim, revoke_claim, transfer_claim 添加 benchmark 用例，并且将 benchmark 运行的结果应用在可调用函数上；
* 选择 node-template 或者其它节点程序，生成 Chain Spec 文件（两种格式都需要）；
* （附加题）根据 Chain Spec，部署公开测试网络

### 答案：
1. [pallets/poe/src/benchmarking.rs](./pallets/poe/src/benchmarking.rs)
2. [nutsoft-staging.json](./nutsoft-staging.json) [nutsoft-staging-raw.json](./nutsoft-staging-raw.json)，生成chain spec的脚本在 [scripts](./scripts) 目录里
3. 部署节点的脚本在 [scripts](./scripts) 目录里
