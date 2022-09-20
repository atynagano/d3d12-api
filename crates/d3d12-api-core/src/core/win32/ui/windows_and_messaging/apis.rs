#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::num::NonZeroUsize;
use std::mem::{MaybeUninit, size_of_val, transmute};
use std::ops::Deref;
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::ui::windows_and_messaging::*;
use crate::core::win32::graphics::gdi::*;
use crate::core::win32::ui::shell::*;


pub unsafe fn LoadStringA() { todo!() }

pub unsafe fn LoadStringW() { todo!() }

pub unsafe fn WvsprintfA() { todo!() }

pub unsafe fn WvsprintfW() { todo!() }

pub unsafe fn WsprintfA() { todo!() }

pub unsafe fn WsprintfW() { todo!() }

pub fn IsHungAppWindow(hwnd: HWnd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsHungAppWindow(hwnd: HWnd) -> Bool;
		} 
		let _ret_ = IsHungAppWindow(hwnd);
		_ret_.to_bool()
	}
}

pub fn DisableProcessWindowsGhosting() -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DisableProcessWindowsGhosting() -> ();
		} 
		let _ret_ = DisableProcessWindowsGhosting();
	}
}

pub fn RegisterWindowMessageA(string: &str) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterWindowMessageA(lpString: *const u8) -> u32;
		} 
		let _ret_ = RegisterWindowMessageA(string.to_null_terminated().as_ptr_or_null());
		_ret_
	}
}

pub fn RegisterWindowMessageW(string: &str) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterWindowMessageW(lpString: *const u16) -> u32;
		} 
		let _ret_ = RegisterWindowMessageW(string.to_utf16().as_ptr_or_null());
		_ret_
	}
}

pub fn GetMessageW(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32) -> Result<Msg, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMessageW(lpMsg: *mut c_void, hWnd: *const c_void, wMsgFilterMin: u32, wMsgFilterMax: u32) -> Bool;
		} 
		let mut msg_out_: MaybeUninit<Msg> = MaybeUninit::zeroed();
		let _ret_ = GetMessageW(transmute(msg_out_.as_mut_ptr()), transmute(wnd), msg_filter_min, msg_filter_max);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(msg_out_.assume_init()) }
	}
}

pub fn TranslateMessage(msg: &Msg) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TranslateMessage(lpMsg: *const c_void) -> Bool;
		} 
		let _ret_ = TranslateMessage(transmute(msg));
		_ret_.to_bool()
	}
}

pub fn DispatchMessageA(msg: &Msg) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DispatchMessageA(lpMsg: *const c_void) -> LResult;
		} 
		let _ret_ = DispatchMessageA(transmute(msg));
		_ret_
	}
}

pub fn DispatchMessageW(msg: &Msg) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DispatchMessageW(lpMsg: *const c_void) -> LResult;
		} 
		let _ret_ = DispatchMessageW(transmute(msg));
		_ret_
	}
}

pub fn SetMessageQueue(messages_max: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMessageQueue(cMessagesMax: i32) -> Bool;
		} 
		let _ret_ = SetMessageQueue(messages_max);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PeekMessageW(wnd: Option<HWnd>, msg_filter_min: u32, msg_filter_max: u32, remove_msg: PeekMessageRemoveType) -> Result<Msg, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PeekMessageW(lpMsg: *mut c_void, hWnd: *const c_void, wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMsg: PeekMessageRemoveType) -> Bool;
		} 
		let mut msg_out_: MaybeUninit<Msg> = MaybeUninit::zeroed();
		let _ret_ = PeekMessageW(transmute(msg_out_.as_mut_ptr()), transmute(wnd), msg_filter_min, msg_filter_max, remove_msg);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(msg_out_.assume_init()) }
	}
}

pub fn GetMessageExtraInfo() -> LParam {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMessageExtraInfo() -> LParam;
		} 
		let _ret_ = GetMessageExtraInfo();
		_ret_
	}
}

pub fn IsWow64Message() -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsWow64Message() -> Bool;
		} 
		let _ret_ = IsWow64Message();
		_ret_.to_bool()
	}
}

pub fn SetMessageExtraInfo(l_param: LParam) -> LParam {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMessageExtraInfo(lParam: LParam) -> LParam;
		} 
		let _ret_ = SetMessageExtraInfo(l_param);
		_ret_
	}
}

pub fn SendMessageA(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageA(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = SendMessageA(wnd, msg, w_param, l_param);
		_ret_
	}
}

pub fn SendMessageW(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageW(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = SendMessageW(wnd, msg, w_param, l_param);
		_ret_
	}
}

pub fn SendMessageTimeoutA(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam, fu_flags: SendMessageTimeoutFlags, u_timeout: u32, lpdw_result: Option<&mut MaybeUninit<usize>>) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageTimeoutA(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam, fuFlags: SendMessageTimeoutFlags, uTimeout: u32, lpdwResult: Option<&mut MaybeUninit<usize>>) -> LResult;
		} 
		let _ret_ = SendMessageTimeoutA(wnd, msg, w_param, l_param, fu_flags, u_timeout, lpdw_result);
		_ret_
	}
}

pub fn send_message_timeout_a(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam, fu_flags: SendMessageTimeoutFlags, u_timeout: u32) -> (LResult, usize) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageTimeoutA(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam, fuFlags: SendMessageTimeoutFlags, uTimeout: u32, lpdwResult: *mut usize) -> LResult;
		} 
		let mut lpdw_result_out_: MaybeUninit<usize> = MaybeUninit::zeroed();
		let _ret_ = SendMessageTimeoutA(wnd, msg, w_param, l_param, fu_flags, u_timeout, lpdw_result_out_.as_mut_ptr());
		(_ret_, lpdw_result_out_.assume_init())
	}
}

pub fn SendMessageTimeoutW(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam, fu_flags: SendMessageTimeoutFlags, u_timeout: u32, lpdw_result: Option<&mut MaybeUninit<usize>>) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageTimeoutW(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam, fuFlags: SendMessageTimeoutFlags, uTimeout: u32, lpdwResult: Option<&mut MaybeUninit<usize>>) -> LResult;
		} 
		let _ret_ = SendMessageTimeoutW(wnd, msg, w_param, l_param, fu_flags, u_timeout, lpdw_result);
		_ret_
	}
}

pub fn send_message_timeout_w(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam, fu_flags: SendMessageTimeoutFlags, u_timeout: u32) -> (LResult, usize) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageTimeoutW(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam, fuFlags: SendMessageTimeoutFlags, uTimeout: u32, lpdwResult: *mut usize) -> LResult;
		} 
		let mut lpdw_result_out_: MaybeUninit<usize> = MaybeUninit::zeroed();
		let _ret_ = SendMessageTimeoutW(wnd, msg, w_param, l_param, fu_flags, u_timeout, lpdw_result_out_.as_mut_ptr());
		(_ret_, lpdw_result_out_.assume_init())
	}
}

pub fn SendNotifyMessageA(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendNotifyMessageA(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> Bool;
		} 
		let _ret_ = SendNotifyMessageA(wnd, msg, w_param, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SendNotifyMessageW(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendNotifyMessageW(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> Bool;
		} 
		let _ret_ = SendNotifyMessageW(wnd, msg, w_param, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SendMessageCallbackA(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam, result_call_back: SendAsyncProc, data: usize) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageCallbackA(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam, lpResultCallBack: SendAsyncProc, dwData: usize) -> Bool;
		} 
		let _ret_ = SendMessageCallbackA(wnd, msg, w_param, l_param, result_call_back, data);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SendMessageCallbackW(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam, result_call_back: SendAsyncProc, data: usize) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendMessageCallbackW(hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam, lpResultCallBack: SendAsyncProc, dwData: usize) -> Bool;
		} 
		let _ret_ = SendMessageCallbackW(wnd, msg, w_param, l_param, result_call_back, data);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PostMessageA(wnd: Option<HWnd>, msg: u32, w_param: WParam, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PostMessageA(hWnd: *const c_void, Msg: u32, wParam: WParam, lParam: LParam) -> Bool;
		} 
		let _ret_ = PostMessageA(transmute(wnd), msg, w_param, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PostMessageW(wnd: Option<HWnd>, msg: u32, w_param: WParam, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PostMessageW(hWnd: *const c_void, Msg: u32, wParam: WParam, lParam: LParam) -> Bool;
		} 
		let _ret_ = PostMessageW(transmute(wnd), msg, w_param, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PostThreadMessageA(id_thread: u32, msg: u32, w_param: WParam, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PostThreadMessageA(idThread: u32, Msg: u32, wParam: WParam, lParam: LParam) -> Bool;
		} 
		let _ret_ = PostThreadMessageA(id_thread, msg, w_param, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PostThreadMessageW(id_thread: u32, msg: u32, w_param: WParam, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PostThreadMessageW(idThread: u32, Msg: u32, wParam: WParam, lParam: LParam) -> Bool;
		} 
		let _ret_ = PostThreadMessageW(id_thread, msg, w_param, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ReplyMessage(result: LResult) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ReplyMessage(lResult: LResult) -> Bool;
		} 
		let _ret_ = ReplyMessage(result);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn WaitMessage() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn WaitMessage() -> Bool;
		} 
		let _ret_ = WaitMessage();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DefWindowProcA(wnd: HWnd, msg: WindowMessage, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefWindowProcA(hWnd: HWnd, Msg: WindowMessage, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefWindowProcA(wnd, msg, w_param, l_param);
		_ret_
	}
}

pub fn DefWindowProcW(wnd: HWnd, msg: WindowMessage, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefWindowProcW(hWnd: HWnd, Msg: WindowMessage, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefWindowProcW(wnd, msg, w_param, l_param);
		_ret_
	}
}

pub fn PostQuitMessage(exit_code: i32) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PostQuitMessage(nExitCode: i32) -> ();
		} 
		let _ret_ = PostQuitMessage(exit_code);
	}
}

pub fn CallWindowProcA(prev_wnd_func: WndProc, wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CallWindowProcA(lpPrevWndFunc: WndProc, hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = CallWindowProcA(prev_wnd_func, wnd, msg, w_param, l_param);
		_ret_
	}
}

pub fn CallWindowProcW(prev_wnd_func: WndProc, wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CallWindowProcW(lpPrevWndFunc: WndProc, hWnd: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = CallWindowProcW(prev_wnd_func, wnd, msg, w_param, l_param);
		_ret_
	}
}

pub fn InSendMessage() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InSendMessage() -> Bool;
		} 
		let _ret_ = InSendMessage();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn InSendMessageEx(reserved: &mut ()) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InSendMessageEx(lpReserved: &mut ()) -> u32;
		} 
		let _ret_ = InSendMessageEx(reserved);
		_ret_
	}
}

pub fn RegisterClassA(wnd_class: &WndClassA) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterClassA(lpWndClass: &WndClassA) -> u16;
		} 
		let _ret_ = RegisterClassA(wnd_class);
		_ret_
	}
}

pub fn RegisterClassW(wnd_class: &WndClassW) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterClassW(lpWndClass: &WndClassW) -> u16;
		} 
		let _ret_ = RegisterClassW(wnd_class);
		_ret_
	}
}

pub fn UnregisterClassA(class_name: &str, instance: Option<HInstance>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UnregisterClassA(lpClassName: *const u8, hInstance: *const c_void) -> Bool;
		} 
		let _ret_ = UnregisterClassA(class_name.to_null_terminated().as_ptr_or_null(), transmute(instance));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn UnregisterClassW(class_name: &str, instance: Option<HInstance>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UnregisterClassW(lpClassName: *const u16, hInstance: *const c_void) -> Bool;
		} 
		let _ret_ = UnregisterClassW(class_name.to_utf16().as_ptr_or_null(), transmute(instance));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetClassInfoA(instance: Option<HInstance>, class_name: &str) -> Result<WndClassA, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassInfoA(hInstance: *const c_void, lpClassName: *const u8, lpWndClass: *mut WndClassA) -> Bool;
		} 
		let mut wnd_class_out_: MaybeUninit<WndClassA> = MaybeUninit::zeroed();
		let _ret_ = GetClassInfoA(transmute(instance), class_name.to_null_terminated().as_ptr_or_null(), wnd_class_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(wnd_class_out_.assume_init()) }
	}
}

pub fn GetClassInfoW(instance: Option<HInstance>, class_name: &str) -> Result<WndClassW, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassInfoW(hInstance: *const c_void, lpClassName: *const u16, lpWndClass: *mut WndClassW) -> Bool;
		} 
		let mut wnd_class_out_: MaybeUninit<WndClassW> = MaybeUninit::zeroed();
		let _ret_ = GetClassInfoW(transmute(instance), class_name.to_utf16().as_ptr_or_null(), wnd_class_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(wnd_class_out_.assume_init()) }
	}
}

pub fn GetClassInfoExA(instance: Option<HInstance>, class: &str) -> Result<WndClassExA, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassInfoExA(hInstance: *const c_void, lpszClass: *const u8, lpwcx: *mut c_void) -> Bool;
		} 
		let mut lpwcx_out_: MaybeUninit<WndClassExA> = MaybeUninit::zeroed();
		let _ret_ = GetClassInfoExA(transmute(instance), class.to_null_terminated().as_ptr_or_null(), transmute(lpwcx_out_.as_mut_ptr()));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpwcx_out_.assume_init()) }
	}
}

pub fn GetClassInfoExW(instance: Option<HInstance>, class: &str) -> Result<WndClassExW, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassInfoExW(hInstance: *const c_void, lpszClass: *const u16, lpwcx: *mut WndClassExW) -> Bool;
		} 
		let mut lpwcx_out_: MaybeUninit<WndClassExW> = MaybeUninit::zeroed();
		let _ret_ = GetClassInfoExW(transmute(instance), class.to_utf16().as_ptr_or_null(), lpwcx_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpwcx_out_.assume_init()) }
	}
}

