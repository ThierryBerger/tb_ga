use libc::free;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(target_os = "ios")]
unsafe extern "C" {
    fn tb_ga_add_business_event(
        currency: *const c_char,
        amount: i32,
        item_id: *const c_char,
        item_type: *const c_char,
        cart_type: *const c_char,
    );
}

pub fn add_business_event(
    currency: &str,
    amount: i32,
    item_id: &str,
    item_type: &str,
    cart_type: &str,
) {
    #[cfg(target_os = "ios")]
    {
        let c_currency = CString::new(currency).ok().unwrap();
        let c_currency = c_currency.as_ptr();
        let c_item_id = CString::new(item_id).ok().unwrap();
        let c_item_id = c_item_id.as_ptr();
        let c_item_type = CString::new(item_type).ok().unwrap();
        let c_item_type = c_item_type.as_ptr();
        let c_cart_type = CString::new(cart_type).ok().unwrap();
        let c_cart_type = c_cart_type.as_ptr();

        unsafe {
            tb_ga_add_business_event(c_currency, amount, c_item_id, c_item_type, c_cart_type);
        }
    }
}
