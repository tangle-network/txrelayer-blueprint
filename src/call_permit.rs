#![allow(clippy::too_many_arguments)]

pub use CallPermit::*;
use alloy::{
    primitives::{Address, address},
    sol,
};

sol! {
    #[sol(rpc)]
    "contracts/src/CallPermit.sol",
}

pub const CALL_PERMIT_ADDRESS: Address = address!("0000000000000000000000000000000000000805");

/// The instance of the CallPermit contract.
pub type Instance = CallPermitInstance<(), crate::ctx::AlloyProviderWithWallet>;
