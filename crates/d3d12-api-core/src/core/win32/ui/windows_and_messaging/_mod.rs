use std::ffi::c_void;
use std::mem::{MaybeUninit, transmute};
use std::num::{NonZeroU16, NonZeroUsize};
use std::ptr::null;
use crate::core::win32::foundation::{Bool, GetLastError, HInstance, HResult, HWnd, Rect, SetLastError, ToBool, Win32Error};
use crate::core::win32::graphics::gdi::ColorRef;
use crate::helpers::{AsPtrOrNull, ToNullTerminated};
use super::*;

pub fn GetClientRect(wnd: HWnd) -> Result<Rect, Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn GetClientRect(wnd: HWnd, _out_rect: *mut Rect) -> Bool;
        }
        let mut _out_rect: MaybeUninit<Rect> = MaybeUninit::zeroed();
        let _ret_ = GetClientRect(wnd, _out_rect.as_mut_ptr());
        if _ret_.to_bool() { Ok(_out_rect.assume_init()) } else { Err(GetLastError()) }
    }
}

pub fn GetWindowRect(wnd: HWnd) -> Result<Rect, Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn GetWindowRect(wnd: HWnd, _out_rect: *mut Rect) -> Bool;
        }
        let mut _out_rect: MaybeUninit<Rect> = MaybeUninit::zeroed();
        let _ret_ = GetWindowRect(wnd, _out_rect.as_mut_ptr());
        if _ret_.to_bool() { Ok(_out_rect.assume_init()) } else { Err(GetLastError()) }
    }
}

pub fn LoadCursorAById(cursor_id: CursorId) -> HCursor {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn LoadCursorA(hInstance: *const c_void, lpCursorName: *const u8) -> *const c_void;
        }
        let ret = LoadCursorA(null(), cursor_id as i32 as isize as *const u8);
        transmute(ret)
    }
}

pub fn GetMessagePos() -> crate::core::win32::foundation::Point {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn GetMessagePos() -> u32;
        }
        let _ret_ = GetMessagePos();
        let x = (_ret_ as u16 & 0xFFFFu16) as i32;
        let y = (((_ret_ >> 16) as u16) & 0xFFFFu16) as i32;
        crate::core::win32::foundation::Point { x, y }
    }
}

pub fn GetMessageTime() -> u32 {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn GetMessageTime() -> u32;
        }
        let _ret_ = GetMessageTime();
        _ret_
    }
}

pub enum WindowMsg {
    Msg(Msg),
    Quit(Msg),
}

pub fn GetMessageA(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32) -> Result<WindowMsg, Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn GetMessageA(_out_msg: *mut c_void, wnd: *const c_void, msg_filter_min: u32, msg_filter_max: u32) -> i32;
        }
        let mut _out_msg: MaybeUninit<Msg> = MaybeUninit::zeroed();
        let _ret_ = GetMessageA(transmute(_out_msg.as_mut_ptr()), transmute(wnd), msg_filter_min, msg_filter_max);
        match _ret_ {
            -1 => Err(GetLastError()),
            0 => Ok(WindowMsg::Quit(_out_msg.assume_init())),
            _ => Ok(WindowMsg::Msg(_out_msg.assume_init())),
        }
    }
}

pub fn PeekMessageA(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, remove_msg: PeekMessageRemoveType) -> Option<WindowMsg> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn PeekMessageA(msg: *mut c_void, wnd: *const c_void, msg_filter_min: u32, msg_filter_max: u32, remove_msg: PeekMessageRemoveType) -> i32;
        }
        let mut _out_msg: MaybeUninit<Msg> = MaybeUninit::uninit();
        let _ret_ = PeekMessageA(_out_msg.as_mut_ptr() as _, transmute(wnd), msg_filter_min, msg_filter_max, remove_msg);
        match _ret_ {
            0 => None,
            _ => {
                let msg = _out_msg.assume_init();
                if msg.message == WindowMessage::Quit {
                    Some(WindowMsg::Quit(msg))
                } else {
                    Some(WindowMsg::Msg(msg))
                }
            }
        }
    }
}

pub fn AdjustWindowRect(rect: &mut Rect, style: WindowStyle, menu: bool) -> Result<(), Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn AdjustWindowRect(rect: &mut Rect, style: WindowStyle, menu: Bool) -> Bool;
        }
        let _ret_ = AdjustWindowRect(rect, style, menu.to_bool());
        match _ret_.to_bool() {
            true => Ok(()),
            false => Err(GetLastError()),
        }
    }
}

