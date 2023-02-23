#![allow(overflowing_literals)]

use std::fmt::{Display, Formatter};
use crate::ffi::c_long;

pub type HRESULT = c_long;

pub const S_OK: HRESULT = 0;

pub const E_NOINTERFACE: HRESULT = 0x8000_4002;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000E;
pub const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110;
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111;
/// Unspecified error
pub const E_FAIL: HRESULT = 0x8000_4005;
pub const E_POINTER: HRESULT = 0x8000_4003;
pub const COR_E_INVALIDPROGRAM: HRESULT = 0x8013_153A;
pub const COR_E_INVALIDOPERATION: HRESULT = 0x8013_1509;
pub const COR_E_INDEXOUTOFRANGE: HRESULT = 0x8;
/// The specified ClassID cannot be inspected by this function because it is an array
pub const CORPROF_E_CLASSID_IS_ARRAY: HRESULT = 0x80131365;
/// A profiler tried to walk the stack of an invalid thread
pub const CORPROF_E_STACKSNAPSHOT_INVALID_TGT_THREAD: HRESULT = 0x8013_135E;
/// A profiler can not walk a thread that is currently executing unmanaged code
pub const CORPROF_E_STACKSNAPSHOT_UNMANAGED_CTX: HRESULT = 0x8013_135F;
/// A stackwalk at this point may cause dead locks or data corruption
pub const CORPROF_E_STACKSNAPSHOT_UNSAFE: HRESULT = 0x8013_1360;
/// Stackwalking callback requested the walk to abort
pub const CORPROF_E_STACKSNAPSHOT_ABORTED: HRESULT = 0x8013_1361;

pub struct HResult {
    pub value: HRESULT
}

impl Display for HResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value { 
            E_NOINTERFACE => write!(f, "{}", stringify!(E_NOINTERFACE)),
            E_OUTOFMEMORY => write!(f, "{}", stringify!(E_OUTOFMEMORY)),
            CLASS_E_NOAGGREGATION => write!(f, "{}", stringify!(CLASS_E_NOAGGREGATION)),
            CLASS_E_CLASSNOTAVAILABLE => write!(f, "{}", stringify!(CLASS_E_CLASSNOTAVAILABLE)),
            E_FAIL => write!(f, "{}", stringify!(E_FAIL)),
            E_POINTER => write!(f, "{}", stringify!(E_POINTER)),
            COR_E_INVALIDPROGRAM => write!(f, "{}", stringify!(COR_E_INVALIDPROGRAM)),
            COR_E_INVALIDOPERATION => write!(f, "{}", stringify!(COR_E_INVALIDOPERATION)),
            COR_E_INDEXOUTOFRANGE => write!(f, "{}", stringify!(COR_E_INDEXOUTOFRANGE)),
            CORPROF_E_CLASSID_IS_ARRAY => write!(f, "{}", stringify!(CORPROF_E_CLASSID_IS_ARRAY)),
            CORPROF_E_STACKSNAPSHOT_INVALID_TGT_THREAD => write!(f, "{}", stringify!(CORPROF_E_STACKSNAPSHOT_INVALID_TGT_THREAD)),
            CORPROF_E_STACKSNAPSHOT_UNMANAGED_CTX => write!(f, "{}", stringify!(CORPROF_E_STACKSNAPSHOT_UNMANAGED_CTX)),
            CORPROF_E_STACKSNAPSHOT_UNSAFE => write!(f, "{}", stringify!(CORPROF_E_STACKSNAPSHOT_UNSAFE)),
            CORPROF_E_STACKSNAPSHOT_ABORTED => write!(f, "{}", stringify!(CORPROF_E_STACKSNAPSHOT_ABORTED)),
            _ => Ok(())
        }
    }
}
