//! # Transaction Relayer built with Blueprint SDK

/// Compatibility layer for the Alloy SDK with the Blueprint SDK.
mod alloy_compat;
/// Call permit contract interaction.
pub mod call_permit;
/// Transaction relayer specific configuration.
pub mod config;
/// Service context for the transaction relayer.
pub mod ctx;
/// Application error types.
pub mod error;
/// HTTP server routes.
pub mod http;

pub use ctx::ServiceContext;
pub use error::Error;
use error::Result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