pub fn CreateWindowExA(ex_style: WindowExStyle, class_name: Option<&str>, window_name: Option<&str>, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: Option<HWnd>, menu: Option<HMenu>, instance: Option<HInstance>, param: *const ()) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateWindowExA(dwExStyle: WindowExStyle, lpClassName: *const u8, lpWindowName: *const u8, dwStyle: WindowStyle, X: i32, Y: i32, nWidth: i32, nHeight: i32, hWndParent: *const c_void, hMenu: *const c_void, hInstance: *const c_void, lpParam: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateWindowExA(ex_style, class_name.map(str::to_null_terminated).as_ptr_or_null(), window_name.map(str::to_null_terminated).as_ptr_or_null(), style, x, y, width, height, transmute(wnd_parent), transmute(menu), transmute(instance), param as _);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateWindowExW(ex_style: WindowExStyle, class_name: Option<&str>, window_name: Option<&str>, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: Option<HWnd>, menu: Option<HMenu>, instance: Option<HInstance>, param: *const ()) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateWindowExW(dwExStyle: WindowExStyle, lpClassName: *const u16, lpWindowName: *const u16, dwStyle: WindowStyle, X: i32, Y: i32, nWidth: i32, nHeight: i32, hWndParent: *const c_void, hMenu: *const c_void, hInstance: *const c_void, lpParam: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateWindowExW(ex_style, class_name.map(str::to_utf16).as_ptr_or_null(), window_name.map(str::to_utf16).as_ptr_or_null(), style, x, y, width, height, transmute(wnd_parent), transmute(menu), transmute(instance), param as _);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn IsWindow(wnd: Option<HWnd>) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsWindow(hWnd: *const c_void) -> Bool;
		} 
		let _ret_ = IsWindow(transmute(wnd));
		_ret_.to_bool()
	}
}

pub fn IsMenu(menu: HMenu) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsMenu(hMenu: HMenu) -> Bool;
		} 
		let _ret_ = IsMenu(menu);
		_ret_.to_bool()
	}
}

pub fn IsChild(wnd_parent: HWnd, wnd: HWnd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsChild(hWndParent: HWnd, hWnd: HWnd) -> Bool;
		} 
		let _ret_ = IsChild(wnd_parent, wnd);
		_ret_.to_bool()
	}
}

pub fn DestroyWindow(wnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DestroyWindow(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = DestroyWindow(wnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ShowWindow(wnd: HWnd, cmd_show: ShowWindowCmd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ShowWindow(hWnd: HWnd, nCmdShow: ShowWindowCmd) -> Bool;
		} 
		let _ret_ = ShowWindow(wnd, cmd_show);
		_ret_.to_bool()
	}
}

pub fn AnimateWindow(wnd: HWnd, time: u32, flags: AnimateWindowFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn AnimateWindow(hWnd: HWnd, dwTime: u32, dwFlags: AnimateWindowFlags) -> Bool;
		} 
		let _ret_ = AnimateWindow(wnd, time, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn UpdateLayeredWindow(wnd: HWnd, hdc_dst: Option<HDc>, ppt_dst: Option<&Point>, psize: Option<&Size>, hdc_src: Option<HDc>, ppt_src: Option<&Point>, cr_key: u32, pblend: Option<&BlendFunction>, flags: UpdateLayeredWindowFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UpdateLayeredWindow(hWnd: HWnd, hdcDst: *const c_void, pptDst: *const c_void, psize: *const c_void, hdcSrc: *const c_void, pptSrc: *const c_void, crKey: u32, pblend: *const c_void, dwFlags: UpdateLayeredWindowFlags) -> Bool;
		} 
		let _ret_ = UpdateLayeredWindow(wnd, transmute(hdc_dst), transmute(ppt_dst), transmute(psize), transmute(hdc_src), transmute(ppt_src), cr_key, transmute(pblend), flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn UpdateLayeredWindowIndirect(wnd: HWnd, ulw_info: &UpdateLayeredWindowInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UpdateLayeredWindowIndirect(hWnd: HWnd, pULWInfo: &UpdateLayeredWindowInfo) -> Bool;
		} 
		let _ret_ = UpdateLayeredWindowIndirect(wnd, ulw_info);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetLayeredWindowAttributes(hwnd: HWnd, pcr_key: Option<&mut MaybeUninit<u32>>, pb_alpha: Option<&mut MaybeUninit<u8>>, pdw_flags: Option<&mut MaybeUninit<LayeredWindowAttributesFlags>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetLayeredWindowAttributes(hwnd: HWnd, pcrKey: Option<&mut MaybeUninit<u32>>, pbAlpha: Option<&mut MaybeUninit<u8>>, pdwFlags: Option<&mut MaybeUninit<LayeredWindowAttributesFlags>>) -> Bool;
		} 
		let _ret_ = GetLayeredWindowAttributes(hwnd, pcr_key, pb_alpha, pdw_flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetLayeredWindowAttributes(hwnd: HWnd, cr_key: ColorRef, alpha: u8, flags: LayeredWindowAttributesFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetLayeredWindowAttributes(hwnd: HWnd, crKey: ColorRef, bAlpha: u8, dwFlags: LayeredWindowAttributesFlags) -> Bool;
		} 
		let _ret_ = SetLayeredWindowAttributes(hwnd, cr_key, alpha, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ShowWindowAsync(wnd: HWnd, cmd_show: ShowWindowCmd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ShowWindowAsync(hWnd: HWnd, nCmdShow: ShowWindowCmd) -> Bool;
		} 
		let _ret_ = ShowWindowAsync(wnd, cmd_show);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FlashWindow(wnd: HWnd, invert: bool) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn FlashWindow(hWnd: HWnd, bInvert: Bool) -> Bool;
		} 
		let _ret_ = FlashWindow(wnd, invert.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FlashWindowEx(pfwi: &FlashWInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn FlashWindowEx(pfwi: &FlashWInfo) -> Bool;
		} 
		let _ret_ = FlashWindowEx(pfwi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ShowOwnedPopups(wnd: HWnd, f_show: bool) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ShowOwnedPopups(hWnd: HWnd, fShow: Bool) -> Bool;
		} 
		let _ret_ = ShowOwnedPopups(wnd, f_show.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn OpenIcon(wnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn OpenIcon(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = OpenIcon(wnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CloseWindow(wnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CloseWindow(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = CloseWindow(wnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn MoveWindow(wnd: HWnd, x: i32, y: i32, width: i32, height: i32, repaint: bool) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MoveWindow(hWnd: HWnd, X: i32, Y: i32, nWidth: i32, nHeight: i32, bRepaint: Bool) -> Bool;
		} 
		let _ret_ = MoveWindow(wnd, x, y, width, height, repaint.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetWindowPos(wnd: HWnd, wnd_insert_after: Option<HWnd>, x: i32, y: i32, cx: i32, cy: i32, u_flags: SetWindowPosFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowPos(hWnd: HWnd, hWndInsertAfter: *const c_void, X: i32, Y: i32, cx: i32, cy: i32, uFlags: SetWindowPosFlags) -> Bool;
		} 
		let _ret_ = SetWindowPos(wnd, transmute(wnd_insert_after), x, y, cx, cy, u_flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetWindowPlacement(wnd: HWnd, lpwndpl: &mut WindowPlacement) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowPlacement(hWnd: HWnd, lpwndpl: &mut WindowPlacement) -> Bool;
		} 
		let _ret_ = GetWindowPlacement(wnd, lpwndpl);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetWindowPlacement(wnd: HWnd, lpwndpl: &WindowPlacement) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowPlacement(hWnd: HWnd, lpwndpl: &WindowPlacement) -> Bool;
		} 
		let _ret_ = SetWindowPlacement(wnd, lpwndpl);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetWindowDisplayAffinity(wnd: HWnd) -> Result<u32, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowDisplayAffinity(hWnd: HWnd, pdwAffinity: *mut u32) -> Bool;
		} 
		let mut pdw_affinity_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let _ret_ = GetWindowDisplayAffinity(wnd, pdw_affinity_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(pdw_affinity_out_.assume_init()) }
	}
}

pub fn SetWindowDisplayAffinity(wnd: HWnd, affinity: WindowDisplayAffinity) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowDisplayAffinity(hWnd: HWnd, dwAffinity: WindowDisplayAffinity) -> Bool;
		} 
		let _ret_ = SetWindowDisplayAffinity(wnd, affinity);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn BeginDeferWindowPos(num_windows: i32) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn BeginDeferWindowPos(nNumWindows: i32) -> Option<NonNull<()>>;
		} 
		let _ret_ = BeginDeferWindowPos(num_windows);
		_ret_
	}
}

pub fn DeferWindowPos(win_pos_info: NonZeroUsize, wnd: HWnd, wnd_insert_after: Option<HWnd>, x: i32, y: i32, cx: i32, cy: i32, u_flags: SetWindowPosFlags) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DeferWindowPos(hWinPosInfo: NonZeroUsize, hWnd: HWnd, hWndInsertAfter: *const c_void, x: i32, y: i32, cx: i32, cy: i32, uFlags: SetWindowPosFlags) -> Option<NonNull<()>>;
		} 
		let _ret_ = DeferWindowPos(win_pos_info, wnd, transmute(wnd_insert_after), x, y, cx, cy, u_flags);
		_ret_
	}
}

pub fn EndDeferWindowPos(win_pos_info: NonZeroUsize) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EndDeferWindowPos(hWinPosInfo: NonZeroUsize) -> Bool;
		} 
		let _ret_ = EndDeferWindowPos(win_pos_info);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn IsWindowVisible(wnd: HWnd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsWindowVisible(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = IsWindowVisible(wnd);
		_ret_.to_bool()
	}
}

pub fn IsIconic(wnd: HWnd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsIconic(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = IsIconic(wnd);
		_ret_.to_bool()
	}
}

pub fn AnyPopup() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn AnyPopup() -> Bool;
		} 
		let _ret_ = AnyPopup();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn BringWindowToTop(wnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn BringWindowToTop(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = BringWindowToTop(wnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn IsZoomed(wnd: HWnd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsZoomed(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = IsZoomed(wnd);
		_ret_.to_bool()
	}
}

pub fn CreateDialogParamA(instance: Option<HInstance>, template_name: &str, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateDialogParamA(hInstance: *const c_void, lpTemplateName: *const u8, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> *const c_void;
		} 
		let _ret_ = CreateDialogParamA(transmute(instance), template_name.to_null_terminated().as_ptr_or_null(), transmute(wnd_parent), transmute(dialog_func), init_param);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDialogParamW(instance: Option<HInstance>, template_name: &str, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateDialogParamW(hInstance: *const c_void, lpTemplateName: *const u16, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> *const c_void;
		} 
		let _ret_ = CreateDialogParamW(transmute(instance), template_name.to_utf16().as_ptr_or_null(), transmute(wnd_parent), transmute(dialog_func), init_param);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDialogIndirectParamA(instance: Option<HInstance>, template: &DlgTemplate, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateDialogIndirectParamA(hInstance: *const c_void, lpTemplate: &DlgTemplate, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> *const c_void;
		} 
		let _ret_ = CreateDialogIndirectParamA(transmute(instance), template, transmute(wnd_parent), transmute(dialog_func), init_param);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDialogIndirectParamW(instance: Option<HInstance>, template: &DlgTemplate, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateDialogIndirectParamW(hInstance: *const c_void, lpTemplate: &DlgTemplate, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> *const c_void;
		} 
		let _ret_ = CreateDialogIndirectParamW(transmute(instance), template, transmute(wnd_parent), transmute(dialog_func), init_param);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DialogBoxParamA(instance: Option<HInstance>, template_name: &str, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DialogBoxParamA(hInstance: *const c_void, lpTemplateName: *const u8, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> Option<NonNull<()>>;
		} 
		let _ret_ = DialogBoxParamA(transmute(instance), template_name.to_null_terminated().as_ptr_or_null(), transmute(wnd_parent), transmute(dialog_func), init_param);
		_ret_
	}
}

pub fn DialogBoxParamW(instance: Option<HInstance>, template_name: &str, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DialogBoxParamW(hInstance: *const c_void, lpTemplateName: *const u16, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> Option<NonNull<()>>;
		} 
		let _ret_ = DialogBoxParamW(transmute(instance), template_name.to_utf16().as_ptr_or_null(), transmute(wnd_parent), transmute(dialog_func), init_param);
		_ret_
	}
}

pub fn DialogBoxIndirectParamA(instance: Option<HInstance>, dialog_template: &DlgTemplate, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DialogBoxIndirectParamA(hInstance: *const c_void, hDialogTemplate: &DlgTemplate, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> Option<NonNull<()>>;
		} 
		let _ret_ = DialogBoxIndirectParamA(transmute(instance), dialog_template, transmute(wnd_parent), transmute(dialog_func), init_param);
		_ret_
	}
}

pub fn DialogBoxIndirectParamW(instance: Option<HInstance>, dialog_template: &DlgTemplate, wnd_parent: Option<HWnd>, dialog_func: Option<DlgProc>, init_param: LParam) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DialogBoxIndirectParamW(hInstance: *const c_void, hDialogTemplate: &DlgTemplate, hWndParent: *const c_void, lpDialogFunc: *const c_void, dwInitParam: LParam) -> Option<NonNull<()>>;
		} 
		let _ret_ = DialogBoxIndirectParamW(transmute(instance), dialog_template, transmute(wnd_parent), transmute(dialog_func), init_param);
		_ret_
	}
}

pub fn EndDialog(dlg: HWnd, result: NonZeroUsize) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EndDialog(hDlg: HWnd, nResult: NonZeroUsize) -> Bool;
		} 
		let _ret_ = EndDialog(dlg, result);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetDlgItem(dlg: Option<HWnd>, id_dlg_item: i32) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDlgItem(hDlg: *const c_void, nIDDlgItem: i32) -> *const c_void;
		} 
		let _ret_ = GetDlgItem(transmute(dlg), id_dlg_item);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetDlgItemInt(dlg: HWnd, id_dlg_item: i32, u_value: u32, signed: bool) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetDlgItemInt(hDlg: HWnd, nIDDlgItem: i32, uValue: u32, bSigned: Bool) -> Bool;
		} 
		let _ret_ = SetDlgItemInt(dlg, id_dlg_item, u_value, signed.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetDlgItemInt(dlg: HWnd, id_dlg_item: i32, translated: Option<&mut MaybeUninit<Bool>>, signed: bool) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDlgItemInt(hDlg: HWnd, nIDDlgItem: i32, lpTranslated: Option<&mut MaybeUninit<Bool>>, bSigned: Bool) -> u32;
		} 
		let _ret_ = GetDlgItemInt(dlg, id_dlg_item, translated, signed.to_bool());
		_ret_
	}
}

