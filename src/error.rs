use blueprint_sdk as sdk;

/// Define the error types for the application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Keystore(#[from] sdk::keystore::Error),
    #[error(transparent)]
    K256(#[from] sdk::crypto::k256::error::K256Error),
    #[error(transparent)]
    Transport(#[from] alloy::transports::TransportError),
    #[error(transparent)]
    Contract(#[from] alloy::contract::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
}

pub type Result<T> = core::result::Result<T, Error>;
