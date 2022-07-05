#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::ui::windows_and_messaging::*;


pub fn GetMessageA(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, ) -> (bool, Msg, ) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMessageA(_out_msg: *mut Msg, wnd: *const c_void, msg_filter_min: u32, msg_filter_max: u32, ) -> Bool;
		}
		let mut _out_msg: MaybeUninit<Msg> = MaybeUninit::uninit();
		let _ret_ = GetMessageA(_out_msg.as_mut_ptr(), transmute(wnd), msg_filter_min, msg_filter_max, );
		(_ret_.to_bool(), _out_msg.assume_init(), )
	}
}

pub fn TranslateMessage(msg: &Msg, ) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TranslateMessage(msg: &Msg, ) -> Bool;
		}
		let _ret_ = TranslateMessage(msg, );
		_ret_.to_bool()
	}
}

pub fn DispatchMessageA(msg: &Msg, ) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DispatchMessageA(msg: &Msg, ) -> LResult;
		}
		let _ret_ = DispatchMessageA(msg, );
		_ret_
	}
}

pub fn DefWindowProcA(wnd: HWnd, msg: WindowMessage, w_param: WParam, l_param: LParam, ) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefWindowProcA(wnd: HWnd, msg: WindowMessage, w_param: WParam, l_param: LParam, ) -> LResult;
		}
		let _ret_ = DefWindowProcA(wnd, msg, w_param, l_param, );
		_ret_
	}
}

pub fn PostQuitMessage(exit_code: i32, ) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PostQuitMessage(exit_code: i32, ) -> ();
		}
		let _ret_ = PostQuitMessage(exit_code, );
	}
}

pub fn RegisterClassExA(param0: &WndClassExA, ) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterClassExA(param0: &WndClassExA, ) -> u16;
		}
		let _ret_ = RegisterClassExA(param0, );
		_ret_
	}
}

pub fn CreateWindowExA(ex_style: WindowExStyle, class_name: Option<&str>, window_name: Option<&str>, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: Option<HWnd>, menu: Option<HMenu>, instance: Option<HInstance>, param: *const (), ) -> HWnd {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateWindowExA(ex_style: WindowExStyle, class_name: *const u8, window_name: *const u8, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: *const c_void, menu: *const c_void, instance: *const c_void, param: *const c_void, ) -> HWnd;
		}
		let _ret_ = CreateWindowExA(ex_style, class_name.map(str::to_null_terminated).as_ptr_or_null(), window_name.map(str::to_null_terminated).as_ptr_or_null(), style, x, y, width, height, transmute(wnd_parent), transmute(menu), transmute(instance), param as _, );
		_ret_
	}
}

pub fn ShowWindow(wnd: HWnd, cmd_show: ShowWindowCmd, ) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ShowWindow(wnd: HWnd, cmd_show: ShowWindowCmd, ) -> Bool;
		}
		let _ret_ = ShowWindow(wnd, cmd_show, );
		_ret_.to_bool()
	}
}

pub fn SetWindowTextA(wnd: HWnd, string: Option<&str>, ) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowTextA(wnd: HWnd, string: *const u8, ) -> Bool;
		}
		let _ret_ = SetWindowTextA(wnd, string.map(str::to_null_terminated).as_ptr_or_null(), );
		_ret_.to_bool()
	}
}

pub fn AdjustWindowRectEx(rect: &mut Rect, style: WindowStyle, menu: bool, ex_style: WindowExStyle, ) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn AdjustWindowRectEx(rect: &mut Rect, style: WindowStyle, menu: Bool, ex_style: WindowExStyle, ) -> Bool;
		}
		let _ret_ = AdjustWindowRectEx(rect, style, menu.to_bool(), ex_style, );
		_ret_.to_bool()
	}
}

pub fn GetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, ) -> NonNull<c_void> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, ) -> NonNull<c_void>;
		}
		let _ret_ = GetWindowLongPtrA(wnd, index, );
		_ret_
	}
}

pub fn SetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, new_long: NonNull<c_void>, ) -> NonNull<c_void> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, new_long: NonNull<c_void>, ) -> NonNull<c_void>;
		}
		let _ret_ = SetWindowLongPtrA(wnd, index, new_long, );
		_ret_
	}
}

pub fn LoadCursorA(instance: Option<HInstance>, cursor_name: &str, ) -> HCursor {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadCursorA(instance: *const c_void, cursor_name: *const u8, ) -> HCursor;
		}
		let _ret_ = LoadCursorA(transmute(instance), cursor_name.to_null_terminated().as_ptr_or_null(), );
		_ret_
	}
}


pub type WndProc 
	= unsafe extern "system" fn(param0: HWnd, param1: WindowMessage, param2: WParam, param3: LParam, ) -> LResult;