pub fn get_dlg_item_int(dlg: HWnd, id_dlg_item: i32, signed: bool) -> (u32, bool) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDlgItemInt(hDlg: HWnd, nIDDlgItem: i32, lpTranslated: &mut Bool, bSigned: Bool) -> u32;
		} 
		let mut translated_out_ = Bool::FALSE;
		let _ret_ = GetDlgItemInt(dlg, id_dlg_item, &mut translated_out_, signed.to_bool());
		(_ret_, translated_out_.to_bool())
	}
}

pub fn SetDlgItemTextA(dlg: HWnd, id_dlg_item: i32, string: &str) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetDlgItemTextA(hDlg: HWnd, nIDDlgItem: i32, lpString: *const u8) -> Bool;
		} 
		let _ret_ = SetDlgItemTextA(dlg, id_dlg_item, string.to_null_terminated().as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetDlgItemTextW(dlg: HWnd, id_dlg_item: i32, string: &str) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetDlgItemTextW(hDlg: HWnd, nIDDlgItem: i32, lpString: *const u16) -> Bool;
		} 
		let _ret_ = SetDlgItemTextW(dlg, id_dlg_item, string.to_utf16().as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn GetDlgItemTextA() { todo!() }

pub unsafe fn GetDlgItemTextW() { todo!() }

pub fn SendDlgItemMessageA(dlg: HWnd, id_dlg_item: i32, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendDlgItemMessageA(hDlg: HWnd, nIDDlgItem: i32, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = SendDlgItemMessageA(dlg, id_dlg_item, msg, w_param, l_param);
		_ret_
	}
}

pub fn SendDlgItemMessageW(dlg: HWnd, id_dlg_item: i32, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendDlgItemMessageW(hDlg: HWnd, nIDDlgItem: i32, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = SendDlgItemMessageW(dlg, id_dlg_item, msg, w_param, l_param);
		_ret_
	}
}

pub fn GetNextDlgGroupItem(dlg: HWnd, ctl: Option<HWnd>, previous: bool) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetNextDlgGroupItem(hDlg: HWnd, hCtl: *const c_void, bPrevious: Bool) -> *const c_void;
		} 
		let _ret_ = GetNextDlgGroupItem(dlg, transmute(ctl), previous.to_bool());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetNextDlgTabItem(dlg: HWnd, ctl: Option<HWnd>, previous: bool) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetNextDlgTabItem(hDlg: HWnd, hCtl: *const c_void, bPrevious: Bool) -> *const c_void;
		} 
		let _ret_ = GetNextDlgTabItem(dlg, transmute(ctl), previous.to_bool());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetDlgCtrlID(wnd: HWnd) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDlgCtrlID(hWnd: HWnd) -> i32;
		} 
		let _ret_ = GetDlgCtrlID(wnd);
		_ret_
	}
}

pub fn GetDialogBaseUnits() -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDialogBaseUnits() -> i32;
		} 
		let _ret_ = GetDialogBaseUnits();
		_ret_
	}
}

pub fn DefDlgProcA(dlg: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefDlgProcA(hDlg: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefDlgProcA(dlg, msg, w_param, l_param);
		_ret_
	}
}

pub fn DefDlgProcW(dlg: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefDlgProcW(hDlg: HWnd, Msg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefDlgProcW(dlg, msg, w_param, l_param);
		_ret_
	}
}

pub fn CallMsgFilterA(msg: &Msg, code: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CallMsgFilterA(lpMsg: *const c_void, nCode: i32) -> Bool;
		} 
		let _ret_ = CallMsgFilterA(transmute(msg), code);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CallMsgFilterW(msg: &Msg, code: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CallMsgFilterW(lpMsg: *const c_void, nCode: i32) -> Bool;
		} 
		let _ret_ = CallMsgFilterW(transmute(msg), code);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn CharToOemA() { todo!() }

pub unsafe fn CharToOemW() { todo!() }

pub unsafe fn OemToCharA() { todo!() }

pub unsafe fn OemToCharW() { todo!() }

pub unsafe fn CharToOemBuffA() { todo!() }

pub unsafe fn CharToOemBuffW() { todo!() }

pub unsafe fn OemToCharBuffA() { todo!() }

pub unsafe fn OemToCharBuffW() { todo!() }

pub unsafe fn CharUpperA() { todo!() }

pub unsafe fn CharUpperW() { todo!() }

pub unsafe fn CharUpperBuffA() { todo!() }

pub unsafe fn CharUpperBuffW() { todo!() }

pub unsafe fn CharLowerA() { todo!() }

pub unsafe fn CharLowerW() { todo!() }

pub unsafe fn CharLowerBuffA() { todo!() }

pub unsafe fn CharLowerBuffW() { todo!() }

pub fn CharNextA(lpsz: &str) -> Result<PStr, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CharNextA(lpsz: *const u8) -> *const c_void;
		} 
		let _ret_ = CharNextA(lpsz.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CharNextW(lpsz: &str) -> Result<PWStr, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CharNextW(lpsz: *const u16) -> *const c_void;
		} 
		let _ret_ = CharNextW(lpsz.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CharNextExA(code_page: u16, current_char: &str, flags: u32) -> Result<PStr, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CharNextExA(CodePage: u16, lpCurrentChar: *const u8, dwFlags: u32) -> *const c_void;
		} 
		let _ret_ = CharNextExA(code_page, current_char.to_null_terminated().as_ptr_or_null(), flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn IsCharAlphaA(ch: Char) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsCharAlphaA(ch: Char) -> Bool;
		} 
		let _ret_ = IsCharAlphaA(ch);
		_ret_.to_bool()
	}
}

pub fn IsCharAlphaW(ch: u16) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsCharAlphaW(ch: u16) -> Bool;
		} 
		let _ret_ = IsCharAlphaW(ch);
		_ret_.to_bool()
	}
}

pub fn IsCharAlphaNumericA(ch: Char) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsCharAlphaNumericA(ch: Char) -> Bool;
		} 
		let _ret_ = IsCharAlphaNumericA(ch);
		_ret_.to_bool()
	}
}

pub fn IsCharAlphaNumericW(ch: u16) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsCharAlphaNumericW(ch: u16) -> Bool;
		} 
		let _ret_ = IsCharAlphaNumericW(ch);
		_ret_.to_bool()
	}
}

pub fn IsCharUpperA(ch: Char) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsCharUpperA(ch: Char) -> Bool;
		} 
		let _ret_ = IsCharUpperA(ch);
		_ret_.to_bool()
	}
}

pub fn IsCharUpperW(ch: u16) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsCharUpperW(ch: u16) -> Bool;
		} 
		let _ret_ = IsCharUpperW(ch);
		_ret_.to_bool()
	}
}

pub fn IsCharLowerA(ch: Char) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsCharLowerA(ch: Char) -> Bool;
		} 
		let _ret_ = IsCharLowerA(ch);
		_ret_.to_bool()
	}
}

pub fn GetInputState() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetInputState() -> Bool;
		} 
		let _ret_ = GetInputState();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetQueueStatus(flags: QueueStatusFlags) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetQueueStatus(flags: QueueStatusFlags) -> u32;
		} 
		let _ret_ = GetQueueStatus(flags);
		_ret_
	}
}

pub fn MsgWaitForMultipleObjects(handles: Option<&[Handle]>, f_wait_all: bool, milliseconds: u32, wake_mask: QueueStatusFlags) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MsgWaitForMultipleObjects(nCount: u32, pHandles: *const Handle, fWaitAll: Bool, dwMilliseconds: u32, dwWakeMask: QueueStatusFlags) -> u32;
		} 
		let (handles_ptr_, handles_len_) = handles.deconstruct();
		let _ret_ = MsgWaitForMultipleObjects(handles_len_ as u32, handles_ptr_, f_wait_all.to_bool(), milliseconds, wake_mask);
		_ret_
	}
}

pub fn MsgWaitForMultipleObjectsEx(handles: Option<&[Handle]>, milliseconds: u32, wake_mask: QueueStatusFlags, flags: MsgWaitForMultipleObjectsExFlags) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MsgWaitForMultipleObjectsEx(nCount: u32, pHandles: *const Handle, dwMilliseconds: u32, dwWakeMask: QueueStatusFlags, dwFlags: MsgWaitForMultipleObjectsExFlags) -> u32;
		} 
		let (handles_ptr_, handles_len_) = handles.deconstruct();
		let _ret_ = MsgWaitForMultipleObjectsEx(handles_len_ as u32, handles_ptr_, milliseconds, wake_mask, flags);
		_ret_
	}
}

pub fn SetTimer(wnd: Option<HWnd>, id_event: usize, u_elapse: u32, timer_func: Option<TimerProc>) -> usize {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetTimer(hWnd: *const c_void, nIDEvent: usize, uElapse: u32, lpTimerFunc: *const c_void) -> usize;
		} 
		let _ret_ = SetTimer(transmute(wnd), id_event, u_elapse, transmute(timer_func));
		_ret_
	}
}

pub fn SetCoalescableTimer(wnd: Option<HWnd>, id_event: usize, u_elapse: u32, timer_func: Option<TimerProc>, u_tolerance_delay: u32) -> usize {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetCoalescableTimer(hWnd: *const c_void, nIDEvent: usize, uElapse: u32, lpTimerFunc: *const c_void, uToleranceDelay: u32) -> usize;
		} 
		let _ret_ = SetCoalescableTimer(transmute(wnd), id_event, u_elapse, transmute(timer_func), u_tolerance_delay);
		_ret_
	}
}

pub fn KillTimer(wnd: Option<HWnd>, u_id_event: usize) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn KillTimer(hWnd: *const c_void, uIDEvent: usize) -> Bool;
		} 
		let _ret_ = KillTimer(transmute(wnd), u_id_event);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn IsWindowUnicode(wnd: HWnd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsWindowUnicode(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = IsWindowUnicode(wnd);
		_ret_.to_bool()
	}
}

pub fn LoadAcceleratorsA(instance: Option<HInstance>, table_name: &str) -> Result<HAccel, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadAcceleratorsA(hInstance: *const c_void, lpTableName: *const u8) -> *const c_void;
		} 
		let _ret_ = LoadAcceleratorsA(transmute(instance), table_name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadAcceleratorsW(instance: Option<HInstance>, table_name: &str) -> Result<HAccel, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadAcceleratorsW(hInstance: *const c_void, lpTableName: *const u16) -> *const c_void;
		} 
		let _ret_ = LoadAcceleratorsW(transmute(instance), table_name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateAcceleratorTableA(paccel: &[Accel]) -> Result<HAccel, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateAcceleratorTableA(paccel: *const Accel, cAccel: i32) -> *const c_void;
		} 
		let (paccel_ptr_, paccel_len_) = paccel.deconstruct();
		let _ret_ = CreateAcceleratorTableA(paccel_ptr_, paccel_len_ as i32);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateAcceleratorTableW(paccel: &[Accel]) -> Result<HAccel, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateAcceleratorTableW(paccel: *const Accel, cAccel: i32) -> *const c_void;
		} 
		let (paccel_ptr_, paccel_len_) = paccel.deconstruct();
		let _ret_ = CreateAcceleratorTableW(paccel_ptr_, paccel_len_ as i32);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DestroyAcceleratorTable(accel: HAccel) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DestroyAcceleratorTable(hAccel: HAccel) -> Bool;
		} 
		let _ret_ = DestroyAcceleratorTable(accel);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn CopyAcceleratorTableA() { todo!() }

pub unsafe fn CopyAcceleratorTableW() { todo!() }

pub fn TranslateAcceleratorA(wnd: HWnd, acc_table: HAccel, msg: &Msg) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TranslateAcceleratorA(hWnd: HWnd, hAccTable: HAccel, lpMsg: *const c_void) -> i32;
		} 
		let _ret_ = TranslateAcceleratorA(wnd, acc_table, transmute(msg));
		_ret_
	}
}

pub fn TranslateAcceleratorW(wnd: HWnd, acc_table: HAccel, msg: &Msg) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TranslateAcceleratorW(hWnd: HWnd, hAccTable: HAccel, lpMsg: *const c_void) -> i32;
		} 
		let _ret_ = TranslateAcceleratorW(wnd, acc_table, transmute(msg));
		_ret_
	}
}

