#![doc = include_str!("../README.md")]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub mod business;
pub mod design;
pub mod progression;

#[cfg(target_os = "ios")]
unsafe extern "C" {
    fn tb_ga_init();
}

/// Safe Rust wrapper: Calls Swift's greet, handles string conversion and free.
pub fn init() {
    #[cfg(target_os = "ios")]
    {
        unsafe {
            tb_ga_init();
        }
    }
}
