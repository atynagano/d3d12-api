#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::foundation::*;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ByHandleFileInformation {
	pub file_attributes: u32,
	pub ft_creation_time: FileTime,
	pub ft_last_access_time: FileTime,
	pub ft_last_write_time: FileTime,
	pub volume_serial_number: u32,
	pub file_size_high: u32,
	pub file_size_low: u32,
	pub number_of_links: u32,
	pub file_index_high: u32,
	pub file_index_low: u32,
}

