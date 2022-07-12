#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::foundation::*;
use crate::core::win32::ui::windows_and_messaging::*;
use crate::core::win32::graphics::gdi::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CreateStructA<'a> {
	pub create_params: &'a c_void,
	pub instance: HInstance,
	pub menu: HMenu,
	pub hwnd_parent: HWnd,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: i32,
	pub name: PStr<'a>,
	pub class: PStr<'a>,
	pub ex_style: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HMenu {
	pub value: NonNull<()>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Msg {
	pub hwnd: Option<HWnd>,
	pub message: WindowMessage,
	pub w_param: WParam,
	pub l_param: LParam,
	pub time: u32,
	pub pt: Point,
}

#[repr(C)]
pub struct WndClassExA<'a> {
	pub size: u32,
	pub style: WndClassStyles,
	pub wnd_proc: WndProc,
	pub cls_extra: i32,
	pub wnd_extra: i32,
	pub instance: HInstance,
	pub icon: Option<HIcon>,
	pub cursor: Option<HCursor>,
	pub hbr_background: Option<HBrush>,
	pub menu_name: Option<PStr<'a>>,
	pub class_name: PStr<'a>,
	pub icon_sm: Option<HIcon>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HIcon {
	pub value: NonNull<()>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HCursor {
	pub value: NonNull<()>,
}

