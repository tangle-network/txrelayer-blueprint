use blueprint_sdk as sdk;

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
}

pub type Result<T> = core::result::Result<T, Error>;
