use tasklist;
use winapi::um::winnt::SERVICE_TYPE_ALL;
use winapi::um::winsvc::*;
use winapi::um::errhandlingapi::GetLastError;
//use tasklist::TaskList;



//{SC_ENUM_PROCESS_INFO, EnumServicesStatusExA, SERVICE_ACTIVE, OpenSCManagerA, SC_MANAGER_ALL_ACCESS};
use std::ptr;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, LPDWORD};
use winapi::um::winuser::*;

// use winapi::um::winsvcctrl::{
//     CloseServiceHandle, EnumServicesStatusExA, OpenSCManagerA, OpenServiceA, SERVICE_ACTIVE,
//     SC_ENUM_PROCESS_INFO, SC_MANAGER_ALL_ACCESS, SERVICE_STATUS_PROCESSA, SERVICE_TYPE_ALL,
//     SERVICE_ALL_ACCESS, SERVICE_STATUS_PROCESS, SC_STATUS_PROCESS_INFO,
//     QueryServiceStatusEx,
// };





fn main() {
    println!("Hello, world!");
    println!("Listing all window services");
    //list_all_services();
    list_all_processes();
    
}
fn list_all_services_1(){
    // unsafe{
    //     let task_list = tasklist::TaskList::new();

    //     // Retrieve the list of services
    // let services = task_list.get_services();

    // // Iterate over the services and print their names
    // for service in services {
    //     println!("Service Name: {}", service.name);
    // }
    // }
}
fn list_all_processes() {
    unsafe{
        let tl = tasklist::Tasklist::new();
        for i in tl{
            println!("{} {} {}",i.get_pid(),i.get_pname(),i.get_user());
        }
    }
}
fn list_all_services(){
    unsafe{
        let manager = OpenSCManagerA(ptr::null_mut(), ptr::null_mut(), SC_MANAGER_ALL_ACCESS);

        if manager.is_null() {
            let error_code = GetLastError();
            println!("Failed to open Service Control Manager. Error code: {}", error_code);
            return;
        }

        if !manager.is_null() {
            let mut needed = 0;
            let mut services_count = 0;

            EnumServicesStatusExA(
                manager,
                SC_ENUM_PROCESS_INFO,
                SERVICE_TYPE_ALL,
                SERVICE_ACTIVE,
                ptr::null_mut(),
                0,
                &mut needed,
                &mut services_count,
                ptr::null_mut(),
                ptr::null_mut(),
                // ptr::null_mut(),
            );

            let mut buffer: Vec<u8> = vec![0; needed as usize];
            let mut services: Vec<ENUM_SERVICE_STATUS_PROCESSA> =
    vec![std::mem::zeroed::<ENUM_SERVICE_STATUS_PROCESSA>(); services_count as usize];


            EnumServicesStatusExA(
                manager,
                SC_ENUM_PROCESS_INFO,
                SERVICE_TYPE_ALL,
                SERVICE_ACTIVE,
                buffer.as_mut_ptr() as *mut u8,
                needed,
                &mut needed,
                &mut services_count,
                ptr::null_mut(),
                ptr::null_mut(),
                // ptr::null_mut(),
            );

            for service in services {
                let service_handle =
                    OpenServiceA(manager, service.lpServiceName, SERVICE_ALL_ACCESS);
                    //OpenServiceA(manager, service.lpServiceName.as_ptr(), SERVICE_ALL_ACCESS);


                if !service_handle.is_null() {
                    let mut service_status: SERVICE_STATUS_PROCESS = std::mem::zeroed(); //ENUM_SERVICE_STATUS_PROCESSA
                    //let mut service_status: ENUM_SERVICE_STATUS_PROCESSA = std::mem::zeroed(); //ENUM_SERVICE_STATUS_PROCESSA

                    let mut bytes_needed = 0;
                    let success = QueryServiceStatusEx(
                        service_handle,
                        SC_STATUS_PROCESS_INFO,
                        &mut service_status as *mut SERVICE_STATUS_PROCESS as *mut u8,
                        std::mem::size_of::<SERVICE_STATUS_PROCESS>() as DWORD,
                        &mut bytes_needed,
                    );

                    if success != 0 {
                        let name =
                            std::ffi::CStr::from_ptr(service.lpServiceName).to_str().unwrap();
                        let display_name =
                            std::ffi::CStr::from_ptr(service.lpDisplayName).to_str().unwrap();
                        let state = match service_status.dwCurrentState {
                            winapi::um::winsvc::SERVICE_RUNNING => "Running",
                            winapi::um::winsvc::SERVICE_STOPPED => "Stopped",
                            winapi::um::winsvc::SERVICE_PAUSED => "Paused",
                            _ => "Unknown",
                        };
                        println!(
                            "Name: {}, Display Name: {}, State: {}",
                            name, display_name, state
                        );
                    }

                    CloseServiceHandle(service_handle);
                }
            }

            CloseServiceHandle(manager);
        }
    }
}