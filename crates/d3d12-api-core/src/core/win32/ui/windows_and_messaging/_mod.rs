use std::ffi::c_void;
use std::mem::{MaybeUninit, transmute};
use std::ptr::null;
use crate::core::win32::ui::windows_and_messaging::{HCursor, LoadCursorA, Msg, PeekMessageRemoveType, WindowStyle};
use crate::core::win32::foundation::{Bool, HInstance, HWnd};
use crate::helpers::{AsPtrOrNull, ToNullTerminated};

pub fn LoadCursorAById(cursor_id: CursorId) -> (HCursor) {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn LoadCursorA(hInstance: *const c_void, lpCursorName: *const u8) -> HCursor;
        }
        LoadCursorA(null(), cursor_id as i32 as isize as *const u8)
    }
}

pub fn PeekMessageA(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, remove_msg: PeekMessageRemoveType) -> Option<Msg> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn PeekMessageA(msg: *mut Msg, wnd: *const c_void, msg_filter_min: u32, msg_filter_max: u32, remove_msg: PeekMessageRemoveType) -> Bool;
        }
        let mut msg: MaybeUninit<Msg> = MaybeUninit::uninit();
        let ret = PeekMessageA(msg.as_mut_ptr(), transmute(wnd), msg_filter_min, msg_filter_max, remove_msg);
        if ret.to_bool() { Some(msg.assume_init()) } else { None }
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CursorId {
    Arrow = 0x7F00,
    IBeam = 0x7F01,
    Wait = 0x7F02,
    Cross = 0x7F03,
    UpArrow = 0x7F04,
    Size = 0x7F80,
    Icon = 0x7F81,
    SizeNWSE = 0x7F82,
    SizeNESW = 0x7F83,
    SizeWE = 0x7F84,
    SizeNS = 0x7F85,
    SizeAll = 0x7F86,
    No = 0x7F88,
    Hand = 0x7F89,
    AppStarting = 0x7F8A,
    Help = 0x7F8B,
    Pin = 0x7F9F,
    Person = 0x7FA0,
}