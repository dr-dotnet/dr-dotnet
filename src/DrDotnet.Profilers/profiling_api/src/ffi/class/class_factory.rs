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

        // info!("--- ClassFactory vtable ---");
        // info!("- {:?} QueryInterface", (*class_factory.lpVtbl).QueryInterface  as *mut c_void);
        // info!("- {:?} AddRef", Self::AddRef as *mut c_void);
        // info!("- {:?} Release", Self::Release as *mut c_void);
        // info!("- {:?} CreateInstance", Self::CreateInstance as *mut c_void);
        // info!("- {:?} LockServer", Self::LockServer as *mut c_void);

        Box::leak(Box::new(class_factory))
    }

    pub unsafe extern "system" fn QueryInterface(
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {

        info!("IClassFactory::QueryInterface");

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
        info!("IClassFactory::AddRef");
        // let ref_count = self.ref_count.fetch_add(1, Ordering::Relaxed) + 1;
        // 
        // println!("ClassFactory addref. Ref count: {}", ref_count);
        // 
        // ref_count
        1
    }

    pub unsafe extern "system" fn Release(&mut self) -> ULONG {
        info!("IClassFactory::Release");
        // let ref_count = self.ref_count.fetch_sub(1, Ordering::Relaxed) - 1;
        // 
        // println!("ClassFactory release. Ref count: {}", ref_count);
        // 
        // if ref_count == 0 {
        //     drop(Box::from_raw(self as *mut ClassFactory<T>));
        // }
        // 
        // ref_count
        1
    }

    pub unsafe extern "system" fn CreateInstance(&mut self, _pUnkOuter: *mut IUnknown<()>, _riid: REFIID, ppvObject: *mut LPVOID,
    //pub unsafe extern "system" fn CreateInstance(&mut self, _pUnkOuter: *mut IUnknown<()>, _riid: REFIID, ppvObject: *mut *mut c_void,
    //pub unsafe extern "system" fn CreateInstance(&mut self, _pUnkOuter: *mut IUnknown<()>, riid: ffi::REFIID, ppv: *mut ffi::LPVOID,
    ) -> HRESULT {
        let uuid: uuid::Uuid = GUID::into(*_riid);
        info!("IClassFactory::CreateInstance({})", uuid);
        // let b = CorProfilerCallback::new(T::default());
        // let hr = b.query_interface(_riid, ppvObject);
        // b.release();
        // hr
        
        info!("first arg={:?}", _pUnkOuter.is_null());
        info!("ptr before={:?}", ppvObject.is_null());
        *ppvObject = CorProfilerCallback::new(T::default()) as *mut CorProfilerCallback<T> as LPVOID;
        info!("ptr after={:?}", ppvObject.is_null());
        S_OK
    }

    pub extern "system" fn LockServer(&mut self, _fLock: BOOL) -> HRESULT {
        info!("IClassFactory::LockServer");
        S_OK
    }
}

unsafe impl<T> Sync for ClassFactory<T> where T: Sync, T: CorProfilerCallback9 {}
unsafe impl<T> Send for ClassFactory<T> where T: Send, T: CorProfilerCallback9 {}