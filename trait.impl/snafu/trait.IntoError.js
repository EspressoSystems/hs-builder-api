(function() {var implementors = {
"hs_builder_api":[["impl&lt;__T0&gt; IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.BlockAvailableSnafu.html\" title=\"struct hs_builder_api::builder::BlockAvailableSnafu\">BlockAvailableSnafu</a>&lt;__T0&gt;<div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,\n    __T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;,</div>"],["impl&lt;__T0, __T1&gt; IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.CustomSnafu.html\" title=\"struct hs_builder_api::builder::CustomSnafu\">CustomSnafu</a>&lt;__T0, __T1&gt;<div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,\n    __T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;,\n    __T1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;StatusCode&gt;,</div>"],["impl&lt;__T0&gt; IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.BlockClaimSnafu.html\" title=\"struct hs_builder_api::builder::BlockClaimSnafu\">BlockClaimSnafu</a>&lt;__T0&gt;<div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,\n    __T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;,</div>"],["impl IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.BuildError.html\" title=\"enum hs_builder_api::builder::BuildError\">BuildError</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.MissingSnafu.html\" title=\"struct hs_builder_api::builder::MissingSnafu\">MissingSnafu</a><div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.BuildError.html\" title=\"enum hs_builder_api::builder::BuildError\">BuildError</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,</div>"],["impl&lt;__T0&gt; IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.BuildError.html\" title=\"enum hs_builder_api::builder::BuildError\">BuildError</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.Snafu.html\" title=\"struct hs_builder_api::builder::Snafu\">Snafu</a>&lt;__T0&gt;<div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.BuildError.html\" title=\"enum hs_builder_api::builder::BuildError\">BuildError</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,\n    __T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;,</div>"],["impl IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.TxnUnpackSnafu.html\" title=\"struct hs_builder_api::builder::TxnUnpackSnafu\">TxnUnpackSnafu</a><div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,</div>"],["impl IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.TxnSubmitSnafu.html\" title=\"struct hs_builder_api::builder::TxnSubmitSnafu\">TxnSubmitSnafu</a><div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,</div>"],["impl IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.RequestSnafu.html\" title=\"struct hs_builder_api::builder::RequestSnafu\">RequestSnafu</a><div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.Error.html\" title=\"enum hs_builder_api::builder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,</div>"],["impl IntoError&lt;<a class=\"enum\" href=\"hs_builder_api/builder/enum.BuildError.html\" title=\"enum hs_builder_api::builder::BuildError\">BuildError</a>&gt; for <a class=\"struct\" href=\"hs_builder_api/builder/struct.NotFoundSnafu.html\" title=\"struct hs_builder_api::builder::NotFoundSnafu\">NotFoundSnafu</a><div class=\"where\">where\n    <a class=\"enum\" href=\"hs_builder_api/builder/enum.BuildError.html\" title=\"enum hs_builder_api::builder::BuildError\">BuildError</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + ErrorCompat,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()