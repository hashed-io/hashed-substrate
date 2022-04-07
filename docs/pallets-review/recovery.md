# Recovery workflow on polkadotjs api (javascript)
Below is a workflow for Recovery. You can run them as Steve and another account you have access to (you have the mnemonic) that will recover steve's account, that account will be referred as `<new_account>`.

Aditionally, your contacts/friends must paricipate on the acount retrieval, their public keys shall be referred as `friend_1,friend_2,...,friend_n`

## Basic setup

```javascript
// Required imports
const { ApiPromise, WsProvider } = require("@polkadot/api");
const { Keyring } = require("@polkadot/keyring");

/* Steve's info
 set SEED="bargain album current caught tragic slab identify squirrel embark black drip imitate"
 set ADDR="5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym"
 */
async function main() {
  // Initialixe the provider to connect to the local node
  const provider = new WsProvider("wss://n4.hashed.systems");

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });

  // Constuct the keyring after the API (crypto has an async init)
  const keyring = new Keyring({ type: "sr25519" });
  // Add Alice to our keyring with a hard-deived path (empty phrase, so uses dev)
  const steve = keyring.addFromUri(
    "bargain album current caught tragic slab identify squirrel embark black drip imitate"
  );
  console.log("Steve's keyring: ", steve.toJson());

  /* Insert the extrinsics and queries here. don't run more than 1 extrinsic per script execution*/

}

main()
  .catch(console.error)
  .finally(() => process.exit());

```

```bash
# You can run the script by typing the following command on your terminal
node <filename>.json
```

## Create a recovery 
It is needed at least 1 additional public key to create the recovery.

```javascript
  /*--- Create recovery---*/
const createRecovery = await api.tx.recovery.createRecovery(
    [
      <friend_1_public_key>,
      <friend_2_public_key>
    ], 2, 0)
    .signAndSend(steve);
console.log( createRecovery.toHex() );
```

## Initiate recovery

```javascript
const new_account = keyring.addFromUri(<new_account_mnemonic>);
const initiateRecovery = await api.tx.recovery
    .initiateRecovery(steve.address).signAndSend(new_account);
console.log(initiateRecovery.toHex());
```

## Vouch recovery
Your friends will have to vouch for that recovery, in this case, it's specified that 2 out of 2 registered friends must sign a vouch transaction:

```javascript
const friend_1 = keyring.addFromUri(
    <friend_1_mnemonic>
  );
const f1_vouch = await api.tx.recovery
    .vouchRecovery(
      "5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym",
      <new_account_public_key>
    )
    .signAndSend(friend_1);
console.log(f1_vouch.toHex());
```

You can check your recovery status with

```javascript
const getActiveRecovery = await api.query.recovery.
    activeRecoveries.entries(steve.address);
console.log(getActiveRecovery.map(
    ([k,v])=>{ return{key: k.toHuman(), val: v.toHuman()}  }) 
);
```

## Claim recovery

```javascript
const claimRecovery = await api.tx.recovery.claimRecovery(steve.address).signAndSend(new_account);
console.log(claimRecovery.toHex());
```

## Close recovery

```javascript
const closeRecovery = await api.tx.recovery
.asRecovered(
    steve.address,
    api.tx.recovery.closeRecovery(new_account.address)
)
.signAndSend(new_account);
console.log(closeRecovery.toHex());
```

## Remove recovery config

```javascript
const removeRecovery = await api.tx.recovery
    .asRecovered(
    steve.address,
    api.tx.recovery.removeRecovery()
).signAndSend(new_account);
```

## Recover all funds

```javascript
const transferAll = await api.tx.recovery
.asRecovered(
  steve.address,
  api.tx.balances.transferAll(new_account.address,false)
)
.signAndSend(new_account);
console.log(transferAll.toHex());
```