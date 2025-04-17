use objc2_core_foundation::{
    CFMutableDictionary, CFString, HRESULT, LPVOID, REFIID, ULONG,
};
use std::{os::raw::c_void, ptr::null};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct MDImporterInterfaceStruct {
    _reserved: *mut c_void,
    query_interface:
        Option<unsafe extern "C-unwind" fn(*mut c_void, REFIID, *mut LPVOID) -> HRESULT>,
    add_ref: Option<unsafe extern "C-unwind" fn(*mut c_void) -> ULONG>,
    release: Option<unsafe extern "C-unwind" fn(*mut c_void) -> ULONG>,
    importer_import_data:
        Option<unsafe extern "C-unwind" fn(*mut c_void, CFMutableDictionary, CFString, CFString)>,
}

#[unsafe(no_mangle)]
pub extern "C" fn MetadataImporterPluginFactory() -> *mut c_void {
    println!("CFPlugin loaded!");
    let a: *const c_void = null();
    let b: *mut c_void = a.cast_mut();
    b
}