pub fn AdjustWindowRectEx(rect: &mut Rect, style: WindowStyle, menu: bool, ex_style: WindowExStyle) -> Result<(), Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn AdjustWindowRectEx(rect: &mut Rect, style: WindowStyle, menu: Bool, ex_style: WindowExStyle) -> Bool;
        }
        let _ret_ = AdjustWindowRectEx(rect, style, menu.to_bool(), ex_style);
        match _ret_.to_bool() {
            true => Ok(()),
            false => Err(GetLastError()),
        }
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

pub struct WindowLongPtrExStyle(pub WindowExStyle);

pub struct WindowLongPtrHInstance(pub Option<HInstance>);

pub struct WindowLongPtrHWndParent(pub Option<HWnd>);

// todo: ?
pub struct WindowLongPtrId();

pub struct WindowLongPtrStyle(pub WindowStyle);

pub struct WindowLongPtrUserData(pub Option<NonZeroUsize>);

pub struct WindowLongPtrWndProc(pub WndProc);

pub trait WindowLongPtrType {
    fn new(v: usize) -> Self;
    const INDEX: WindowLongPtrIndex;
}

impl WindowLongPtrType for WindowLongPtrExStyle {
    fn new(v: usize) -> Self { unsafe { transmute(v as u32) } }
    const INDEX: WindowLongPtrIndex = WindowLongPtrIndex::ExStyle;
}

impl WindowLongPtrType for WindowLongPtrHInstance {
    fn new(v: usize) -> Self { unsafe { transmute(v) } }
    const INDEX: WindowLongPtrIndex = WindowLongPtrIndex::HInstance;
}

impl WindowLongPtrType for WindowLongPtrHWndParent {
    fn new(v: usize) -> Self { unsafe { transmute(v) } }
    const INDEX: WindowLongPtrIndex = WindowLongPtrIndex::HWndParent;
}

impl WindowLongPtrType for WindowLongPtrStyle {
    fn new(v: usize) -> Self { unsafe { transmute(v as u32) } }
    const INDEX: WindowLongPtrIndex = WindowLongPtrIndex::Style;
}

impl WindowLongPtrType for WindowLongPtrUserData {
    fn new(v: usize) -> Self { unsafe { transmute(v) } }
    const INDEX: WindowLongPtrIndex = WindowLongPtrIndex::UserData;
}

impl WindowLongPtrType for WindowLongPtrWndProc {
    fn new(v: usize) -> Self { unsafe { transmute(v) } }
    const INDEX: WindowLongPtrIndex = WindowLongPtrIndex::WndProc;
}

pub fn GetWindowLongPtrA<T: WindowLongPtrType>(wnd: HWnd) -> Result<T, Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn GetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex) -> usize;
        }
        SetLastError(Win32Error::NoError);
        let _ret_ = GetWindowLongPtrA(wnd, T::INDEX);
        match _ret_ {
            0 => match GetLastError() {
                Win32Error::NoError => Ok(T::new(0)),
                err @ _ => Err(err),
            }
            v @ _ => Ok(T::new(v))
        }
    }
}

pub fn GetLayeredWindowAttributes(hwnd: HWnd, pcr_key: Option<&mut ColorRef>, pb_alpha: Option<&mut u8>, pdw_flags: Option<&mut LayeredWindowAttributesFlags>) -> Result<(), Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn GetLayeredWindowAttributes(hwnd: HWnd, pcr_key: Option<&mut ColorRef>, pb_alpha: Option<&mut u8>, pdw_flags: Option<&mut LayeredWindowAttributesFlags>) -> Bool;
        }
        let _ret_ = GetLayeredWindowAttributes(hwnd, pcr_key, pb_alpha, pdw_flags);
        match _ret_.to_bool() {
            false => Err(GetLastError()),
            true => Ok(())
        }
    }
}

pub fn RegisterClassExA(param0: &WndClassExA) -> Result<NonZeroU16, Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn RegisterClassExA(param0: *const c_void) -> u16;
        }
        let _ret_ = RegisterClassExA(transmute(param0));
        match NonZeroU16::new(_ret_) {
            None => Err(GetLastError()),
            Some(v) => Ok(v),
        }
    }
}

pub fn RegisterClassExW(param0: &WndClassExW) -> Result<NonZeroU16, Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn RegisterClassExW(param0: &WndClassExW) -> u16;
        }
        let _ret_ = RegisterClassExW(param0);
        match NonZeroU16::new(_ret_) {
            None => Err(GetLastError()),
            Some(v) => Ok(v),
        }
    }
}