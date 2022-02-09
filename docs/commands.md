Below is a workflow for the create, read, update, delete of Uniques. You can run them as Steve.

```
# steve's info
set SEED="bargain album current caught tragic slab identify squirrel embark black drip imitate"
set ADDR="5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym"
```
### Install `polkadot-js-api`
```bash
yarn add @polkadot/api
```
### Check Steve's Identity
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```
### Create a new class of `Uniques`/(NFTs)
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.create 1 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Set a Class Attribute (WIP)
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 <empty> "project" "cannabis" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Mint a New Unique, class=1, id=0, Steve as owner
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.mint 1 0 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Set an Instance Attribute, class=1, id=0, key=label
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 0 "label" "100 seeds of Runtz strain" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Mint a second Unique, a germinated seed as it is cubed
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.mint 1 1 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Set a Label of this one
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 1 "label" "Plumply germinated Runtz sprout" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Query the Label of the new sprout
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.uniques.attribute 1 1 label
```
### Set `parent` Attribute
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 1 "parent" "0" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Check Steve's Balance?
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.uniques.account 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym 1 
```

```
# steve's info
set SEED="bargain album current caught tragic slab identify squirrel embark black drip imitate"
set ADDR="5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym"
```