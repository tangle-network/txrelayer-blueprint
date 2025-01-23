use std::sync::Arc;

use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::Address;
use alloy::providers::fillers::{FillProvider, JoinFill, RecommendedFillers, WalletFiller};
use alloy::providers::RootProvider;
use alloy::transports::BoxTransport;
use blueprint_sdk::config::GadgetConfiguration;
use blueprint_sdk::keystore::{Keystore, KeystoreConfig};

use crate::alloy_compat::BlueprintKeystoreEcdsaSigner;
use crate::call_permit;
use crate::config::AppConfig;

pub type RecommendedFillersOf<T> = <T as RecommendedFillers>::RecommendedFillers;

/// A type alias for the Alloy provider with wallet.
pub type AlloyProviderWithWallet = FillProvider<
    JoinFill<RecommendedFillersOf<Ethereum>, WalletFiller<EthereumWallet>>,
    RootProvider<BoxTransport>,
    BoxTransport,
    Ethereum,
>;

/// The service context for the transaction relayer.
#[derive(Clone)]
pub struct ServiceContext {
    pub(crate) config: GadgetConfiguration,
    pub(crate) alloy_provider: AlloyProviderWithWallet,
    pub(crate) call_permit_instance: call_permit::Instance,
    pub(crate) app_config: Arc<AppConfig>,
}

impl ServiceContext {
    pub async fn new(
        config: GadgetConfiguration,
        call_permit_address: Address,
        app_config_name: &str,
    ) -> crate::Result<Self> {
        let keystore_cfg = KeystoreConfig::new().fs_root(&config.keystore_uri);
        let keystore = Keystore::new(keystore_cfg)?;
        let signer = BlueprintKeystoreEcdsaSigner::new(keystore).local_signer()?;
        let wallet = EthereumWallet::new(signer);
        let root_provider = alloy::providers::ProviderBuilder::new()
            .on_builtin(&config.http_rpc_endpoint)
            .await?;
        let alloy_provider = FillProvider::new(root_provider, Ethereum::recommended_fillers())
            .join_with(WalletFiller::new(wallet));
        let call_permit_instance =
            call_permit::Instance::new(call_permit_address, alloy_provider.clone());
        let app_config = Arc::new(AppConfig::load(app_config_name)?);
        Ok(Self {
            config,
            alloy_provider,
            call_permit_instance,
            app_config,
        })
    }

    pub fn alloy_provider(&self) -> &AlloyProviderWithWallet {
        &self.alloy_provider
    }

    pub fn call_permit_instance(&self) -> &call_permit::Instance {
        &self.call_permit_instance
    }

    pub fn config(&self) -> &GadgetConfiguration {
        &self.config
    }

    pub fn app_config(&self) -> Arc<AppConfig> {
        self.app_config.clone()
    }
}
