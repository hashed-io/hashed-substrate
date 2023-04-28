(function() {var implementors = {
"hashed_parachain_runtime":[["impl&lt;__SR_API_BLOCK__: BlockT + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>, RuntimeApiImplCall: CallApiAt&lt;__SR_API_BLOCK__&gt; + 'static&gt; TransactionPaymentApi&lt;__SR_API_BLOCK__, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"hashed_parachain_runtime/struct.RuntimeApiImpl.html\" title=\"struct hashed_parachain_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;__SR_API_BLOCK__, RuntimeApiImplCall&gt;<span class=\"where fmt-newline\">where\n    RuntimeApiImplCall::StateBackend: StateBackend&lt;HashFor&lt;__SR_API_BLOCK__&gt;&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.reference.html\">&amp;'static RuntimeApiImplCall</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    &lt;__SR_API_BLOCK__ as BlockT&gt;::Extrinsic: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u32.html\">u32</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    RuntimeDispatchInfo&lt;<a class=\"type\" href=\"hashed_parachain_runtime/type.Balance.html\" title=\"type hashed_parachain_runtime::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    FeeDetails&lt;<a class=\"type\" href=\"hashed_parachain_runtime/type.Balance.html\" title=\"type hashed_parachain_runtime::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    Weight: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"hashed_parachain_runtime/type.Balance.html\" title=\"type hashed_parachain_runtime::Balance\">Balance</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    __SR_API_BLOCK__::Header: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>"]],
"hashed_runtime":[["impl&lt;__SR_API_BLOCK__: BlockT + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>, RuntimeApiImplCall: CallApiAt&lt;__SR_API_BLOCK__&gt; + 'static&gt; TransactionPaymentApi&lt;__SR_API_BLOCK__, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"hashed_runtime/struct.RuntimeApiImpl.html\" title=\"struct hashed_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;__SR_API_BLOCK__, RuntimeApiImplCall&gt;<span class=\"where fmt-newline\">where\n    RuntimeApiImplCall::StateBackend: StateBackend&lt;HashFor&lt;__SR_API_BLOCK__&gt;&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.reference.html\">&amp;'static RuntimeApiImplCall</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    &lt;__SR_API_BLOCK__ as BlockT&gt;::Extrinsic: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u32.html\">u32</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    RuntimeDispatchInfo&lt;<a class=\"type\" href=\"hashed_runtime/type.Balance.html\" title=\"type hashed_runtime::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    FeeDetails&lt;<a class=\"type\" href=\"hashed_runtime/type.Balance.html\" title=\"type hashed_runtime::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"struct\" href=\"hashed_runtime/struct.Weight.html\" title=\"struct hashed_runtime::Weight\">Weight</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"hashed_runtime/type.Balance.html\" title=\"type hashed_runtime::Balance\">Balance</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    __SR_API_BLOCK__::Header: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()