fn main() {
    windows::build! {
        Windows::Win32::Foundation::{HINSTANCE, BOOL},
        Windows::Win32::System::SystemServices::{
            DLL_PROCESS_ATTACH,
            DLL_THREAD_ATTACH,
            DLL_THREAD_DETACH,
            DLL_PROCESS_DETACH,
        },
    }
}