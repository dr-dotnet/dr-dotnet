#![allow(overflowing_literals)]

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum HRESULT {
    S_OK = 0,
    E_NOINTERFACE = 0x8000_4002,
    E_OUTOFMEMORY = 0x8007_000E,
    CLASS_E_NOAGGREGATION = 0x8004_0110,
    CLASS_E_CLASSNOTAVAILABLE = 0x8004_0111,
    /// Unspecified error
    E_FAIL = 0x8000_4005,
    E_POINTER = 0x8000_4003,
    COR_E_INVALIDPROGRAM = 0x8013_153A,
    COR_E_INVALIDOPERATION = 0x8013_1509,
    COR_E_INDEXOUTOFRANGE = 0x8,
    /// The specified ClassID cannot be inspected by this function because it is an array
    CORPROF_E_CLASSID_IS_ARRAY = 0x8013_1365,
    /// A profiler tried to walk the stack of an invalid thread
    CORPROF_E_STACKSNAPSHOT_INVALID_TGT_THREAD = 0x8013_135E,
    /// A profiler can not walk a thread that is currently executing unmanaged code
    CORPROF_E_STACKSNAPSHOT_UNMANAGED_CTX = 0x8013_135F,
    /// A stackwalk at this point may cause dead locks or data corruption
    CORPROF_E_STACKSNAPSHOT_UNSAFE = 0x8013_1360,
    /// Stackwalking callback requested the walk to abort
    CORPROF_E_STACKSNAPSHOT_ABORTED = 0x8013_1361,
}