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
use crate::core::win32::ui::input::keyboard_and_mouse::*;
use crate::core::win32::ui::text_services::*;


pub fn _TrackMouseEvent(event_track: &mut TrackMouseEvent) -> bool {
	unsafe {
		#[link(name = "COMCTL32")]
		extern "system" {
			fn _TrackMouseEvent(lpEventTrack: &mut TrackMouseEvent) -> Bool;
		} 
		let _ret_ = _TrackMouseEvent(event_track);
		_ret_.to_bool()
	}
}

pub fn LoadKeyboardLayoutA(pwsz_klid: &str, flags: ActivateKeyboardLayoutFlags) -> Result<Hkl, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadKeyboardLayoutA(pwszKLID: *const u8, Flags: ActivateKeyboardLayoutFlags) -> *const c_void;
		} 
		let _ret_ = LoadKeyboardLayoutA(pwsz_klid.to_null_terminated().as_ptr_or_null(), flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadKeyboardLayoutW(pwsz_klid: &str, flags: ActivateKeyboardLayoutFlags) -> Result<Hkl, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadKeyboardLayoutW(pwszKLID: *const u16, Flags: ActivateKeyboardLayoutFlags) -> *const c_void;
		} 
		let _ret_ = LoadKeyboardLayoutW(pwsz_klid.to_utf16().as_ptr_or_null(), flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn ActivateKeyboardLayout(hkl: Hkl, flags: ActivateKeyboardLayoutFlags) -> Result<Hkl, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ActivateKeyboardLayout(hkl: Hkl, Flags: ActivateKeyboardLayoutFlags) -> *const c_void;
		} 
		let _ret_ = ActivateKeyboardLayout(hkl, flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub unsafe fn ToUnicodeEx() { todo!() }

pub fn UnloadKeyboardLayout(hkl: Hkl) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UnloadKeyboardLayout(hkl: Hkl) -> Bool;
		} 
		let _ret_ = UnloadKeyboardLayout(hkl);
		_ret_.to_bool()
	}
}

pub unsafe fn GetKeyboardLayoutNameA() { todo!() }

pub unsafe fn GetKeyboardLayoutNameW() { todo!() }

pub unsafe fn GetKeyboardLayoutList() { todo!() }

pub fn GetKeyboardLayout(id_thread: u32) -> Result<Hkl, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetKeyboardLayout(idThread: u32) -> *const c_void;
		} 
		let _ret_ = GetKeyboardLayout(id_thread);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub unsafe fn GetMouseMovePointsEx() { todo!() }

pub fn TrackMouseEvent(event_track: &mut TrackMouseEvent) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TrackMouseEvent(lpEventTrack: &mut TrackMouseEvent) -> Bool;
		} 
		let _ret_ = TrackMouseEvent(event_track);
		_ret_.to_bool()
	}
}

pub fn RegisterHotKey(wnd: Option<HWnd>, id: i32, fs_modifiers: HotKeyModifiers, vk: u32) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RegisterHotKey(hWnd: *const c_void, id: i32, fsModifiers: HotKeyModifiers, vk: u32) -> Bool;
		} 
		let _ret_ = RegisterHotKey(transmute(wnd), id, fs_modifiers, vk);
		_ret_.to_bool()
	}
}

pub fn UnregisterHotKey(wnd: Option<HWnd>, id: i32) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UnregisterHotKey(hWnd: *const c_void, id: i32) -> Bool;
		} 
		let _ret_ = UnregisterHotKey(transmute(wnd), id);
		_ret_.to_bool()
	}
}

pub fn SwapMouseButton(f_swap: bool) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SwapMouseButton(fSwap: Bool) -> Bool;
		} 
		let _ret_ = SwapMouseButton(f_swap.to_bool());
		_ret_.to_bool()
	}
}

pub fn GetDoubleClickTime() -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDoubleClickTime() -> u32;
		} 
		let _ret_ = GetDoubleClickTime();
		_ret_
	}
}

pub fn SetDoubleClickTime(param0: u32) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetDoubleClickTime(param0: u32) -> Bool;
		} 
		let _ret_ = SetDoubleClickTime(param0);
		_ret_.to_bool()
	}
}

