//! Big number types

#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::manual_range_contains)]

use crate::error::SwapError;
use std::convert::TryInto;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

impl U256 {
    /// Convert u256 to u64
    pub fn to_u64(val: U256) -> Result<u64, SwapError> {
        val.try_into().map_err(|_| SwapError::ConversionFailure)
    }
    /// Convert u256 to u128
    pub fn to_u128(val: U256) -> Result<u128, SwapError> {
        val.try_into().map_err(|_| SwapError::ConversionFailure)
    }
}
