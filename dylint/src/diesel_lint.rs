use rustc_lint::LateLintPass;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub DIESEL_LINT,
    Warn,
    "description goes here"
}

declare_lint_pass!(DieselLint => [DIESEL_LINT]);

impl<'hir> LateLintPass<'hir> for DieselLint {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html
}
