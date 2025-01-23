#![allow(clippy::too_many_arguments)]

use alloy::{
    primitives::{address, Address},
    sol,
    transports::BoxTransport,
};
pub use CallPermit::*;

sol! {
    #[sol(rpc)]
    "contracts/src/CallPermit.sol",
}

pub const CALL_PERMIT_ADDRESS: Address = address!("0000000000000000000000000000000000000805");

/// The instance of the CallPermit contract.
pub type Instance = CallPermitInstance<BoxTransport, crate::ctx::AlloyProviderWithWallet>;