pub fn GetSystemMetrics(index: SystemMetricsIndex) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetSystemMetrics(nIndex: SystemMetricsIndex) -> i32;
		} 
		let _ret_ = GetSystemMetrics(index);
		_ret_
	}
}

pub fn LoadMenuA(instance: Option<HInstance>, menu_name: &str) -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadMenuA(hInstance: *const c_void, lpMenuName: *const u8) -> *const c_void;
		} 
		let _ret_ = LoadMenuA(transmute(instance), menu_name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadMenuW(instance: Option<HInstance>, menu_name: &str) -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadMenuW(hInstance: *const c_void, lpMenuName: *const u16) -> *const c_void;
		} 
		let _ret_ = LoadMenuW(transmute(instance), menu_name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadMenuIndirectA(menu_template: *const impl Sized) -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadMenuIndirectA(lpMenuTemplate: *const c_void) -> *const c_void;
		} 
		let _ret_ = LoadMenuIndirectA(menu_template as _);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadMenuIndirectW(menu_template: *const impl Sized) -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadMenuIndirectW(lpMenuTemplate: *const c_void) -> *const c_void;
		} 
		let _ret_ = LoadMenuIndirectW(menu_template as _);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetMenu(wnd: HWnd) -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenu(hWnd: HWnd) -> *const c_void;
		} 
		let _ret_ = GetMenu(wnd);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetMenu(wnd: HWnd, menu: Option<HMenu>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMenu(hWnd: HWnd, hMenu: *const c_void) -> Bool;
		} 
		let _ret_ = SetMenu(wnd, transmute(menu));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ChangeMenuA(menu: HMenu, cmd: u32, new_item: Option<&str>, cmd_insert: u32, flags: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChangeMenuA(hMenu: HMenu, cmd: u32, lpszNewItem: *const u8, cmdInsert: u32, flags: u32) -> Bool;
		} 
		let _ret_ = ChangeMenuA(menu, cmd, new_item.map(str::to_null_terminated).as_ptr_or_null(), cmd_insert, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ChangeMenuW(menu: HMenu, cmd: u32, new_item: Option<&str>, cmd_insert: u32, flags: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChangeMenuW(hMenu: HMenu, cmd: u32, lpszNewItem: *const u16, cmdInsert: u32, flags: u32) -> Bool;
		} 
		let _ret_ = ChangeMenuW(menu, cmd, new_item.map(str::to_utf16).as_ptr_or_null(), cmd_insert, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn HiliteMenuItem(wnd: HWnd, menu: HMenu, u_id_hilite_item: u32, u_hilite: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn HiliteMenuItem(hWnd: HWnd, hMenu: HMenu, uIDHiliteItem: u32, uHilite: u32) -> Bool;
		} 
		let _ret_ = HiliteMenuItem(wnd, menu, u_id_hilite_item, u_hilite);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn GetMenuStringA() { todo!() }

pub unsafe fn GetMenuStringW() { todo!() }

pub fn GetMenuState(menu: HMenu, u_id: u32, u_flags: MenuItemFlags) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuState(hMenu: HMenu, uId: u32, uFlags: MenuItemFlags) -> u32;
		} 
		let _ret_ = GetMenuState(menu, u_id, u_flags);
		_ret_
	}
}

pub fn DrawMenuBar(wnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawMenuBar(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = DrawMenuBar(wnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetSystemMenu(wnd: HWnd, revert: bool) -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetSystemMenu(hWnd: HWnd, bRevert: Bool) -> *const c_void;
		} 
		let _ret_ = GetSystemMenu(wnd, revert.to_bool());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateMenu() -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateMenu() -> *const c_void;
		} 
		let _ret_ = CreateMenu();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreatePopupMenu() -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreatePopupMenu() -> *const c_void;
		} 
		let _ret_ = CreatePopupMenu();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DestroyMenu(menu: HMenu) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DestroyMenu(hMenu: HMenu) -> Bool;
		} 
		let _ret_ = DestroyMenu(menu);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CheckMenuItem(menu: HMenu, u_id_check_item: u32, u_check: u32) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CheckMenuItem(hMenu: HMenu, uIDCheckItem: u32, uCheck: u32) -> u32;
		} 
		let _ret_ = CheckMenuItem(menu, u_id_check_item, u_check);
		_ret_
	}
}

pub fn EnableMenuItem(menu: HMenu, u_id_enable_item: u32, u_enable: MenuItemFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnableMenuItem(hMenu: HMenu, uIDEnableItem: u32, uEnable: MenuItemFlags) -> Bool;
		} 
		let _ret_ = EnableMenuItem(menu, u_id_enable_item, u_enable);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetSubMenu(menu: HMenu, pos: i32) -> Result<HMenu, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetSubMenu(hMenu: HMenu, nPos: i32) -> *const c_void;
		} 
		let _ret_ = GetSubMenu(menu, pos);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetMenuItemID(menu: HMenu, pos: i32) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuItemID(hMenu: HMenu, nPos: i32) -> u32;
		} 
		let _ret_ = GetMenuItemID(menu, pos);
		_ret_
	}
}

pub fn GetMenuItemCount(menu: Option<HMenu>) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuItemCount(hMenu: *const c_void) -> i32;
		} 
		let _ret_ = GetMenuItemCount(transmute(menu));
		_ret_
	}
}

pub fn InsertMenuA(menu: HMenu, u_position: u32, u_flags: MenuItemFlags, u_id_new_item: usize, new_item: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InsertMenuA(hMenu: HMenu, uPosition: u32, uFlags: MenuItemFlags, uIDNewItem: usize, lpNewItem: *const u8) -> Bool;
		} 
		let _ret_ = InsertMenuA(menu, u_position, u_flags, u_id_new_item, new_item.map(str::to_null_terminated).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn InsertMenuW(menu: HMenu, u_position: u32, u_flags: MenuItemFlags, u_id_new_item: usize, new_item: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InsertMenuW(hMenu: HMenu, uPosition: u32, uFlags: MenuItemFlags, uIDNewItem: usize, lpNewItem: *const u16) -> Bool;
		} 
		let _ret_ = InsertMenuW(menu, u_position, u_flags, u_id_new_item, new_item.map(str::to_utf16).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn AppendMenuA(menu: HMenu, u_flags: MenuItemFlags, u_id_new_item: usize, new_item: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn AppendMenuA(hMenu: HMenu, uFlags: MenuItemFlags, uIDNewItem: usize, lpNewItem: *const u8) -> Bool;
		} 
		let _ret_ = AppendMenuA(menu, u_flags, u_id_new_item, new_item.map(str::to_null_terminated).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn AppendMenuW(menu: HMenu, u_flags: MenuItemFlags, u_id_new_item: usize, new_item: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn AppendMenuW(hMenu: HMenu, uFlags: MenuItemFlags, uIDNewItem: usize, lpNewItem: *const u16) -> Bool;
		} 
		let _ret_ = AppendMenuW(menu, u_flags, u_id_new_item, new_item.map(str::to_utf16).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ModifyMenuA(mnu: HMenu, u_position: u32, u_flags: MenuItemFlags, u_id_new_item: usize, new_item: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ModifyMenuA(hMnu: HMenu, uPosition: u32, uFlags: MenuItemFlags, uIDNewItem: usize, lpNewItem: *const u8) -> Bool;
		} 
		let _ret_ = ModifyMenuA(mnu, u_position, u_flags, u_id_new_item, new_item.map(str::to_null_terminated).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ModifyMenuW(mnu: HMenu, u_position: u32, u_flags: MenuItemFlags, u_id_new_item: usize, new_item: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ModifyMenuW(hMnu: HMenu, uPosition: u32, uFlags: MenuItemFlags, uIDNewItem: usize, lpNewItem: *const u16) -> Bool;
		} 
		let _ret_ = ModifyMenuW(mnu, u_position, u_flags, u_id_new_item, new_item.map(str::to_utf16).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RemoveMenu(menu: HMenu, u_position: u32, u_flags: MenuItemFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RemoveMenu(hMenu: HMenu, uPosition: u32, uFlags: MenuItemFlags) -> Bool;
		} 
		let _ret_ = RemoveMenu(menu, u_position, u_flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DeleteMenu(menu: HMenu, u_position: u32, u_flags: MenuItemFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DeleteMenu(hMenu: HMenu, uPosition: u32, uFlags: MenuItemFlags) -> Bool;
		} 
		let _ret_ = DeleteMenu(menu, u_position, u_flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetMenuItemBitmaps(menu: HMenu, u_position: u32, u_flags: MenuItemFlags, bitmap_unchecked: Option<HBitmap>, bitmap_checked: Option<HBitmap>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMenuItemBitmaps(hMenu: HMenu, uPosition: u32, uFlags: MenuItemFlags, hBitmapUnchecked: *const c_void, hBitmapChecked: *const c_void) -> Bool;
		} 
		let _ret_ = SetMenuItemBitmaps(menu, u_position, u_flags, transmute(bitmap_unchecked), transmute(bitmap_checked));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetMenuCheckMarkDimensions() -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuCheckMarkDimensions() -> i32;
		} 
		let _ret_ = GetMenuCheckMarkDimensions();
		_ret_
	}
}

pub fn TrackPopupMenu(menu: HMenu, u_flags: TrackPopupMenuFlags, x: i32, y: i32, reserved: i32, wnd: HWnd, prc_rect: &Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TrackPopupMenu(hMenu: HMenu, uFlags: TrackPopupMenuFlags, x: i32, y: i32, nReserved: i32, hWnd: HWnd, prcRect: &Rect) -> Bool;
		} 
		let _ret_ = TrackPopupMenu(menu, u_flags, x, y, reserved, wnd, prc_rect);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn TrackPopupMenuEx(menu: HMenu, u_flags: u32, x: i32, y: i32, hwnd: HWnd, lptpm: Option<&TpmParams>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TrackPopupMenuEx(hMenu: HMenu, uFlags: u32, x: i32, y: i32, hwnd: HWnd, lptpm: *const c_void) -> Bool;
		} 
		let _ret_ = TrackPopupMenuEx(menu, u_flags, x, y, hwnd, transmute(lptpm));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CalculatePopupWindowPosition(anchor_point: &Point, window_size: &Size, flags: u32, exclude_rect: Option<&Rect>) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CalculatePopupWindowPosition(anchorPoint: &Point, windowSize: &Size, flags: u32, excludeRect: *const c_void, popupWindowPosition: *mut Rect) -> Bool;
		} 
		let mut popup_window_position_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = CalculatePopupWindowPosition(anchor_point, window_size, flags, transmute(exclude_rect), popup_window_position_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(popup_window_position_out_.assume_init()) }
	}
}

pub fn GetMenuInfo(param0: HMenu, param1: &mut MenuInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuInfo(param0: HMenu, param1: &mut MenuInfo) -> Bool;
		} 
		let _ret_ = GetMenuInfo(param0, param1);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetMenuInfo(param0: HMenu, param1: &MenuInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMenuInfo(param0: HMenu, param1: &MenuInfo) -> Bool;
		} 
		let _ret_ = SetMenuInfo(param0, param1);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EndMenu() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EndMenu() -> Bool;
		} 
		let _ret_ = EndMenu();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn InsertMenuItemA(hmenu: HMenu, item: u32, f_by_position: bool, lpmi: &MenuItemInfoA) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InsertMenuItemA(hmenu: HMenu, item: u32, fByPosition: Bool, lpmi: &MenuItemInfoA) -> Bool;
		} 
		let _ret_ = InsertMenuItemA(hmenu, item, f_by_position.to_bool(), lpmi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn InsertMenuItemW(hmenu: HMenu, item: u32, f_by_position: bool, lpmi: &MenuItemInfoW) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InsertMenuItemW(hmenu: HMenu, item: u32, fByPosition: Bool, lpmi: &MenuItemInfoW) -> Bool;
		} 
		let _ret_ = InsertMenuItemW(hmenu, item, f_by_position.to_bool(), lpmi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetMenuItemInfoA(hmenu: HMenu, item: u32, f_by_position: bool, lpmii: &mut MenuItemInfoA) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuItemInfoA(hmenu: HMenu, item: u32, fByPosition: Bool, lpmii: &mut MenuItemInfoA) -> Bool;
		} 
		let _ret_ = GetMenuItemInfoA(hmenu, item, f_by_position.to_bool(), lpmii);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetMenuItemInfoW(hmenu: HMenu, item: u32, f_by_position: bool, lpmii: &mut MenuItemInfoW) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuItemInfoW(hmenu: HMenu, item: u32, fByPosition: Bool, lpmii: &mut MenuItemInfoW) -> Bool;
		} 
		let _ret_ = GetMenuItemInfoW(hmenu, item, f_by_position.to_bool(), lpmii);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetMenuItemInfoA(hmenu: HMenu, item: u32, f_by_positon: bool, lpmii: &MenuItemInfoA) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMenuItemInfoA(hmenu: HMenu, item: u32, fByPositon: Bool, lpmii: &MenuItemInfoA) -> Bool;
		} 
		let _ret_ = SetMenuItemInfoA(hmenu, item, f_by_positon.to_bool(), lpmii);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetMenuItemInfoW(hmenu: HMenu, item: u32, f_by_positon: bool, lpmii: &MenuItemInfoW) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMenuItemInfoW(hmenu: HMenu, item: u32, fByPositon: Bool, lpmii: &MenuItemInfoW) -> Bool;
		} 
		let _ret_ = SetMenuItemInfoW(hmenu, item, f_by_positon.to_bool(), lpmii);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetMenuDefaultItem(menu: HMenu, f_by_pos: u32, gmdi_flags: GetMenuDefaultItemFlags) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuDefaultItem(hMenu: HMenu, fByPos: u32, gmdiFlags: GetMenuDefaultItemFlags) -> u32;
		} 
		let _ret_ = GetMenuDefaultItem(menu, f_by_pos, gmdi_flags);
		_ret_
	}
}

