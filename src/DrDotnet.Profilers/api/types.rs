use crate::ffi::*;

pub struct ArrayClassInfo {
    pub element_type: CorElementType,
    pub element_class_id: Option<ClassID>,
    pub rank: u32,
}
#[derive(Debug)]
pub struct ClassInfo {
    pub module_id: ModuleID,
    pub token: mdTypeDef,
}
#[derive(Debug)]
pub struct FunctionInfo {
    pub class_id: ClassID,
    pub module_id: ModuleID,
    pub token: mdMethodDef,
}
pub struct FunctionTokenAndMetadata {
    pub metadata_import: super::MetadataImport,
    pub token: mdMethodDef,
}
#[derive(Debug)]
pub struct ModuleInfo {
    pub base_load_address: LPCBYTE,
    pub file_name: String,
    pub assembly_id: AssemblyID,
}
pub struct IlFunctionBody {
    pub method_header: LPCBYTE,
    pub method_size: u32,
}
pub struct AppDomainInfo {
    pub name: String,
    pub process_id: ProcessID,
}
pub struct AssemblyInfo {
    pub name: String,
    pub app_domain_id: AppDomainID,
    pub module_id: ModuleID,
}
pub struct FunctionInfo2 {
    pub class_id: ClassID,
    pub module_id: ModuleID,
    pub token: mdMethodDef,
    pub type_args: Vec<ClassID>,
}
pub struct ClassLayout {
    pub field_offset: Vec<COR_FIELD_OFFSET>,
    pub class_size_bytes: u32,
}
#[derive(Debug)]
pub struct ClassInfo2 {
    pub module_id: ModuleID,
    pub token: mdTypeDef,
    pub parent_class_id: ClassID,
    pub type_args: Vec<ClassID>,
}
pub struct ArrayObjectInfo {
    pub dimension_sizes: Vec<u32>,
    pub dimension_lower_bounds: Vec<i32>,
    pub data: *mut BYTE, // TODO: This should be the raw buffer for the array, which is laid out according to the C++ convention.
}

pub struct StringLayout {
    pub string_length_offset: u32,
    pub buffer_offset: u32,
}

pub struct FunctionEnter3Info {
    pub frame_info: COR_PRF_FRAME_INFO,
    pub argument_info_length: u32,
    pub argument_info: COR_PRF_FUNCTION_ARGUMENT_INFO,
}

pub struct FunctionLeave3Info {
    pub frame_info: COR_PRF_FRAME_INFO,
    pub retval_range: COR_PRF_FUNCTION_ARGUMENT_RANGE,
}

pub struct RuntimeInfo {
    pub clr_instance_id: ClrInstanceID,
    pub runtime_type: COR_PRF_RUNTIME_TYPE,
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub qfe_version: u16,
    pub version_string: String,
}

pub struct ModuleInfo2 {
    pub base_load_address: LPCBYTE,
    pub file_name: String,
    pub assembly_id: AssemblyID,
    pub module_flags: COR_PRF_MODULE_FLAGS,
}

pub struct FunctionAndRejit {
    pub function_id: FunctionID,
    pub rejit_id: ReJITID,
}

pub struct EventMask2 {
    pub events_low: COR_PRF_MONITOR,
    pub events_high: COR_PRF_HIGH_MONITOR,
}

pub struct EnumNgenModuleMethodsInliningThisMethod<'a> {
    pub incomplete_data: bool,
    pub method_enum: &'a mut CorProfilerMethodEnum,
}

pub struct DynamicFunctionInfo {
    pub module_id: ModuleID,
    pub sig: PCCOR_SIGNATURE,
    pub sig_length: u32,
    pub name: String,
}

pub struct MethodProps {
    pub class_token: mdTypeDef,
    pub name: String,
    pub attr_flags: CorMethodAttr,
    pub sig: PCCOR_SIGNATURE,
    pub sig_length: u32,
    pub rva: u32,
    pub impl_flags: CorMethodImpl,
}

pub struct TypeProps {
    pub name: String,
    pub type_def_flags: CorTypeAttr,
    pub base_type: mdTypeDef,
}
