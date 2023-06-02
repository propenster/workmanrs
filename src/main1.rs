use std::ptr;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, LPDWORD};
use winapi::um::winsvc::{
    EnumServicesStatusExA, CloseServiceHandle, EnumServicesStatusProcessA, OpenSCManagerA,
    OpenServiceA, SERVICE_ACTIVE, SC_ENUM_PROCESS_INFO, SC_MANAGER_ALL_ACCESS, SERVICE_STATUS_PROCESS,
    SERVICE_STATUS_PROCESSA, SERVICE_TYPE_ALL,
};

fn main() {
    unsafe {
        
    }
}
