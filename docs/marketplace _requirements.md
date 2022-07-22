# Characteristics and/or validations

- [ ] Should we handle all with our gated_marketplace pallet?

Users cannot match their own offers. I think a fee will be applied to every transaction, which would be bad for the users' balance. 

General Afloat workflow extracted from the website/Excel:



## Buy

### Validations:
- Minimum balance to place orders
- Fixed fee

### Design
 
- Tax credits
- Buy price per credit
- Type of credit
* Add each type depending on the state
* Where do we obtain that information?
- Expiration Date
* Can't start in the past
* Would it start from the day it is created?
* What would be the maximum period?
- Total (includes fee)

-> Submit
- Confirmation message

### Metadata: 

- Side
- ID
- Size
- Price
- Type
- Fee
- Credit Kind
- Expires date
- Status: Accepted (matched) / Pending

- We should allow to cancell it

## Sell

### Validations:


### Design 

- Tax credits
- Type of credit
- Expiration Date
- Sell price per credit
* Add each type depending on the state
- Total (less fee)

-> Submit


### Metadata: 
- Side
- ID
- Size
- Price
- Type
- Fee
- Credit Kind
- Expires date
- Status: Accepted (matched) / Pending

- We should allow to cancell it

## Wallet
- [ ] Should we need an external service to handle this?
- Maybe acombination of both, local extrinsics and external services. 

- Must have a linked bank account
- Deposit Money to Wallet
- Withdraw Money from Wallet
* Status: Pending / approved / rejected
- Modify balances

## Clients

## Balances
I think balances pallet will handle this.


## Order Matching system
"An order matching system or simply matching system is an system that matches buy and sell orders for a stock market, commodity market or other financial exchange."

- [ ] Should we implement a pallet/external service/design our own system?

- If one investor wants to buy a quantity of stock and another wants to sell the same quantity at the same price, their orders match, and a transaction is effected. 

**Algorithms:**
- Price/Time algorithm (or First-in-First-out) -> aka FIFO
* The FIFO algorithm, also called the Price/Time algorithm, prioritizes orders with better price, and made first. So when two orders are submitted at the same price, the order submitted first will fulfilled first. But if the latter order has a better price (narrowed the bid-ask spread), this order will be matched first.

- Pro-Rata algorithm
* If the orders are placed in the same price, they both get fulfilled in the same time. So, let’s say I placed an order to buy 100 of stock A at $100. You placed an order to buy 300 of stock A at $100. When there is a sell order of 200 stock A at $100, I would get 100 / (100 + 300) of the 200 sell order, which is 50. You will get 150. As more sell order come in at this price, both you and I will eventually be done buying. 


---


# Marketplace protocol from Max:
I'll summarize the current state of our gated_marketplace pallet based on Max requirements.

--- 

## Roles

- [x] Each marketplace has ann owner
- [x] Each marketplace has an administrator

Roles that backend-team has added:
- [x] Appraiser
- [x] Redemption specialist

- [ ] CPA (?)

**QUESTION**

- [ ] At what point in the workflow should roles be assigned to each user? For example, before or after the user has applied to a marketplace?
The current state allows us to assign roles to any user, we can limit the scope on the front side and display a menu where the administrator can select a user, that has been accepted, to assign it a role.

**QUESTION**
- [ ] Authorities can be added without being participant on the market?

**QUESTION**
- [ ] Should we set an exact number of roles per user per marketplace?
Currently the number of roles per user per marketplace is 1.
- I wanted to change it to 3, but as Alex told me that he'd need to change his frontside queries, I've returned it to 1. Idk how much work it would be to change it to an arbitrary number.

**QUESTION**
This document mentions that some roles may have more than one user.
- [ ] How should we hanlde this issue? There is currently an arbitrary parameter for the number of roles per user per marketplace. I think we can increase it to 10 and through validation restrict the number of users allowed for each role instead of creating different configuration parameters for each role.

- [ ] Should we set a limit of users per marketplace?

## Owner role 
- [x] Owner is required and enforced from signer
- [x] Owner may change the administrator.