pub fn SetMenuDefaultItem(menu: HMenu, u_item: u32, f_by_pos: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetMenuDefaultItem(hMenu: HMenu, uItem: u32, fByPos: u32) -> Bool;
		} 
		let _ret_ = SetMenuDefaultItem(menu, u_item, f_by_pos);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetMenuItemRect(wnd: Option<HWnd>, menu: HMenu, u_item: u32) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuItemRect(hWnd: *const c_void, hMenu: HMenu, uItem: u32, lprcItem: *mut Rect) -> Bool;
		} 
		let mut lprc_item_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = GetMenuItemRect(transmute(wnd), menu, u_item, lprc_item_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_item_out_.assume_init()) }
	}
}

pub fn MenuItemFromPoint(wnd: Option<HWnd>, menu: HMenu, pt_screen: Point) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MenuItemFromPoint(hWnd: *const c_void, hMenu: HMenu, ptScreen: Point) -> i32;
		} 
		let _ret_ = MenuItemFromPoint(transmute(wnd), menu, pt_screen);
		_ret_
	}
}

pub fn DragObject(hwnd_parent: HWnd, hwnd_from: HWnd, fmt: u32, data: usize, hcur: Option<HCursor>) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DragObject(hwndParent: HWnd, hwndFrom: HWnd, fmt: u32, data: usize, hcur: *const c_void) -> u32;
		} 
		let _ret_ = DragObject(hwnd_parent, hwnd_from, fmt, data, transmute(hcur));
		_ret_
	}
}

pub fn DrawIcon(dc: HDc, x: i32, y: i32, icon: HIcon) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawIcon(hDC: HDc, X: i32, Y: i32, hIcon: HIcon) -> Bool;
		} 
		let _ret_ = DrawIcon(dc, x, y, icon);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetForegroundWindow() -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetForegroundWindow() -> *const c_void;
		} 
		let _ret_ = GetForegroundWindow();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SwitchToThisWindow(hwnd: HWnd, f_unknown: bool) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SwitchToThisWindow(hwnd: HWnd, fUnknown: Bool) -> ();
		} 
		let _ret_ = SwitchToThisWindow(hwnd, f_unknown.to_bool());
	}
}

pub fn SetForegroundWindow(wnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetForegroundWindow(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = SetForegroundWindow(wnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn AllowSetForegroundWindow(process_id: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn AllowSetForegroundWindow(dwProcessId: u32) -> Bool;
		} 
		let _ret_ = AllowSetForegroundWindow(process_id);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LockSetForegroundWindow(u_lock_code: ForegroundWindowLockCode) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LockSetForegroundWindow(uLockCode: ForegroundWindowLockCode) -> Bool;
		} 
		let _ret_ = LockSetForegroundWindow(u_lock_code);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ScrollWindow(wnd: HWnd, x_amount: i32, y_amount: i32, rect: Option<&Rect>, clip_rect: Option<&Rect>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ScrollWindow(hWnd: HWnd, XAmount: i32, YAmount: i32, lpRect: *const c_void, lpClipRect: *const c_void) -> Bool;
		} 
		let _ret_ = ScrollWindow(wnd, x_amount, y_amount, transmute(rect), transmute(clip_rect));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ScrollDC(dc: HDc, dx: i32, dy: i32, lprc_scroll: Option<&Rect>, lprc_clip: Option<&Rect>, hrgn_update: Option<HRgn>, lprc_update: Option<&mut MaybeUninit<Rect>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ScrollDC(hDC: HDc, dx: i32, dy: i32, lprcScroll: *const c_void, lprcClip: *const c_void, hrgnUpdate: *const c_void, lprcUpdate: Option<&mut MaybeUninit<Rect>>) -> Bool;
		} 
		let _ret_ = ScrollDC(dc, dx, dy, transmute(lprc_scroll), transmute(lprc_clip), transmute(hrgn_update), lprc_update);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn scroll_dc(dc: HDc, dx: i32, dy: i32, lprc_scroll: Option<&Rect>, lprc_clip: Option<&Rect>, hrgn_update: Option<HRgn>) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ScrollDC(hDC: HDc, dx: i32, dy: i32, lprcScroll: *const c_void, lprcClip: *const c_void, hrgnUpdate: *const c_void, lprcUpdate: *mut Rect) -> Bool;
		} 
		let mut lprc_update_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = ScrollDC(dc, dx, dy, transmute(lprc_scroll), transmute(lprc_clip), transmute(hrgn_update), lprc_update_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_update_out_.assume_init()) }
	}
}

pub fn ScrollWindowEx(wnd: HWnd, dx: i32, dy: i32, prc_scroll: Option<&Rect>, prc_clip: Option<&Rect>, hrgn_update: Option<HRgn>, prc_update: Option<&mut MaybeUninit<Rect>>, flags: ShowWindowCmd) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ScrollWindowEx(hWnd: HWnd, dx: i32, dy: i32, prcScroll: *const c_void, prcClip: *const c_void, hrgnUpdate: *const c_void, prcUpdate: Option<&mut MaybeUninit<Rect>>, flags: ShowWindowCmd) -> i32;
		} 
		let _ret_ = ScrollWindowEx(wnd, dx, dy, transmute(prc_scroll), transmute(prc_clip), transmute(hrgn_update), prc_update, flags);
		_ret_
	}
}

pub fn scroll_window_ex(wnd: HWnd, dx: i32, dy: i32, prc_scroll: Option<&Rect>, prc_clip: Option<&Rect>, hrgn_update: Option<HRgn>, flags: ShowWindowCmd) -> (i32, Rect) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ScrollWindowEx(hWnd: HWnd, dx: i32, dy: i32, prcScroll: *const c_void, prcClip: *const c_void, hrgnUpdate: *const c_void, prcUpdate: *mut Rect, flags: ShowWindowCmd) -> i32;
		} 
		let mut prc_update_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = ScrollWindowEx(wnd, dx, dy, transmute(prc_scroll), transmute(prc_clip), transmute(hrgn_update), prc_update_out_.as_mut_ptr(), flags);
		(_ret_, prc_update_out_.assume_init())
	}
}

pub fn GetScrollPos(wnd: HWnd, bar: ScrollbarConstants) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetScrollPos(hWnd: HWnd, nBar: ScrollbarConstants) -> i32;
		} 
		let _ret_ = GetScrollPos(wnd, bar);
		_ret_
	}
}

pub fn GetScrollRange(wnd: HWnd, bar: ScrollbarConstants) -> Result<(i32, i32), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetScrollRange(hWnd: HWnd, nBar: ScrollbarConstants, lpMinPos: *mut i32, lpMaxPos: *mut i32) -> Bool;
		} 
		let mut min_pos_out_: MaybeUninit<i32> = MaybeUninit::zeroed();
		let mut max_pos_out_: MaybeUninit<i32> = MaybeUninit::zeroed();
		let _ret_ = GetScrollRange(wnd, bar, min_pos_out_.as_mut_ptr(), max_pos_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok((min_pos_out_.assume_init(), max_pos_out_.assume_init())) }
	}
}

pub fn SetPropA(wnd: HWnd, string: &str, data: Option<Handle>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetPropA(hWnd: HWnd, lpString: *const u8, hData: *const c_void) -> Bool;
		} 
		let _ret_ = SetPropA(wnd, string.to_null_terminated().as_ptr_or_null(), transmute(data));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetPropW(wnd: HWnd, string: &str, data: Option<Handle>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetPropW(hWnd: HWnd, lpString: *const u16, hData: *const c_void) -> Bool;
		} 
		let _ret_ = SetPropW(wnd, string.to_utf16().as_ptr_or_null(), transmute(data));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetPropA(wnd: HWnd, string: &str) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetPropA(hWnd: HWnd, lpString: *const u8) -> *const c_void;
		} 
		let _ret_ = GetPropA(wnd, string.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetPropW(wnd: HWnd, string: &str) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetPropW(hWnd: HWnd, lpString: *const u16) -> *const c_void;
		} 
		let _ret_ = GetPropW(wnd, string.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn RemovePropA(wnd: HWnd, string: &str) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RemovePropA(hWnd: HWnd, lpString: *const u8) -> *const c_void;
		} 
		let _ret_ = RemovePropA(wnd, string.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn RemovePropW(wnd: HWnd, string: &str) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RemovePropW(hWnd: HWnd, lpString: *const u16) -> *const c_void;
		} 
		let _ret_ = RemovePropW(wnd, string.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn EnumPropsExA(wnd: HWnd, enum_func: PropEnumProcExA, l_param: LParam) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumPropsExA(hWnd: HWnd, lpEnumFunc: PropEnumProcExA, lParam: LParam) -> i32;
		} 
		let _ret_ = EnumPropsExA(wnd, enum_func, l_param);
		_ret_
	}
}

pub fn EnumPropsExW(wnd: HWnd, enum_func: PropEnumProcExW, l_param: LParam) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumPropsExW(hWnd: HWnd, lpEnumFunc: PropEnumProcExW, lParam: LParam) -> i32;
		} 
		let _ret_ = EnumPropsExW(wnd, enum_func, l_param);
		_ret_
	}
}

pub fn EnumPropsA(wnd: HWnd, enum_func: PropEnumProcA) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumPropsA(hWnd: HWnd, lpEnumFunc: PropEnumProcA) -> i32;
		} 
		let _ret_ = EnumPropsA(wnd, enum_func);
		_ret_
	}
}

pub fn EnumPropsW(wnd: HWnd, enum_func: PropEnumProcW) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumPropsW(hWnd: HWnd, lpEnumFunc: PropEnumProcW) -> i32;
		} 
		let _ret_ = EnumPropsW(wnd, enum_func);
		_ret_
	}
}

pub fn SetWindowTextA(wnd: HWnd, string: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowTextA(hWnd: HWnd, lpString: *const u8) -> Bool;
		} 
		let _ret_ = SetWindowTextA(wnd, string.map(str::to_null_terminated).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetWindowTextW(wnd: HWnd, string: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowTextW(hWnd: HWnd, lpString: *const u16) -> Bool;
		} 
		let _ret_ = SetWindowTextW(wnd, string.map(str::to_utf16).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn GetWindowTextA() { todo!() }

pub unsafe fn GetWindowTextW() { todo!() }

pub fn GetWindowTextLengthA(wnd: HWnd) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowTextLengthA(hWnd: HWnd) -> i32;
		} 
		let _ret_ = GetWindowTextLengthA(wnd);
		_ret_
	}
}

pub fn GetWindowTextLengthW(wnd: HWnd) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowTextLengthW(hWnd: HWnd) -> i32;
		} 
		let _ret_ = GetWindowTextLengthW(wnd);
		_ret_
	}
}

pub fn MessageBoxA(wnd: Option<HWnd>, text: Option<&str>, caption: Option<&str>, u_type: MessageBoxStyle) -> MessageBoxResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MessageBoxA(hWnd: *const c_void, lpText: *const u8, lpCaption: *const u8, uType: MessageBoxStyle) -> MessageBoxResult;
		} 
		let _ret_ = MessageBoxA(transmute(wnd), text.map(str::to_null_terminated).as_ptr_or_null(), caption.map(str::to_null_terminated).as_ptr_or_null(), u_type);
		_ret_
	}
}

pub fn MessageBoxW(wnd: Option<HWnd>, text: Option<&str>, caption: Option<&str>, u_type: MessageBoxStyle) -> MessageBoxResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MessageBoxW(hWnd: *const c_void, lpText: *const u16, lpCaption: *const u16, uType: MessageBoxStyle) -> MessageBoxResult;
		} 
		let _ret_ = MessageBoxW(transmute(wnd), text.map(str::to_utf16).as_ptr_or_null(), caption.map(str::to_utf16).as_ptr_or_null(), u_type);
		_ret_
	}
}

pub fn MessageBoxExA(wnd: Option<HWnd>, text: Option<&str>, caption: Option<&str>, u_type: MessageBoxStyle, language_id: u16) -> MessageBoxResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MessageBoxExA(hWnd: *const c_void, lpText: *const u8, lpCaption: *const u8, uType: MessageBoxStyle, wLanguageId: u16) -> MessageBoxResult;
		} 
		let _ret_ = MessageBoxExA(transmute(wnd), text.map(str::to_null_terminated).as_ptr_or_null(), caption.map(str::to_null_terminated).as_ptr_or_null(), u_type, language_id);
		_ret_
	}
}

pub fn MessageBoxExW(wnd: Option<HWnd>, text: Option<&str>, caption: Option<&str>, u_type: MessageBoxStyle, language_id: u16) -> MessageBoxResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MessageBoxExW(hWnd: *const c_void, lpText: *const u16, lpCaption: *const u16, uType: MessageBoxStyle, wLanguageId: u16) -> MessageBoxResult;
		} 
		let _ret_ = MessageBoxExW(transmute(wnd), text.map(str::to_utf16).as_ptr_or_null(), caption.map(str::to_utf16).as_ptr_or_null(), u_type, language_id);
		_ret_
	}
}

pub fn MessageBoxIndirectA(lpmbp: &MsgBoxParamsA) -> MessageBoxResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MessageBoxIndirectA(lpmbp: &MsgBoxParamsA) -> MessageBoxResult;
		} 
		let _ret_ = MessageBoxIndirectA(lpmbp);
		_ret_
	}
}

