use crate::api::ffi::*;
use crate::api::*;

pub trait ObjectReferencesCallbackReceiver {
    type AssociatedType: ObjectReferencesCallbackReceiver;

    fn callback(&mut self, referencer: ObjectID, reference: ObjectID);

    fn enum_references(&mut self, pinfo: ClrProfilerInfo, referencer: ObjectID) {
        let _ = pinfo.enumerate_object_references(referencer, Self::enum_references_callback, self as *mut Self as *mut std::ffi::c_void);
    }

    unsafe extern "system" fn enum_references_callback(referencer: ObjectID, reference: *const ObjectID, client_data: *mut libc::c_void) -> BOOL {
        let receiver_ptr = client_data as *mut Self::AssociatedType;
        let receiver = &mut *receiver_ptr;
        receiver.callback(referencer, *reference);
        return 1;
    }
}

pub unsafe extern "system" fn enum_references_callback(_root: ObjectID, reference: *const ObjectID, client_data: *mut libc::c_void) -> BOOL {
    let vec = &mut *client_data.cast::<Vec<ObjectID>>();
    vec.push(*reference);
    return 1;
}
