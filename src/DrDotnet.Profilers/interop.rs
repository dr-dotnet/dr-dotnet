#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_must_use)]

extern crate libc;
use std::ffi::CStr;

use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;

use crate::get_profiler_infos;
use crate::rust_protobuf_protos::interop::*;

#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
pub extern "C" fn GetAvailableProfilers() -> Buffer {
    let mut profilers_info: ProfilersInfo = ProfilersInfo::new();
    profilers_info.profilers = get_profiler_infos().to_vec();

    let bytes = protobuf::Message::write_to_bytes(&profilers_info).unwrap();

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
