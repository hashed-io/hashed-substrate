Create a Collection
```
polkadot-js-api --seed "//Alice" tx.rmrkCore.createCollection "my metadata" 45 AFLOAT
```

Query a Collection
```
polkadot-js-api --seed "//Alice" query.rmrkCore.collections 1
```

Query an NFT
```
polkadot-js-api --seed "//Alice" query.rmrkCore.nfts 0 0
```

Mint an NFT
```
polkadot-js-api --seed "//Alice" tx.rmrkCore.mintNft 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 0 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 10000 "Sams tax credit"
```

Send an NFT
```
polkadot-js-api --seed "//Alice" tx.rmrkCore.send 0 0 '{"AccountId":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}'
```

Set a Property on an NFT
```
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "NFT Type" "Afloat Tax Credit"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "Title" "My VA Land Prez Credit"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "State" "Virginia"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "Tax Credit Type" "Virginia Land Preservation"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "Entity Type" "Individual"
```

Set a Resource on an NFT
```
polkadot-js-api --seed "//Alice" tx.rmrkCore.addBasicResource 0 1 "BasicResource"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "Title" "My VA Land Prez Credit"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "State" "Virginia"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "Tax Credit Type" "Virginia Land Preservation"
polkadot-js-api --seed "//Alice" tx.rmrkCore.setProperty 0 1 "Entity Type" "Individual"
```

