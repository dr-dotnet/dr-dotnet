#![allow(non_snake_case)]
use crate::{
    ffi::{
        CorProfilerCallback, IClassFactory, IUnknown, BOOL, E_NOINTERFACE, HRESULT, LPVOID, REFIID,
        S_OK, ULONG,
    },
    traits::CorProfilerCallback9,
};
use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
pub struct ClassFactoryVtbl<T>
where
    T: CorProfilerCallback9 + Clone,
{
    pub IUnknown: IUnknown<ClassFactory<T>>,
    pub IClassFactory: IClassFactory<ClassFactory<T>>,
}

#[repr(C)]
pub struct ClassFactory<T>
where
    T: CorProfilerCallback9 + Clone,
{
    pub lpVtbl: *const ClassFactoryVtbl<T>,
    ref_count: AtomicU32,
    profiler: T,
}

impl<T> ClassFactory<T>
where
    T: CorProfilerCallback9 + Clone,
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
        if *riid == IUnknown::IID || *riid == IClassFactory::IID {
            *ppvObject = self as *mut ClassFactory<T> as LPVOID;
            self.AddRef();
            S_OK
        } else {
            *ppvObject = ptr::null_mut();
            E_NOINTERFACE
        }
    }

    pub unsafe extern "system" fn AddRef(&mut self) -> ULONG {
        // TODO: Which ordering is appropriate?
        let prev_ref_count = self.ref_count.fetch_add(1, Ordering::Relaxed);
        prev_ref_count + 1
    }

    pub unsafe extern "system" fn Release(&mut self) -> ULONG {
        // Ensure we are not trying to release the memory twice if
        // client calls release despite the ref_count being zero.
        // TODO: Which ordering is appropriate?
        if self.ref_count.load(Ordering::Relaxed) == 0 {
            panic!("Cannot release the COM object, it has already been released.");
        }

        let prev_ref_count = self.ref_count.fetch_sub(1, Ordering::Relaxed);
        let ref_count = prev_ref_count - 1;

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
        *ppvObject = CorProfilerCallback::new(self.profiler.clone()) as *mut CorProfilerCallback<T>
            as LPVOID;
        S_OK
    }

    pub extern "system" fn LockServer(&mut self, _fLock: BOOL) -> HRESULT {
        S_OK
    }
}
