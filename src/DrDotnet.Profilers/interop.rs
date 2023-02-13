#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate libc;
use std::ffi::CStr; 

use std::alloc::GlobalAlloc;
use std::alloc::System;
use std::alloc::Layout;

use crate::get_profiler_infos;
use crate::profilers::ProfilerData;

#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
}

mod rust_protobuf_protos {
    include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));
}

#[no_mangle]
pub extern "C" fn GetAvailableProfilers() -> Buffer
{
    let mut profilers_info_proto = rust_protobuf_protos::interop::ProfilersInfo::new();

    let profilers_info = get_profiler_infos();
    let len = profilers_info.len() as usize;

    for n in 0..len {
        let mut profiler_info_proto = rust_protobuf_protos::interop::ProfilerInfo::new();
        profiler_info_proto.name = profilers_info[n].name.to_owned();
        profiler_info_proto.description = profilers_info[n].description.to_owned();
        profiler_info_proto.uuid = profilers_info[n].profiler_id.to_string();
        profiler_info_proto.isReleased = profilers_info[n].is_released;

        profilers_info_proto.profilers.push(profiler_info_proto);
    }

    let bytes = protobuf::Message::write_to_bytes(&profilers_info_proto).unwrap();

    let mut buf = bytes.into_boxed_slice();
    let data = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);
    Buffer { data, len }
}

#[no_mangle]
pub extern "C" fn FreeBuffer(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}