pub fn MessageBoxIndirectW(lpmbp: &MsgBoxParamsW) -> MessageBoxResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MessageBoxIndirectW(lpmbp: &MsgBoxParamsW) -> MessageBoxResult;
		} 
		let _ret_ = MessageBoxIndirectW(lpmbp);
		_ret_
	}
}

pub fn ShowCursor(show: bool) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ShowCursor(bShow: Bool) -> i32;
		} 
		let _ret_ = ShowCursor(show.to_bool());
		_ret_
	}
}

pub fn SetCursorPos(x: i32, y: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetCursorPos(X: i32, Y: i32) -> Bool;
		} 
		let _ret_ = SetCursorPos(x, y);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetPhysicalCursorPos(x: i32, y: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetPhysicalCursorPos(X: i32, Y: i32) -> Bool;
		} 
		let _ret_ = SetPhysicalCursorPos(x, y);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetCursor(cursor: Option<HCursor>) -> Result<HCursor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetCursor(hCursor: *const c_void) -> *const c_void;
		} 
		let _ret_ = SetCursor(transmute(cursor));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetCursorPos() -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetCursorPos(lpPoint: *mut Point) -> Bool;
		} 
		let mut point_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetCursorPos(point_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(point_out_.assume_init()) }
	}
}

pub fn GetPhysicalCursorPos() -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetPhysicalCursorPos(lpPoint: *mut Point) -> Bool;
		} 
		let mut point_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetPhysicalCursorPos(point_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(point_out_.assume_init()) }
	}
}

pub fn GetClipCursor() -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClipCursor(lpRect: *mut Rect) -> Bool;
		} 
		let mut rect_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = GetClipCursor(rect_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(rect_out_.assume_init()) }
	}
}

pub fn GetCursor() -> Result<HCursor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetCursor() -> *const c_void;
		} 
		let _ret_ = GetCursor();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateCaret(wnd: HWnd, bitmap: Option<HBitmap>, width: i32, height: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateCaret(hWnd: HWnd, hBitmap: *const c_void, nWidth: i32, nHeight: i32) -> Bool;
		} 
		let _ret_ = CreateCaret(wnd, transmute(bitmap), width, height);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetCaretBlinkTime() -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetCaretBlinkTime() -> u32;
		} 
		let _ret_ = GetCaretBlinkTime();
		_ret_
	}
}

pub fn SetCaretBlinkTime(u_m_seconds: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetCaretBlinkTime(uMSeconds: u32) -> Bool;
		} 
		let _ret_ = SetCaretBlinkTime(u_m_seconds);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DestroyCaret() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DestroyCaret() -> Bool;
		} 
		let _ret_ = DestroyCaret();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn HideCaret(wnd: Option<HWnd>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn HideCaret(hWnd: *const c_void) -> Bool;
		} 
		let _ret_ = HideCaret(transmute(wnd));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ShowCaret(wnd: Option<HWnd>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ShowCaret(hWnd: *const c_void) -> Bool;
		} 
		let _ret_ = ShowCaret(transmute(wnd));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetCaretPos(x: i32, y: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetCaretPos(X: i32, Y: i32) -> Bool;
		} 
		let _ret_ = SetCaretPos(x, y);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetCaretPos() -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetCaretPos(lpPoint: *mut Point) -> Bool;
		} 
		let mut point_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetCaretPos(point_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(point_out_.assume_init()) }
	}
}

pub fn LogicalToPhysicalPoint(wnd: HWnd, point: &mut Point) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LogicalToPhysicalPoint(hWnd: HWnd, lpPoint: &mut Point) -> Bool;
		} 
		let _ret_ = LogicalToPhysicalPoint(wnd, point);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PhysicalToLogicalPoint(wnd: HWnd, point: &mut Point) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PhysicalToLogicalPoint(hWnd: HWnd, lpPoint: &mut Point) -> Bool;
		} 
		let _ret_ = PhysicalToLogicalPoint(wnd, point);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn WindowFromPoint(point: Point) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn WindowFromPoint(Point: Point) -> *const c_void;
		} 
		let _ret_ = WindowFromPoint(point);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn WindowFromPhysicalPoint(point: Point) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn WindowFromPhysicalPoint(Point: Point) -> *const c_void;
		} 
		let _ret_ = WindowFromPhysicalPoint(point);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn ChildWindowFromPoint(wnd_parent: HWnd, point: Point) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChildWindowFromPoint(hWndParent: HWnd, Point: Point) -> *const c_void;
		} 
		let _ret_ = ChildWindowFromPoint(wnd_parent, point);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn ClipCursor(rect: Option<&Rect>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ClipCursor(lpRect: *const c_void) -> Bool;
		} 
		let _ret_ = ClipCursor(transmute(rect));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ChildWindowFromPointEx(hwnd: HWnd, pt: Point, flags: CwpFlags) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChildWindowFromPointEx(hwnd: HWnd, pt: Point, flags: CwpFlags) -> *const c_void;
		} 
		let _ret_ = ChildWindowFromPointEx(hwnd, pt, flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetSysColor(index: SysColorIndex) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetSysColor(nIndex: SysColorIndex) -> u32;
		} 
		let _ret_ = GetSysColor(index);
		_ret_
	}
}

	pub unsafe fn SetSysColors() { todo!() }

pub fn GetWindowWord(wnd: HWnd, index: i32) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowWord(hWnd: HWnd, nIndex: i32) -> u16;
		} 
		let _ret_ = GetWindowWord(wnd, index);
		_ret_
	}
}

pub fn SetWindowWord(wnd: HWnd, index: i32, new_word: u16) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowWord(hWnd: HWnd, nIndex: i32, wNewWord: u16) -> u16;
		} 
		let _ret_ = SetWindowWord(wnd, index, new_word);
		_ret_
	}
}

pub fn GetWindowLongA(wnd: HWnd, index: WindowLongPtrIndex) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowLongA(hWnd: HWnd, nIndex: WindowLongPtrIndex) -> i32;
		} 
		let _ret_ = GetWindowLongA(wnd, index);
		_ret_
	}
}

pub fn GetWindowLongW(wnd: HWnd, index: WindowLongPtrIndex) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowLongW(hWnd: HWnd, nIndex: WindowLongPtrIndex) -> i32;
		} 
		let _ret_ = GetWindowLongW(wnd, index);
		_ret_
	}
}

pub fn SetWindowLongA(wnd: HWnd, index: WindowLongPtrIndex, new_long: i32) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowLongA(hWnd: HWnd, nIndex: WindowLongPtrIndex, dwNewLong: i32) -> i32;
		} 
		let _ret_ = SetWindowLongA(wnd, index, new_long);
		_ret_
	}
}

pub fn SetWindowLongW(wnd: HWnd, index: WindowLongPtrIndex, new_long: i32) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowLongW(hWnd: HWnd, nIndex: WindowLongPtrIndex, dwNewLong: i32) -> i32;
		} 
		let _ret_ = SetWindowLongW(wnd, index, new_long);
		_ret_
	}
}

pub fn GetWindowLongPtrW(wnd: HWnd, index: WindowLongPtrIndex) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowLongPtrW(hWnd: HWnd, nIndex: WindowLongPtrIndex) -> Option<NonNull<()>>;
		} 
		let _ret_ = GetWindowLongPtrW(wnd, index);
		_ret_
	}
}

pub fn SetWindowLongPtrA(wnd: HWnd, index: WindowLongPtrIndex, new_long: NonZeroUsize) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowLongPtrA(hWnd: HWnd, nIndex: WindowLongPtrIndex, dwNewLong: NonZeroUsize) -> Option<NonNull<()>>;
		} 
		let _ret_ = SetWindowLongPtrA(wnd, index, new_long);
		_ret_
	}
}

pub fn SetWindowLongPtrW(wnd: HWnd, index: WindowLongPtrIndex, new_long: NonZeroUsize) -> Option<NonNull<()>> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowLongPtrW(hWnd: HWnd, nIndex: WindowLongPtrIndex, dwNewLong: NonZeroUsize) -> Option<NonNull<()>>;
		} 
		let _ret_ = SetWindowLongPtrW(wnd, index, new_long);
		_ret_
	}
}

pub fn GetClassWord(wnd: HWnd, index: i32) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassWord(hWnd: HWnd, nIndex: i32) -> u16;
		} 
		let _ret_ = GetClassWord(wnd, index);
		_ret_
	}
}

pub fn SetClassWord(wnd: HWnd, index: i32, new_word: u16) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetClassWord(hWnd: HWnd, nIndex: i32, wNewWord: u16) -> u16;
		} 
		let _ret_ = SetClassWord(wnd, index, new_word);
		_ret_
	}
}

pub fn GetClassLongA(wnd: HWnd, index: GetClassLongIndex) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassLongA(hWnd: HWnd, nIndex: GetClassLongIndex) -> u32;
		} 
		let _ret_ = GetClassLongA(wnd, index);
		_ret_
	}
}

pub fn GetClassLongW(wnd: HWnd, index: GetClassLongIndex) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassLongW(hWnd: HWnd, nIndex: GetClassLongIndex) -> u32;
		} 
		let _ret_ = GetClassLongW(wnd, index);
		_ret_
	}
}

pub fn SetClassLongA(wnd: HWnd, index: GetClassLongIndex, new_long: i32) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetClassLongA(hWnd: HWnd, nIndex: GetClassLongIndex, dwNewLong: i32) -> u32;
		} 
		let _ret_ = SetClassLongA(wnd, index, new_long);
		_ret_
	}
}

pub fn SetClassLongW(wnd: HWnd, index: GetClassLongIndex, new_long: i32) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetClassLongW(hWnd: HWnd, nIndex: GetClassLongIndex, dwNewLong: i32) -> u32;
		} 
		let _ret_ = SetClassLongW(wnd, index, new_long);
		_ret_
	}
}

pub fn GetClassLongPtrA(wnd: HWnd, index: GetClassLongIndex) -> usize {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassLongPtrA(hWnd: HWnd, nIndex: GetClassLongIndex) -> usize;
		} 
		let _ret_ = GetClassLongPtrA(wnd, index);
		_ret_
	}
}

pub fn GetClassLongPtrW(wnd: HWnd, index: GetClassLongIndex) -> usize {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetClassLongPtrW(hWnd: HWnd, nIndex: GetClassLongIndex) -> usize;
		} 
		let _ret_ = GetClassLongPtrW(wnd, index);
		_ret_
	}
}

pub fn SetClassLongPtrA(wnd: HWnd, index: GetClassLongIndex, new_long: NonZeroUsize) -> usize {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetClassLongPtrA(hWnd: HWnd, nIndex: GetClassLongIndex, dwNewLong: NonZeroUsize) -> usize;
		} 
		let _ret_ = SetClassLongPtrA(wnd, index, new_long);
		_ret_
	}
}

pub fn SetClassLongPtrW(wnd: HWnd, index: GetClassLongIndex, new_long: NonZeroUsize) -> usize {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetClassLongPtrW(hWnd: HWnd, nIndex: GetClassLongIndex, dwNewLong: NonZeroUsize) -> usize;
		} 
		let _ret_ = SetClassLongPtrW(wnd, index, new_long);
		_ret_
	}
}

pub fn GetProcessDefaultLayout() -> Result<u32, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetProcessDefaultLayout(pdwDefaultLayout: *mut u32) -> Bool;
		} 
		let mut pdw_default_layout_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let _ret_ = GetProcessDefaultLayout(pdw_default_layout_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(pdw_default_layout_out_.assume_init()) }
	}
}

pub fn SetProcessDefaultLayout(default_layout: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetProcessDefaultLayout(dwDefaultLayout: u32) -> Bool;
		} 
		let _ret_ = SetProcessDefaultLayout(default_layout);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetDesktopWindow() -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDesktopWindow() -> *const c_void;
		} 
		let _ret_ = GetDesktopWindow();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetParent(wnd: HWnd) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetParent(hWnd: HWnd) -> *const c_void;
		} 
		let _ret_ = GetParent(wnd);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetParent(wnd_child: HWnd, wnd_new_parent: Option<HWnd>) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetParent(hWndChild: HWnd, hWndNewParent: *const c_void) -> *const c_void;
		} 
		let _ret_ = SetParent(wnd_child, transmute(wnd_new_parent));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn EnumChildWindows(wnd_parent: Option<HWnd>, enum_func: WndEnumProc, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumChildWindows(hWndParent: *const c_void, lpEnumFunc: WndEnumProc, lParam: LParam) -> Bool;
		} 
		let _ret_ = EnumChildWindows(transmute(wnd_parent), enum_func, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FindWindowA(class_name: Option<&str>, window_name: Option<&str>) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn FindWindowA(lpClassName: *const u8, lpWindowName: *const u8) -> *const c_void;
		} 
		let _ret_ = FindWindowA(class_name.map(str::to_null_terminated).as_ptr_or_null(), window_name.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn FindWindowW(class_name: Option<&str>, window_name: Option<&str>) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> *const c_void;
		} 
		let _ret_ = FindWindowW(class_name.map(str::to_utf16).as_ptr_or_null(), window_name.map(str::to_utf16).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn FindWindowExA(wnd_parent: Option<HWnd>, wnd_child_after: Option<HWnd>, class: Option<&str>, window: Option<&str>) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn FindWindowExA(hWndParent: *const c_void, hWndChildAfter: *const c_void, lpszClass: *const u8, lpszWindow: *const u8) -> *const c_void;
		} 
		let _ret_ = FindWindowExA(transmute(wnd_parent), transmute(wnd_child_after), class.map(str::to_null_terminated).as_ptr_or_null(), window.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn FindWindowExW(wnd_parent: Option<HWnd>, wnd_child_after: Option<HWnd>, class: Option<&str>, window: Option<&str>) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn FindWindowExW(hWndParent: *const c_void, hWndChildAfter: *const c_void, lpszClass: *const u16, lpszWindow: *const u16) -> *const c_void;
		} 
		let _ret_ = FindWindowExW(transmute(wnd_parent), transmute(wnd_child_after), class.map(str::to_utf16).as_ptr_or_null(), window.map(str::to_utf16).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetShellWindow() -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetShellWindow() -> *const c_void;
		} 
		let _ret_ = GetShellWindow();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn RegisterShellHookWindow(hwnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterShellHookWindow(hwnd: HWnd) -> Bool;
		} 
		let _ret_ = RegisterShellHookWindow(hwnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DeregisterShellHookWindow(hwnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DeregisterShellHookWindow(hwnd: HWnd) -> Bool;
		} 
		let _ret_ = DeregisterShellHookWindow(hwnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumWindows(enum_func: WndEnumProc, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumWindows(lpEnumFunc: WndEnumProc, lParam: LParam) -> Bool;
		} 
		let _ret_ = EnumWindows(enum_func, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumThreadWindows(thread_id: u32, lpfn: WndEnumProc, l_param: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumThreadWindows(dwThreadId: u32, lpfn: WndEnumProc, lParam: LParam) -> Bool;
		} 
		let _ret_ = EnumThreadWindows(thread_id, lpfn, l_param);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn GetClassNameA() { todo!() }

pub unsafe fn GetClassNameW() { todo!() }

pub fn GetTopWindow(wnd: Option<HWnd>) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetTopWindow(hWnd: *const c_void) -> *const c_void;
		} 
		let _ret_ = GetTopWindow(transmute(wnd));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetWindowThreadProcessId(wnd: HWnd, lpdw_process_id: Option<&mut MaybeUninit<u32>>) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowThreadProcessId(hWnd: HWnd, lpdwProcessId: Option<&mut MaybeUninit<u32>>) -> u32;
		} 
		let _ret_ = GetWindowThreadProcessId(wnd, lpdw_process_id);
		_ret_
	}
}

