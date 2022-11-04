window.SIDEBAR_ITEMS = {"constant":[["DAYS",""],["HOURS",""],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MINUTES",""],["SLOT_DURATION",""],["VERSION",""],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""],["WEIGHT_PER_SECOND",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["Call",""],["Event",""],["OriginCaller",""],["SystemCall","Contains one variant per dispatchable that can be called by an extrinsic."],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["native_version","The version information used to identify this runtime when compiled natively."]],"macro":[["construct_runtime","Construct a runtime, with the given name and the given pallets."],["parameter_types","Create new implementations of the `Get` trait."]],"mod":[["api",""],["constants",""],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don’t need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core data structures."]],"struct":[["ApprovalDeposit",""],["AsEnsureOriginWithArg",""],["AssetDeposit",""],["BasicDeposit",""],["BlockExecutionWeight","Time to execute an empty block. Calculated by multiplying the Average with `1` and adding `0`."],["BlockHashCount",""],["BlockLength",""],["BlockWeights","We allow for 2 seconds of compute with a 6 second average block time."],["BountyCuratorDeposit",""],["BountyDepositBase",""],["BountyDepositPayoutDelay",""],["BountyUpdatePeriod",""],["BountyValueMinimum",""],["Burn",""],["CIDMaxLen",""],["CandidateDeposit",""],["ChallengePeriod",""],["ChildMaxLen",""],["CollectionDeposit",""],["CollectionSymbolLimit",""],["ConfigDepositBase",""],["ConstU128","Const getter for a basic type."],["ConstantMultiplier","Implementor of [`WeightToFee`] that uses a constant multiplier."],["CouncilMaxMembers",""],["CouncilMaxProposals",""],["CouncilMotionDuration",""],["CuratorDepositMax",""],["CuratorDepositMin",""],["CuratorDepositMultiplier",""],["DataDepositPerByte",""],["DocDescMaxLen",""],["DocDescMinLen",""],["DocNameMaxLen",""],["DocNameMinLen",""],["EitherOfDiverse","“OR gate” implementation of `EnsureOrigin` allowing for different `Success` types for `L` and `R`, with them combined using an `Either` type."],["ExistentialDeposit",""],["ExtrinsicBaseWeight","Time to execute a NO-OP extrinsic, for example `System::remark`. Calculated by multiplying the Average with `1` and adding `0`."],["FieldDeposit",""],["FriendDepositFactor",""],["GenesisConfig",""],["IdentityFee","Implementor of `WeightToFee` that maps one unit of weight to one unit of fee."],["IndexDeposit",""],["ItemDeposit",""],["KeyLimit",""],["LabelMaxLen",""],["LimitBoundedVec",""],["MaxAccountsPerTransaction",""],["MaxAdditionalFields",""],["MaxApplicants",""],["MaxApplicationsPerCustodian",""],["MaxApprovals",""],["MaxAuthorities",""],["MaxAuthsPerMarket",""],["MaxBoundedVecs",""],["MaxBuildersPerProject",""],["MaxCandidateIntake",""],["MaxCosignersPerVault",""],["MaxDocuments",""],["MaxDrawdownsByStatus",""],["MaxDrawdownsPerProject",""],["MaxExpendituresPerProject",""],["MaxFeedbackLen",""],["MaxFiles",""],["MaxFriends",""],["MaxInvestorsPerProject",""],["MaxIssuersPerProject",""],["MaxKeys",""],["MaxLockDuration",""],["MaxLocks",""],["MaxMarketsPerItem",""],["MaxOffersPerMarket",""],["MaxOwnedDocs",""],["MaxPeerDataEncodingSize",""],["MaxPeerIdLength",""],["MaxPeerInHeartbeats",""],["MaxPermissionsPerRole",""],["MaxPriorities",""],["MaxProjectsPerUser",""],["MaxProposalsPerVault",""],["MaxRecursions",""],["MaxRegionalCenterPerProject",""],["MaxRegistrars",""],["MaxRegistrationsAtTime",""],["MaxRolesPerAuth",""],["MaxRolesPerPallet",""],["MaxRolesPerUser",""],["MaxScopesPerPallet",""],["MaxSharedFromDocs",""],["MaxSharedToDocs",""],["MaxStrikes",""],["MaxSubAccounts",""],["MaxTransactionsPerDrawdown",""],["MaxUserPerProject",""],["MaxUsersPerRole",""],["MaxVaultsPerUser",""],["MaxWellKnownNodes",""],["MaximumReasonLength",""],["MembershipMaxMembers",""],["MetadataDepositBase",""],["MetadataDepositPerByte",""],["MinimumPeriod",""],["NameMaxLen",""],["NotesMaxLen",""],["OperationalFeeMultiplier",""],["Origin","The runtime origin type representing the origin of a call."],["OutputDescriptorMaxLen",""],["PSBTMaxLen",""],["PalletId","A pallet identifier. These are per pallet and should be stored in a registry somewhere."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["PartsLimit",""],["Perbill","A fixed point representation of a number in the range [0, 1]."],["PeriodSpend",""],["Permill","A fixed point representation of a number in the range [0, 1]."],["PermissionMaxLen",""],["ProjectDescMaxLen",""],["ProjectNameMaxLen",""],["ProposalBond",""],["ProposalBondMaximum",""],["ProposalBondMinimum",""],["RecoveryDeposit",""],["ResourceSymbolLimit",""],["RocksDbWeight","By default, Substrate uses RocksDB, so this will be the weight used throughout the runtime."],["RoleMaxLen",""],["RotationPeriod",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["SS58Prefix",""],["SocietyPalletId",""],["SpendPeriod",""],["StorageInfo","Metadata about storage from the runtime."],["StringLimit",""],["SubAccountDeposit",""],["TipCountdown",""],["TipFindersFee",""],["TipReportDepositBase",""],["TransactionByteFee",""],["TreasuryPalletId",""],["ValueLimit",""],["VaultDescriptionMaxLen",""],["Version",""],["WrongSideDeduction",""],["XPubLen",""]],"trait":[["BuildStorage","Complex storage builder stuff."],["KeyOwnerProofSystem","Something which can compute and check proofs of a historical key owner and return full identification data of that key owner."],["Randomness","A trait that is able to provide randomness."],["StorageValue","A trait for working with macro-generated storage values under the substrate storage API."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex",""],["Address","The address format for describing accounts."],["Afloat",""],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Assets",""],["AssetsConfig",""],["Aura",""],["AuraConfig",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["BitcoinVaults",""],["BitcoinVaultsConfig",""],["Block","Block type as expected by this runtime."],["BlockNumber","An index to a block."],["Bounties",""],["ConfidentialDocs",""],["Council",""],["CouncilConfig",""],["Executive","Executive: handles dispatch to the various modules."],["Fruniques",""],["Grandpa",""],["GrandpaConfig",""],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Identity",""],["Index","Index of a transaction in the chain."],["Indices",""],["IndicesConfig",""],["Membership",""],["MembershipConfig",""],["Moment",""],["NodeAuthorization",""],["NodeAuthorizationConfig",""],["ProxyFinancial",""],["RBAC",""],["RandomnessCollectiveFlip",""],["Recovery",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedExtra","The SignedExtension to the basic transaction logic."],["SignedPayload","The payload being signed in transactions."],["Society",""],["SocietyConfig",""],["Sudo",""],["SudoConfig",""],["System",""],["SystemConfig",""],["TemplateModule",""],["Timestamp",""],["TransactionPayment",""],["TransactionPaymentConfig",""],["Treasury",""],["TreasuryConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Uniques",""],["Weight","Numeric range of a transaction weight."]]};