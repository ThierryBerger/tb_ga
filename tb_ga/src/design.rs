use libc::free;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(target_os = "ios")]
unsafe extern "C" {
    fn tb_ga_add_design_event_with_value(event_id: *const c_char, value: f32);
    fn tb_ga_add_design_event(event_id: *const c_char);
}

pub fn add_design_event_with_value(event_id: &str, value: f32) {
    #[cfg(target_os = "ios")]
    {
        let c_event_id = CString::new(event_id).ok().unwrap();
        let c_event_id = c_event_id.as_ptr();
        unsafe {
            tb_ga_add_design_event_with_value(c_event_id, value);
        }
    }
}

pub fn add_design_event(event_id: &str) {
    #[cfg(target_os = "ios")]
    {
        let c_event_id = CString::new(event_id).ok().unwrap();
        let c_event_id = c_event_id.as_ptr();
        unsafe {
            tb_ga_add_design_event(c_event_id);
        }
    }
}
