//! # Unix/Linux Platform Support Module
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  zfish — os/unix.rs                                            ║
//! ║  Unix/Linux-specific platform code (unsafe allowed here)      ║
//! ║  Copyright © 2025 Jeet Karena <karenajeet@proton.me>        ║
//! ║  Licensed under MIT OR Apache-2.0                             ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! This module contains Unix/Linux-specific unsafe code for terminal operations.
//! All unsafe blocks must have SAFETY comments explaining why they are sound.

use std::io;
use std::os::unix::io::AsRawFd;

/// Read a password with echo disabled on Unix/Linux
pub fn read_password() -> io::Result<String> {
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

    unsafe extern "C" {
        fn tcgetattr(fd: i32, termios: *mut Termios) -> i32;
        fn tcsetattr(fd: i32, optional_actions: i32, termios: *const Termios) -> i32;
    }

    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = std::mem::MaybeUninit::<Termios>::uninit();

    // SAFETY: tcgetattr is called with a valid file descriptor and a properly
    // allocated termios struct. The FFI call is checked for errors.
    unsafe {
        // Get the current terminal attributes
        if tcgetattr(stdin_fd, termios.as_mut_ptr()) != 0 {
            return Err(io::Error::last_os_error());
        }

        let mut termios = termios.assume_init();
        let original_lflag = termios.c_lflag;

        // Disable echo
        termios.c_lflag &= !(ECHO | ECHONL);

        // Set the new terminal attributes
        if tcsetattr(stdin_fd, TCSANOW, &termios) != 0 {
            return Err(io::Error::last_os_error());
        }

        // Ensure we restore the terminal attributes even if reading fails
        struct TermiosResetter {
            fd: i32,
            original_lflag: u32,
            termios: Termios,
        }

        impl Drop for TermiosResetter {
            fn drop(&mut self) {
                // SAFETY: Restoring original termios settings
                unsafe extern "C" {
                    fn tcsetattr(fd: i32, optional_actions: i32, termios: *const Termios) -> i32;
                }

                self.termios.c_lflag = self.original_lflag;
                unsafe {
                    // Restore original settings
                    const TCSANOW: i32 = 0;
                    tcsetattr(self.fd, TCSANOW, &self.termios);
                }
            }
        }

        let _resetter = TermiosResetter {
            fd: stdin_fd,
            original_lflag,
            termios,
        };

        // Read the password from stdin
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;

        // Remove trailing newline if present
        if password.ends_with('\n') {
            password.pop();
            if password.ends_with('\r') {
                password.pop();
            }
        }

        Ok(password)
    }
}

/// Get terminal size on Unix/Linux using ioctl
pub fn get_terminal_size() -> Option<(u16, u16)> {
    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    const TIOCGWINSZ: u64 = 0x5413;

    unsafe extern "C" {
        fn ioctl(fd: i32, request: u64, argp: *mut Winsize) -> i32;
    }

    // SAFETY: ioctl is called with a valid file descriptor (stdout),
    // a proper request code for getting window size, and a properly
    // allocated Winsize struct. The FFI call is checked for errors.
    unsafe {
        let mut ws: Winsize = std::mem::zeroed();
        let stdout_fd = io::stdout().as_raw_fd();

        if ioctl(stdout_fd, TIOCGWINSZ, &mut ws) == 0 {
            // Success - return (width, height)
            if ws.ws_col > 0 && ws.ws_row > 0 {
                return Some((ws.ws_col, ws.ws_row));
            }
        }

        // Fall back to default if ioctl fails
        None
    }
}