pub fn SetFocus(wnd: Option<HWnd>) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetFocus(hWnd: *const c_void) -> *const c_void;
		} 
		let _ret_ = SetFocus(transmute(wnd));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetActiveWindow() -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetActiveWindow() -> *const c_void;
		} 
		let _ret_ = GetActiveWindow();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetFocus() -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetFocus() -> *const c_void;
		} 
		let _ret_ = GetFocus();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetKBCodePage() -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetKBCodePage() -> u32;
		} 
		let _ret_ = GetKBCodePage();
		_ret_
	}
}

pub fn GetKeyState(virt_key: i32) -> i16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetKeyState(nVirtKey: i32) -> i16;
		} 
		let _ret_ = GetKeyState(virt_key);
		_ret_
	}
}

pub fn GetAsyncKeyState(v_key: i32) -> i16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetAsyncKeyState(vKey: i32) -> i16;
		} 
		let _ret_ = GetAsyncKeyState(v_key);
		_ret_
	}
}

pub unsafe fn GetKeyboardState() { todo!() }

pub fn SetKeyboardState(key_state: &[u8; 256]) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetKeyboardState(lpKeyState: *const u8) -> Bool;
		} 
		let _ret_ = SetKeyboardState(key_state.as_ptr_or_null());
		_ret_.to_bool()
	}
}

pub unsafe fn GetKeyNameTextA() { todo!() }

pub unsafe fn GetKeyNameTextW() { todo!() }

pub fn GetKeyboardType(type_flag: i32) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetKeyboardType(nTypeFlag: i32) -> i32;
		} 
		let _ret_ = GetKeyboardType(type_flag);
		_ret_
	}
}

pub fn ToAscii(u_virt_key: u32, u_scan_code: u32, key_state: Option<&[u8; 256]>, u_flags: u32) -> (i32, u16) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ToAscii(uVirtKey: u32, uScanCode: u32, lpKeyState: *const u8, lpChar: *mut u16, uFlags: u32) -> i32;
		} 
		let mut char_out_: MaybeUninit<u16> = MaybeUninit::zeroed();
		let _ret_ = ToAscii(u_virt_key, u_scan_code, key_state.as_ptr_or_null(), char_out_.as_mut_ptr(), u_flags);
		(_ret_, char_out_.assume_init())
	}
}

pub fn ToAsciiEx(u_virt_key: u32, u_scan_code: u32, key_state: Option<&[u8; 256]>, u_flags: u32, dwhkl: Option<Hkl>) -> (i32, u16) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ToAsciiEx(uVirtKey: u32, uScanCode: u32, lpKeyState: *const u8, lpChar: *mut u16, uFlags: u32, dwhkl: *const c_void) -> i32;
		} 
		let mut char_out_: MaybeUninit<u16> = MaybeUninit::zeroed();
		let _ret_ = ToAsciiEx(u_virt_key, u_scan_code, key_state.as_ptr_or_null(), char_out_.as_mut_ptr(), u_flags, transmute(dwhkl));
		(_ret_, char_out_.assume_init())
	}
}

pub unsafe fn ToUnicode() { todo!() }

pub fn OemKeyScan(oem_char: u16) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn OemKeyScan(wOemChar: u16) -> u32;
		} 
		let _ret_ = OemKeyScan(oem_char);
		_ret_
	}
}

pub fn VkKeyScanA(ch: Char) -> i16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn VkKeyScanA(ch: Char) -> i16;
		} 
		let _ret_ = VkKeyScanA(ch);
		_ret_
	}
}

pub fn VkKeyScanW(ch: u16) -> i16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn VkKeyScanW(ch: u16) -> i16;
		} 
		let _ret_ = VkKeyScanW(ch);
		_ret_
	}
}

pub fn VkKeyScanExA(ch: Char, dwhkl: Hkl) -> i16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn VkKeyScanExA(ch: Char, dwhkl: Hkl) -> i16;
		} 
		let _ret_ = VkKeyScanExA(ch, dwhkl);
		_ret_
	}
}

pub fn VkKeyScanExW(ch: u16, dwhkl: Hkl) -> i16 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn VkKeyScanExW(ch: u16, dwhkl: Hkl) -> i16;
		} 
		let _ret_ = VkKeyScanExW(ch, dwhkl);
		_ret_
	}
}

