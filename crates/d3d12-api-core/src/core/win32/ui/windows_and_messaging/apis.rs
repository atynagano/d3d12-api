#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::ui::windows_and_messaging::*;

pub fn GetMessageA(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, ) -> (bool, Msg) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMessageA(_msg: &mut Msg, wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, ) -> Bool;
		}
		let mut _msg: Msg = Msg::zeroed();
		let ret = GetMessageA(&mut _msg, wnd, msg_filter_min, msg_filter_max, );
		return (ret.to_bool(), _msg);
	}
}

pub fn TranslateMessage(msg: &Msg, ) -> (bool) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TranslateMessage(msg: &Msg, ) -> Bool;
		}
		let ret = TranslateMessage(msg, );
		return (ret.to_bool());
	}
}

pub fn DispatchMessageA(msg: &Msg, ) -> (LResult) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DispatchMessageA(msg: &Msg, ) -> LResult;
		}
		let ret = DispatchMessageA(msg, );
		return (ret);
	}
}

pub fn PeekMessageA(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, remove_msg: PeekMessageRemoveType, ) -> (bool, Msg) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PeekMessageA(_msg: &mut Msg, wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, remove_msg: PeekMessageRemoveType, ) -> Bool;
		}
		let mut _msg: Msg = Msg::zeroed();
		let ret = PeekMessageA(&mut _msg, wnd, msg_filter_min, msg_filter_max, remove_msg, );
		return (ret.to_bool(), _msg);
	}
}

pub fn DefWindowProcA(wnd: HWnd, msg: WindowMessage, w_param: WParam, l_param: LParam, ) -> (LResult) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefWindowProcA(wnd: HWnd, msg: WindowMessage, w_param: WParam, l_param: LParam, ) -> LResult;
		}
		let ret = DefWindowProcA(wnd, msg, w_param, l_param, );
		return (ret);
	}
}

pub fn PostQuitMessage(exit_code: i32, ) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PostQuitMessage(exit_code: i32, ) -> ();
		}
		let ret = PostQuitMessage(exit_code, );
	}
}

pub fn RegisterClassExA(param0: &WndClassExA, ) -> (u16) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterClassExA(param0: &WndClassExA, ) -> u16;
		}
		let ret = RegisterClassExA(param0, );
		return (ret);
	}
}

pub fn CreateWindowExA(ex_style: WindowExStyle, class_name: Option<&str>, window_name: Option<&str>, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: Option<HWnd>, menu: Option<HMenu>, instance: Option<HInstance>, param: Option<*const c_void>, ) -> (HWnd) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateWindowExA(ex_style: WindowExStyle, class_name: *const u8, window_name: *const u8, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: Option<HWnd>, menu: Option<HMenu>, instance: Option<HInstance>, param: *const c_void, ) -> HWnd;
		}
		let ret = CreateWindowExA(ex_style, class_name.map(str::to_null_terminated).as_ptr_or_null(), window_name.map(str::to_null_terminated).as_ptr_or_null(), style, x, y, width, height, wnd_parent, menu, instance, param.as_ptr_or_null(), );
		return (ret);
	}
}

pub fn ShowWindow(wnd: HWnd, cmd_show: ShowWindowCmd, ) -> (bool) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ShowWindow(wnd: HWnd, cmd_show: ShowWindowCmd, ) -> Bool;
		}
		let ret = ShowWindow(wnd, cmd_show, );
		return (ret.to_bool());
	}
}

pub fn SetWindowTextA(wnd: HWnd, string: Option<&str>, ) -> (bool) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowTextA(wnd: HWnd, string: *const u8, ) -> Bool;
		}
		let ret = SetWindowTextA(wnd, string.map(str::to_null_terminated).as_ptr_or_null(), );
		return (ret.to_bool());
	}
}

pub fn AdjustWindowRectEx(rect: &mut Rect, style: WindowStyle, menu: bool, ex_style: WindowExStyle, ) -> (bool) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn AdjustWindowRectEx(rect: &mut Rect, style: WindowStyle, menu: Bool, ex_style: WindowExStyle, ) -> Bool;
		}
		let ret = AdjustWindowRectEx(rect, style, menu.to_bool(), ex_style, );
		return (ret.to_bool());
	}
}

pub fn GetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, ) -> (NonNull<c_void>) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, ) -> NonNull<c_void>;
		}
		let ret = GetWindowLongPtrA(wnd, index, );
		return (ret);
	}
}

pub fn SetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, new_long: NonNull<c_void>, ) -> (NonNull<c_void>) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, new_long: NonNull<c_void>, ) -> NonNull<c_void>;
		}
		let ret = SetWindowLongPtrA(wnd, index, new_long, );
		return (ret);
	}
}

pub fn LoadCursorA(instance: Option<HInstance>, cursor_name: &str, ) -> (HCursor) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadCursorA(instance: Option<HInstance>, cursor_name: *const u8, ) -> HCursor;
		}
		let ret = LoadCursorA(instance, cursor_name.to_null_terminated().as_ptr_or_null(), );
		return (ret);
	}
}


pub type WndProc 
	= unsafe extern "system" fn(param0: HWnd, param1: WindowMessage, param2: WParam, param3: LParam, ) -> LResult;

