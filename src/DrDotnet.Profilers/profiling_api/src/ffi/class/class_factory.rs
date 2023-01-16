#![allow(non_snake_case)]
use crate::{
    ffi::*,
    traits::CorProfilerCallback9,
};
use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
pub struct ClassFactoryVtbl<T>
where
    T: CorProfilerCallback9,
{
    pub IUnknown: IUnknown<ClassFactory<T>>,
    pub IClassFactory: IClassFactory<ClassFactory<T>>,
}

#[repr(C)]
pub struct ClassFactory<T>
where
    T: CorProfilerCallback9,
{
    pub lpVtbl: *const ClassFactoryVtbl<T>,
    ref_count: AtomicU32,
    profiler: T,
}

impl<T> ClassFactory<T>
where
    T: CorProfilerCallback9,
{
    pub fn new<'b>(profiler: T) -> &'b mut ClassFactory<T> {
        let class_factory = ClassFactory {
            lpVtbl: &ClassFactoryVtbl {
                IUnknown: IUnknown {
                    QueryInterface: Self::QueryInterface,
                    AddRef: Self::AddRef,
                    Release: Self::Release,
                },
                IClassFactory: IClassFactory {
                    CreateInstance: Self::CreateInstance,
                    LockServer: Self::LockServer,
                },
            },
            ref_count: AtomicU32::new(0),
            profiler,
        };
        Box::leak(Box::new(class_factory))
    }

    pub unsafe extern "system" fn QueryInterface(
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {

        if ppvObject.is_null() {
            return E_POINTER;
        }

        if *riid == IUnknown::IID || *riid == IClassFactory::IID {
            *ppvObject = self as *mut ClassFactory<T> as LPVOID;
            self.AddRef();
            S_OK
        } else {
            //*ppvObject = ptr::null_mut();
            E_NOINTERFACE
        }
    }

    pub unsafe extern "system" fn AddRef(&mut self) -> ULONG {
        let ref_count = self.ref_count.fetch_add(1, Ordering::Relaxed) + 1;

        println!("ClassFactory addref. Ref count: {}", ref_count);
        
        ref_count
    }

    pub unsafe extern "system" fn Release(&mut self) -> ULONG {
        let ref_count = self.ref_count.fetch_sub(1, Ordering::Relaxed) - 1;

        println!("ClassFactory release. Ref count: {}", ref_count);
        
        if ref_count == 0 {
            drop(Box::from_raw(self as *mut ClassFactory<T>));
        }

        ref_count
    }

    pub unsafe extern "system" fn CreateInstance(
        &mut self,
        _pUnkOuter: *mut IUnknown<()>,
        _riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        let b = CorProfilerCallback::new(T::default());
        let hr = b.query_interface(_riid, ppvObject);
        b.release();
        hr
        
        // *ppvObject = CorProfilerCallback::new(T::default()) as *mut CorProfilerCallback<T> as LPVOID;
        // S_OK
    }

    pub extern "system" fn LockServer(&mut self, _fLock: BOOL) -> HRESULT {
        S_OK
    }
}
