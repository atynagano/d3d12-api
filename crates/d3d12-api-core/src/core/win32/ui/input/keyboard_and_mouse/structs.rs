#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::ui::input::keyboard_and_mouse::*;
use crate::core::win32::foundation::*;

/// VK_TO_BIT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToBit {
	pub vk: u8,
	pub mod_bits: u8,
}

/// MODIFIERS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Modifiers<'a> {
	pub vk_to_bit: &'a VkToBit,
	pub max_mod_bits: u16,
	pub mod_number: [u8; 1],
}

/// VSC_VK
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VscVk {
	pub vsc: u8,
	pub vk: u16,
}

/// VK_VSC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkVsc {
	pub vk: u8,
	pub vsc: u8,
}

/// VK_TO_WCHARS1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars1 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 1],
}

/// VK_TO_WCHARS2
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars2 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 2],
}

/// VK_TO_WCHARS3
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars3 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 3],
}

/// VK_TO_WCHARS4
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars4 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 4],
}

/// VK_TO_WCHARS5
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars5 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 5],
}

/// VK_TO_WCHARS6
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars6 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 6],
}

/// VK_TO_WCHARS7
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars7 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 7],
}

/// VK_TO_WCHARS8
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars8 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 8],
}

/// VK_TO_WCHARS9
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars9 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 9],
}

/// VK_TO_WCHARS10
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWChars10 {
	pub virtual_key: u8,
	pub attributes: u8,
	pub wch: [u16; 10],
}

/// VK_TO_WCHAR_TABLE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToWCharTable<'a> {
	pub vk_to_wchars: &'a VkToWChars1,
	pub modifications: u8,
	pub size: u8,
}

/// DEADKEY
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeadKey {
	pub both: u32,
	pub wch_composed: u16,
	pub u_flags: u16,
}

/// LIGATURE1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ligature1 {
	pub virtual_key: u8,
	pub modification_number: u16,
	pub wch: [u16; 1],
}

/// LIGATURE2
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ligature2 {
	pub virtual_key: u8,
	pub modification_number: u16,
	pub wch: [u16; 2],
}

/// LIGATURE3
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ligature3 {
	pub virtual_key: u8,
	pub modification_number: u16,
	pub wch: [u16; 3],
}

/// LIGATURE4
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ligature4 {
	pub virtual_key: u8,
	pub modification_number: u16,
	pub wch: [u16; 4],
}

/// LIGATURE5
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ligature5 {
	pub virtual_key: u8,
	pub modification_number: u16,
	pub wch: [u16; 5],
}

/// VSC_LPWSTR
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VscLPWStr<'a> {
	pub vsc: u8,
	pub pwsz: PWStr<'a>,
}

/// tagKbdLayer
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct tagKbdLayer<'a> {
	pub char_modifiers: &'a Modifiers<'a>,
	pub vk_to_wchar_table: &'a VkToWCharTable<'a>,
	pub dead_key: &'a DeadKey,
	pub key_names: &'a VscLPWStr<'a>,
	pub key_names_ext: &'a VscLPWStr<'a>,
	pub key_names_dead: &'a &'a u16,
	pub pus_vs_cto_vk: &'a u16,
	pub max_vs_cto_vk: u8,
	pub vs_cto_vk_e0: &'a VscVk,
	pub vs_cto_vk_e1: &'a VscVk,
	pub f_locale_flags: u32,
	pub lg_max: u8,
	pub lg_entry: u8,
	pub ligature: &'a Ligature1,
	pub ty: u32,
	pub sub_type: u32,
}

/// _VK_FUNCTION_PARAM
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkFunctionParam {
	pub nlsfe_proc_index: u8,
	pub nlsfe_proc_param: u32,
}

/// _VK_TO_FUNCTION_TABLE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkToFunctionTable {
	pub vk: u8,
	pub nlsfe_proc_type: u8,
	pub nlsfe_proc_current: u8,
	pub nlsfe_proc_switch: u8,
	pub nlsfe_proc: [VkFunctionParam; 8],
	pub nlsfe_proc_alt: [VkFunctionParam; 8],
}

/// tagKbdNlsLayer
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct tagKbdNlsLayer<'a> {
	pub oem_identifier: u16,
	pub layout_information: u16,
	pub num_of_vk_to_f: u32,
	pub vk_to_f: &'a VkToFunctionTable,
	pub num_of_mouse_v_key: i32,
	pub pus_mouse_v_key: &'a u16,
}

/// KBDTABLE_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct KbdTableDesc {
	pub wsz_dll_name: [u16; 32],
	pub ty: u32,
	pub sub_type: u32,
}

/// KBDTABLE_MULTI
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct KbdTableMulti {
	pub tables: u32,
	pub a_kbd_tables: [KbdTableDesc; 8],
}

/// KBD_TYPE_INFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct KbdTypeInfo {
	pub version: u32,
	pub ty: u32,
	pub sub_type: u32,
}

/// MOUSEMOVEPOINT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MouseMovePoint {
	pub x: i32,
	pub y: i32,
	pub time: u32,
	pub extra_info: usize,
}

/// TRACKMOUSEEVENT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TrackMouseEvent {
	pub size: u32,
	pub flags: TrackMouseEventFlags,
	pub hwnd_track: HWnd,
	pub hover_time: u32,
}

/// MOUSEINPUT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MouseInput {
	pub dx: i32,
	pub dy: i32,
	pub mouse_data: i32,
	pub flags: MouseEventFlags,
	pub time: u32,
	pub extra_info: usize,
}

/// KEYBDINPUT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct KeyBDInput {
	pub vk: VirtualKey,
	pub scan: u16,
	pub flags: KeyBDEventFlags,
	pub time: u32,
	pub extra_info: usize,
}

/// HARDWAREINPUT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HardwareInput {
	pub u_msg: u32,
	pub param_l: u16,
	pub param_h: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union InputAnonymousUnion {
	pub mi: MouseInput,
	pub ki: KeyBDInput,
	pub hi: HardwareInput,
}

impl std::fmt::Debug for InputAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// INPUT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Input {
	pub ty: InputType,
	pub anonymous: InputAnonymousUnion,
}

/// LASTINPUTINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LastInputInfo {
	pub size: u32,
	pub time: u32,
}

