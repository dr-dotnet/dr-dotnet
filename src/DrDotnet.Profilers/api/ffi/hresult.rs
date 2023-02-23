#![allow(overflowing_literals)]
use crate::ffi::c_long;

pub type HRESULT = c_long;

pub const S_OK: HRESULT = 0;

pub const E_NOINTERFACE: HRESULT = 0x8000_4002;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000E;
pub const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110;
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111;
pub const E_FAIL: HRESULT = 0x8000_4005;
pub const E_POINTER: HRESULT = 0x8000_4003;
pub const COR_E_INVALIDPROGRAM: HRESULT = 0x8013_153A;
pub const COR_E_INVALIDOPERATION: HRESULT = 0x8013_1509;
pub const COR_E_INDEXOUTOFRANGE: HRESULT = 0x8;
/// The specified ClassID cannot be inspected by this function because it is an array
pub const CORPROF_E_CLASSID_IS_ARRAY: HRESULT = 0x80131365;
/// A profiler can not walk a thread that is currently executing unmanaged code
pub const CORPROF_E_STACKSNAPSHOT_UNMANAGED_CTX: HRESULT = 0x8013135F;
/// A stackwalk at this point may cause dead locks or data corruption
pub const CORPROF_E_STACKSNAPSHOT_UNSAFE: HRESULT = 0x80131360;
