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
    /// A call was made at an unsupported time. Examples include illegally calling a profiling API method asynchronously, calling a method that might trigger a GC at an unsafe time, and calling a method at a time that could cause locks to be taken out of order.
    CORPROF_E_UNSUPPORTED_CALL_SEQUENCE = 0x8013_1363,
    /// The profiler's call into the CLR is disallowed because the profiler is attempting to detach.
    CORPROF_E_PROFILER_DETACHING = 0x8013_1367,
    // The profiler does not support attaching to a live process.
    CORPROF_E_PROFILER_NOT_ATTACHABLE = 0x8013_1368,
    /// The request to attach a profiler was denied because a profiler is already loaded.
    CORPROF_E_PROFILER_ALREADY_ACTIVE = 0x8013_136A,
}
