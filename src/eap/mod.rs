// Early Access Program (EAP) namespace
// Expose the internal `audit` module here while it stabilizes.
pub mod audit {
    pub use crate::audit::*;
}
