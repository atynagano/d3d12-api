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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::foundation::*;

/// DXGI_SAMPLE_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiSampleDesc {
	pub count: u32,
	pub quality: u32,
}

/// DXGI_MODE_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiModeDesc {
	pub width: u32,
	pub height: u32,
	pub refresh_rate: DxgiRational,
	pub format: DxgiFormat,
	pub scanline_ordering: DxgiModeScanLineOrder,
	pub scaling: DxgiModeScaling,
}

/// DXGI_RATIONAL
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiRational {
	pub numerator: u32,
	pub denominator: u32,
}

/// DXGI_GAMMA_CONTROL_CAPABILITIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiGammaControlCapabilities {
	pub scale_and_offset_supported: Bool,
	pub max_converted_value: f32,
	pub min_converted_value: f32,
	pub num_gamma_control_points: u32,
	pub control_point_positions: [f32; 1025],
}

/// DXGI_GAMMA_CONTROL
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiGammaControl {
	pub scale: DxgiRgb,
	pub offset: DxgiRgb,
	pub gamma_curve: [DxgiRgb; 1025],
}

/// DXGI_RGB
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiRgb {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
}

