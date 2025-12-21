#![allow(dead_code)]

pub use crate::deflate::*;
pub use crate::inflate::*;
pub use crate::utilities::*;

pub mod deflate;
pub mod eap;
pub mod inflate;
pub mod utilities;

// Experimental and EAP modules follow here; those are kept internal until they are ready for
// public consumption., but are re-exposed under the `eap` namespace.
mod audit;

#[cfg(test)]
mod tests;
