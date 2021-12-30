#![allow(overflowing_literals)]
use crate::ffi::c_long;

pub type HRESULT = c_long;

pub const S_OK: HRESULT = 0;

pub const E_NOINTERFACE: HRESULT = 0x8000_4002;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000E;
pub const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110;
pub const E_FAIL: HRESULT = 0x8000_4005;
pub const COR_E_INVALIDPROGRAM: HRESULT = 0x8013_153A;
pub const COR_E_INVALIDOPERATION: HRESULT = 0x8013_1509;
pub const COR_E_INDEXOUTOFRANGE: HRESULT = 0x8;
