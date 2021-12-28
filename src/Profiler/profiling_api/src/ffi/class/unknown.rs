#![allow(non_snake_case)]
use crate::ffi::{IUnknown, HRESULT, REFIID, ULONG};
use std::ffi::c_void;

#[repr(C)]
pub struct UnknownVtbl {
    pub IUnknown: IUnknown<Unknown>,
}

#[repr(C)]
pub struct Unknown {
    pub lpVtbl: *const UnknownVtbl,
}

impl Unknown {
    pub unsafe fn i_unknown(&self) -> &IUnknown<Self> {
        &(*self.lpVtbl).IUnknown
    }
    pub unsafe fn QueryInterface(&mut self, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
        (self.i_unknown().QueryInterface)(self, riid, ppvObject)
    }
    pub unsafe fn AddRef(&mut self) -> ULONG {
        (self.i_unknown().AddRef)(self)
    }
    pub unsafe fn Release(&mut self) -> ULONG {
        (self.i_unknown().Release)(self)
    }
}
