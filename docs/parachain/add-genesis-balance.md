# Add your key to Genesis Spec for MD5 

1. Install `subkey`
https://docs.substrate.io/reference/command-line-tools/subkey/


2. Generate a key 
```bash
subkey generate 
```

Output looks like this: 
```
Secret phrase:       pear afraid genre damage fury visa gentle divert vocal risk local boil
  Network ID:        substrate
  Secret seed:       0xddb2eb2b38cf69a0db0397e9aaf47bb48d71437d037a225be92acf178db3810c
  Public key (hex):  0x70140a32dbd165c862b5d1a51b8cebb4ffd07a92ab72fc0beef7c220b8050a5a
  Account ID:        0x70140a32dbd165c862b5d1a51b8cebb4ffd07a92ab72fc0beef7c220b8050a5a
  Public key (SS58): 5EbfB1K1xes3uywAZ5MwXZc1vUZMYbGZuMiY5BojSWs2r7FD
  SS58 Address:      5EbfB1K1xes3uywAZ5MwXZc1vUZMYbGZuMiY5BojSWs2r7FD
```

3. Edit `node/src/chain_spec/md5.rs`

In the genesis configuration, there is a vector of addresses: 

```rust 
vec![
    // 5HgAxuAcEybo448w5BZdoceCuHMAbEW9AetBKsj9s5GEBZT3
    hex!["f83a0218e100ce3ede12c5d403116ef034124c62b181fff6935403cea9396d2f"].into(),
    // 5DkJvQp2gqHraWZU1BNCDxEKTQHezn2Qy7z5hLPksUdjtEG9             
    hex!["4a70d789b0f0897e0880e8d3d532187ac77cbda04228cfadf8bededdd0b1005e"].into(),
    get_account_id_from_seed::<sr25519::Public>("Alice"),
    get_account_id_from_seed::<sr25519::Public>("Bob"),
```

Add a new `hex!` line with the hex public key from `subkey`.  Be sure to remove the `0x` prefix. 

4. Recompile, test, push
