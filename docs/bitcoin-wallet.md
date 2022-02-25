# Bitcoin Treasury Wallet- powered by a pallet
## Bitcoin xpub as Identity attribute
A user can set their xpub information on their profile, such as: 
```bash
$ polkadot-js-api --ws wss://n1.hashed.systems tx.identity.setIdentity '{
    "display": {
        "Raw": "Paul McCartney"
    },
    "additional": [[{
            "Raw": "xpub"
        },{
            "Raw": "[dca67f77/84/1/0/0]tpubDEQ2wZuuDfizT2aThADBimVaDwWb3aK6WtA3VRMoCDog2Gg3PtDa1gHWhZYEiGba5XA2D2opry9MxZVVjgAaGM8MCnvW6kt6v5AURRyLHPh/*"
          }
        ]
    ]
}' --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```
## Bitcoin Developer Kit (BDK) 
[Bitcoin Dev Kit](https://bitcoindevkit.org) is a Rust-based library for working with Bitcoin wallets, with a special focus on output descriptors. There's a CLI (`bdk-cli`) to run the commands, and there's also a library/crate that I imagine can be used from within a pallet.

Here are some helpful commands.
```bash
$ bdk-cli key generate
```
```json
{
  "fingerprint": "dca67f77",
  "mnemonic": "rose poet odor pole impose stamp boat cruel melt nut eight anchor jar obey tip mention accuse dry member stay pepper final alert live",
  "xprv": "tprv8ZgxMBicQKsPdRxBuQZegC2R3k9R1m4SB2Vy8wAaonownndjLrAdTsTiapvWNXQSN8N9XUvKAWukvm2evPS8yCqmvd1mmL8qAEnbe3PDNpD"
}
```

```bash
bdk-cli key derive --path m/84'/1'/0'/0 '--xprv tprv8ZgxMBicQKsPdRxBuQZegC2R3k9R1m4SB2Vy8wAaonownndjLrAdTsTiapvWNXQSN8N9XUvKAWukvm2evPS8yCqmvd1mmL8qAEnbe3PDNpD
```
```json
{
  "xprv": "[dca67f77/84/1/0/0]tprv8hhzo9sf5J3KZZYfoWYbKMqTeuzetF8BwaZGCuKVmx1HBnRGmVPyqBfeXRWZPCBkSAbZabuDCZZ26J6eWeDk9qAQq8oYK97WpXmkQdpT6S8/*",
  "xpub": "[dca67f77/84/1/0/0]tpubDEQ2wZuuDfizT2aThADBimVaDwWb3aK6WtA3VRMoCDog2Gg3PtDa1gHWhZYEiGba5XA2D2opry9MxZVVjgAaGM8MCnvW6kt6v5AURRyLHPh/*"
}
```

## Multisig wallet
### Receiving
If 5 users all generated xpub keys on their own and attested them in the profile, the pallet would be able to generate an output descriptor for a 3 of 5 multisig wallet, such as below:

```
wsh(multi(3,tpubDEQ2wZuuDfizYa8Vxo92Jz96nDhwwHTczsHTpSt4hnSRaWhQbj8Nrb46QitDpeEABLQSHPSyxdCn8gUDE6uZ2TWPLreLzvhFZLPPyrSizBz/1/0/*,tpubDEQ2wZuuDfizZR2aCmD5gpHJtsXET1zpYmR1JA9nMp4EWDcnnC957ekfaysjF4T8hSNJj98fEcUocnhds3Gwot8G145AZDsYjpwuJto4DFQ/0/0/*,tpubDEQ2wZuuDfizUWke1ZhreeVoybZiYiRept7ifSNSefbmPEM7yeNkbH1Kx4uMBnCtq2bB95oT1YX1ZAFuTfA1LetiTTrYuP6ShXsUUv6Bd8Q/0/0/*,tpubDEQ2wZuuDfizT2aThADBimVaDwWb3aK6WtA3VRMoCDog2Gg3PtDa1gHWhZYEiGba5XA2D2opry9MxZVVjgAaGM8MCnvW6kt6v5AURRyLHPh/0/0/*,tpubDEQ2wZuuDfizdnKYinDkouHHo7CeDdgScMfPYLMR8cnq3PYj85SccVnXa2Yt9HfVXq1riCkDLQG7R5YwcR8HY5z79M5b6zNsX4pZ12ngu1i/0/0/*))
```

Once we have the descriptor for the full wallet, we can generate new receiving addresses.

#### BENEFIT: Verifiable Receiving Addresses 
Contributors/investors of BTC to a multisig wallet can be highly confident that the intended signers have control over the sent BTC (UTXO).

```bash 
$ bdk-cli wallet --descriptor 'wsh(multi(3,tpubDEQ2wZuuDfizYa8Vxo92Jz96nDhwwHTczsHTpSt4hnSRaWhQbj8Nrb46QitDpeEABLQSHPSyxdCn8gUDE6uZ2TWPLreLzvhFZLPPyrSizBz/1/0/*,tpubDEQ2wZuuDfizZR2aCmD5gpHJtsXET1zpYmR1JA9nMp4EWDcnnC957ekfaysjF4T8hSNJj98fEcUocnhds3Gwot8G145AZDsYjpwuJto4DFQ/0/0/*,tpubDEQ2wZuuDfizUWke1ZhreeVoybZiYiRept7ifSNSefbmPEM7yeNkbH1Kx4uMBnCtq2bB95oT1YX1ZAFuTfA1LetiTTrYuP6ShXsUUv6Bd8Q/0/0/*,tpubDEQ2wZuuDfizT2aThADBimVaDwWb3aK6WtA3VRMoCDog2Gg3PtDa1gHWhZYEiGba5XA2D2opry9MxZVVjgAaGM8MCnvW6kt6v5AURRyLHPh/0/0/*,tpubDEQ2wZuuDfizdnKYinDkouHHo7CeDdgScMfPYLMR8cnq3PYj85SccVnXa2Yt9HfVXq1riCkDLQG7R5YwcR8HY5z79M5b6zNsX4pZ12ngu1i/0/0/*))' get_new_address
```
```json
{
  "address": "tb1q433j97374mss5na5eu7f0ja29rx2fsretgs2h4f5p886x5mqg65q74fhzv"
}
```

### Sending
There are existing wallet UIs (Spectre Desktop, Caravan, Sparrow) that support output descriptors and facilitate the user experience, including signing via a variety of hot or cold wallets. 

To focus only on the pallet logic, we can use `bdk-cli` to simulate the signing steps.

```bash
$ bdk-cli wallet sign -h

bdk-cli-wallet-sign 0.3.1-dev
Signs and tries to finalize a PSBT

USAGE:
    bdk-cli wallet --descriptor <DESCRIPTOR> sign [OPTIONS] --psbt <BASE64_PSBT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --psbt <BASE64_PSBT>              Sets the PSBT to sign
        --assume_height <HEIGHT>          Assume the blockchain has reached a specific height. This affects the
                                          transaction finalization, if there are timelocks in the descriptor
        --trust_witness_utxo <WITNESS>    Whether the signer should trust the witness_utxo, if the non_witness_utxo
                                          hasn’t been provided


```
- [ ] TODO: Add example signing step for PSBT 


The intermediate PSBT files (the output from above) are only needed temporarily and can be saved directly on chain or in IPFS. These files then are combined and broadcast.
#### BENEFIT: User doesn't need to transport PSBT files 

```bash
$ bdk-cli wallet combine_psbt -h

bdk-cli-wallet-combine_psbt 0.3.1-dev
Combines multiple PSBTs into one

USAGE:
    bdk-cli wallet --descriptor <DESCRIPTOR> combine_psbt --psbt <BASE64_PSBT>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --psbt <BASE64_PSBT>...    Add one PSBT to combine. This option can be repeated multiple times, one for each
                                   PSBT

```
- [ ] TODO: Add example combine-psbt and broadcast 

## Appendix: How PSBTs work
![image](https://user-images.githubusercontent.com/32852271/155020594-ccca7a2f-68fd-41b3-a54c-a0ea682a0798.png)