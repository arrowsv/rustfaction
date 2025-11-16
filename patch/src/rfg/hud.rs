use crate::{addr_fn, addr_var, utils::address::addr};

#[allow(dead_code)]
pub fn show_secondary_message(text: &str, display_time: f32, use_secondary_anim: bool, force_redisplay: bool) {
    use widestring::U16CString;
    let c_text = U16CString::from_str(text).expect("Failed to convert to U16CString");
    ui_add_secondary_message()(c_text.as_ptr(), display_time, use_secondary_anim as u8, force_redisplay as u8);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GrState {
    value: u32,
    tnl_value: u32,
}

addr_fn!(
    addr(0x4D8270), 
    extern "C" fn(text: *const u16, display_time: f32, use_secondary_anim: u8, force_redisplay: u8) -> (), 
    ui_add_secondary_message
);
addr_fn!(
    addr(0x15EC80),
    extern "C" fn(x: i32, y: i32, text: *const std::ffi::c_char, font_num: i32, state: *const GrState) -> (),
    gr_string
);
addr_fn!(
    addr(0x1090C0),
    extern "C" fn(r: i32, g: i32, b: i32, a: i32) -> (),
    gr_set_color
);

addr_var!(addr(0x1109648), GrState, gm_filter);
addr_var!(addr(0x123E170), i32, credits_body_font);