**QUESTION**
Current state allows to delete an administrator then choose another admin if needed. 
- [ ] Should we need to implement a "change" like swap accounts to move all data, permissions, and documents to another account?

- [x] Owner is also a required signer to erase a marketplace.

**QUESTION**
Current state allows to admin or owner roles to delete a marketplace. - - - [ ] Should we limit this action only to owner role?

**QUESTION**
I added an extrinsic that allows to delete a particular marketplace and ALL the data related from that marketplace. Sebastian told me that this actions sounds extreme. He thinks maybe we should implement an status for each marketplace "active/inactive" instead of delete it. Front team can develop this feature.  
I think at some point it will be necessary to delete a marketplace. 
- [ ] Should we need to implement a status for the marketplace? 

The following actions for owner role haven't been implemented
- [ ] Specify accounts to use for commission payments
- [ ] Protocol fees

**QUESTION**
- [ ] How will the above actions work? The first one sounds like to call an extrinsic where the owner can select which accounts he wants to add.
maybe we should add a status to check that the selected user is participating in the commission payments.
- [ ] Protocol fees???

## Administrator role

- [ ] New market participants being allowed to originate assets or place orders.

**QUESTION**
- [ ] After a user has been accepted into a marketplace, should there be a new role that is only allowed to originate assets and place orders? 
-> `Market Participant` (?).
- Or we can treat it as a permission.

- [x] New asset graders/appraisers
- [x] New redemption specialists

**QUESTION**
Currently owner and administrator can add/remove "authorities".
- [ ] Should we limit it to just one role (admin or owner)?

## Asset Grader/Appraiser

- The name we use is Appraiser. 
- Currently this role has no participation in the flow.

- [x] We've added it to the list of roles

- [ ] This role has the authority to add a grade, score, or boolean to an asset.
- An asset may have many grades, and graders can edit their own, but graders may not have multiple simultaneous grades of the same asset.
- There may be many of these roles, and they can set the price for their services.

**QUESTION**
- [ ] Does the Appraiser have to be a role belonging to a Marketplace or a kind of general role outside the marketplace (they don't have to apply to each marketplace)?

## Redemption Specialist role

- Currently this role has no participation in the flow.

- [x] We've added it to the list of roles

- [ ] This role is responsible for transforming the on-chain asset into the IRL asset. 
- For example, to be redeemed, a tax credit NFT needs to be assigned to an identity and submitted to (IRS or states?).
- Or an asset may need to be mailed from a warehouse.
- There may be many of these roles, and they can set the price for their services.

**QUESTION**
- [ ] Does the Redemption Specialist have to be a role belonging to a Marketplace or a kind of general role outside the marketplace (they don't have to apply to each marketplace)?


**QUESTION**
- [ ] What is the Redemption specialist flow? Is it like a registrar?


## Market Participant role

- Currently this role has no participation in the flow.
- [ ] This role has the authority to originate and purchase assets. 

- [ ] We can start by adding the role to the list of roles.

## Arbitrator role 

- Currently this role has no participation in the flow.

- [ ] This role has the authority to originate and purchase assets. 
- It depends on administrator approval to grant this permission.

- [ ] We can start by adding the role to the list of roles.

**QUESTION**
- [ ] How does the arbitrator data flow looks like or the general actions that role can take?

## Fractionalizable

- [ ] Each asset is infinitesimally fractionalizable.

**QUESTION**
- [ ] How many significant figures should we allow?


- [ ] An asset starts as a single whole asset; n=1
- [ ] The asset has an amount value. 
- [ ] An asset owner can spawn a new asset from the original, parent asset and specify the amount value of the child.
- [ ] The pallet enforces the hierarchical math of the amount value on each spawn.
- [ ] Children can inherit attributes from their parents, override them (if permitted by attribute-creator), or add new ones.


## Privacy Preserving Storage

- [x] Users can save data on the environment that is encrypted, 
- IPFS & encryption system

- [ ] and selectively decide which other users on the Network can view it.
- Hashed Private Client API -> 
It seems that currently state it works by pairs of users: 
Custodian-Applicant
Custodian-Administrator
 




