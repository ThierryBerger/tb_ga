use libc::free;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(target_os = "ios")]
unsafe extern "C" {
    fn tb_ga_add_progression_event_p123_score(
        status: i32,
        p01: *const c_char,
        p02: *const c_char,
        p03: *const c_char,
        score: i32,
    );
    fn tb_ga_add_progression_event_p12_score(
        status: i32,
        p01: *const c_char,
        p02: *const c_char,
        score: i32,
    );
    fn tb_ga_add_progression_event_p12(status: i32, p01: *const c_char, p02: *const c_char);
}

/// Values correspond to `GAProgressionStatus`.
pub enum ProgressionStatus {
    Start = 1,
    Complete = 2,
    Fail = 3,
}

pub fn add_progression_event_p12_score(
    status: ProgressionStatus,
    p01: &str,
    p02: &str,
    score: i32,
) {
    #[cfg(target_os = "ios")]
    {
        let c_p01 = CString::new(p01).ok().unwrap();
        let c_p01 = c_p01.as_ptr();
        let c_p02 = CString::new(p02).ok().unwrap();
        let c_p02 = c_p02.as_ptr();
        unsafe {
            tb_ga_add_progression_event_p12_score(status as i32, c_p01, c_p02, score);
        }
    }
}
pub fn add_progression_event_p12(status: ProgressionStatus, p01: &str, p02: &str) {
    #[cfg(target_os = "ios")]
    {
        let c_p01 = CString::new(p01).ok().unwrap();
        let c_p01 = c_p01.as_ptr();
        let c_p02 = CString::new(p02).ok().unwrap();
        let c_p02 = c_p02.as_ptr();
        unsafe {
            tb_ga_add_progression_event_p12(status as i32, c_p01, c_p02);
        }
    }
}
