/// Implements `Default` for a `#[serde(untagged)]` enum by selecting a specific variant.
///
/// # Example
/// ```ignore
/// impl_untagged_default!(FontWeight, String, FontWeightString::Normal);
/// ```
macro_rules! impl_untagged_default {
    ($ty:ty, $variant:ident, $val:expr) => {
        impl Default for $ty {
            fn default() -> Self {
                <$ty>::$variant($val)
            }
        }
    };
}

pub(crate) use impl_untagged_default;