pub fn get_window_thread_process_id(wnd: HWnd) -> (u32, u32) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowThreadProcessId(hWnd: HWnd, lpdwProcessId: *mut u32) -> u32;
		} 
		let mut lpdw_process_id_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let _ret_ = GetWindowThreadProcessId(wnd, lpdw_process_id_out_.as_mut_ptr());
		(_ret_, lpdw_process_id_out_.assume_init())
	}
}

pub fn IsGUIThread(convert: bool) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsGUIThread(bConvert: Bool) -> Bool;
		} 
		let _ret_ = IsGUIThread(convert.to_bool());
		_ret_.to_bool()
	}
}

pub fn GetLastActivePopup(wnd: HWnd) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetLastActivePopup(hWnd: HWnd) -> *const c_void;
		} 
		let _ret_ = GetLastActivePopup(wnd);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetWindow(wnd: HWnd, u_cmd: GetWindowCmd) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindow(hWnd: HWnd, uCmd: GetWindowCmd) -> *const c_void;
		} 
		let _ret_ = GetWindow(wnd, u_cmd);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetWindowsHookA(filter_type: i32, pfn_filter_proc: HookProc) -> Result<HHook, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowsHookA(nFilterType: i32, pfnFilterProc: HookProc) -> *const c_void;
		} 
		let _ret_ = SetWindowsHookA(filter_type, pfn_filter_proc);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetWindowsHookW(filter_type: i32, pfn_filter_proc: HookProc) -> Result<HHook, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowsHookW(nFilterType: i32, pfnFilterProc: HookProc) -> *const c_void;
		} 
		let _ret_ = SetWindowsHookW(filter_type, pfn_filter_proc);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn UnhookWindowsHook(code: i32, pfn_filter_proc: HookProc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UnhookWindowsHook(nCode: i32, pfnFilterProc: HookProc) -> Bool;
		} 
		let _ret_ = UnhookWindowsHook(code, pfn_filter_proc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetWindowsHookExA(id_hook: WindowsHookId, lpfn: HookProc, hmod: Option<HInstance>, thread_id: u32) -> Result<HHook, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowsHookExA(idHook: WindowsHookId, lpfn: HookProc, hmod: *const c_void, dwThreadId: u32) -> *const c_void;
		} 
		let _ret_ = SetWindowsHookExA(id_hook, lpfn, transmute(hmod), thread_id);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetWindowsHookExW(id_hook: WindowsHookId, lpfn: HookProc, hmod: Option<HInstance>, thread_id: u32) -> Result<HHook, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowsHookExW(idHook: WindowsHookId, lpfn: HookProc, hmod: *const c_void, dwThreadId: u32) -> *const c_void;
		} 
		let _ret_ = SetWindowsHookExW(id_hook, lpfn, transmute(hmod), thread_id);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn UnhookWindowsHookEx(hhk: HHook) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UnhookWindowsHookEx(hhk: HHook) -> Bool;
		} 
		let _ret_ = UnhookWindowsHookEx(hhk);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CallNextHookEx(hhk: Option<HHook>, code: i32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CallNextHookEx(hhk: *const c_void, nCode: i32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = CallNextHookEx(transmute(hhk), code, w_param, l_param);
		_ret_
	}
}

pub fn CheckMenuRadioItem(hmenu: HMenu, first: u32, last: u32, check: u32, flags: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CheckMenuRadioItem(hmenu: HMenu, first: u32, last: u32, check: u32, flags: u32) -> Bool;
		} 
		let _ret_ = CheckMenuRadioItem(hmenu, first, last, check, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LoadCursorA(instance: Option<HInstance>, cursor_name: &str) -> Result<HCursor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadCursorA(hInstance: *const c_void, lpCursorName: *const u8) -> *const c_void;
		} 
		let _ret_ = LoadCursorA(transmute(instance), cursor_name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadCursorW(instance: Option<HInstance>, cursor_name: &str) -> Result<HCursor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadCursorW(hInstance: *const c_void, lpCursorName: *const u16) -> *const c_void;
		} 
		let _ret_ = LoadCursorW(transmute(instance), cursor_name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadCursorFromFileA(file_name: &str) -> Result<HCursor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadCursorFromFileA(lpFileName: *const u8) -> *const c_void;
		} 
		let _ret_ = LoadCursorFromFileA(file_name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadCursorFromFileW(file_name: &str) -> Result<HCursor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadCursorFromFileW(lpFileName: *const u16) -> *const c_void;
		} 
		let _ret_ = LoadCursorFromFileW(file_name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateCursor(inst: Option<HInstance>, x_hot_spot: i32, y_hot_spot: i32, width: i32, height: i32, pv_and_plane: *const impl Sized, pv_xor_plane: *const impl Sized) -> Result<HCursor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateCursor(hInst: *const c_void, xHotSpot: i32, yHotSpot: i32, nWidth: i32, nHeight: i32, pvANDPlane: *const c_void, pvXORPlane: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateCursor(transmute(inst), x_hot_spot, y_hot_spot, width, height, pv_and_plane as _, pv_xor_plane as _);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DestroyCursor(cursor: HCursor) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DestroyCursor(hCursor: HCursor) -> Bool;
		} 
		let _ret_ = DestroyCursor(cursor);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetSystemCursor(hcur: HCursor, id: SystemCursorId) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetSystemCursor(hcur: HCursor, id: SystemCursorId) -> Bool;
		} 
		let _ret_ = SetSystemCursor(hcur, id);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LoadIconA(instance: Option<HInstance>, icon_name: &str) -> Result<HIcon, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadIconA(hInstance: *const c_void, lpIconName: *const u8) -> *const c_void;
		} 
		let _ret_ = LoadIconA(transmute(instance), icon_name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadIconW(instance: Option<HInstance>, icon_name: &str) -> Result<HIcon, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadIconW(hInstance: *const c_void, lpIconName: *const u16) -> *const c_void;
		} 
		let _ret_ = LoadIconW(transmute(instance), icon_name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

	pub unsafe fn PrivateExtractIconsA() { todo!() }

	pub unsafe fn PrivateExtractIconsW() { todo!() }

	pub unsafe fn CreateIcon() { todo!() }

pub fn DestroyIcon(icon: HIcon) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DestroyIcon(hIcon: HIcon) -> Bool;
		} 
		let _ret_ = DestroyIcon(icon);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LookupIconIdFromDirectory(presbits: &u8, f_icon: bool) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LookupIconIdFromDirectory(presbits: &u8, fIcon: Bool) -> i32;
		} 
		let _ret_ = LookupIconIdFromDirectory(presbits, f_icon.to_bool());
		_ret_
	}
}

pub fn LookupIconIdFromDirectoryEx(presbits: &u8, f_icon: bool, cx_desired: i32, cy_desired: i32, flags: ImageFlags) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LookupIconIdFromDirectoryEx(presbits: &u8, fIcon: Bool, cxDesired: i32, cyDesired: i32, Flags: ImageFlags) -> i32;
		} 
		let _ret_ = LookupIconIdFromDirectoryEx(presbits, f_icon.to_bool(), cx_desired, cy_desired, flags);
		_ret_
	}
}

pub fn CreateIconFromResource(presbits: &[u8], f_icon: bool, ver: u32) -> Result<HIcon, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateIconFromResource(presbits: *const u8, dwResSize: u32, fIcon: Bool, dwVer: u32) -> *const c_void;
		} 
		let (presbits_ptr_, presbits_len_) = presbits.deconstruct();
		let _ret_ = CreateIconFromResource(presbits_ptr_, presbits_len_ as u32, f_icon.to_bool(), ver);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateIconFromResourceEx(presbits: &[u8], f_icon: bool, ver: u32, cx_desired: i32, cy_desired: i32, flags: ImageFlags) -> Result<HIcon, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateIconFromResourceEx(presbits: *const u8, dwResSize: u32, fIcon: Bool, dwVer: u32, cxDesired: i32, cyDesired: i32, Flags: ImageFlags) -> *const c_void;
		} 
		let (presbits_ptr_, presbits_len_) = presbits.deconstruct();
		let _ret_ = CreateIconFromResourceEx(presbits_ptr_, presbits_len_ as u32, f_icon.to_bool(), ver, cx_desired, cy_desired, flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadImageA(inst: Option<HInstance>, name: &str, r#type: GdiImageType, cx: i32, cy: i32, fu_load: ImageFlags) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadImageA(hInst: *const c_void, name: *const u8, r#type: GdiImageType, cx: i32, cy: i32, fuLoad: ImageFlags) -> *const c_void;
		} 
		let _ret_ = LoadImageA(transmute(inst), name.to_null_terminated().as_ptr_or_null(), r#type, cx, cy, fu_load);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadImageW(inst: Option<HInstance>, name: &str, r#type: GdiImageType, cx: i32, cy: i32, fu_load: ImageFlags) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadImageW(hInst: *const c_void, name: *const u16, r#type: GdiImageType, cx: i32, cy: i32, fuLoad: ImageFlags) -> *const c_void;
		} 
		let _ret_ = LoadImageW(transmute(inst), name.to_utf16().as_ptr_or_null(), r#type, cx, cy, fu_load);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CopyImage(h: Handle, r#type: GdiImageType, cx: i32, cy: i32, flags: ImageFlags) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CopyImage(h: Handle, r#type: GdiImageType, cx: i32, cy: i32, flags: ImageFlags) -> *const c_void;
		} 
		let _ret_ = CopyImage(h, r#type, cx, cy, flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DrawIconEx(hdc: HDc, x_left: i32, y_top: i32, icon: HIcon, cx_width: i32, cy_width: i32, istep_if_ani_cur: u32, hbr_flicker_free_draw: Option<HBrush>, di_flags: DiFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawIconEx(hdc: HDc, xLeft: i32, yTop: i32, hIcon: HIcon, cxWidth: i32, cyWidth: i32, istepIfAniCur: u32, hbrFlickerFreeDraw: *const c_void, diFlags: DiFlags) -> Bool;
		} 
		let _ret_ = DrawIconEx(hdc, x_left, y_top, icon, cx_width, cy_width, istep_if_ani_cur, transmute(hbr_flicker_free_draw), di_flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CreateIconIndirect(piconinfo: &IconInfo) -> Result<HIcon, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateIconIndirect(piconinfo: &IconInfo) -> *const c_void;
		} 
		let _ret_ = CreateIconIndirect(piconinfo);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CopyIcon(icon: HIcon) -> Result<HIcon, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CopyIcon(hIcon: HIcon) -> *const c_void;
		} 
		let _ret_ = CopyIcon(icon);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetIconInfo(icon: HIcon) -> Result<IconInfo, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetIconInfo(hIcon: HIcon, piconinfo: *mut IconInfo) -> Bool;
		} 
		let mut piconinfo_out_: MaybeUninit<IconInfo> = MaybeUninit::zeroed();
		let _ret_ = GetIconInfo(icon, piconinfo_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(piconinfo_out_.assume_init()) }
	}
}

pub fn GetIconInfoExA(hicon: HIcon, piconinfo: &mut IconInfoExA) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetIconInfoExA(hicon: HIcon, piconinfo: &mut IconInfoExA) -> Bool;
		} 
		let _ret_ = GetIconInfoExA(hicon, piconinfo);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetIconInfoExW(hicon: HIcon, piconinfo: &mut IconInfoExW) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetIconInfoExW(hicon: HIcon, piconinfo: &mut IconInfoExW) -> Bool;
		} 
		let _ret_ = GetIconInfoExW(hicon, piconinfo);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn IsDialogMessageA(dlg: HWnd, msg: &Msg) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsDialogMessageA(hDlg: HWnd, lpMsg: *const c_void) -> Bool;
		} 
		let _ret_ = IsDialogMessageA(dlg, transmute(msg));
		_ret_.to_bool()
	}
}

