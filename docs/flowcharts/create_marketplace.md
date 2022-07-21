# Create marketplace flowchart

Explanation of the general flow...

```mermaid
graph TD
    A1[Start] --> |user Call extrinsic| A2{Ensure signed}
    A2 -->|Yes| A5[Create new struct Marketplace]
    A2 -- No --> A3[Error: 1011: Unknown Transaction Validity: UnknownTransaction::NoUnsignedValidator] 
    A3 --> A4[End]

    A5 -- Call helper function --> A6[do_create_marketplace]
    A6 -- Generate marketplace id --> A7[marketplace_id] 
    A7 -- Ensure the generated id is unique --> A8[Marketplaces::contains_key]
    A8 -- Insert marketplace id --> A9[/Insert on Marketplaces/]
    A8 -- Insert authorities --> A10[Call insert_in_auth_market_lists]
    --insert owner--
    
    A10 --> A11[/Insert on MarketplacesByAuthority/]
    A10 --> A12[/Insert on MarketplacesByAuthority/]


``` 