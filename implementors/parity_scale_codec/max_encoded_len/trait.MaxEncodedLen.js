(function() {var implementors = {};
implementors["hashed_parachain_runtime"] = [{"text":"impl MaxEncodedLen for <a class=\"enum\" href=\"hashed_parachain_runtime/enum.ProxyType.html\" title=\"enum hashed_parachain_runtime::ProxyType\">ProxyType</a>","synthetic":false,"types":["hashed_parachain_runtime::ProxyType"]},{"text":"impl MaxEncodedLen for <a class=\"enum\" href=\"hashed_parachain_runtime/enum.OriginCaller.html\" title=\"enum hashed_parachain_runtime::OriginCaller\">OriginCaller</a>","synthetic":false,"types":["hashed_parachain_runtime::OriginCaller"]}];
implementors["hashed_runtime"] = [{"text":"impl MaxEncodedLen for <a class=\"enum\" href=\"hashed_runtime/enum.OriginCaller.html\" title=\"enum hashed_runtime::OriginCaller\">OriginCaller</a>","synthetic":false,"types":["hashed_runtime::OriginCaller"]}];
implementors["pallet_bitcoin_vaults"] = [{"text":"impl MaxEncodedLen for <a class=\"struct\" href=\"pallet_bitcoin_vaults/types/crypto/struct.Public.html\" title=\"struct pallet_bitcoin_vaults::types::crypto::Public\">Public</a>","synthetic":false,"types":["pallet_bitcoin_vaults::types::crypto::Public"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_bitcoin_vaults/pallet/trait.Config.html\" title=\"trait pallet_bitcoin_vaults::pallet::Config\">Config</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_bitcoin_vaults/types/struct.Vault.html\" title=\"struct pallet_bitcoin_vaults::types::Vault\">Vault</a>&lt;T&gt;","synthetic":false,"types":["pallet_bitcoin_vaults::types::Vault"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_bitcoin_vaults/pallet/trait.Config.html\" title=\"trait pallet_bitcoin_vaults::pallet::Config\">Config</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_bitcoin_vaults/types/struct.ProposalSignatures.html\" title=\"struct pallet_bitcoin_vaults::types::ProposalSignatures\">ProposalSignatures</a>&lt;T&gt;","synthetic":false,"types":["pallet_bitcoin_vaults::types::ProposalSignatures"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_bitcoin_vaults/pallet/trait.Config.html\" title=\"trait pallet_bitcoin_vaults::pallet::Config\">Config</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_bitcoin_vaults/types/struct.Proposal.html\" title=\"struct pallet_bitcoin_vaults::types::Proposal\">Proposal</a>&lt;T&gt;","synthetic":false,"types":["pallet_bitcoin_vaults::types::Proposal"]},{"text":"impl&lt;MaxLen:&nbsp;Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u32.html\">u32</a>&gt;&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_bitcoin_vaults/types/struct.Descriptors.html\" title=\"struct pallet_bitcoin_vaults::types::Descriptors\">Descriptors</a>&lt;MaxLen&gt;","synthetic":false,"types":["pallet_bitcoin_vaults::types::Descriptors"]},{"text":"impl&lt;DescriptorMaxLen:&nbsp;Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u32.html\">u32</a>&gt;, XPubLen:&nbsp;Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u32.html\">u32</a>&gt;&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_bitcoin_vaults/types/struct.ProposalRequest.html\" title=\"struct pallet_bitcoin_vaults::types::ProposalRequest\">ProposalRequest</a>&lt;DescriptorMaxLen, XPubLen&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"pallet_bitcoin_vaults/types/struct.Descriptors.html\" title=\"struct pallet_bitcoin_vaults::types::Descriptors\">Descriptors</a>&lt;DescriptorMaxLen&gt;: MaxEncodedLen,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"pallet_bitcoin_vaults/types/struct.Descriptors.html\" title=\"struct pallet_bitcoin_vaults::types::Descriptors\">Descriptors</a>&lt;DescriptorMaxLen&gt;: MaxEncodedLen,<br>&nbsp;&nbsp;&nbsp;&nbsp;BoundedVec&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>, XPubLen&gt;: MaxEncodedLen,<br>&nbsp;&nbsp;&nbsp;&nbsp;BoundedVec&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>, XPubLen&gt;: MaxEncodedLen,&nbsp;</span>","synthetic":false,"types":["pallet_bitcoin_vaults::types::ProposalRequest"]},{"text":"impl MaxEncodedLen for <a class=\"enum\" href=\"pallet_bitcoin_vaults/types/enum.ProposalStatus.html\" title=\"enum pallet_bitcoin_vaults::types::ProposalStatus\">ProposalStatus</a>","synthetic":false,"types":["pallet_bitcoin_vaults::types::ProposalStatus"]},{"text":"impl&lt;MaxLen:&nbsp;Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u32.html\">u32</a>&gt;&gt; MaxEncodedLen for <a class=\"enum\" href=\"pallet_bitcoin_vaults/types/enum.BDKStatus.html\" title=\"enum pallet_bitcoin_vaults::types::BDKStatus\">BDKStatus</a>&lt;MaxLen&gt;","synthetic":false,"types":["pallet_bitcoin_vaults::types::BDKStatus"]}];
implementors["pallet_confidential_docs"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_confidential_docs/pallet/trait.Config.html\" title=\"trait pallet_confidential_docs::pallet::Config\">Config</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_confidential_docs/types/struct.Vault.html\" title=\"struct pallet_confidential_docs::types::Vault\">Vault</a>&lt;T&gt;","synthetic":false,"types":["pallet_confidential_docs::types::Vault"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_confidential_docs/pallet/trait.Config.html\" title=\"trait pallet_confidential_docs::pallet::Config\">Config</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_confidential_docs/types/struct.OwnedDoc.html\" title=\"struct pallet_confidential_docs::types::OwnedDoc\">OwnedDoc</a>&lt;T&gt;","synthetic":false,"types":["pallet_confidential_docs::types::OwnedDoc"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_confidential_docs/pallet/trait.Config.html\" title=\"trait pallet_confidential_docs::pallet::Config\">Config</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_confidential_docs/types/struct.SharedDoc.html\" title=\"struct pallet_confidential_docs::types::SharedDoc\">SharedDoc</a>&lt;T&gt;","synthetic":false,"types":["pallet_confidential_docs::types::SharedDoc"]}];
implementors["pallet_fruniques"] = [{"text":"impl MaxEncodedLen for <a class=\"struct\" href=\"pallet_fruniques/types/struct.ChildInfo.html\" title=\"struct pallet_fruniques::types::ChildInfo\">ChildInfo</a>","synthetic":false,"types":["pallet_fruniques::types::ChildInfo"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_fruniques/pallet/trait.Config.html\" title=\"trait pallet_fruniques::pallet::Config\">Config</a>&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_fruniques/types/struct.FruniqueInheritance.html\" title=\"struct pallet_fruniques::types::FruniqueInheritance\">FruniqueInheritance</a>&lt;T&gt;","synthetic":false,"types":["pallet_fruniques::types::FruniqueInheritance"]},{"text":"impl MaxEncodedLen for <a class=\"enum\" href=\"pallet_fruniques/types/enum.FruniqueRole.html\" title=\"enum pallet_fruniques::types::FruniqueRole\">FruniqueRole</a>","synthetic":false,"types":["pallet_fruniques::types::FruniqueRole"]},{"text":"impl MaxEncodedLen for <a class=\"enum\" href=\"pallet_fruniques/types/enum.Permission.html\" title=\"enum pallet_fruniques::types::Permission\">Permission</a>","synthetic":false,"types":["pallet_fruniques::types::Permission"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()