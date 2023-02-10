# Fruniques pallet

## Spawn mechanism

Taken from #1 

`Fruniques` is a stateful pallet. It needs to store additional data to maintain various relationships and state. We need to design/build the data structure for this additional state, as described below.

There are a few NFT protocols in the Polkadot ecosystem: https://wiki.polkadot.network/docs/learn-nft

Of these, we should build to the [`Uniques` ](https://wiki.polkadot.network/docs/learn-nft#uniques) patterns. It is the implementation from Parity and I believe the most recent. It is the only one compatible with Statemint/Statemine. We can build to multiple protocols if it makes sense, but let's start with `Uniques`. 

In addition to a regular `Unique`, a [`Frunique`](https://hashed.systems/hashed-chain) needs to store a reference to the parent, a different `Unique`. There also needs to be a heuristic for specifying if metadata is inherited from the parent or not. It seems like Metadata is a set of Key:Value pairs that can be assigned at the `class` level (a group or collection of NFTs) and at the `instance` level (a single NFT). 

Here's the function `set_attribute`: 
https://github.com/paritytech/substrate/blob/master/frame/uniques/src/lib.rs#L959

Let's map the cannabis lifecycle. 
> NOTE: the cannabis use case may be able to be implemented with a lighter weight protocol, but it seems like it might be handy to use the same structure
1. Seeds come from a vendor as a package with a count, e.g. 100 seeds in a bag. This bag is an `InstanceId` even though it actually contains 100 seeds. 
2. Seeds that germinate get cubed; others are scrapped.
3. When a seed is cubed, it receives its own `InstanceID` (I've been calling this a `spawn` function) for the first time. The count of seeds that did germinate should be tracked, but not individually, and they are scrapped.
4. Successful cubed seeds become mother plants; perhaps through some iteration or trial/error to discover most productive mother(s).
5. Mother plants produce clones (and may produce flower directly).
7. The parent-->child relationship is well represented as a [Directed Acyclic Graph](https://hazelcast.com/glossary/directed-acyclic-graph), which is what we are building on chain. 
8. Clones may be sold directly to clone buyers.
7. Clones produce flower, measured in weight. When flower is harvested, the weight values of the material are recorded as continuous value. So the `InstanceId` would map this specific `bag of weed`, and there would also be a data element for weight. 

The sum of this continuous value for all peers should always equal the continuous value of the parent. This is a critical feature that maintains the economic hierarchy of the NFTs. Tax credits can be subdivided based on this continuous value, but just like the weed, none can be lost or compromised along the way. This feature - the `NFT Rollup` enables many use cases. 

9. Flower gets tested, and results are implied across that entire harvest/mother?  The test results include a set of files and also a set of values. We need a structure to assign this data/metadata across the appropriate `InstanceIds`.
10. Flower is sold to dispensaries. 

- [ ] Research and prototype a pallet data storage mapping to hold the appropriate data to maintain the hierarchy and enforce the aggregation rules.

![hashed-chain-arch](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/hashed-io/hashed-substrate/main/docs/traceability-tree.iuml)
