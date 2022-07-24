# 区块链

<!--ts-->
* [区块链](#区块链)
   * [Layers](#layers)
      * [OSI网络七层模型](#osi网络七层模型)
      * [数据层](#数据层)
      * [网络层](#网络层)
      * [共识层](#共识层)
         * [工作量证明共识机制（ PoW ）](#工作量证明共识机制-pow-)
      * [激励层](#激励层)
      * [合约层](#合约层)
      * [应用层](#应用层)
   * [Blockchains](#blockchains)
   * [Blockchain Frameworks](#blockchain-frameworks)
   * [Cross-Chain](#cross-chain)
   * [Virtual Machines](#virtual-machines)
   * [General-Purpose Consensus](#general-purpose-consensus)
   * [P2P Network Libraries](#p2p-network-libraries)
   * [Cryptography](#cryptography)
   * [Layer2](#layer2)
   * [Dapps](#dapps)
   * [Other](#other)
   * [参考资源](#参考资源)
      * [online-book](#online-book)
      * [fragment](#fragment)
      * [local](#local)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Jul 24 09:07:46 UTC 2022 -->

<!--te-->

## Layers

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/df67d027097043d3d1c8b180558bb3cc.png)

### OSI网络七层模型

![图片](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/640.png)

> 下图详细说明了各层作用

![图片](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/640-20220621104210621.png)

实际应用过程中，五层协议结构里面是没有表示层和会话层的。应该说它们和应用层合并了。

### 数据层

区块链是通过区块（block）存储数据，每个数据节点之间都包含所有数据，即分布式账本。每个区块都包括了区块的大小、区块头、区块所包含的交易数量及部分或所有的近期新交易。

数据层主要是解决这些数据以什么样的形式组合在一起，形成一个有意义的区块。

区块链的数据结构中包括两种哈希指针，它们均是不可篡改特性的数据结构基础。

1. 一个是形成“区块+链”（block+chain）的链状数据结构

2. 另一个是哈希指针形成的梅克尔树（如下图所示）。

```admonish tip title='用处'
链状数据结构使得对某一区块内的数据的修改很容易被发现；
梅克尔树的结构起类似作用，使得对其中的任何交易数据的修改很容易被发现。
```

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/9b432d8f0369921e21ac33f14c2bdeda.png)

### 网络层

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/b93a9b3345d59f5e13c0ec662a46f05a.png)

区块链使用的是去中心化的网络架构，没有中心化服务器，依靠用户点对点交换信息，主要包括 P2P 组网机制、数据传播和验证机制。

```admonish tip title='P2P特性'
正是由于节点的 P2P 特性，数据传输是分散在各个节点之间进行的，部分节点或网络遭到破坏对其他部分影响很小。
```

节点指的是区块链客户端软件（比如比特币客户端、以太坊客户端），一般分为全节点和轻节点，全节点包含了所有区块链的区块数据，轻节点仅包括与自己相关的数据。

### 共识层

共识层的功能是让高度分散的节点在 P2P 网络中，针对区块数据的有效性达成共识，决定了谁可以将新的区块添加到主链中（挖矿机制）。

#### 工作量证明共识机制（ PoW ）

```admonish info title='PoW'
矿工需要将网络中未确认的交易按梅克尔树组装成候选区块，在候选区块的头部有一个 32位的随机数区域，矿工需要反复调整随机数并计算，目标是让整个区块的哈希值小于一个“目标值”，谁先完成这个目标谁就有权力将交易记录到区块链分布式账本中并获得一定的奖励。
```

挖矿的过程比拼的就是各个矿工节点的算力，可以变相认为谁的算力高谁的工作量就高，就有权力记账和获得奖励。

- 比特币使用的是 PoW 机制；
- 以太坊开始使用的是 PoW 机制，后来改成了 PoS 机制，原因是该机制交易速度更快、资源消耗更低。

这种挖矿计算是非对称的，挖矿可能需要经过许多次哈希计算，而要验证的确找到有效的随机数，只需要一次计算就可以，因此其他节点能够很快验证交易是否已经被记入账本。

### 激励层

激励层的功能主要是提供一些激励措施，鼓励节点参与记账，保证整个网络的安全运行。通过共识机制胜出取得记账权的节点能获得一定的奖励。

目前比特币的激励措施是新区块产生时系统会奖励矿工一定的比特币（系统产生的新比特币，也会记录在分布式账本，来源地址是 0，因此整个过程叫挖矿），奖励最初是 50 个比特币，每四年减半一次，分别为 25 个、12.5
个，以此类推。当比特币数量达到 2100 万枚的上限后（2140 年），激励就全靠交易的手续费了。以太坊交易是靠 gas 手续费来激励矿工。

### 合约层

合约层封装了各类脚本、算法和智能合约，使得区块链具有可编程能力。例如，**比特币的脚本**中就规定了比特币的交易方式和过程中的种种细节，不过这种脚本使用不够便捷且不是图灵完备的。

以太坊提出了智能合约的解决方案，提供了一种图灵完备的高级编程语言来编写智能合约，并使智能合约能够运行在分布式的以太坊虚拟机 EVM 上。

```admonish tip title='智能合约'
智能合约就是存储在区块链上的一段代码，它们可以被区块链上的交易所触发，触发后，这段代码可以从区块链上读取数据或者向区块链上写入数据。
```

区块链系统（比特币、以太坊）可以认为是一个分布式状态机，通过交易触发合约（脚本、智能合约）运行来改变状态机的状态。

### 应用层

应用层封装了区块链的各种应用场景，具体应用可参见**90+ #Ethereum Apps You Can Use Right Now**。

以下为一笔比特币转账交易的过程

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/e6b83641ed6467b392d04a3e6fe69492.png)

## Blockchains

- [Aleo](https://developer.aleo.org/aleo/getting_started/overview). Leo is a rust flavoured zk language.
- [Aleph Zero](https://alephzero.org/). DAG, PoS, snark smart contracts (substrate based).
- [Anoma.network](https://anoma.network/). PoS blockchain with privacy.
- [Bitcoin Cash](https://github.com/be-cash/bitcoin-cash). A library for creating and parsing Bitcoin Cash trasactions.
- [CITA](https://github.com/cryptape/cita). A high performance blockchain kernel for enterprise users.
- [CodeChain](https://github.com/CodeChain-io/codechain). Programmable multi-asset chain.
- [Concordium](https://concordium.com/). Privacy centric (zk) PoS chain, yet with built in identities and rust smart
  contracts.
- [Conflux](https://github.com/Conflux-Chain/conflux-rust). The Rust implementation of Conflux protocol.
- [Darwinia](https://github.com/darwinia-network/darwinia). Relay chain of Darwinia Network, can connect to Polkadot as
  parachain in Polkadot Model.
- [Dusk.network](https://dusk.network/). Privacy PoS using zk (plonk).
- [Enigma](https://github.com/enigmampc/enigma-core) secures the decentralized web.
- [Elrond](https://elrond.com/). Elrond (EGOLD( - scalable and usable blockchain, written is Rust and has smart
  contracts in Rust.
- [Exonum](https://github.com/exonum/exonum). An extensible open-source framework for creating private/permissioned
  blockchain applications.
- [Forest](https://github.com/ChainSafe/forest). An implementation of Filecoin written in Rust.
- [Fuel](https://github.com/FuelLabs/fuel-core). Rust full node implementation of the Fuel v2 protocol.
- [Gear](https://github.com/gear-tech/gear). Computational component of Polkadot network.
- [Grin](https://github.com/mimblewimble/grin). Minimal implementation of the MimbleWimble protocol.
- [Holochain](https://github.com/holochain/holochain). The core Holochain framework written in rust, a container, and
  hdk-rust library for writing Zomes.
- [Huobi Chain](https://github.com/HuobiGroup/huobi-chain). The next generation high performance public chain for
  financial infrastructure.
- [Interledger](https://github.com/interledger-rs/interledger-rs). An easy-to-use, high-performance Interledger
  implementation written in Rust.
- [Internet of People](https://github.com/Internet-of-People/iop-rs). Decentralized software stack that provides the
  building blocks and tools to support a decentralized society.
- [Libra](https://github.com/libra/libra). Global currency and financial infrastructure that empowers billions of
  people.
- [Lighthouse](https://github.com/sigp/lighthouse). Fast and secure Ethereum 2.0 client.
- [NEAR](https://github.com/nearprotocol/nearcore). NEAR Protocol - scalable and usable blockchain.
- [Nervos CKB](https://github.com/nervosnetwork/ckb). Nervos CKB is a public permissionless blockchain, the common
  knowledge layer of Nervos network.
- [NYM](https://github.com/nymtech/nym). Selective privacy via a mixnet preventing metadata analysis.
- [Nomic](https://github.com/nomic-io/nomic). Nomic is a high-performance Bitcoin sidechain which is part of the Cosmos
  network.
- [Mina Protocol](https://github.com/ChainSafe/mina-rs). A rust implementation of the mina succinct blockchain.
- [Mir Protocol](https://mirprotocol.org/). A succinct blockchain powered by zero-knowledge proofs. (plonk based)
- [OpenEthereum](https://github.com/openethereum/openethereum). The Ethereum Rust client
- [Parity Bitcoin](https://github.com/paritytech/parity-bitcoin). The Parity Bitcoin client.
- [Parity Ethereum](https://github.com/paritytech/parity-ethereum). The fast, light, and robust EVM and WASM client.
- [Parity Zcash](https://github.com/paritytech/parity-zcash). Rust implementation of Zcash protocol.
- [Polkadot](https://github.com/paritytech/polkadot). Polkadot Node Implementation.
- [Polymesh](https://github.com/PolymathNetwork/Polymesh). The Polymesh blockchain (built on Substrate) is an identity
  orientated chain for the issuance, lifecycle management and settlement of regulated securities.
- [QAN](https://github.com/QANplatform/its_alive). Post-quantum blockchain.
- [Radix](https://github.com/radixdlt/radixdlt-scrypto). Sharded smart contract DeFi platform.
- [Setheum](https://github.com/Setheum-Labs/Setheum). SETHEUM : “Secure Evergreen Truthful Heterogeneous Economically
  Unbiased Market” is an Ethical DeFi-friendly Blockchain (built on Substrate) working on achieving mass adoption,
  security, scalability, affordability, inclusivity and ethical DeFi Governance.
- [Shasper](https://github.com/paritytech/shasper). Parity Shasper beacon chain implementation using the Substrate
  framework.
- [Solana](https://github.com/solana-labs/solana). Blockchain Rebuilt for Scale.
- [Stacks 2.0](https://github.com/blockstack/stacks-blockchain). Proof of Transfer blockchain from Blockstack.
- [Tari](https://github.com/tari-project). The Tari Digital Assets Protocol.
- [Tendermint](https://github.com/informalsystems/tendermint-rs). Tendermint is a high-performance blockchain consensus
  engine for Byzantine fault tolerant applications.
- [Witnet](https://github.com/witnet/witnet-rust). Open source implementation of Witnet decentralized oracle network
  protocol in Rust.
- [xx-network](https://github.com/xx-labs/xxchain). Post-quantum blockchain, mixnet privacy preventing metadata
  analysis. (Substrate rust+go)
- [Zebra](https://github.com/ZcashFoundation/zebra). An ongoing Rust implementation of a Zcash node.
- [Zero-chain](https://github.com/LayerXcom/zero-chain). A privacy-preserving blockchain on Substrate.

## Blockchain Frameworks

- [Substrate](https://github.com/paritytech/substrate). The platform for blockchain innovators.
- [slingshot](https://github.com/stellar/slingshot). A new blockchain architecture under active development, with a
  strong focus on scalability, privacy and safety.
- [Tendermint ABCI](https://github.com/tendermint/rust-abci). Tendermint ABCI server, written in the Rust programming
  language.
- [Orga](https://github.com/nomic-io/orga). A high-performance state machine engine designed for Tendermint-based
  blockchain applications.

## Cross-Chain

- [Comit](https://comit.network/) is an open protocol facilitating trustless cross-blockchain applications.
- [IBC](https://github.com/informalsystems/ibc-rs). Rust implementation of Cosmos' Interblockchain Communication
  Protocol
  (IBC).

## Virtual Machines

- [CKB-VM](https://github.com/nervosnetwork/ckb-vm). RISC-V virtual machine.
- [CosmWasm](https://www.cosmwasm.com). Multi-chain smart contract platform built for the Cosmos ecosystem.
- [EVM Parity](https://github.com/paritytech/parity-ethereum/tree/master/evmbin). Parity implementation of EVM.
- [FuelVM](https://github.com/FuelLabs/fuel-vm)
  FuelVM interpreter in Rust.
- [Lunatic](https://github.com/lunatic-solutions/lunatic). Erlang-inspired runtime for WebAssembly.
- [Polygon Miden](https://github.com/maticnetwork/miden). SNARK based VM.
- [SVM](https://github.com/spacemeshos/svm)
  Spacemesh Virtual Machine.
- [Wasmi](https://github.com/paritytech/wasmi). WebAssembly interpreter.
- [Wasmer](https://wasmer.io/). A convenient Rust wrapper over WebAssembly backends.
- [Wasmtime](https://github.com/CraneStation/wasmtime). Standalone JIT-style runtime for WebAssembly, using Cranelift.
- [Zinc](https://github.com/matter-labs/zinc). Zinc zk smart contract language.

## General-Purpose Consensus

- [Raft](https://github.com/pingcap/raft-rs). Raft distributed consensus algorithm implemented in Rust.
- [Honey Badger](https://github.com/poanetwork/hbbft). An implementation of the paper "Honey Badger of BFT Protocols" in
  Rust.
- [Narwhal](https://github.com/MystenLabs/narwhal). The consensus layer used by Sui.

## P2P Network Libraries

- [chamomile](https://github.com/placefortea/chamomile). P2P library. Support build robust stable connection on
  p2p/distributed network.
- [crust](https://github.com/maidsafe/crust). Reliable P2P network connections in Rust with NAT traversal. One of the
  most needed libraries for any server-less / decentralised projects.
- [rust-libp2p](https://github.com/libp2p/rust-libp2p). The Rust Implementation of the libp2p networking stack.
- [Tentacle](https://github.com/driftluo/tentacle). A multiplexed p2p network framework that supports custom protocols
- [P2P NAT-Traversal](https://github.com/ustulation/p2p). NAT Traversal techniques for p2p communication.
- [qp2p](https://github.com/maidsafe/qp2p). Peer-to-peer communications library for Rust based on QUIC protocol.
- [sn_routing](https://github.com/maidsafe/sn_routing). Routing - specialised storage DHT.

## Cryptography

- [Awesome Cryptography Rust](https://github.com/rust-cc/awesome-cryptography-rust).
- [Dalek Cryptography](https://github.com/dalek-cryptography).
- [Za!](https://github.com/adria0/za). An experimental rust zksnarks compiler with embeeded bellman-bn128 prover.
- [OpenZKP](https://github.com/0xProject/OpenZKP). Pure Rust implementations of Zero-Knowledge Proof systems.
- [Microsoft Nova](https://github.com/microsoft/Nova). Rust recursive snark without trusted setup.
- [Arkworks](https://github.com/arkworks-rs). An ecosystem for developing and programming with zkSNARKs

## Layer2

- [Arbitrum's arb-os](https://github.com/OffchainLabs/arb-os)
  ArbOS is the "operating system" that runs an eth Layer 2 on an Arbitrum chain,
- [Noir language](https://github.com/noir-lang/noir). Noir is a Domain Specific Language for SNARK proving systems. (
  Aztec eth L2)
- [Penumbra](https://penumbra.zone/). PoS network providing privacy to the Cosmos ecosystem.
- [Rust-Lightning](https://github.com/rust-bitcoin/rust-lightning)
  is a Bitcoin Lightning library written in Rust. The main crate, lightning, does not handle networking, persistence, or
  any other I/O. Thus, it is runtime-agnostic, but users must implement basic networking logic, chain interactions, and
  disk storage.
- [zkSync](https://github.com/matter-labs/zksync). Matter Labs' scaling eth L2 engine secured by zero-knowledge proofs.

## Dapps

- [Serum-dex](https://github.com/project-serum/serum-dex). A decentralized exchange built on Solana.
- [SewUp](https://github.com/second-state/SewUp). A library to help you build your Ethereum webassembly contract with
  Rust and just like develop in a common backend.

## Other

- [abscissa](https://github.com/iqlusioninc/abscissa). Micro-framework for CLI tools with strong focus on security.
- [tesseracts](https://github.com/adria0/tesseracts). A small block explorer for geth PoAs written in rust.
- [merk](https://github.com/nomic-io/merk). High performance Merkle key/value store written in Rust, based on RocksDB.

## 参考资源

### online-book

### fragment

- [Web3.0开发入门-技术圈](https://web.archive.org/web/20220620094224/https://jishuin.proginn.com/p/763bfbd79e0a)
- [rust-in-blockchain/awesome-blockchain-rust: Collect libraries and packages about blockchain/cryptography in Rust](https://github.com/rust-in-blockchain/awesome-blockchain-rust)
- [有了HTTP，为什么还要RPC？](https://mp.weixin.qq.com/s/jeoJeL7P7Q4J2vrIKVB5ZA)
- [使用rust建立一个简单的区块链](/layer5_ecosystem/7_business/blockchain/build-a-block-in-rust.html)

### local
