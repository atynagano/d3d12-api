pub use root_sig_macro::hlsl_root_sig_core;

#[macro_export]
macro_rules! hlsl_root_sig {
    ($($tt:tt)*) => {{
        const ROOT_SIG: &[u8] = $crate::utils::hlsl::hlsl_root_sig_core![$($tt)*];
        ROOT_SIG
    }};
}