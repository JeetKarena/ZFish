//! User input prompting utilities.

use std::io::{self, Write};

/// Utilities for prompting user input.
pub struct Prompt;

impl Prompt {
    /// Prompt for a yes/no confirmation.
    /// 
    /// Returns true if the user answered yes, false otherwise.
    pub fn confirm(prompt: &str, default: bool) -> io::Result<bool> {
        let yes_no = if default { "[Y/n]" } else { "[y/N]" };
        let full_prompt = format!("{} {} ", prompt, yes_no);
        
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(full_prompt.as_bytes())?;
        handle.flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim().to_lowercase();
        
        Ok(match input.as_str() {
            "y" | "yes" => true,
            "n" | "no" => false,
            "" => default,
            _ => false,
        })
    }
    
    /// Prompt for a line of text input.
    pub fn input(prompt: &str) -> io::Result<String> {
        let full_prompt = format!("{} ", prompt);
        
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(full_prompt.as_bytes())?;
        handle.flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }
    
    /// Prompt for a password with hidden input.
    /// 
    /// This uses platform-specific code to disable terminal echo.
    pub fn password(prompt: &str) -> io::Result<String> {
        let full_prompt = format!("{} ", prompt);
        
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(full_prompt.as_bytes())?;
        handle.flush()?;
        
        #[cfg(windows)]
        let password = Self::read_password_windows()?;
        
        #[cfg(unix)]
        let password = Self::read_password_unix()?;
        
        #[cfg(not(any(windows, unix)))]
        let password = Self::read_password_fallback()?;
        
        // Print a newline since we don't echo during password input
        println!();
        
        Ok(password)
    }
    
    /// Read a password on Windows systems with hidden input.
    #[cfg(windows)]
    fn read_password_windows() -> io::Result<String> {
        use std::ptr;
        
        // Windows API constants
        const STD_INPUT_HANDLE: u32 = 0xFFFFFFF6;
        const ENABLE_ECHO_INPUT: u32 = 0x0004;
        const ENABLE_LINE_INPUT: u32 = 0x0002;
        const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
        
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
        
        unsafe {
            // Get the console input handle
            let handle = GetStdHandle(STD_INPUT_HANDLE);
            if handle == ptr::null_mut() {
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
            for i in 0..chars_read as usize {
                // Skip CR/LF at the end
                if buffer[i] == b'\r' || buffer[i] == b'\n' {
                    continue;
                }
                password.push(buffer[i] as char);
            }
            
            Ok(password)
        }
    }
    
    /// Read a password on Unix systems with hidden input.
    #[cfg(unix)]
    fn read_password_unix() -> io::Result<String> {
        use std::os::unix::io::AsRawFd;
        
        // Define the termios structs and constants using raw FFI
        #[repr(C)]
        struct Termios {
            c_iflag: u32,
            c_oflag: u32,
            c_cflag: u32,
            c_lflag: u32,
            c_line: u8,
            c_cc: [u8; 32],
            c_ispeed: u32,
            c_ospeed: u32,
        }
        
        const ECHO: u32 = 0x00000008;
        const ECHONL: u32 = 0x00000040;
        const TCSANOW: i32 = 0;
        
        extern "C" {
            fn tcgetattr(fd: i32, termios: *mut Termios) -> i32;
            fn tcsetattr(fd: i32, optional_actions: i32, termios: *const Termios) -> i32;
        }
        
        let stdin_fd = io::stdin().as_raw_fd();
        let mut termios = std::mem::MaybeUninit::<Termios>::uninit();
        
        unsafe {
            // Get the current terminal attributes
            if tcgetattr(stdin_fd, termios.as_mut_ptr()) != 0 {
                return Err(io::Error::last_os_error());
            }
            
            let mut termios = termios.assume_init();
            let original_termios = termios;
            
            // Disable echo
            termios.c_lflag &= !ECHO;
            // But enable echo for newline (for better UX)
            termios.c_lflag |= ECHONL;
            
            // Set the new terminal attributes
            if tcsetattr(stdin_fd, TCSANOW, &termios) != 0 {
                return Err(io::Error::last_os_error());
            }
            
            // Ensure we restore terminal attributes even if reading fails
            struct TermiosResetter {
                fd: i32,
                termios: Termios,
            }
            
            impl Drop for TermiosResetter {
                fn drop(&mut self) {
                    unsafe {
                        tcsetattr(self.fd, TCSANOW, &self.termios);
                    }
                }
            }
            
            let _resetter = TermiosResetter {
                fd: stdin_fd,
                termios: original_termios,
            };
            
            // Read the password
            let mut password = String::new();
            if io::stdin().read_line(&mut password).is_err() {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to read password"));
            }
            
            // Trim the trailing newline
            if password.ends_with('\n') {
                password.pop();
            }
            if password.ends_with('\r') {
                password.pop();
            }
            
            Ok(password)
        }
    }
    
    /// Fallback method for platforms that don't support hidden input.
    #[cfg(not(any(windows, unix)))]
    fn read_password_fallback() -> io::Result<String> {
        // For platforms where we can't easily hide input, we use a character-by-character
        // reader that echoes asterisks instead of the typed characters
        Self::read_password_with_asterisks()
    }
    
    /// A fallback method that reads a password character by character,
    /// displaying asterisks for each character typed.
    fn read_password_with_asterisks() -> io::Result<String> {
        use std::io::Read;
        
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        
        let mut password = String::new();
        let mut buffer = [0; 1]; // Read one byte at a time
        
        loop {
            // Read a single character
            match stdin.read_exact(&mut buffer) {
                Ok(_) => {
                    match buffer[0] {
                        // Enter key
                        b'\r' | b'\n' => {
                            stdout.write_all(b"\n")?;
                            stdout.flush()?;
                            break;
                        },
                        // Backspace
                        8 | 127 => {
                            if !password.is_empty() {
                                password.pop();
                                stdout.write_all(b"\x08 \x08")?;
                                stdout.flush()?;
                            }
                        },
                        // Regular character
                        _ => {
                            password.push(buffer[0] as char);
                            stdout.write_all(b"*")?;
                            stdout.flush()?;
                        }
                    }
                },
                Err(_) => break,
            }
        }
        
        Ok(password)
    }
}