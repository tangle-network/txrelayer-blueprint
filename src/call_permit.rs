use alloy::{
    primitives::{address, Address},
    sol,
};
pub use CallPermit::*;

sol! {
    #[sol(rpc)]
    "contracts/src/CallPermit.sol",
}

pub const CALL_PERMIT_ADDRESS: Address = address!("0000000000000000000000000000000000000805");

/// The instance of the CallPermit contract.
pub type Instance = CallPermitInstance<super::BoxTransport, super::AlloyProviderWithWallet>;
