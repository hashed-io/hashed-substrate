This document is just a notebook of useful commands



source .env
set SEED="bargain album current caught tragic slab identify squirrel embark black drip imitate"
set ADDR="5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym"

polkadot-js-api --ws wss://n1.hashed.systems query.uniques.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym

### Create a new class
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.create 1 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
### Set a Class Attribute (WIP)
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 <empty> "project" "cannabis" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"

### Mint a New Unique, class=1, id=0, Steve as owner
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.mint 1 0 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"

### Set an Instance Attribute, class=1, id=0, key=label
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 0 "label" "100 seeds of Runtz strain" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"

### Mint a second Unique, a germinated seed as it is cubed
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.mint 1 1 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"

### Set a Label of this one
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 1 "label" "Plumply germinated Runtz sprout" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"

### Query the Label of the new sprout
polkadot-js-api --ws wss://n1.hashed.systems query.uniques.attribute 1 1 label

## Set `parent` Attribute
polkadot-js-api --ws wss://n1.hashed.systems tx.uniques.setAttribute 1 1 "parent" "0" --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"

### Check Steve's Balance?
polkadot-js-api --ws wss://n1.hashed.systems query.uniques.account 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym 1 