pub fn IsDialogMessageW(dlg: HWnd, msg: &Msg) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsDialogMessageW(hDlg: HWnd, lpMsg: *const c_void) -> Bool;
		} 
		let _ret_ = IsDialogMessageW(dlg, transmute(msg));
		_ret_.to_bool()
	}
}

pub fn MapDialogRect(dlg: HWnd, rect: &mut Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MapDialogRect(hDlg: HWnd, lpRect: &mut Rect) -> Bool;
		} 
		let _ret_ = MapDialogRect(dlg, rect);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetScrollInfo(hwnd: HWnd, bar: ScrollbarConstants, lpsi: &mut ScrollInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetScrollInfo(hwnd: HWnd, nBar: ScrollbarConstants, lpsi: &mut ScrollInfo) -> Bool;
		} 
		let _ret_ = GetScrollInfo(hwnd, bar, lpsi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DefFrameProcA(wnd: HWnd, wnd_mdi_client: Option<HWnd>, u_msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefFrameProcA(hWnd: HWnd, hWndMDIClient: *const c_void, uMsg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefFrameProcA(wnd, transmute(wnd_mdi_client), u_msg, w_param, l_param);
		_ret_
	}
}

pub fn DefFrameProcW(wnd: HWnd, wnd_mdi_client: Option<HWnd>, u_msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefFrameProcW(hWnd: HWnd, hWndMDIClient: *const c_void, uMsg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefFrameProcW(wnd, transmute(wnd_mdi_client), u_msg, w_param, l_param);
		_ret_
	}
}

pub fn DefMDIChildProcA(wnd: HWnd, u_msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefMDIChildProcA(hWnd: HWnd, uMsg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefMDIChildProcA(wnd, u_msg, w_param, l_param);
		_ret_
	}
}

pub fn DefMDIChildProcW(wnd: HWnd, u_msg: u32, w_param: WParam, l_param: LParam) -> LResult {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DefMDIChildProcW(hWnd: HWnd, uMsg: u32, wParam: WParam, lParam: LParam) -> LResult;
		} 
		let _ret_ = DefMDIChildProcW(wnd, u_msg, w_param, l_param);
		_ret_
	}
}

pub fn TranslateMDISysAccel(wnd_client: HWnd, msg: &Msg) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TranslateMDISysAccel(hWndClient: HWnd, lpMsg: *const c_void) -> Bool;
		} 
		let _ret_ = TranslateMDISysAccel(wnd_client, transmute(msg));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ArrangeIconicWindows(wnd: HWnd) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ArrangeIconicWindows(hWnd: HWnd) -> u32;
		} 
		let _ret_ = ArrangeIconicWindows(wnd);
		_ret_
	}
}

pub fn CreateMDIWindowA(class_name: &str, window_name: &str, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: Option<HWnd>, instance: Option<HInstance>, l_param: LParam) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateMDIWindowA(lpClassName: *const u8, lpWindowName: *const u8, dwStyle: WindowStyle, X: i32, Y: i32, nWidth: i32, nHeight: i32, hWndParent: *const c_void, hInstance: *const c_void, lParam: LParam) -> *const c_void;
		} 
		let _ret_ = CreateMDIWindowA(class_name.to_null_terminated().as_ptr_or_null(), window_name.to_null_terminated().as_ptr_or_null(), style, x, y, width, height, transmute(wnd_parent), transmute(instance), l_param);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateMDIWindowW(class_name: &str, window_name: &str, style: WindowStyle, x: i32, y: i32, width: i32, height: i32, wnd_parent: Option<HWnd>, instance: Option<HInstance>, l_param: LParam) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CreateMDIWindowW(lpClassName: *const u16, lpWindowName: *const u16, dwStyle: WindowStyle, X: i32, Y: i32, nWidth: i32, nHeight: i32, hWndParent: *const c_void, hInstance: *const c_void, lParam: LParam) -> *const c_void;
		} 
		let _ret_ = CreateMDIWindowW(class_name.to_utf16().as_ptr_or_null(), window_name.to_utf16().as_ptr_or_null(), style, x, y, width, height, transmute(wnd_parent), transmute(instance), l_param);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn TileWindows(hwnd_parent: Option<HWnd>, how: TileWindowsHow, rect: Option<&Rect>, kids: Option<&[HWnd]>) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TileWindows(hwndParent: *const c_void, wHow: TileWindowsHow, lpRect: *const c_void, cKids: u32, lpKids: *const HWnd) -> u16;
		} 
		let (kids_ptr_, kids_len_) = kids.deconstruct();
		let _ret_ = TileWindows(transmute(hwnd_parent), how, transmute(rect), kids_len_ as u32, kids_ptr_);
		_ret_
	}
}

pub fn CascadeWindows(hwnd_parent: Option<HWnd>, how: CascadeWindowsHow, rect: Option<&Rect>, kids: Option<&[HWnd]>) -> u16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CascadeWindows(hwndParent: *const c_void, wHow: CascadeWindowsHow, lpRect: *const c_void, cKids: u32, lpKids: *const HWnd) -> u16;
		} 
		let (kids_ptr_, kids_len_) = kids.deconstruct();
		let _ret_ = CascadeWindows(transmute(hwnd_parent), how, transmute(rect), kids_len_ as u32, kids_ptr_);
		_ret_
	}
}

pub fn SystemParametersInfoA(ui_action: SystemParametersInfoAction, ui_param: u32, pv_param: Option<&mut MaybeUninit<()>>, f_win_ini: SystemParametersInfoUpdateFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SystemParametersInfoA(uiAction: SystemParametersInfoAction, uiParam: u32, pvParam: Option<&mut MaybeUninit<()>>, fWinIni: SystemParametersInfoUpdateFlags) -> Bool;
		} 
		let _ret_ = SystemParametersInfoA(ui_action, ui_param, pv_param, f_win_ini);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn system_parameters_info_a() { todo!() }

pub fn SystemParametersInfoW(ui_action: SystemParametersInfoAction, ui_param: u32, pv_param: Option<&mut MaybeUninit<()>>, f_win_ini: SystemParametersInfoUpdateFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SystemParametersInfoW(uiAction: SystemParametersInfoAction, uiParam: u32, pvParam: Option<&mut MaybeUninit<()>>, fWinIni: SystemParametersInfoUpdateFlags) -> Bool;
		} 
		let _ret_ = SystemParametersInfoW(ui_action, ui_param, pv_param, f_win_ini);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn system_parameters_info_w() { todo!() }

pub fn SoundSentry() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SoundSentry() -> Bool;
		} 
		let _ret_ = SoundSentry();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetDebugErrorLevel(level: u32) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetDebugErrorLevel(dwLevel: u32) -> ();
		} 
		let _ret_ = SetDebugErrorLevel(level);
	}
}

pub unsafe fn InternalGetWindowText() { todo!() }

pub fn CancelShutdown() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CancelShutdown() -> Bool;
		} 
		let _ret_ = CancelShutdown();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetGUIThreadInfo(id_thread: u32, pgui: &mut GuiThreadInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetGUIThreadInfo(idThread: u32, pgui: &mut GuiThreadInfo) -> Bool;
		} 
		let _ret_ = GetGUIThreadInfo(id_thread, pgui);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetProcessDPIAware() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetProcessDPIAware() -> Bool;
		} 
		let _ret_ = SetProcessDPIAware();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn IsProcessDPIAware() -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsProcessDPIAware() -> Bool;
		} 
		let _ret_ = IsProcessDPIAware();
		_ret_.to_bool()
	}
}

pub fn InheritWindowMonitor(hwnd: HWnd, hwnd_inherit: Option<HWnd>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InheritWindowMonitor(hwnd: HWnd, hwndInherit: *const c_void) -> Bool;
		} 
		let _ret_ = InheritWindowMonitor(hwnd, transmute(hwnd_inherit));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn GetWindowModuleFileNameA() { todo!() }

pub unsafe fn GetWindowModuleFileNameW() { todo!() }

pub fn GetCursorInfo(pci: &mut CursorInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetCursorInfo(pci: &mut CursorInfo) -> Bool;
		} 
		let _ret_ = GetCursorInfo(pci);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetWindowInfo(hwnd: HWnd, pwi: &mut WindowInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowInfo(hwnd: HWnd, pwi: &mut WindowInfo) -> Bool;
		} 
		let _ret_ = GetWindowInfo(hwnd, pwi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetTitleBarInfo(hwnd: HWnd, pti: &mut TitleBarInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetTitleBarInfo(hwnd: HWnd, pti: &mut TitleBarInfo) -> Bool;
		} 
		let _ret_ = GetTitleBarInfo(hwnd, pti);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetMenuBarInfo(hwnd: HWnd, id_object: ObjectIdentifier, id_item: i32, pmbi: &mut MenuBarInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMenuBarInfo(hwnd: HWnd, idObject: ObjectIdentifier, idItem: i32, pmbi: &mut MenuBarInfo) -> Bool;
		} 
		let _ret_ = GetMenuBarInfo(hwnd, id_object, id_item, pmbi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetScrollBarInfo(hwnd: HWnd, id_object: ObjectIdentifier, psbi: &mut ScrollbarInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetScrollBarInfo(hwnd: HWnd, idObject: ObjectIdentifier, psbi: &mut ScrollbarInfo) -> Bool;
		} 
		let _ret_ = GetScrollBarInfo(hwnd, id_object, psbi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetAncestor(hwnd: HWnd, ga_flags: GetAncestorFlags) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetAncestor(hwnd: HWnd, gaFlags: GetAncestorFlags) -> *const c_void;
		} 
		let _ret_ = GetAncestor(hwnd, ga_flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn RealChildWindowFromPoint(hwnd_parent: HWnd, pt_parent_client_coords: Point) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RealChildWindowFromPoint(hwndParent: HWnd, ptParentClientCoords: Point) -> *const c_void;
		} 
		let _ret_ = RealChildWindowFromPoint(hwnd_parent, pt_parent_client_coords);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub unsafe fn RealGetWindowClassA() { todo!() }

pub unsafe fn RealGetWindowClassW() { todo!() }

pub unsafe fn GetAltTabInfoA() { todo!() }

pub unsafe fn GetAltTabInfoW() { todo!() }

pub fn ChangeWindowMessageFilter(message: u32, flag: ChangeWindowMessageFilterFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChangeWindowMessageFilter(message: u32, dwFlag: ChangeWindowMessageFilterFlags) -> Bool;
		} 
		let _ret_ = ChangeWindowMessageFilter(message, flag);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub unsafe fn ChangeWindowMessageFilterEx() { todo!() }

pub fn DestroyResourceIndexer(resource_indexer: *const ()) -> () {
	unsafe {
		#[link(name = "MrmSupport")]
		extern "system" {
			fn DestroyResourceIndexer(resourceIndexer: *const c_void) -> ();
		} 
		let _ret_ = DestroyResourceIndexer(resource_indexer as _);
	}
}

pub unsafe fn IndexFilePath() { todo!() }

pub fn DestroyIndexedResults(resource_uri: Option<&str>, qualifiers: Option<&[IndexedResourceQualifier]>) -> () {
	unsafe {
		#[link(name = "MrmSupport")]
		extern "system" {
			fn DestroyIndexedResults(resourceUri: *const u16, qualifierCount: u32, qualifiers: *const IndexedResourceQualifier) -> ();
		} 
		let (qualifiers_ptr_, qualifiers_len_) = qualifiers.deconstruct();
		let _ret_ = DestroyIndexedResults(resource_uri.map(str::to_utf16).as_ptr_or_null(), qualifiers_len_ as u32, qualifiers_ptr_);
	}
}


pub type WndProc 
	= unsafe extern "system" fn(param0: HWnd, param1: WindowMessage, param2: WParam, param3: LParam, ) -> LResult;

pub type DlgProc 
	= unsafe extern "system" fn(param0: HWnd, param1: u32, param2: WParam, param3: LParam, ) -> NonZeroUsize;

pub type TimerProc 
	= unsafe extern "system" fn(param0: HWnd, param1: u32, param2: usize, param3: u32, ) -> ();

pub type WndEnumProc 
	= unsafe extern "system" fn(param0: HWnd, param1: LParam, ) -> Bool;

pub type HookProc 
	= unsafe extern "system" fn(code: i32, w_param: WParam, l_param: LParam, ) -> LResult;

pub type SendAsyncProc 
	= unsafe extern "system" fn(param0: HWnd, param1: u32, param2: usize, param3: LResult, ) -> ();

pub type PropEnumProcA 
	= unsafe extern "system" fn(param0: HWnd, param1: PStr, param2: Handle, ) -> Bool;

pub type PropEnumProcW 
	= unsafe extern "system" fn(param0: HWnd, param1: PWStr, param2: Handle, ) -> Bool;

pub type PropEnumProcExA 
	= unsafe extern "system" fn(param0: HWnd, param1: PStr, param2: Handle, param3: usize, ) -> Bool;

pub type PropEnumProcExW 
	= unsafe extern "system" fn(param0: HWnd, param1: PWStr, param2: Handle, param3: usize, ) -> Bool;

pub type NameEnumProcA 
	= unsafe extern "system" fn(param0: PStr, param1: LParam, ) -> Bool;

pub type NameEnumProcW 
	= unsafe extern "system" fn(param0: PWStr, param1: LParam, ) -> Bool;

pub type PRegisterClassNameW 
	= unsafe extern "system" fn(param0: PWStr, ) -> bool;

pub type MsgBoxCallback 
	= unsafe extern "system" fn(help_info: &mut HelpInfo, ) -> ();

