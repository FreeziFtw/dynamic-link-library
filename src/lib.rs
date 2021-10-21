windows::include_bindings!();

use Windows::Win32::Foundation::*;
use Windows::Win32::System::SystemServices::*;

use std::ffi::c_void;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(_module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => { }
        DLL_THREAD_ATTACH => { }
        DLL_THREAD_DETACH => { }
        DLL_PROCESS_DETACH => { }
        _ => { }
    }

    BOOL::from(true)
}