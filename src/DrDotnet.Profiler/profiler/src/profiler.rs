use profiling_api::{
    cil::{nop, Method},
    ffi::{CorOpenFlags, FunctionID, COR_PRF_MONITOR, E_FAIL, HRESULT, ObjectID},
    ClrProfiler, CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3,
    CorProfilerCallback4, CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7,
    CorProfilerCallback8, CorProfilerCallback9, CorProfilerInfo, MetadataImportTrait, ProfilerInfo,
};
use std::slice;
use uuid::Uuid;

#[derive(Clone)]
struct Profiler {
    clsid: Uuid,
    profiler_info: Option<ProfilerInfo>,
}

impl Profiler {
    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl ClrProfiler for Profiler {
    fn new() -> Profiler {
        Profiler {
            clsid: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E82").unwrap(),
            profiler_info: None,
        }
    }
    fn clsid(&self) -> &Uuid {
        &self.clsid
    }
}

impl CorProfilerCallback for Profiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), HRESULT> {
        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("initialize at start");

        // Set the event mask
        self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;

        Ok(())
    }

    fn jit_compilation_started(
        &mut self,
        function_id: FunctionID,
        _is_safe_to_block: bool,
    ) -> Result<(), HRESULT> {
        let function_info = self.profiler_info().get_function_info(function_id)?;
        let module_metadata = self
            .profiler_info()
            .get_module_metadata(function_info.module_id, CorOpenFlags::ofRead)?;
        let method_props = module_metadata.get_method_props(function_info.token)?;
        let il_body = self
            .profiler_info()
            .get_il_function_body(function_info.module_id, function_info.token)?;
        if method_props.name == "TMethod" || method_props.name == "FMethod" {
            // let bytes = unsafe {
            //     slice::from_raw_parts(il_body.method_header, il_body.method_size as usize)
            // };
            let mut method =
                Method::new(il_body.method_header, il_body.method_size).or(Err(E_FAIL))?;
            println!("{:#?}", method);
            let il = vec![nop()];
        }
        // 1. Modify method header
        // 2. Add a prologue
        // 3. Add an epilogue
        // 4. Modify SEH tables
        Ok(())
    }

    fn exception_thrown(&mut self, thrown_object_id: ObjectID) -> Result<(), HRESULT> {
        println!("exception_thrown");
        Ok(())
    }
}

impl CorProfilerCallback2 for Profiler {}

impl CorProfilerCallback3 for Profiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), HRESULT> {

        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("initialize with attach");

        // Set the event mask
        self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;

        Ok(())
    }
}

impl CorProfilerCallback4 for Profiler {}
impl CorProfilerCallback5 for Profiler {}
impl CorProfilerCallback6 for Profiler {}
impl CorProfilerCallback7 for Profiler {}
impl CorProfilerCallback8 for Profiler {}
impl CorProfilerCallback9 for Profiler {}

use profiling_api::ffi::{ClassFactory as FFIClassFactory, CorProfilerCallback as FFICorProfilerCallback, E_FAIL as FFI_E_FAIL, GUID as FFI_GUID, HRESULT as FFI_HRESULT, LPVOID as FFI_LPVOID, REFCLSID as FFI_REFCLSID, REFIID as FFI_REFIID};
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(
    rclsid: FFI_REFCLSID,
    riid: FFI_REFIID,
    ppv: *mut FFI_LPVOID,
) -> FFI_HRESULT {

    println!("DllGetClassObject");

    let profiler = Profiler::new();
    let clsid = FFI_GUID::from(*profiler.clsid());
    if ppv.is_null() || *rclsid != clsid {
        println!("CLSID didn't match. CLSID: {:?}", clsid);
        FFI_E_FAIL
    } else {
        let class_factory: &mut FFIClassFactory<Profiler> = FFIClassFactory::new(profiler);
        class_factory.QueryInterface(riid, ppv)
    }
}