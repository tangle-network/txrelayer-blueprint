use alloy::signers::{k256::ecdsa::SigningKey, local::LocalSigner};
use blueprint_sdk::{
    crypto::k256::K256Ecdsa,
    keystore::{Keystore, backends::Backend},
};

/// A compatibility layer for the Alloy Keystore with the Blueprint Keystore.
pub struct BlueprintKeystoreEcdsaSigner(pub(crate) Keystore);

impl BlueprintKeystoreEcdsaSigner {
    pub fn new(keystore: Keystore) -> Self {
        Self(keystore)
    }

    pub fn local_signer(&self) -> crate::Result<LocalSigner<SigningKey>> {
        let first_ecdsa = self.0.first_local::<K256Ecdsa>()?;
        let secret = self.0.get_secret::<K256Ecdsa>(&first_ecdsa)?;
        let local_signer = secret.alloy_key()?;
        Ok(local_signer)
    }
}
