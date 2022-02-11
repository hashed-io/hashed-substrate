Below is a workflow for setting and approving identity attributes

```
# Steve's (pretend user) info
set SEED="bargain album current caught tragic slab identify squirrel embark black drip imitate"
set ADDR="5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym"
```

### Check if Address has an Identity
```
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```
If you see results, run the `clearIdentity` command below

### Clear Identity
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.clearIdentity --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```

### Set Identity
The `additional` field of `ApprovedForMarketplace` == 1 is the respresentation that this account/identity desires to apply/become involved in the marketplace, exchange, etc.

```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.setIdentity '{
    "display": {
        "Raw": "Steve Harvey"
    },
    "web": {
        "Raw": "https://steveharvey.com"
    },
    "riot": {
        "Raw": "@surveysays:matrix.org"
    },
    "email": {
        "Raw": "notrealsteve@email.com"
    }, 
    "additional": [[{
            "Raw": "ApprovedForMarketplace"
        },{
            "Raw": "1"
          }
        ]
    ]
}' --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```

### About Registrars
- The identity pallet has **registrars**, which has the ability to provide Identity **judgements** 
- A **gatekeeper** (KYC or otherwise) account (e.g. `5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR`) would be set by Governance to be a Registrar on this particular chain. 
- A registrar is equivalent to the role of an KYC Agency, in terms of on-chain duties.

#### Check the list of registrars
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.registrars 
```
```json
{
  "registrars": [
    {
      "account": "5ERkMY7QzuBLYegNgKJ3YT8GDuCA3jCgWoRSmbNLaB23rEEQ",
      "fee": "0",
      "fields": []
    },
    {
      "account": "5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR",
      "fee": "0",
      "fields": []
    }
  ]
}
```

### Query the identity again 
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```
The applicant has added this information to their profile, notice that there are no judgements 
```json
{
  "identityOf": {
    "judgements": [],
    "deposit": "41,666,666,250",
    "info": {
      "additional": [
        [
          {
            "Raw": "ApprovedForMarketplace"
          },
          {
            "Raw": "1"
          }
        ]
      ],
      "display": {
        "Raw": "Steve Harvey"
      },
      "legal": "None",
      "web": {
        "Raw": "https://steveharvey.com"
      },
      "riot": {
        "Raw": "@surveysays:matrix.org"
      },
      "email": {
        "Raw": "steve@email.com"
      },
      "pgpFingerprint": null,
      "image": "None",
      "twitter": "None"
    }
  }
}
```

### Applicant calls `requestJudgement` 
Applicant specifies that they are seeking a judgement from a specific Registrar, etc. the KYC Agency
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.requestJudgement 1 500 --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```

### Query to see Judgement Awaiting Review
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```
```json
  "identityOf": {
    "judgements": [
      [
        "1",
        {
          "FeePaid": "0"
        }
      ]
    ],
```

### Minister of Interior calls `provideJudgement` 
Parameters are `RegistrarIndex`, the `AccountId` to judge, and one from a list of possible judgement ratings.
- Fee Paid
- Reasonable
- KnownGood
- OutOfDate
- LowQuality

`KnownGood` indicates fully approved (in this example, as an `ApprovedForMarketplace`), although we can likely override the above enum.

```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.provideJudgement 1 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym KnownGood --seed "exercise museum credit crystal various nature defy human cable any split help"
```

### Check the Identity
The indication that an account is an `ApprovedForMarketplace` is that they have a `judgement` with a value of `[1,KnownGood]` and an additional field of `ApprovedForMarketplace` set to a value of `1`.

```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```

```json
{
  "identityOf": {
    "judgements": [
      [
        "1",
        "KnownGood"
      ]
    ],
    "deposit": "41,666,666,250",
    "info": {
      "additional": [
        [
          {
            "Raw": "ApprovedForMarketplace"
          },
          {
            "Raw": "1"
          }
        ]
      ],
      "display": {
        "Raw": "Steve Harvey"
      },
      "legal": "None",
      "web": {
        "Raw": "https://steveharvey.com"
      },
      "riot": {
        "Raw": "@surveysays:matrix.org"
      },
      "email": {
        "Raw": "notrealsteve@email.com"
      },
      "pgpFingerprint": null,
      "image": "None",
      "twitter": "None"
    }
  }
}
```

If the `AccountId`, now `ApprovedForMarketplace` changes any of their Identity information, the Judgement is automatically removed. This protects against users changing any information without being re-verified. This prevents identity theft and also could deter identity exchange/selling.

### Call `setIdentity` again
Let's say that Steve's key is compromised and an Evil Steve wants to capture his communications as an ApprovedForMarketplace user. Once he changes the Identity data, he must request a new judgement and the process repeats.
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.setIdentity '{
    "display": {
        "Raw": "Evil Steve Harvey"
    },
    "web": {
        "Raw": "https://sleveharvey.com"
    },
    "riot": {
        "Raw": "@surveysay5:matrix.org"
    },
    "email": {
        "Raw": "evilsteve@email.com"
    }, 
    "additional": [[{
            "Raw": "ApprovedForMarketplace"
        },{
            "Raw": "1"
          }
        ]
    ]
}' --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```

### Query Identity again
The new data is saved but the Judgement has been erased. 
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
{
  "identityOf": {
    "judgements": [],
    "deposit": "41,666,666,250",
    "info": {
      "additional": [
        [
          {
            "Raw": "ApprovedForMarketplace"
          },
          {
            "Raw": "1"
          }
        ]
      ],
      "display": {
        "Raw": "Evil Steve Harvey"
      },
      "legal": "None",
      "web": {
        "Raw": "https://sleveharvey.com"
      },
      "riot": {
        "Raw": "@surveysay5:matrix.org"
      },
      "email": {
        "Raw": "evilsteve@email.com"
      },
      "pgpFingerprint": null,
      "image": "None",
      "twitter": "None"
    }
  }
}

The marketplace eligibility check looks for both the `ApprovedForMarketplace` field and a `KnownGood` certification from our trusted `registrar(s)`