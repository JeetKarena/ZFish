//! # Windows Platform Support Module
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  Kite — os/windows.rs                                         ║
//! ║  Windows-specific platform code (unsafe allowed here)         ║
//! ║  Copyright © 2025 Jeet Karena <karenajeet@proton.me>        ║
//! ║  Licensed under MIT OR Apache-2.0                             ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! This module contains Windows-specific unsafe code for terminal operations.
//! All unsafe blocks must have SAFETY comments explaining why they are sound.

use std::io;
use std::ptr;

/// Read a password with echo disabled on Windows
pub fn read_password() -> io::Result<String> {
    // Windows API constants
    const STD_INPUT_HANDLE: u32 = 0xFFFFFFF6;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;
    
    // FFI signatures for Windows Console functions
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetStdHandle(nStdHandle: u32) -> *mut core::ffi::c_void;
        fn GetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, lpMode: *mut u32) -> i32;
        fn SetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, dwMode: u32) -> i32;
        fn ReadConsoleA(
            hConsoleInput: *mut core::ffi::c_void,
            lpBuffer: *mut u8,
            nNumberOfCharsToRead: u32,
            lpNumberOfCharsRead: *mut u32,
            pInputControl: *mut core::ffi::c_void,
        ) -> i32;
    }
    
    // SAFETY: All Windows API calls are properly checked for errors.
    // The console handle is valid for the process lifetime.
    // Buffer is properly sized and null-terminated.
    unsafe {
        // Get the console input handle
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }
        
        // Get the current console mode
        let mut original_mode: u32 = 0;
        if GetConsoleMode(handle, &mut original_mode) == 0 {
            return Err(io::Error::last_os_error());
        }
        
        // Set console mode to disable echo but keep line input and processing
        let new_mode = original_mode & !ENABLE_ECHO_INPUT;
        if SetConsoleMode(handle, new_mode) == 0 {
            return Err(io::Error::last_os_error());
        }
        
        // Ensure we restore the console mode even if reading fails
        struct ConsoleResetter {
            handle: *mut core::ffi::c_void,
            mode: u32,
        }
        
        impl Drop for ConsoleResetter {
            fn drop(&mut self) {
                // SAFETY: handle and mode are valid for the lifetime of this struct
                unsafe {
                    SetConsoleMode(self.handle, self.mode);
                }
            }
        }
        
        let _resetter = ConsoleResetter {
            handle,
            mode: original_mode,
        };
        
        // Read password (up to 1024 chars)
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut chars_read: u32 = 0;
        
        if ReadConsoleA(
            handle,
            buffer.as_mut_ptr(),
            (BUFFER_SIZE - 1) as u32,
            &mut chars_read,
            ptr::null_mut(),
        ) == 0 {
            return Err(io::Error::last_os_error());
        }
        
        // Convert the read bytes to a string
        let mut password = String::new();
        for &byte in buffer.iter().take(chars_read as usize) {
            // Skip CR/LF at the end
            if byte == b'\r' || byte == b'\n' {
                continue;
            }
            password.push(byte as char);
        }
        
        Ok(password)
    }
}
