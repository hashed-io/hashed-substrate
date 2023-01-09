# Fund Administration
Fund administration pallet allows the creation of multiple projects and manages their funds. 
- [Fund Administration](#fund-administration)
  - [Overview](#overview)
    - [Terminology](#terminology)
  - [Interface](#interface)
    - [Dispachable functions](#dispachable-functions)
    - [Getters](#getters)
    - [Constants](#constants)
  - [Usage](#usage)
    - [**Polkadot-js CLI**](#polkadot-js-cli)
      - [Submit initial role setup (requires sudo)](#submit-initial-role-setup-requires-sudo)
      - [Create an admmistrator with sudo](#create-an-admmistrator-with-sudo)
      - [Get a user info](#get-a-user-info)
      - [Remove an admmistrator with sudo](#remove-an-admmistrator-with-sudo)
      - [Register a new administrator account](#register-a-new-administrator-account)
      - [Register a new builder account](#register-a-new-builder-account)
      - [Register a new investor account](#register-a-new-investor-account)
      - [Register a new issuer account](#register-a-new-issuer-account)
      - [Register a new regional center account](#register-a-new-regional-center-account)
      - [Register multiple users](#register-multiple-users)
      - [Update a user](#update-a-user)
      - [Delete a user](#delete-a-user)
      - [Edit user information](#edit-user-information)
      - [Create a project with some budget expenditures](#create-a-project-with-some-budget-expenditures)
      - [Create a project with some budget expenditures \& job eligibble revenues](#create-a-project-with-some-budget-expenditures--job-eligibble-revenues)
      - [Create a project with some budget expenditures \& job eligibble revenues \& user assignments](#create-a-project-with-some-budget-expenditures--job-eligibble-revenues--user-assignments)
      - [Edit a project](#edit-a-project)
      - [Delete a project](#delete-a-project)
      - [Assign a user to a project](#assign-a-user-to-a-project)
      - [Unassign a user to a project](#unassign-a-user-to-a-project)
      - [Assign multiple users to a project](#assign-multiple-users-to-a-project)
      - [Create, update \& delete budget expenditures](#create-update--delete-budget-expenditures)
      - [Create, Update \& Delete job eligibles](#create-update--delete-job-eligibles)
      - [Save a drawdown as draft](#save-a-drawdown-as-draft)
      - [Submit a drawdown](#submit-a-drawdown)
      - [Reject a drawdown](#reject-a-drawdown)
      - [Approve a drawdown](#approve-a-drawdown)
      - [Submit a bulkupload drawdown](#submit-a-bulkupload-drawdown)
      - [Set inflation rate for a selected project](#set-inflation-rate-for-a-selected-project)
      - [Update inflation rate for a selected project](#update-inflation-rate-for-a-selected-project)
      - [Delete inflation rate for a selected project](#delete-inflation-rate-for-a-selected-project)
      - [Save a revenue as draft](#save-a-revenue-as-draft)
      - [Submit a revenue](#submit-a-revenue)
      - [Reject a revenue](#reject-a-revenue)
      - [Approve a revenue](#approve-a-revenue)
      - [Upload bank confirming document](#upload-bank-confirming-document)
      - [Update bank confirming document](#update-bank-confirming-document)
      - [Delete bank confirming document](#delete-bank-confirming-document)
      - [Reset a drawdown](#reset-a-drawdown)
      - [RESET WHOLE PALLET (requires sudo)](#reset-whole-pallet-requires-sudo)
    - [Getters](#getters-1)
      - [global\_scope](#global_scope)
      - [users\_info](#users_info)
      - [projects\_info](#projects_info)
      - [users\_by\_project](#users_by_project)
      - [projects\_by\_user](#projects_by_user)
      - [expenditures\_info](#expenditures_info)
      - [expenditures\_by\_project](#expenditures_by_project)
      - [drawdowns\_info](#drawdowns_info)
      - [drawdowns\_by\_project](#drawdowns_by_project)
      - [transactions\_info](#transactions_info)
      - [transactions\_by\_drawdown](#transactions_by_drawdown)
      - [job\_eligibles\_info](#job_eligibles_info)
      - [job\_eligibles\_by\_project](#job_eligibles_by_project)
      - [revenues\_info](#revenues_info)
      - [revenues\_by\_project](#revenues_by_project)
      - [revenue\_transactions\_info](#revenue_transactions_info)
      - [transactions\_by\_revenue](#transactions_by_revenue)
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
      - [Submit initial role setup (requires sudo)](#submit-initial-role-setup-requires-sudo-1)
      - [Create an admmistrator with sudo](#create-an-admmistrator-with-sudo-1)
      - [Remove an admmistrator with sudo](#remove-an-admmistrator-with-sudo-1)
      - [Register a new administrator account](#register-a-new-administrator-account-1)
      - [Register a new builder account](#register-a-new-builder-account-1)
      - [Register a new investor account](#register-a-new-investor-account-1)
      - [Register a new issuer account](#register-a-new-issuer-account-1)
      - [Register a new regional center account](#register-a-new-regional-center-account-1)
      - [Register multiple users](#register-multiple-users-1)
      - [Update a user](#update-a-user-1)
      - [Delete a user](#delete-a-user-1)
      - [Edit user information](#edit-user-information-1)
      - [Create a project with some budget expenditures](#create-a-project-with-some-budget-expenditures-1)
      - [Create a project with some budget expenditures \& job eligibble revenues](#create-a-project-with-some-budget-expenditures--job-eligibble-revenues-1)
      - [Create a project with some budget expenditures \& job eligibble revenues \& user assignments](#create-a-project-with-some-budget-expenditures--job-eligibble-revenues--user-assignments-1)
      - [Edit a project](#edit-a-project-1)
      - [Delete a project](#delete-a-project-1)
      - [Assign a user to a project](#assign-a-user-to-a-project-1)
      - [Unassign a user to a project](#unassign-a-user-to-a-project-1)
      - [Assign multiple users to a project](#assign-multiple-users-to-a-project-1)
      - [Create, update \& delete budget expenditures](#create-update--delete-budget-expenditures-1)
      - [Create, Update \& Delete job eligibles](#create-update--delete-job-eligibles-1)
      - [Save a drawdown as draft](#save-a-drawdown-as-draft-1)
      - [Submit a drawdown](#submit-a-drawdown-1)
      - [Reject a drawdown](#reject-a-drawdown-1)
      - [Approve a drawdown](#approve-a-drawdown-1)
      - [Submit a bulkupload drawdown](#submit-a-bulkupload-drawdown-1)
      - [Set inflation rate for a selected project](#set-inflation-rate-for-a-selected-project-1)
      - [Update inflation rate for a selected project](#update-inflation-rate-for-a-selected-project-1)
      - [Delete inflation rate for a selected project](#delete-inflation-rate-for-a-selected-project-1)
      - [Save a revenue as draft](#save-a-revenue-as-draft-1)
      - [Submit a revenue](#submit-a-revenue-1)
      - [Reject a revenue](#reject-a-revenue-1)
      - [Approve a revenue](#approve-a-revenue-1)
      - [Upload bank confirming document](#upload-bank-confirming-document-1)
      - [Update bank confirming document](#update-bank-confirming-document-1)
      - [Delete bank confirming document](#delete-bank-confirming-document-1)
      - [Reset a drawdown](#reset-a-drawdown-1)
      - [RESET WHOLE PALLET (requires sudo)](#reset-whole-pallet-requires-sudo-1)
    - [Getters](#getters-2)
      - [global\_scope](#global_scope-1)
      - [users\_info](#users_info-1)
      - [projects\_info](#projects_info-1)
      - [users\_by\_project](#users_by_project-1)
      - [projects\_by\_user](#projects_by_user-1)
      - [expenditures\_info](#expenditures_info-1)
      - [expenditures\_by\_project](#expenditures_by_project-1)
      - [drawdowns\_info](#drawdowns_info-1)
      - [drawdowns\_by\_project](#drawdowns_by_project-1)
      - [transactions\_info](#transactions_info-1)
      - [transactions\_by\_drawdown](#transactions_by_drawdown-1)
      - [job\_eligibles\_info](#job_eligibles_info-1)
      - [job\_eligibles\_by\_project](#job_eligibles_by_project-1)
      - [revenues\_info](#revenues_info-1)
      - [revenues\_by\_project](#revenues_by_project-1)
      - [revenue\_transactions\_info](#revenue_transactions_info-1)
      - [transactions\_by\_revenue](#transactions_by_revenue-1)
  - [Events](#events)
  - [Errors](#errors)



## Overview

This module provides functionality to handle the project creation and fund management, as well as the following actions:
- The creation of multiple projects with a unique identifier.
- Register users with a specific role 
- Add or remove project participants. Each participant has a specific role in the project, roles are:   `Administrator`, `Builder`, `Investor`, `Issuer` and `RegionalCenter`.
- The creation of multiple budget expenditures to keep track of expenses. Each budget expenditure needs a NAICS code and Jobs Multiplier to calculate the number of jobs created.
- Modify the inflation rate of a project for the selected year. 
- Keeps a record of the transactions made in a drawdown
- Keeps a record of the transactions made in a drawdown

### Terminology

- **Project:** Is the small unit of work, it is identified by a `ProjectID`. Once a project has been created, it is possible to assign or remove users, perform transactions, etc.
- **Inflation rate:** Is the rate of increase in the money supply of a country.
- **User**: Is the person who is registered in the system, it can be an administrator, builder, investor, issuer or regional center.
- **Proxy Role**: Is the role that a user has in a project, depending on the role, the user will have different permissions.
- **Permission**: Is the ability to perform a specific action.
- **Project status**: Is the status of a project, it can be `Started` or `Completed`.
- **Document**: Is the document that is uploaded to the system while performing a transaction.
- ** Expenditure type**: Budget expenditure types are: `HardCost`, `SoftCost`, `Operational` & `Others`.
- **Bulk upload documents**: For Construction Loan & Develor Equity drawdowns, the builder uploads the required documents in bulk. Later, the administrator can review the documents and approve or reject them.
- **CUDAction**: Is the acronym for `Create`, `Update` or `Delete`, it is used to classify the type of action that is performed.
- **AssignAction**: Is used to `Assign` or `Unassign` a user to a project.
- **Transaction**: Is the smalles actions that is performed in a drawdown or a revenue, always requires a CUDAction.
- **Feedback**: Is the reason why a transaction or drawdown is rejected.
- **Builder**: Fund administration role who is responsible to issue drawdowns.
- **Investor**: EB-5 investors are foreign nationals who invest capital into a US operation, typically either a real estate project or operating business, in order to gain certain immigration benefits such as conditional residency, commonly known as a green card.
- **Administrator**: Tracks project performance and serves as an overseer of the flow of funds from the Issuer to the Builder.
- **Issuer**: The Issuer collects capital from the investors and the Builder then requests that capital so the money can be spent on building the project. 
- **Regional Center**:  An EB-5 regional center is an economic unit, public or private, in the United States that is involved with promoting economic growth. One of the ways they do so is sponsoring real estate projects that are financed by capital from foreign EB-5 investors.
- **Drawdown**: In the world of real estate development there are typically two entities involved. The fund entity (the Issuer) and the project entity (the Builder). The Issuer collects capital from the investors and the Builder then requests that capital so the money can be spent on building the project. This is done through a drawdown request. The Builder issues a drawdown request to the Issuer who then releases the money to the Builder assuming all requirements are met. That movement of funds is called a Drawdown.
- **Revenue**: Works similar to a drawdown, but is used to track incomes generated by the project.
- **Budget Expenditure**: A budget expenditure is the Builder paying funds needed to build the project. A payment from the Builder to a construction contractor is an example of a Budget Expenditure.
- **Job Eligible Revenue**: All EB-5 projects must create American jobs. Some projects, such as hotels as one example, are able to use revenues from operations as a source for calculating jobs created. Not all EB-5 projects have Job Eligible Revenue as some are only able to use expenditures rather than revenue for calculating job creation.
- **NAICS code**: NAICS stands for North American Industry Classification System. It is a system used by the United States government to classify business establishments for the purpose of collecting, analyzing, and publishing statistical data related to the U.S. business economy. NAICS codes are used to classify industries. The NAICS code is used to calculate the number of jobs created by a project.
- **Jobs Multiplier**: Indicate how important an industry is in regional job creation. The multiplier is used to calculate the number of jobs created by a project.
## Interface

### Dispachable functions
- `initial_setup` initializes the pallet by setting the permissions for each role
& creates the global scope
- `sudo_add_administrator` adds a new administrator to the system, requires the sudo key. It's the only way to register the first administrator.
- `sudo_remove_administrator` removes an administrator from the system, requires the sudo key. It's the only way to remove the first administrator. 
- `users` allows to create, update or delete a user account from the system. Only administrators can perform this action.
- `users_edit_user` allows to edit the user informaction (name, email, etc). Any registered user can perform this action.
- `projects_create_project` creates a new project instance, only administrators can perform this action. During the project creation, the administrator can create budget expenditures, job eligibles & assign users to the project. 
- `projects_edit_project` edits the information of the selected project (title, description, image, etc). It also allwos administrators to modify the project creation date & completion date (remeber that completion date must be greater than creation date).Projects can only be edited if they are in `Started` status.
- `projects_delete_project` deletes a project instance, only administrators can perform this action. Deleting a project will also delete ALL stored data related to the project. BE CAREFUL!
- `projects_assign_user` assigns or unassigns a user to a project, only administrators can perform this action. Before assigning a user to a project, the user must be registered in the system. Users can only have one role per project.
- `expenditures_and_job_eligibles` allows to create, update or delete a budget expenditure or a job eligible. Multiple CUDActions can be performed at the same time, but not over the same budget expenditure or job eligible.
- `submit_drawdown` allows to create, update or delete transactions in a drawdown. Multiple CUDActions can be performed at the same time, but not over the same transaction. A drawdown can be submitted only if it has at least one transaction. Exists a draft status for drawdowns, this status allows to save the drawdown without submitting to the administrator.
- `approve_drawdown` approves a drawdown, only administrators can perform this action. All transactions in the drawdown must be right in order to approve the drawdown. Bulkupload drawdows requieres that an administrator fills individual transactions before approving the drawdown.
- `reject_drawdown` rejects a drawdown, only administrators can perform this action. If a single transaction is wrong, the administrator will reject the drawdown. A feedback is required when rejecting a drawdown. After a drawdown is rejected, it can be edited and submitted again.
- `up_bulkupload` allows builders to upload documents in bulk. Documents are uploaded to the system and then the administrator will review them. Bulkupload drawdowns are only allowed for Construction Loan & Developer Equity drawdowns.
- `inflation_rate` modifies the inflation rate of a project for the selected year. Inflation value is used for reporting purposes.
- `submit_revenue` allows to create, update or delete transactions in a revenue. Multiple CUDActions can be performed at the same time, but not over the same transaction. A revenue can be submitted only if it has at least one transaction. Draft status allows to save the revenue without submitting to the administrator.
- `approve_revenue` approves a revenue, only administrators can perform this action. All transactions in the revenue must be right in order to approve the revenue.
- `reject_revenue` rejects a revenue, only administrators can perform this action. If a single transaction is wrong, the administrator will reject the revenue. A feedback is required when rejecting a revenue. After a revenue is rejected, it can be edited and submitted again.
- `bank_confirming_documents` uploads the bank confirmation documents for a given drawdown. Only administrators can perform this action. After a drawdown is approved, the administrator will upload the bank confirmation documents & the drawdown status will change to `Confirmed`.
- `reset_drawdown` resets a selected drawdown to its default status. Drawdown needs to be in `Submitted` status in order to be reseted. It will delete all transactions & documents related to the drawdown. Be careful!
- `Kill storage` deletes all the data stored in the pallet. Requires the sudo key. Be careful!

### Getters

|Name| Type | Description |
|--|--|--|
| `global_scope` | StorageValue | Returns the global scope |
| `users_info` | StorageMap | Returns the user information |
| `projects_info` | StorageMap | Returns the project information |
| `users_by_project`| StorageMap | Returns the users assigned to a project |
| `projects_by_user` | StorageMap | Returns the projects assigned to a user |
| `expenditures_info` | StorageMap | Returns the budget expenditure information |
| `expenditures_by_project` | StorageMap | Returns the budget expenditures by project |
| `drawdowns_info` | StorageMap | Returns the drawdown information |
| `drawdowns_by_project` | StorageMap | Returns the drawdowns by project |
| `transactions_info` | StorageMap | Returns the transaction information |
| `transactions_by_drawdown` | DoubleStorageMap | Returns the transactions by drawdown |
| `job_eligibles_info` | StorageMap | Returns the job eligible information |
| `job_eligibles_by_project` | StorageMap | Returns the job eligibles by project |
| `revenues_info` | StorageMap | Returns the revenue information |
| `revenues_by_project` | StorageMap | Returns the revenues by project |
| `revenue_transactions_info` | StorageMap | Returns the revenue transaction information |
| `transactions_by_revenue` | DoubleStorageMap | Returns the revenue transactions by revenue |

### Constants
Constants are used to limit the number of elements that can be created in the system. They are declared in the `pallets/fund-admin/src/lib.rs` file and its value can be modified in the `runtime/src/lib.rs` file. The following constants are available:

- `MaxDocuments`: Maximun number of documents that can be uploaded to the system within a single extrinsic call.

- `MaxProjectsPerUser`: Maximun number of projects that can be assigned to a user.

- `MaxUserPerProject`: Maximun number of users that can be assigned to a project.

- `MaxBuildersPerProject`: Maximun number of builders that can be assigned to a project.

- `MaxInvestorsPerProject`: Maximun number of investors that can be assigned to a project.

- `MaxIssuersPerProject`: Maximun number of issuers that can be assigned to a project.

- `MaxRegionalCenterPerProject`: Maximun number of regional centers that can be assigned to a project.

- `MaxDrawdownsPerProject`: Maximun number of drawdowns that can be created for a project.

- `MaxTransactionsPerDrawdown`: Maximun number of transactions that can be created for a drawdown.

- `MaxRegistrationsAtTime`: Maximun number of users that can be registered at the same time.

- `MaxExpendituresPerProject`: Maximun number of budget expenditures that can be created for a project.

- `MaxProjectsPerBuilder`: Maximun number of projects that can be assigned to a builder.

- `MaxProjectsPerInvestor`: Maximun number of projects that can be assigned to an investor.

- `MaxProjectsPerIssuer`: Maximun number of projects that can be assigned to an issuer.

- `MaxProjectsPerRegionalCenter`: Maximun number of projects that can be assigned to a regional center.

- `MaxBanksPerProject`: Maximun number of banks that can be assigned to a project.

- `MaxJobEligiblesByProject`: Maximun number of job eligibles that can be created for a project.

- `MaxRevenuesByProject`: Maximun number of revenues that can be created for a project.

- `MaxTransactionsPerRevenue`: Maximun number of transactions that can be created for a revenue.

- `MaxStatusChangesPerDrawdown`: Maximun number of status changes that can be performed for a drawdown.

- `MaxStatusChangesPerRevenue`: Maximun number of status changes that can be performed for a revenue.

## Usage

The following examples will be using these prefunded accounts and testing data:

```bash
# Alice's mnemonic seed
"//Alice"
# Alice's public address 
"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# Alices stash's mnemonic seed
"//Alice//stash"
# Alices stash's public address
"5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"

# Bob's mnemonic seed
"//Bob"
# Bob's public address
"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"

# Charlie's mnemonic seed
"//Charlie"
# Charlie's public address
"5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"

# Dave's mnemonic seed
"//Dave"
# Dave's public address
"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"

# Eve's mnemonic seed
"//Eve"
# Eve's public address
"5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"

# Ferdie's mnemonic seed
"//Ferdie"
# Ferdie's public address
"5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"
```


### **Polkadot-js CLI**

Only pre-funded accounts can be used to perform transactions, this may vary depending on the chain spec. For development purposes, the following accounts are prefunded with tokens:
Alice, Alice Stash, Bob & Bob Stash.
```bash
(Alice, Alice Stash, Bob & Bob Stash are prefunded with tokens. If you want to use other accounts, you will need to transfer tokens to them. The following command shows how to transfer tokens to a new account:

```bash
# Alice transfers 10 000 000 to Ferdie
polkadot-js-api tx.balances.transfer "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL" 1000000000000 --seed "//Alice"
```

#### Submit initial role setup (requires sudo)
```bash
polkadot-js-api tx.fundAdmin.initialSetup --sudo --seed "//Alice"
```

#### Create an admmistrator with sudo
```bash
polkadot-js-api tx.fundAdmin.sudoAddAdministrator "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" "Sudo Administrator" --sudo --seed "//Alice"
```

#### Get a user info
```bash
# User id
polkadot-js-api query.fundAdmin.usersInfo "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```
```bash
# Expected output
{
  "usersInfo": {
    "name": "Alice Administrator",
    "role": "Administrator",
    "image": "",
    "dateRegistered": "1,672,770,708,001",
    "email": "",
    "documents": null
  }
}
```

#### Remove an admmistrator with sudo
```bash
polkadot-js-api tx.fundAdmin.sudoRemoveAdministrator "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" --sudo --seed "//Alice"
```
```bash
# We ensure that the user was removed successfully by querying the user info again:
polkadot-js-api query.fundAdmin.usersInfo "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```
```bash
# Output should look like this: 
{
  "usersInfo": null
}
```

#### Register a new administrator account
```bash
# Sudo Administrator registers a new administrator account -> //Alice//stash
polkadot-js-api tx.fundAdmin.users '[["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", "Administrator Test", "Administrator", "Create"]]' --seed "//Alice"
```

#### Register a new builder account
```bash
# Sudo Administrator registers a new investor account -> //Bob
polkadot-js-api tx.fundAdmin.users '[["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "Builder Test", "Builder", "Create"]]' --seed "//Alice"
```

#### Register a new investor account
```bash
# Sudo Administrator registers a new investor account -> //Charlie
polkadot-js-api tx.fundAdmin.users '[["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", "Investor Test", "Investor", "Create"]]' --seed "//Alice"
```

#### Register a new issuer account
```bash
# Sudo Administrator registers a new investor account -> //Dave
polkadot-js-api tx.fundAdmin.users '[["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", "Issuer Test", "Issuer", "Create"]]' --seed "//Alice"
```

#### Register a new regional center account
```bash
# Sudo Administrator registers a new investor account -> //Eve
polkadot-js-api tx.fundAdmin.users '[["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", "Regional Center Test", "RegionalCenter", "Create"]]' --seed "//Alice"
```

#### Register multiple users
```bash
# Sudo Administrator registers multiple users
polkadot-js-api tx.fundAdmin.users '[["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", "Administrator Test", "Administrator", "Create"], ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "Builder Test", "Builder", "Create"], ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", "Investor Test", "Investor", "Create"], ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", "Issuer Test", "Issuer", "Create"], ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", "Regional Center Test", "RegionalCenter", "Create"]]' --seed "//Alice"
```

#### Update a user
```bash
# Sudo Administrator updates a user -> //Administrator
polkadot-js-api tx.fundAdmin.users '[["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", "Administrator Test Modified", "Administrator", "Update"]]' --seed "//Alice"
```

#### Delete a user
```bash
# Sudo Administrator deletes a user -> //Issuer
polkadot-js-api tx.fundAdmin.users '[["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", null, null, "Delete"]]' --seed "//Alice"
```

#### Edit user information
```bash
# Users edit their own information -> //Builder
polkadot-js-api tx.fundAdmin.usersEditUser "Builder Test Modified" "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg" "builder@fundadmin.com" "null" --seed "//Bob"
```
```bash
# Only investors can upload documents
# Investors upload documents -> //Investor
polkadot-js-api tx.fundAdmin.usersEditUser "Investor Test Modified" "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg" "investor@fundadmin.com" '[["Investor document 1", "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.pdf"]]' --seed "//Charlie"
```

#### Create a project with some budget expenditures
```bash
# Sudo Administrator creates a project -> //Alice
polkadot-js-api tx.fundAdmin.projectsCreateProject "Project Test" "Description test" "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg" "San Francisco" "null" "1672782546001" "1672789546001" '[["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software", "SoftCost", "100000000000", "32312", "131", "Create", null], ["Marketing", "Operational", "100000000000", "58963", "896", "Create", null], ["Legal", "Others", "100000000000", "64039", "248", "Create", null]]' "null" "null" "6546161313" --seed "//Alice"
```

#### Create a project with some budget expenditures & job eligibble revenues
```bash
# Sudo Administrator creates a project -> //Alice
polkadot-js-api tx.fundAdmin.projectsCreateProject "Project Test" "Description test" "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg" "San Francisco" "null" "1672782546001" "1672789546001" '[["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software", "SoftCost", "100000000000", "32312", "131", "Create", null], ["Marketing", "Operational", "100000000000", "58963", "896", "Create", null], ["Legal", "Others", "100000000000", "64039", "248", "Create", null]]' '[["Job Eligible 1", "235354354343", "45897", "785", "Create", null], ["Job Eligible 2", "235354354343", "84467", "631", "Create", null]]' "null" "6546161313" --seed "//Alice"
```

#### Create a project with some budget expenditures & job eligibble revenues & user assignments
```bash
# Sudo Administrator creates a project -> //Alice
polkadot-js-api tx.fundAdmin.projectsCreateProject "Project Test" "Description test" "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg" "San Francisco" "null" "1672782546001" "1672789546001" '[["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software", "SoftCost", "100000000000", "32312", "131", "Create", null], ["Marketing", "Operational", "100000000000", "58963", "896", "Create", null], ["Legal", "Others", "100000000000", "64039", "248", "Create", null]]' '[["Job Eligible 1", "235354354343", "45897", "785", "Create", null], ["Job Eligible 2", "235354354343", "84467", "631", "Create", null]]' '[["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "Builder", "Assign"], ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", "Investor", "Assign"], ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", "Issuer", "Assign"], ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", "RegionalCenter", "Assign"]]'  "6546161313" --seed "//Alice"
```

#### Edit a project
```bash
polkadot-js-api tx.fundAdmin.projectsEditProject "0xa08ef1de8cc4914e0550c88e8a086f1271cc297e8bd4eab28102b2a859173d81" "Project title modified" "Description modified" "image modified" "null" "null" "null" "null" --seed "//Alice"
```

#### Delete a project
```bash
# Be careful, this will delete the project and all its data from the blockchain
polkadot-js-api tx.fundAdmin.projectsDeleteProject "0xa08ef1de8cc4914e0550c88e8a086f1271cc297e8bd4eab28102b2a859173d81" --seed "//Alice"
```

#### Assign a user to a project
```bash
polkadot-js-api tx.fundAdmin.projectsAssignUser "0x6a091e51a91fcb6f5554473fb12e4cfebfac09b678f9113c3361fd2b97b93cab" '[["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "Builder", "Assign"]]' --seed "//Alice"
```

#### Unassign a user to a project
```bash
polkadot-js-api tx.fundAdmin.projectsAssignUser "0x6a091e51a91fcb6f5554473fb12e4cfebfac09b678f9113c3361fd2b97b93cab" '[["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "Builder", "Unassign"]]' --seed "//Alice"
```

#### Assign multiple users to a project
```bash
polkadot-js-api tx.fundAdmin.projectsAssignUser "0x6a091e51a91fcb6f5554473fb12e4cfebfac09b678f9113c3361fd2b97b93cab" '[["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "Builder", "Assign"], ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", "Investor", "Assign"], ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", "Issuer", "Assign"], ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", "RegionalCenter", "Assign"]]' --seed "//Alice"
``` 

#### Create, update & delete budget expenditures
```bash
# Create budget expenditures
polkadot-js-api tx.fundAdmin.expendituresAndJobEligibles  "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" '[["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software Updated", "SoftCost", "100000000000", "32312", "131", "Update", "0xa06d982a5b5d8fc7c6452e758b06926e5013b64f91bb2023aeaa4f82433609ba"], ["null", "null", "null", "null", "null", "Delete", "0x82642a1ed981e475de4d125b6ed545afb239eaaa21ba68359682afb6906ff1a2"]]' "null" --seed "//Alice"
``` 

#### Create, Update & Delete job eligibles
```bash
polkadot-js-api tx.fundAdmin.expendituresAndJobEligibles "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "null" '[["Job Eligible 3" "165561686" "4646" "456" "Create" "null"], ["Job Eligible 2 modified" "165561686" "4646" "466" "Update" "0xe2af8e11cddb2e0e8610b268d1ddeae6568bebbda208e4c608251f84e36c8e27"], ["null" "null" "null" "null" "Delete" "0xecba5e198b10e62f024621ba25696a11a32a364c545eeadee9d354fcafad3a9b"]]' --seed "//Alice"
```

#### Save a drawdown as draft
```bash
polkadot-js-api tx.fundAdmin.submitDrawdown "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" '[["0xa06d982a5b5d8fc7c6452e758b06926e5013b64f91bb2023aeaa4f82433609ba", "1541416", [["Document name", "CID"]], "Create", null]]' "false" --seed "//Alice"
```

#### Submit a drawdown
```bash
polkadot-js-api tx.fundAdmin.submitDrawdown "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" '[["0xa06d982a5b5d8fc7c6452e758b06926e5013b64f91bb2023aeaa4f82433609ba", "1541416", [["Document name", "CID"]], "Create", null]]' "true" --seed "//Alice"
```

#### Reject a drawdown
```bash
polkadot-js-api tx.fundAdmin.rejectDrawdown "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" '[["0x85f8b45f2b1620825218dc0664bd1150f30f7e61fce123f8ce680dffca791a8c", "Feedback"]]' "null" --seed "//Alice"
```

#### Approve a drawdown
```bash
polkadot-js-api tx.fundAdmin.approveDrawdown "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" "null" "null" --seed "//Alice"
```

#### Submit a bulkupload drawdown
```bash
polkadot-js-api tx.fundAdmin.upBulkupload "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1c8dfb4f4e084eceb7cc33d8b48ddfeda53636eaef94ddd899b3751476e0253e" "Bulkupload" "35415315" '[["file1", "CID"]]' --seed "//Bob"
```

#### Set inflation rate for a selected project
```bash
polkadot-js-api tx.fundAdmin.inflationRate '[["0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527", "35", "Create"]]' --seed "//Alice"
```

#### Update inflation rate for a selected project
```bash
polkadot-js-api tx.fundAdmin.inflationRate '[["0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527", "22", "Update"]]' --seed "//Alice"
```

#### Delete inflation rate for a selected project
```bash
polkadot-js-api tx.fundAdmin.inflationRate '[["0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527", null, "Delete"]]' --seed "//Alice"
```

#### Save a revenue as draft
```bash
polkadot-js-api tx.fundAdmin.submitRevenue "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x6fb6e078dc1ada06554f9ba10a4c4c2ed5c4333ade28fde3579ab4f223c62669" '[["0xe2af8e11cddb2e0e8610b268d1ddeae6568bebbda208e4c608251f84e36c8e27", "1541416", [["File 1", "CID"]], "Create", null]]' "false" --seed "//Bob"
```

#### Submit a revenue
```bash
polkadot-js-api tx.fundAdmin.submitRevenue "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x6fb6e078dc1ada06554f9ba10a4c4c2ed5c4333ade28fde3579ab4f223c62669" '[["0xe2af8e11cddb2e0e8610b268d1ddeae6568bebbda208e4c608251f84e36c8e27", "1541416", [["File 1", "CID"]], "Create", null]]' "true" --seed "//Bob"
```

#### Reject a revenue
```bash
polkadot-js-api tx.fundAdmin.rejectRevenue "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x6fb6e078dc1ada06554f9ba10a4c4c2ed5c4333ade28fde3579ab4f223c62669" '[["0x75980300167262de3c6475ee8373e1bb6e3a14bb4f0af2a6642b03798ddaca5e", "Feedback"]]' --seed "//Alice"
```

#### Approve a revenue
```bash
polkadot-js-api tx.fundAdmin.approveRevenue "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x6fb6e078dc1ada06554f9ba10a4c4c2ed5c4333ade28fde3579ab4f223c62669" --seed "//Alice"
```

#### Upload bank confirming document
```bash
polkadot-js-api tx.fundAdmin.bankConfirmingDocuments "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" '[["File1", "CID"]]' "Create" --seed "//Alice"
```

#### Update bank confirming document
```bash
polkadot-js-api tx.fundAdmin.bankConfirmingDocuments "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" '[["File2", "CID2"]]' "Update" --seed "//Alice"
```

#### Delete bank confirming document
```bash
polkadot-js-api tx.fundAdmin.bankConfirmingDocuments "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" "null" "Delete" --seed "//Alice"
```

#### Reset a drawdown
```bash
polkadot-js-api tx.fundAdmin.resetDrawdown "0x7b637ee141c4d2dfc2114fa3194c7a78df122f818287bb531178d733e8f86527" "0x1357070edd7435842606ec2fef65f9b41bf61b65e4b2a3d84e5727f45dbf263f" --seed "//Bob"
```

#### RESET WHOLE PALLET (requires sudo)
```bash
# Be careful, this will delete all data
polkadot-js-api tx.fundAdmin.killStorage --sudo --seed "//Alice"
```
### Getters
#### global_scope
```bash
polkadot-js-api query.fundAdmin.globalScope
```
```bash
# Expected output
{
  "globalScope": "0x37fc42b060f5cd1796cb5ed5c1bb6b3bbab4df1a77aa557692a7841233204c7b"
}
```

#### users_info
```bash
# User id
polkadot-js-api query.fundAdmin.usersInfo "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```
```bash
# Expected output
{
  "usersInfo": {
    "name": "Alice Administrator",
    "role": "Administrator",
    "image": "",
    "dateRegistered": "1,672,770,708,001",
    "email": "",
    "documents": null
  }
}
```

#### projects_info
```bash
polkadot-js-api query.fundAdmin.projectsInfo "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92"
``` 
```bash
# Expected output:
{
  "projectsInfo": {
    "builder": [
      "5Ft1pwMVeLRdRFiZNTtfxvnn1W8vPp71u215uoU4eDWixCok",
      "5DkGxLznMjUSur4pUACo2h44xC2ANjA6hssqgcbrCiNjRgFJ"
    ],
    "investor": [
      "5CmFmVadzNQFaeiyXXNugRXT1MuaoocUyogtYHEQeWjGp7pX",
      "5F1SBPePR7RPhGeJ5ByPc1EkrHLEHCiDvsXMvLHWqsDrt41h"
    ],
    "issuer": [
      "5CcC6fkHkJ4jzUXfCeRizy8Z7PpWbUtMZHJvLDsZ2H4G6Zsg"
    ],
    "regionalCenter": [
      "5HipuVWKxsAHTAcu5M1cfq9pfEqcjhyWtSktGQPpgjCX8n5v"
    ],
    "title": "Test project",
    "description": "This is a test project for new budgets feature",
    "image": "QmTrJN5Rvr5reHWZoFCvEPuZyLfWHRK223Ani6GgNCPcHB:png",
    "address": "NY",
    "status": "Started",
    "inflationRate": null,
    "banks": [
      [
        "",
        ""
      ]
    ],
    "creationDate": "1,672,380,000,000",
    "completionDate": "1,703,743,200,000",
    "registrationDate": "1,672,430,298,000",
    "updatedDate": "1,672,430,298,000",
    "eb5DrawdownStatus": "Draft",
    "constructionLoanDrawdownStatus": "Draft",
    "developerEquityDrawdownStatus": "Draft",
    "revenueStatus": "Draft",
    "privateGroupId": "cd3e84b3-f4ba-4882-9eaa-58702fd82316"
  }
}
```

#### users_by_project
```bash
polkadot-js-api query.fundAdmin.usersByProject "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92"
```
```bash
# Expected output
{
  "usersByProject": [
    "5CcC6fkHkJ4jzUXfCeRizy8Z7PpWbUtMZHJvLDsZ2H4G6Zsg",
    "5HipuVWKxsAHTAcu5M1cfq9pfEqcjhyWtSktGQPpgjCX8n5v",
    "5Ft1pwMVeLRdRFiZNTtfxvnn1W8vPp71u215uoU4eDWixCok",
    "5DkGxLznMjUSur4pUACo2h44xC2ANjA6hssqgcbrCiNjRgFJ",
    "5CmFmVadzNQFaeiyXXNugRXT1MuaoocUyogtYHEQeWjGp7pX",
    "5F1SBPePR7RPhGeJ5ByPc1EkrHLEHCiDvsXMvLHWqsDrt41h"
  ]
```

#### projects_by_user
```bash
polkadot-js-api query.fundAdmin.projectsByUser "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```
```bash
# Expected output
{
  "projectsByUser": [
    "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "0x9851ccff4d99f40b222ac8c4655962980d574f327f9a1a7167ab461ed040d37e",
    "0xd083fdc0d51dae061ad448c409cb500e433a9db6f9dd0395c3167b03594539ba",
    "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92"
  ]
}
```

#### expenditures_info
```bash
polkadot-js-api query.fundAdmin.expendituresInfo "0x009eebf8f8c3cce4f631d0d4c35e10f5a20c00f6796ad6cb14404ba6337a992a"
```
```bash
# Expected output
{
  "expendituresInfo": {
    "projectId": "0x9851ccff4d99f40b222ac8c4655962980d574f327f9a1a7167ab461ed040d37e",
    "name": "Advertising",
    "expenditureType": "SoftCost",
    "expenditureAmount": "1,000,000",
    "naicsCode": "5418",
    "jobsMultiplier": "158,633"
  }
}
```

#### expenditures_by_project
```bash
polkadot-js-api query.fundAdmin.expendituresByProject "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92"
```
```bash
#  Expected output
{
  "expendituresByProject": [
    "0x8145cef3e6b08e40e736823ca1a9b5e52896fd38973e9c5e8c6443dd26378f18",
    "0x91054afdeede9be3d88f143a23fc3af967332f22cc7b2ef6c0f35d9bb7306e54",
    "0x2827e8a5ea1447d467e2f57a870060f8a7cdc1f5c7b48f18d0fd7a80f2ce53fa"
  ]
}
```

#### drawdowns_info
```bash
polkadot-js-api query.fundAdmin.drawdownsInfo "0x0eeb139d7d667e51f35d380708bb0ed00b0ce0c4aca0e76c1393d4d953657920"
```
```bash
# Expected output
{
  "drawdownsInfo": {
    "projectId": "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92",
    "drawdownNumber": "2",
    "drawdownType": "DeveloperEquity",
    "totalAmount": "0",
    "status": "Draft",
    "bulkuploadDocuments": null,
    "bankDocuments": null,
    "description": null,
    "feedback": null,
    "statusChanges": [
      [
        "Draft",
        "1,671,794,670,000"
      ]
    ],
    "createdDate": "1,671,794,670,000",
    "closedDate": "0"
  }
}
```

#### drawdowns_by_project
```bash
polkadot-js-api query.fundAdmin.drawdownsByProject "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92"
```
```bash
# Expected output
{
  "drawdownsByProject": [
    "0x61e548501ef8981eb9fba29fe456d7bef5c9bf457022c289c4286534b49c9f32",
    "0xe527d76fea2f395e8be2e6a10bc3f878d257b3201ff547f3d3c5e182f3731ee4",
    "0x1724a87c4328fd553e0297b7a774978fbde5b304ea92de433094d33f7ca2be83",
    "0x8310fd9a5b2a01d21f1efe285851ddf351ddfc189b390f9002e53b96969acc8f",
    "0x812232b06c9651cf71e61f5456bc0784c3d9ecffbece63c96b383f0178c625d8"
  ]
}
```

#### transactions_info
```bash
polkadot-js-api query.fundAdmin.transactionsInfo "0x00366329cb6173796e6539a77892174d3acf2d64bd7e419d3dc421cbe3c16cac"
```
```bash
# Expected output
{
  "transactionsInfo": {
    "projectId": "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "drawdownId": "0x25b63b708673b22e41db77261de131f0327f4a3a9c2deaa5a945dd4b26ab85fb",
    "expenditureId": "0xacbd46710a091a5e6f3909bf76253bbcd687df100adecc87293304876577700e",
    "createdDate": "1,671,794,670,000",
    "updatedDate": "1,671,794,670,000",
    "closedDate": "1,671,794,670,000",
    "feedback": null,
    "amount": "0",
    "status": "Approved",
    "documents": null
  }
}
```

#### transactions_by_drawdown
```bash
polkadot-js-api query.fundAdmin.transactionsByDrawdown "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b" "0x235394d53fa74aac09a7f975631286c8978150476432b0eecd7353bdfa6f4c9a" 
```
```bash
# Expected output
{
  "transactionsByDrawdown": [
    "0x41ef1d7b4816c0fd53b65d3dfc098fc2e39264072c9a149bc6376df11d0bf797",
    "0xb8172ca7784fa846d329dab9894b1fc251875d8279a5796991ae3eec2f52ee64",
    "0xe54ea21085ad2cfcf157a567d0b227e90ffdb6fdf9ae78d739c6ad51613a2352",
    "0x80ccd67e2326e83437e5a0d00d270d3ecdf91893d2e60c8e09435a75152f8c2a"
  ]
}
```

#### job_eligibles_info
```bash
polkadot-js-api query.fundAdmin.jobEligiblesInfo "0x104f6097b0e96da0211abd9395eddcc1f58e98c313f6392ffe77773761207253" 
```
```bash
# Expected output
{
  "jobEligiblesInfo": {
    "projectId": "0x9851ccff4d99f40b222ac8c4655962980d574f327f9a1a7167ab461ed040d37e",
    "name": "Tenant Housing Revenue (2025)",
    "jobEligibleAmount": "178,451,000",
    "naicsCode": "448",
    "jobsMultiplier": "106,902"
  }
}
```

#### job_eligibles_by_project
```bash
polkadot-js-api query.fundAdmin.jobEligiblesByProject "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b"
```
```bash
# Expected output
{
  "jobEligiblesByProject": [
    "0x189ffd4f0d71648febaede340cf6b5458030bd614fc9e458cf3b1c9a47fc4e24"
  ]
}
```

#### revenues_info
```bash
polkadot-js-api query.fundAdmin.revenuesInfo "0x0ce43f093ff109ba2c70ee6b7d6fcc5a0b66738efdd53da6df840226952b059c"
```
```bash
# Expected output
{
  "revenuesInfo": {
    "projectId": "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "revenueNumber": "3",
    "totalAmount": "0",
    "status": "Draft",
    "statusChanges": [
      [
        "Draft",
        "1,672,176,054,001"
      ]
    ],
    "createdDate": "1,672,176,054,001",
    "closedDate": "0"
  }
}
```

#### revenues_by_project
```bash
polkadot-js-api query.fundAdmin.revenuesByProject "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b" 
```
```bash
# Expected output
{
  "revenuesByProject": [
    "0x31dc37dd57dcb887c6a6c83c1905208284fb6f0dc31e35af41b6114ac89f51db",
    "0xd21c7da07916b89dbf4997900e52b2e33dbc9507ad224cc0ac1aad98f1acc4ff",
    "0x0ce43f093ff109ba2c70ee6b7d6fcc5a0b66738efdd53da6df840226952b059c"
  ]
}
```

#### revenue_transactions_info
```bash
polkadot-js-api query.fundAdmin.revenueTransactionsInfo "0x15aa93d98f038330e5343d9e1e78e22c6044a60f00147b8b263c6ccf6252213a"
```
```bash
# Expected output
{
  "revenueTransactionsInfo": {
    "projectId": "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "revenueId": "0x31dc37dd57dcb887c6a6c83c1905208284fb6f0dc31e35af41b6114ac89f51db",
    "jobEligibleId": "0x189ffd4f0d71648febaede340cf6b5458030bd614fc9e458cf3b1c9a47fc4e24",
    "createdDate": "1,671,793,920,000",
    "updatedDate": "1,671,793,920,000",
    "closedDate": "1,671,817,128,001",
    "feedback": null,
    "amount": "687,688",
    "status": "Approved",
    "documents": [
      [
        "message_1.txt",
        "Qmaa6u3D2kCjQHXYimoSKdH6eXvVFXbPcoPHpEE4NhHyFZ"
      ]
    ]
  }
}
```

#### transactions_by_revenue
```bash
polkadot-js-api query.fundAdmin.transactionsByRevenue "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b" "0x31dc37dd57dcb887c6a6c83c1905208284fb6f0dc31e35af41b6114ac89f51db"
```
```bash
# Expected output
{
  "transactionsByRevenue": [
    "0x15aa93d98f038330e5343d9e1e78e22c6044a60f00147b8b263c6ccf6252213a"
  ]
}
```

### Polkadot-js api (javascript library)
While most of the data flow is almost identical to its CLI counter part, the javascript library is much more versatile regarding queries.

#### Submit initial role setup (requires sudo)
```js
const initial_set_up = await api.tx.sudo.sudo(api.tx.fundAdmin.initialSetup()).signAndSend(alice);
```

#### Create an admmistrator with sudo
```js
const register_admin = await api.tx.sudo.sudo(api.tx.fundAdmin.sudoAddAdministrator(admin.address, "Administrator Name")).signAndSend(alice);
```

#### Remove an admmistrator with sudo
```js
const remove_admin = await api.tx.sudo.sudo(api.tx.fundAdmin.sudoAddAdministrator(admin.address)).signAndSend(alice);
```

#### Register a new administrator account
```js
const register_admin = await api.tx.fundAdmin.users([[admin.address, "Administrator Test", "Administrator", "Create"]]).signAndSend(alice);
```

#### Register a new builder account
```js
const register_builder = await api.tx.fundAdmin.users([[builder.address, "Builder Test", "Builder", "Create"]]).signAndSend(alice);
```

#### Register a new investor account
```js
const register_investor = await api.tx.fundAdmin.users([[builder.address, "Builder Test", "Builder", "Create"]]).signAndSend(alice);
```

#### Register a new issuer account
```js
const register_issuer = await api.tx.fundAdmin.users([[issuer.address, "Issuer Test", "Issuer", "Create"]]).signAndSend(alice);
```

#### Register a new regional center account
```js
const register_regional_center = await api.tx.fundAdmin.users([[regional_center.address, "Regional Center Test", "Regional Center", "Create"]]).signAndSend(alice);
```

#### Register multiple users
```js
const register_users = await api.tx.fundAdmin.users([[admin.address, "Administrator Test", "Administrator", "Create"], [builder.address, "Builder Test", "Builder", "Create"], [investor.address, "Investor Test", "Investor", "Create"], [issuer.address, "Issuer Test", "Issuer", "Create"], [regional_center.address, "Regional Center Test", "RegionalCenter", "Create"]]).signAndSend(alice);
```

#### Update a user
```js
const update_user = await api.tx.fundAdmin.users([[admin.address, "Administrator Test Modified", "Administrator", "Update"]]).signAndSend(alice);
```

#### Delete a user
```js
const delete_user = await api.tx.fundAdmin.users([[builder.address, null, null, "Delete"]]).signAndSend(alice);
```

#### Edit user information
```js
const edit_user_information = await api.tx.fundAdmin.usersEditUser("Builder Test Modified", "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg", "builder@fundadmin.com", null).signAndSend(alice);
```
```js
// Only investor can update documents
const edit_user = await api.tx.fundAdmin.usersEditUser("Investor Test Modified", "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg", "investor@fundadmin.com", [["Investor document 1", "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.pdf"]]).signAndSend(charlie);
```

#### Create a project with some budget expenditures
```js
const create_project = await api.tx.fundAdmin.projectsCreateProject("Project Test", "Description test", "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg", "San Francisco", null, "1672782546001", "1672789546001", [["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software", "SoftCost", "100000000000", "32312", "131", "Create", null], ["Marketing", "Operational", "100000000000", "58963", "896", "Create", null], ["Legal", "Others", "100000000000", "64039", "248", "Create", null]], null, null, "6546161313").signAndSend(alice);
```

#### Create a project with some budget expenditures & job eligibble revenues
```js
const create_project = await api.tx.fundAdmin.projectsCreateProject("Project Test", "Description test", "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg", "San Francisco", null, "1672782546001", "1672789546001", [["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software", "SoftCost", "100000000000", "32312", "131", "Create", null], ["Marketing", "Operational", "100000000000", "58963", "896", "Create", null], ["Legal", "Others", "100000000000", "64039", "248", "Create", null]], [["Job Eligible 1", "235354354343", "45897", "785", "Create", null], ["Job Eligible 2", "235354354343", "84467", "631", "Create", null]], null, "6546161313").signAndSend(alice);
```

#### Create a project with some budget expenditures & job eligibble revenues & user assignments
```js
const create_project = await api.tx.fundAdmin.projectsCreateProject("Project Test", "Description test", "QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg", "San Francisco", null, "1672782546001", "1672789546001", [["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software", "SoftCost", "100000000000", "32312", "131", "Create", null], ["Marketing", "Operational", "100000000000", "58963", "896", "Create", null], ["Legal", "Others", "100000000000", "64039", "248", "Create", null]], [["Job Eligible 1", "235354354343", "45897", "785", "Create", null], ["Job Eligible 2", "235354354343", "84467", "631", "Create", null]], [["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "Builder", "Assign"], ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", "Investor", "Assign"], ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", "Issuer", "Assign"], ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", "RegionalCenter", "Assign"]], "6546161313").signAndSend(alice);
```

#### Edit a project
```js
const edit_project = await api.tx.fundAdmin.projectsEditProject(project.id, "Project title modified", "Description modified", "image modified", null, null, null, null).signAndSend(alice);
```

#### Delete a project
```js
const delete_project = await api.tx.fundAdmin.projectsDeleteProject(project.id).signAndSend(alice);
```

#### Assign a user to a project
```js
const assign_user = await api.tx.fundAdmin.projectsAssignUser(project.id, [[builder.address, "Builder", "Assign"]]).signAndSend(alice);
```

#### Unassign a user to a project
```js
const assign_user = await api.tx.fundAdmin.projectsAssignUser(project.id, [[builder.address, "Builder", "Unassign"]]).signAndSend(alice);
```

#### Assign multiple users to a project
```js
const assign_user = await api.tx.fundAdmin.projectsAssignUser(project.id, [[builder.address, "Builder", "Assign"], [investor.address, "Investor", "Assign"], [issuer.address, "Issuer", "Assign"], [regional_center.address, "RegionalCenter", "Assign"]]).signAndSend(alice);
```

#### Create, update & delete budget expenditures
```js
const budget_expenditures = await api.tx.fundAdmin.expendituresAndJobEligibles(project.id, [["Furniture", "Hardcost", "100000000000", "15897", "145", "Create", null], ["Software Updated", "SoftCost", "100000000000", "32312", "131", "Update", expenditure.id], ["null", "null", "null", "null", "null", "Delete", expenditure.id]], null).signAndSend(alice);
```

#### Create, Update & Delete job eligibles
```js
const budget_expenditures = await api.tx.fundAdmin.expendituresAndJobEligibles(project.id, null, [["Job Eligible 3", "165561686", "4646", "456", "Create", null], ["Job Eligible 2 modified", "165561686", "4646", "466", "Update", job_eligible.id], [null, null, null, null, "Delete" job_eligible.id]]).signAndSend(alice);
```

#### Save a drawdown as draft
```js
const save_drawdown = await api.tx.fundAdmin.submitDrawdown(project.id, drawdown.id, [[expenditure.id, "1541416", [["File 1", "CID"]], "Create", null]], "false").signAndSend(alice);
```

#### Submit a drawdown
```js
const submit_drawdown = await api.tx.fundAdmin.submitDrawdown(project.id, drawdown.id, [[expenditure.id, "1541416", [["File 1", "CID"]], "Create", null]], "true").signAndSend(alice);
```

#### Reject a drawdown
```js
const reject_drawdown = await api.tx.fundAdmin.rejectDrawdown(project.id, drawdown.id, [[transaction.id, "Feedback"]], null).signAndSend(alice);
```

#### Approve a drawdown
```js
const approve_drawdown = await api.tx.fundAdmin.approveDrawdown(project.id, drawdown.id, null, null).signAndSend(alice);
```

#### Submit a bulkupload drawdown
```js
const submit_bulkupload_drawdown = await api.tx.fundAdmin.upBulkupload(project.id, drawdown.id, "Bulkupload", "35415315", [["file1", "CID"]]).signAndSend(bob);
```

#### Set inflation rate for a selected project
```js
const set_inflation_rate = await api.tx.fundAdmin.inflationRate([[project.id, "35", "Create"]]).signAndSend(alice);
```

#### Update inflation rate for a selected project
```js
const update_inflation_rate = await api.tx.fundAdmin.inflationRate([[project.id, "22", "Update"]]).signAndSend(alice);
```

#### Delete inflation rate for a selected project
```js
const update_inflation_rate = await api.tx.fundAdmin.inflationRate([[project.id, null, "Delete"]]).signAndSend(alice);
```

#### Save a revenue as draft
```js
const save_revenue = await api.tx.fundAdmin.submitRevenue(project.id, revenue.id, [[job_eligible.id, "1541416", [["File 1", "CID"]], "Create", null]], "false").signAndSend(bob);
```

#### Submit a revenue
```js
const submit_revenue = await api.tx.fundAdmin.submitRevenue(project.id, revenue.id, [[job_eligible.id, "1541416", [["File 1", "CID"]], "Create", null]], "false").signAndSend(bob);
```

#### Reject a revenue
```js
const reject_revenue = await api.tx.fundAdmin.rejectRevenue(project.id, revenue.id, [[revenue_transaction.id, "Feedback"]]).signAndSend(alice);
```

#### Approve a revenue
```js
const approve_revenue = await api.tx.fundAdmin.approveRevenue(project.id, revenue.id).signAndSend(alice);
```

#### Upload bank confirming document
```js
const upload_bank_documents = await api.tx.fundAdmin.bankConfirmingDocuments(project.id, drawdown.id, [["File1", "CID"]], "Create").signAndSend(alice);
```

#### Update bank confirming document
```js
const update_bank_documents = await api.tx.fundAdmin.bankConfirmingDocuments(project.id, drawdown.id, [["File2", "CID2"]], "Update").signAndSend(alice);
```

#### Delete bank confirming document
```js
const delete_bank_documents = await api.tx.fundAdmin.bankConfirmingDocuments(project.id, drawdown.id, null, "Delete").signAndSend(alice);
```

#### Reset a drawdown
```js
const reset_drawdown = await api.tx.fundAdmin.resetDrawdown(project.id, drawdown.id).signAndSend(bob);
```

#### RESET WHOLE PALLET (requires sudo)
```js
// Be careful, this will delete all data
const reset_pallet = await api.tx.sudo.sudo(api.tx.fundAdmin.killStorage()).signAndSend(alice);
```

### Getters
#### global_scope
```js
const global_scope = await api.query.fundAdmin.globalScope.entries();
  console.log(global_scope.toHuman());
```
```bash
# Expected output
{
  globalScope: '0x37fc42b060f5cd1796cb5ed5c1bb6b3bbab4df1a77aa557692a7841233204c7b'
}
```

#### users_info
```js
const user_info = await api.query.fundAdmin.usersInfo(alice.address);
  console.log(user_info.toHuman());
```
```bash
# Expected output
{
  "usersInfo": {
    "name": "Alice Administrator",
    "role": "Administrator",
    "image": "",
    "dateRegistered": "1,672,770,708,001",
    "email": "",
    "documents": null
  }
}
```
```js
// Get all users
const user_info = await api.query.fundAdmin.usersInfo.entries();
  user_info.forEach(([key, exposure]) => {
    console.log('key account_id:', key.args.map((k) => k.toHuman()));
    console.log('account:', exposure.toHuman(),"\n");
  });
```

#### projects_info
```js
const projects_info = await api.query.fundAdmin.projectsInfo(project.id);
  console.log(projects_info.toHuman());
```
```bash
# Expected output:
{
  "projectsInfo": {
    "builder": [
      "5Ft1pwMVeLRdRFiZNTtfxvnn1W8vPp71u215uoU4eDWixCok",
      "5DkGxLznMjUSur4pUACo2h44xC2ANjA6hssqgcbrCiNjRgFJ"
    ],
    "investor": [
      "5CmFmVadzNQFaeiyXXNugRXT1MuaoocUyogtYHEQeWjGp7pX",
      "5F1SBPePR7RPhGeJ5ByPc1EkrHLEHCiDvsXMvLHWqsDrt41h"
    ],
    "issuer": [
      "5CcC6fkHkJ4jzUXfCeRizy8Z7PpWbUtMZHJvLDsZ2H4G6Zsg"
    ],
    "regionalCenter": [
      "5HipuVWKxsAHTAcu5M1cfq9pfEqcjhyWtSktGQPpgjCX8n5v"
    ],
    "title": "Test project",
    "description": "This is a test project for new budgets feature",
    "image": "QmTrJN5Rvr5reHWZoFCvEPuZyLfWHRK223Ani6GgNCPcHB:png",
    "address": "NY",
    "status": "Started",
    "inflationRate": null,
    "banks": [
      [
        "",
        ""
      ]
    ],
    "creationDate": "1,672,380,000,000",
    "completionDate": "1,703,743,200,000",
    "registrationDate": "1,672,430,298,000",
    "updatedDate": "1,672,430,298,000",
    "eb5DrawdownStatus": "Draft",
    "constructionLoanDrawdownStatus": "Draft",
    "developerEquityDrawdownStatus": "Draft",
    "revenueStatus": "Draft",
    "privateGroupId": "cd3e84b3-f4ba-4882-9eaa-58702fd82316"
  }
}
```
```js
// Get all projects
const projects_info = await api.query.fundAdmin.projectsInfo.entries();
  projects_info.forEach(([key, exposure]) => {
    console.log('key project_id:', key.args.map((k) => k.toHuman()));
    console.log('project:', exposure.toHuman(),"\n");
  });
```

#### users_by_project
```js
const users_by_project = await api.query.fundAdmin.usersByProject(project.id);
  console.log(users_by_project.toHuman());
```
```bash
# Expected output
{
  "usersByProject": [
    "5CcC6fkHkJ4jzUXfCeRizy8Z7PpWbUtMZHJvLDsZ2H4G6Zsg",
    "5HipuVWKxsAHTAcu5M1cfq9pfEqcjhyWtSktGQPpgjCX8n5v",
    "5Ft1pwMVeLRdRFiZNTtfxvnn1W8vPp71u215uoU4eDWixCok",
    "5DkGxLznMjUSur4pUACo2h44xC2ANjA6hssqgcbrCiNjRgFJ",
    "5CmFmVadzNQFaeiyXXNugRXT1MuaoocUyogtYHEQeWjGp7pX",
    "5F1SBPePR7RPhGeJ5ByPc1EkrHLEHCiDvsXMvLHWqsDrt41h"
  ]
```
```js
// Get all users by project
const users_by_project = await api.query.fundAdmin.usersByProject.entries();
  users_by_project.forEach(([key, exposure]) => {
    console.log('key project_id:', key.args.map((k) => k.toHuman()));
    console.log('users:', exposure.toHuman(),"\n");
  });
```

#### projects_by_user
```js
const projects_by_user = await api.query.fundAdmin.projectsByUser(project.id);
  console.log(projects_by_user.toHuman());
```
```bash
# Expected output
{
  "projectsByUser": [
    "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "0x9851ccff4d99f40b222ac8c4655962980d574f327f9a1a7167ab461ed040d37e",
    "0xd083fdc0d51dae061ad448c409cb500e433a9db6f9dd0395c3167b03594539ba",
    "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92"
  ]
}
```
```js
// Get all projects by user
const projects_by_user = await api.query.fundAdmin.projectsByUser.entries();
  projects_by_user.forEach(([key, exposure]) => {
    console.log('key account_id:', key.args.map((k) => k.toHuman()));
    console.log('projects:', exposure.toHuman(),"\n");
  });
```

#### expenditures_info
```js
const expenditures_info = await api.query.fundAdmin.expendituresInfo(expenditure.id);
  console.log(expenditures_info.toHuman());
```
```bash
# Expected output
{
  "expendituresInfo": {
    "projectId": "0x9851ccff4d99f40b222ac8c4655962980d574f327f9a1a7167ab461ed040d37e",
    "name": "Advertising",
    "expenditureType": "SoftCost",
    "expenditureAmount": "1,000,000",
    "naicsCode": "5418",
    "jobsMultiplier": "158,633"
  }
}
```
```js
// Get all expenditures info
const expenditures_info = await api.query.fundAdmin.expendituresInfo.entries();
  expenditures_info.forEach(([key, exposure]) => {
    console.log('key expenditure_id:', key.args.map((k) => k.toHuman()));
    console.log('expenditure:', exposure.toHuman(),"\n");
  });
```

#### expenditures_by_project
```js
const expenditures_by_project = await api.query.fundAdmin.expendituresByProject(expenditure.id);
  console.log(expenditures_info.toHuman());
```
```bash
#  Expected output
{
  "expendituresByProject": [
    "0x8145cef3e6b08e40e736823ca1a9b5e52896fd38973e9c5e8c6443dd26378f18",
    "0x91054afdeede9be3d88f143a23fc3af967332f22cc7b2ef6c0f35d9bb7306e54",
    "0x2827e8a5ea1447d467e2f57a870060f8a7cdc1f5c7b48f18d0fd7a80f2ce53fa"
  ]
}
```
```js
// Get all expenditures by project
const expenditures_by_project = await api.query.fundAdmin.expendituresByProject.entries();
  expenditures_by_project.forEach(([key, exposure]) => {
    console.log('key project_id:', key.args.map((k) => k.toHuman()));
    console.log('expenditures:', exposure.toHuman(),"\n");
  });
```

#### drawdowns_info
```js
const drawdowns_info = await api.query.fundAdmin.drawdownsInfo(drawdown.id);
  console.log(drawdowns_info.toHuman());
```
```bash
# Expected output
{
  "drawdownsInfo": {
    "projectId": "0x4ba9d47e29d7136fdb010f902f030b6926b820fc7d5c2f75a4468a0ba934ca92",
    "drawdownNumber": "2",
    "drawdownType": "DeveloperEquity",
    "totalAmount": "0",
    "status": "Draft",
    "bulkuploadDocuments": null,
    "bankDocuments": null,
    "description": null,
    "feedback": null,
    "statusChanges": [
      [
        "Draft",
        "1,671,794,670,000"
      ]
    ],
    "createdDate": "1,671,794,670,000",
    "closedDate": "0"
  }
}
```
```js
// Get all drawdowns info
const drawdowns_info = await api.query.fundAdmin.drawdownsInfo.entries();
  drawdowns_info.forEach(([key, exposure]) => {
    console.log('key drawdown_id:', key.args.map((k) => k.toHuman()));
    console.log('drawdown:', exposure.toHuman(),"\n");
  });
```

#### drawdowns_by_project
```js
const drawdowns_by_project = await api.query.fundAdmin.drawdownsByProject(project.id);
  console.log(drawdowns_by_project.toHuman());
```
```bash
# Expected output
{
  "drawdownsByProject": [
    "0x61e548501ef8981eb9fba29fe456d7bef5c9bf457022c289c4286534b49c9f32",
    "0xe527d76fea2f395e8be2e6a10bc3f878d257b3201ff547f3d3c5e182f3731ee4",
    "0x1724a87c4328fd553e0297b7a774978fbde5b304ea92de433094d33f7ca2be83",
    "0x8310fd9a5b2a01d21f1efe285851ddf351ddfc189b390f9002e53b96969acc8f",
    "0x812232b06c9651cf71e61f5456bc0784c3d9ecffbece63c96b383f0178c625d8"
  ]
}
```
```js
// Get all drawdowns by project
const drawdowns_by_project = await api.query.fundAdmin.drawdownsInfo.entries();
  drawdowns_by_project.forEach(([key, exposure]) => {
    console.log('key project_id:', key.args.map((k) => k.toHuman()));
    console.log('drawdowns:', exposure.toHuman(),"\n");
  });
```

#### transactions_info
```js
const transactions_info = await api.query.fundAdmin.transactionsInfo(transaction.id);
  console.log(transactions_info.toHuman());
```
```bash
# Expected output
{
  "transactionsInfo": {
    "projectId": "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "drawdownId": "0x25b63b708673b22e41db77261de131f0327f4a3a9c2deaa5a945dd4b26ab85fb",
    "expenditureId": "0xacbd46710a091a5e6f3909bf76253bbcd687df100adecc87293304876577700e",
    "createdDate": "1,671,794,670,000",
    "updatedDate": "1,671,794,670,000",
    "closedDate": "1,671,794,670,000",
    "feedback": null,
    "amount": "0",
    "status": "Approved",
    "documents": null
  }
}
```
```js
// Get all transactions info
const transactions_info = await api.query.fundAdmin.transactionsInfo.entries();
  transactions_info.forEach(([key, exposure]) => {
    console.log('key transaction_id:', key.args.map((k) => k.toHuman()));
    console.log('transaction:', exposure.toHuman(),"\n");
  });
```

#### transactions_by_drawdown
```js
const transactions_by_drawdown = await api.query.fundAdmin.transactionsByDrawdown(project.id, drawdown.id);
  console.log(transactions_by_drawdown.toHuman());
```
```bash
# Expected output
{
  "transactionsByDrawdown": [
    "0x41ef1d7b4816c0fd53b65d3dfc098fc2e39264072c9a149bc6376df11d0bf797",
    "0xb8172ca7784fa846d329dab9894b1fc251875d8279a5796991ae3eec2f52ee64",
    "0xe54ea21085ad2cfcf157a567d0b227e90ffdb6fdf9ae78d739c6ad51613a2352",
    "0x80ccd67e2326e83437e5a0d00d270d3ecdf91893d2e60c8e09435a75152f8c2a"
  ]
}
```
```js
// Get all transactions by drawdown
const transactions_by_drawdown = await api.query.fundAdmin.transactionsByDrawdown.entries();
  transactions_by_drawdown.forEach(([key, exposure]) => {
    console.log('key project_id and drawdown_id:', key.args.map((k) => k.toHuman()));
    console.log('transactions:', exposure.toHuman(),"\n");
  });
```

#### job_eligibles_info
```js
const job_eligibles_info = await api.query.fundAdmin.jobEligiblesInfo(transaction.id);
  console.log(job_eligibles_info.toHuman());
```
```bash
# Expected output
{
  "jobEligiblesInfo": {
    "projectId": "0x9851ccff4d99f40b222ac8c4655962980d574f327f9a1a7167ab461ed040d37e",
    "name": "Tenant Housing Revenue (2025)",
    "jobEligibleAmount": "178,451,000",
    "naicsCode": "448",
    "jobsMultiplier": "106,902"
  }
}
```
```js
// Get all job eligibles info
const job_eligibles_info = await api.query.fundAdmin.jobEligiblesInfo.entries();
  job_eligibles_info.forEach(([key, exposure]) => {
    console.log('key transaction_id :', key.args.map((k) => k.toHuman()));
    console.log('Job Eligible:', exposure.toHuman(),"\n");
  });
```

#### job_eligibles_by_project
```js
const job_eligibles_by_project = await api.query.fundAdmin.jobEligiblesInfo(project.id);
  console.log(job_eligibles_by_project.toHuman());
```
```bash
# Expected output
{
  "jobEligiblesByProject": [
    "0x189ffd4f0d71648febaede340cf6b5458030bd614fc9e458cf3b1c9a47fc4e24"
  ]
}
```
```js
// Get all job eligibles by project
const job_eligibles_info = await api.query.fundAdmin.jobEligiblesInfo.entries();
  job_eligibles_info.forEach(([key, exposure]) => {
    console.log('key project_id :', key.args.map((k) => k.toHuman()));
    console.log('Job Eligibles:', exposure.toHuman(),"\n");
  });
```

#### revenues_info
```js
const revenues_info = await api.query.fundAdmin.revenuesInfo(revenue.id);
  console.log(revenues_info.toHuman());
```
```bash
# Expected output
{
  "revenuesInfo": {
    "projectId": "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "revenueNumber": "3",
    "totalAmount": "0",
    "status": "Draft",
    "statusChanges": [
      [
        "Draft",
        "1,672,176,054,001"
      ]
    ],
    "createdDate": "1,672,176,054,001",
    "closedDate": "0"
  }
}
```
```js
// Get all revenues info
const revenues_info = await api.query.fundAdmin.revenuesInfo.entries();
  revenues_info.forEach(([key, exposure]) => {
    console.log('key revenue_id :', key.args.map((k) => k.toHuman()));
    console.log('revenue:', exposure.toHuman(),"\n");
  });
```

#### revenues_by_project
```js
const revenues_by_project = await api.query.fundAdmin.revenuesByProject(project.id);
  console.log(revenues_by_project.toHuman());
```
```bash
# Expected output
{
  "revenuesByProject": [
    "0x31dc37dd57dcb887c6a6c83c1905208284fb6f0dc31e35af41b6114ac89f51db",
    "0xd21c7da07916b89dbf4997900e52b2e33dbc9507ad224cc0ac1aad98f1acc4ff",
    "0x0ce43f093ff109ba2c70ee6b7d6fcc5a0b66738efdd53da6df840226952b059c"
  ]
}
```
```js
// Get all revenues by project
const revenues_info = await api.query.fundAdmin.revenuesInfo.entries();
  revenues_info.forEach(([key, exposure]) => {
    console.log('key project_id :', key.args.map((k) => k.toHuman()));
    console.log('revenues:', exposure.toHuman(),"\n");
  });
```

#### revenue_transactions_info
```js
const revenue_transactions_info = await api.query.fundAdmin.revenueTransactionsInfo(revenue_transaction.id);
  console.log(revenue_transactions_info.toHuman());
```
```bash
# Expected output
{
  "revenueTransactionsInfo": {
    "projectId": "0xcd16d877d2a5eb7eb7fb0ebf9fe27d9baa421a208e7c48c40cb88c178c81fc4b",
    "revenueId": "0x31dc37dd57dcb887c6a6c83c1905208284fb6f0dc31e35af41b6114ac89f51db",
    "jobEligibleId": "0x189ffd4f0d71648febaede340cf6b5458030bd614fc9e458cf3b1c9a47fc4e24",
    "createdDate": "1,671,793,920,000",
    "updatedDate": "1,671,793,920,000",
    "closedDate": "1,671,817,128,001",
    "feedback": null,
    "amount": "687,688",
    "status": "Approved",
    "documents": [
      [
        "message_1.txt",
        "Qmaa6u3D2kCjQHXYimoSKdH6eXvVFXbPcoPHpEE4NhHyFZ"
      ]
    ]
  }
}
```
```js
// Get all revenue transactions info
const revenues_info = await api.query.fundAdmin.revenuesInfo.entries();
  revenues_info.forEach(([key, exposure]) => {
    console.log('key transaction_id :', key.args.map((k) => k.toHuman()));
    console.log('revenue transactions:', exposure.toHuman(),"\n");
  });
```

#### transactions_by_revenue
```js
const transactions_by_revenue = await api.query.fundAdmin.transactionsByRevenue(project.id, revenue.id);
  console.log(transactions_by_revenue.toHuman());
```
```bash
# Expected output
{
  "transactionsByRevenue": [
    "0x15aa93d98f038330e5343d9e1e78e22c6044a60f00147b8b263c6ccf6252213a"
  ]
}
```
```js
// Get all transactions by revenue
const transactions_by_revenue = await api.query.fundAdmin.transactionsByRevenue.entries();
  transactions_by_revenue.forEach(([key, exposure]) => {
    console.log('key project_id and revenue_id :', key.args.map((k) => k.toHuman()));
    console.log('revenue transactions:', exposure.toHuman(),"\n");
  });
```

## Events
```rust
ProxySetupCompleted,
/// Project was created successfully
ProjectCreated(T::AccountId, ProjectId)

/// The selected roject was edited successfully
ProjectEdited(T::AccountId, ProjectId)

/// The selected project was deleted successfully
ProjectDeleted(T::AccountId, ProjectId)

/// Administrator was registered successfully using the sudo pallet
AdministratorAssigned(T::AccountId)

/// Administrator was removed successfully using the sudo pallet
AdministratorRemoved(T::AccountId)

/// The user was assigned to the selected project
UserAssignmentCompleted(T::AccountId, ProjectId)

/// The user was unassigned to the selected project
UserUnassignmentCompleted(T::AccountId, ProjectId)

/// Users extrinsic was executed, individual CUDActions were applied
UsersExecuted(T::AccountId)

/// A new user account was created successfully
UserCreated(T::AccountId)

/// The selected user was edited successfully
UserUpdated(T::AccountId)

/// The selected user was deleted successfully
UserDeleted(T::AccountId)

/// An array of expenditures was executed depending on the CUDAction
ExpendituresExecuted(T::AccountId, ProjectId)

/// Expenditure was created successfully
ExpenditureCreated(ProjectId, ExpenditureId)

/// Expenditure was updated successfully
ExpenditureUpdated(ProjectId, ExpenditureId)

/// Expenditure was deleted successfully
ExpenditureDeleted(ProjectId, ExpenditureId)

/// An array of transactions was executed depending on the CUDAction
TransactionsExecuted(ProjectId, DrawdownId)

/// Transaction was created successfully
TransactionCreated(ProjectId, DrawdownId, TransactionId)

/// Transaction was edited successfully
TransactionEdited(ProjectId, DrawdownId, TransactionId)

/// Transaction was deleted successfully
TransactionDeleted(ProjectId, DrawdownId, TransactionId)

/// Assign users extrinsic was completed successfully
UsersAssignationExecuted(T::AccountId, ProjectId)

/// Drawdowns were initialized successfully at the beginning of the project
DrawdownsInitialized(T::AccountId, ProjectId)

/// Drawdown was created successfully
DrawdownCreated(ProjectId, DrawdownId)

/// Drawdown was submitted successfully
DrawdownSubmitted(ProjectId, DrawdownId)

/// Drawdown was approved successfully
DrawdownApproved(ProjectId, DrawdownId)

/// Drawdown was rejected successfully
DrawdownRejected(ProjectId, DrawdownId)

/// Drawdown was cancelled successfully
DrawdownSubmissionCancelled(ProjectId, DrawdownId)

/// Bulkupload drawdown was submitted successfully
BulkUploadSubmitted(ProjectId, DrawdownId)

/// An array of adjustments was executed depending on the CUDAction
InflationRateAdjusted(T::AccountId)

/// An array of job eligibles was executed depending on the CUDAction
JobEligiblesExecuted(T::AccountId, ProjectId)

/// Job eligible was created successfully
JobEligibleCreated(ProjectId, JobEligibleId)

/// Job eligible was updated successfully
JobEligibleUpdated(ProjectId, JobEligibleId)

/// Job eligible was deleted successfully
JobEligibleDeleted(ProjectId, JobEligibleId)

/// Revenue transaction was created successfully
RevenueTransactionCreated(ProjectId, RevenueId, RevenueTransactionId)

/// Revenue transaction was updated successfully
RevenueTransactionUpdated(ProjectId, RevenueId, RevenueTransactionId)

/// Revenue transaction was deleted successfully
RevenueTransactionDeleted(ProjectId, RevenueId, RevenueTransactionId)

/// An array of revenue transactions was executed depending on the CUDAction
RevenueTransactionsExecuted(ProjectId, RevenueId)

/// Revenue was created successfully
RevenueCreated(ProjectId, RevenueId)

/// Revenue was submitted successfully
RevenueSubmitted(ProjectId, RevenueId)

/// Revenue was approved successfully
RevenueApproved(ProjectId, RevenueId)

/// Revenue was rejected successfully
RevenueRejected(ProjectId, RevenueId)

/// Bank's confirming documents were uploaded successfully
BankDocumentsUploaded(ProjectId, DrawdownId)

/// Bank's confirming documents were updated successfully
BankDocumentsUpdated(ProjectId, DrawdownId)

/// Bank's confirming documents were deleted successfully
BankDocumentsDeleted(ProjectId, DrawdownId)
```

## Errors

```rust
/// No value was found for the global scope
NoGlobalScopeValueWasFound
/// Project ID is already in use
ProjectIdAlreadyInUse
/// Timestamp was not genereated correctly
TimestampError
/// Completion date must be later than creation date
CompletionDateMustBeLater
/// User is already registered in the site
UserAlreadyRegistered
/// Project was not found
ProjectNotFound
/// Project is not active anymore
ProjectIsAlreadyCompleted
/// Can not delete a completed project
CannotDeleteCompletedProject
/// User is not registered
UserNotRegistered
/// User has been already added to the project
UserAlreadyAssignedToProject
/// Max number of users per project reached
MaxUsersPerProjectReached
/// Max number of projects per user reached
MaxProjectsPerUserReached
/// User is not assigned to the project
UserNotAssignedToProject
/// Can not register administrator role
CannotRegisterAdminRole
/// Max number of builders per project reached
MaxBuildersPerProjectReached
/// Max number of investors per project reached
MaxInvestorsPerProjectReached
/// Max number of issuers per project reached
MaxIssuersPerProjectReached
/// Max number of regional centers per project reached
MaxRegionalCenterPerProjectReached
/// Can not remove administrator role
CannotRemoveAdminRole
/// Can not add admin role at user project assignment
CannotAddAdminRole
/// User can not have more than one role at the same time
UserCannotHaveMoreThanOneRole
/// Expenditure not found
ExpenditureNotFound
/// Expenditure already exist
ExpenditureAlreadyExists
/// Max number of expenditures per project reached
MaxExpendituresPerProjectReached
/// Field name can not be empty
EmptyExpenditureName
/// Expenditure does not belong to the project
ExpenditureDoesNotBelongToProject
/// Drowdown id is not found
DrawdownNotFound
/// Invalid amount
InvalidAmount
/// Documents field is empty
DocumentsIsEmpty
/// Transaction id is not found
TransactionNotFound
/// Transaction already exist
TransactionAlreadyExists
/// Max number of transactions per drawdown reached
MaxTransactionsPerDrawdownReached
/// Drawdown already exist
DrawdownAlreadyExists
/// Max number of drawdowns per project reached
MaxDrawdownsPerProjectReached
/// Max number of status changes per drawdown reached
MaxStatusChangesPerDrawdownReached
/// Can not modify a completed drawdown
CannotEditDrawdown
/// Can not perform any action on a submitted transaction
CannotPerformActionOnSubmittedTransaction
/// Can not perform any action on a approved transaction
CannotPerformActionOnApprovedTransaction
/// Can not perform any action on a confirmed transaction
CannotPerformActionOnConfirmedTransaction
/// Can not perform any action on a submitted drawdown
CannotPerformActionOnSubmittedDrawdown
/// Can not perform any action on a approved drawdown
CannotPerformActionOnApprovedDrawdown
/// Can not perform any action on a confirmed drawdown
CannotPerformActionOnConfirmedDrawdown
/// Transaction is already completed
TransactionIsAlreadyCompleted
/// User does not have the specified role
UserDoesNotHaveRole
/// Transactions vector is empty
EmptyTransactions
/// Transaction ID was not found in do_execute_transaction
TransactionIdNotFound
/// Drawdown can not be submitted if does not has any transactions
DrawdownHasNoTransactions
/// Cannot submit transaction
CannotSubmitTransaction
/// Drawdown can not be approved if is not in submitted status
DrawdownIsNotInSubmittedStatus
/// Transactions is not in submitted status
TransactionIsNotInSubmittedStatus
/// Array of expenditures is empty
EmptyExpenditures
/// Expenditure name is required
ExpenditureNameRequired
/// Expenditure type is required
ExpenditureTypeRequired
/// Expenditure amount is required
ExpenditureAmountRequired
/// Expenditure id is required
ExpenditureIdRequired
/// User name is required
UserNameRequired
/// User role is required
UserRoleRequired
/// Amount is required
AmountRequired
/// Can not delete a user if the user is assigned to a project
UserHasAssignedProjects
/// Can not send a drawdown to submitted status if it has no transactions
NoTransactionsToSubmit
/// Bulk upload description is required
BulkUploadDescriptionRequired
/// Bulk upload documents are required
BulkUploadDocumentsRequired
/// Administrator can not delete themselves
AdministratorsCannotDeleteThemselves
/// No feedback was provided for bulk upload
NoFeedbackProvidedForBulkUpload
/// NO feedback for EN5 drawdown was provided
EB5MissingFeedback
/// Inflation rate extrinsic is missing an array of project ids
InflationRateMissingProjectIds
/// Inflation rate was not provided
InflationRateRequired
/// Inflation rate has been already set for the selected project
InflationRateAlreadySet
/// Inflation rate was not set for the selected project
InflationRateNotSet
/// Bulkupload drawdowns are only allowed for Construction Loan & Developer Equity
DrawdownTypeNotSupportedForBulkUpload
/// Cannot edit user role if the user is assigned to a project
UserHasAssignedProjectsCannotUpdateRole
/// Cannot delete user if the user is assigned to a project
UserHasAssignedProjectsCannotDelete
/// Cannot send a bulkupload drawdown if the drawdown status isn't in draft or rejected
DrawdownStatusNotSupportedForBulkUpload
/// Cannot submit a drawdown if the drawdown status isn't in draft or rejected
DrawdownIsNotInDraftOrRejectedStatus
/// Only investors can update/edit their documents
UserIsNotAnInvestor
/// Max number of projects per builder has been reached
MaxProjectsPerBuilderReached
/// Max number of projects per investor has been reached
MaxProjectsPerInvestorReached
/// Max number of projects per issuer has been reached
MaxProjectsPerIssuerReached
/// Max number of projects per regional center has been reached
MaxProjectsPerRegionalCenterReached
/// Jobs eligibles array is empty
JobEligiblesIsEmpty
/// JOb eligible name is required
JobEligiblesNameIsRequired
/// Job eligible id already exists
JobEligibleIdAlreadyExists
/// Max number of job eligibles per project reached
MaxJobEligiblesPerProjectReached
/// Job eligible id not found
JobEligibleNotFound
/// Jopb eligible does not belong to the project
JobEligibleDoesNotBelongToProject
/// Job eligible name is required
JobEligibleNameRequired
/// Job eligible amount is required
JobEligibleAmountRequired
/// Job eligible id is required
JobEligibleIdRequired
/// Revenue id was not found
RevenueNotFound
/// Transactions revenue array is empty
RevenueTransactionsEmpty
/// Revenue transaction is not in submitted status
RevenueTransactionNotSubmitted
/// Revenue can not be edited
CannotEditRevenue
/// Revenue transaction id already exists
RevenueTransactionIdAlreadyExists
/// Max number of transactions per revenue reached
MaxTransactionsPerRevenueReached
/// Revenue transaction id not found
RevenueTransactionNotFound
/// Revenue transaction can not be edited
CannotEditRevenueTransaction
/// Max number of status changes per revenue reached
MaxStatusChangesPerRevenueReached
/// Can not perform any action on a submitted revenue
CannotPerformActionOnSubmittedRevenue
/// Can not perform any action on a approved revenue
CannotPerformActionOnApprovedRevenue
/// Can not perform any action on a submitted revenue transaction
CannotPerformActionOnApprovedRevenueTransaction
/// Can not perform any action on a approved revenue transaction
CannotPerformActionOnSubmittedRevenueTransaction
/// Revenue amoun is required
RevenueAmountRequired
/// Revenue transaction id is required
RevenueTransactionIdRequired
/// Revenue Id already exists
RevenueIdAlreadyExists
/// Maximun number of revenues per project reached
MaxRevenuesPerProjectReached
/// Can not send a revenue to submitted status if it has no transactions
RevenueHasNoTransactions
/// Revenue is not in submitted status
RevenueIsNotInSubmittedStatus
/// Revenue transaction is not in submitted status
RevenueTransactionIsNotInSubmittedStatus
/// The revenue is not in submitted status
RevenueNotSubmitted
/// Can not upload bank confirming documents if the drawdown is not in Approved status
DrawdownNotApproved
/// Drawdown is not in Confirmed status
DrawdownNotConfirmed
/// Drawdown is not in Submitted status
DrawdownNotSubmitted
/// Can not insert (CUDAction: Create) bank confmirng documents if the drawdown has already bank confirming documents
DrawdownHasAlreadyBankConfirmingDocuments
/// Drawdown has no bank confirming documents (CUDAction: Update or Delete)
DrawdownHasNoBankConfirmingDocuments
/// Bank confirming documents are required
BankConfirmingDocumentsNotProvided
/// Banck confirming documents array is empty
BankConfirmingDocumentsAreEmpty
/// No scope was provided checking if the user has permissions. No applies for administrator users
NoScopeProvided
/// Only eb5 drawdowns are allowed to upload bank documentation
OnlyEB5DrawdownsCanUploadBankDocuments
/// The private group id is empty
PrivateGroupIdIsEmpty
``` 