pub fn Keybd_event(vk: u8, scan: u8, flags: KeyBDEventFlags, extra_info: usize) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn keybd_event(bVk: u8, bScan: u8, dwFlags: KeyBDEventFlags, dwExtraInfo: usize) -> ();
		} 
		let _ret_ = keybd_event(vk, scan, flags, extra_info);
	}
}

pub fn Mouse_event(flags: MouseEventFlags, dx: i32, dy: i32, data: u32, extra_info: usize) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn mouse_event(dwFlags: MouseEventFlags, dx: i32, dy: i32, dwData: u32, dwExtraInfo: usize) -> ();
		} 
		let _ret_ = mouse_event(flags, dx, dy, data, extra_info);
	}
}

pub fn SendInput(inputs: &[Input], size: i32) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SendInput(cInputs: u32, pInputs: *const Input, cbSize: i32) -> u32;
		} 
		let (inputs_ptr_, inputs_len_) = inputs.deconstruct();
		let _ret_ = SendInput(inputs_len_ as u32, inputs_ptr_, size);
		_ret_
	}
}

pub fn GetLastInputInfo(plii: &mut LastInputInfo) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetLastInputInfo(plii: &mut LastInputInfo) -> Bool;
		} 
		let _ret_ = GetLastInputInfo(plii);
		_ret_.to_bool()
	}
}

pub fn MapVirtualKeyA(u_code: u32, u_map_type: u32) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MapVirtualKeyA(uCode: u32, uMapType: u32) -> u32;
		} 
		let _ret_ = MapVirtualKeyA(u_code, u_map_type);
		_ret_
	}
}

pub fn MapVirtualKeyW(u_code: u32, u_map_type: u32) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MapVirtualKeyW(uCode: u32, uMapType: u32) -> u32;
		} 
		let _ret_ = MapVirtualKeyW(u_code, u_map_type);
		_ret_
	}
}

pub fn MapVirtualKeyExA(u_code: u32, u_map_type: u32, dwhkl: Option<Hkl>) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MapVirtualKeyExA(uCode: u32, uMapType: u32, dwhkl: *const c_void) -> u32;
		} 
		let _ret_ = MapVirtualKeyExA(u_code, u_map_type, transmute(dwhkl));
		_ret_
	}
}

pub fn MapVirtualKeyExW(u_code: u32, u_map_type: u32, dwhkl: Option<Hkl>) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MapVirtualKeyExW(uCode: u32, uMapType: u32, dwhkl: *const c_void) -> u32;
		} 
		let _ret_ = MapVirtualKeyExW(u_code, u_map_type, transmute(dwhkl));
		_ret_
	}
}

pub fn GetCapture() -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetCapture() -> *const c_void;
		} 
		let _ret_ = GetCapture();
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetCapture(wnd: HWnd) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetCapture(hWnd: HWnd) -> *const c_void;
		} 
		let _ret_ = SetCapture(wnd);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn ReleaseCapture() -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ReleaseCapture() -> Bool;
		} 
		let _ret_ = ReleaseCapture();
		_ret_.to_bool()
	}
}

pub fn EnableWindow(wnd: HWnd, enable: bool) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnableWindow(hWnd: HWnd, bEnable: Bool) -> Bool;
		} 
		let _ret_ = EnableWindow(wnd, enable.to_bool());
		_ret_.to_bool()
	}
}

pub fn IsWindowEnabled(wnd: HWnd) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsWindowEnabled(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = IsWindowEnabled(wnd);
		_ret_.to_bool()
	}
}

pub fn DragDetect(hwnd: HWnd, pt: Point) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DragDetect(hwnd: HWnd, pt: Point) -> Bool;
		} 
		let _ret_ = DragDetect(hwnd, pt);
		_ret_.to_bool()
	}
}

pub fn SetActiveWindow(wnd: HWnd) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetActiveWindow(hWnd: HWnd) -> *const c_void;
		} 
		let _ret_ = SetActiveWindow(wnd);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn BlockInput(f_block_it: bool) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn BlockInput(fBlockIt: Bool) -> Bool;
		} 
		let _ret_ = BlockInput(f_block_it.to_bool());
		_ret_.to_bool()
	}
}


