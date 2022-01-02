#![allow(non_snake_case)]
use crate::ffi::{IMethodMalloc, IUnknown, HRESULT, ULONG};

#[repr(C)]
pub struct MethodMallocVtbl {
    pub IUnknown: IUnknown<MethodMalloc>,
    pub IMethodMalloc: IMethodMalloc<MethodMalloc>,
}

#[repr(C)]
pub struct MethodMalloc {
    pub lpVtbl: *const MethodMallocVtbl,
}

impl MethodMalloc {
    pub unsafe fn i_method_malloc(&self) -> &IMethodMalloc<Self> {
        &(*self.lpVtbl).IMethodMalloc
    }
    pub unsafe fn Alloc(&self, cb: ULONG) -> HRESULT {
        (self.i_method_malloc().Alloc)(self, cb)
    }
}
