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
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::gdi::*;
use crate::core::win32::graphics::dwm::*;

/// DWM_BLURBEHIND
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DwmBlurBehind {
	pub flags: u32,
	pub f_enable: Bool,
	pub rgn_blur: HRgn,
	pub f_transition_on_maximized: Bool,
}

/// DWM_THUMBNAIL_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DwmThumbnailProperties {
	pub flags: u32,
	pub rc_destination: Rect,
	pub rc_source: Rect,
	pub opacity: u8,
	pub f_visible: Bool,
	pub f_source_client_area_only: Bool,
}

/// UNSIGNED_RATIO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UnsignedRatio {
	pub ui_numerator: u32,
	pub ui_denominator: u32,
}

/// DWM_TIMING_INFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DwmTimingInfo {
	pub size: u32,
	pub rate_refresh: UnsignedRatio,
	pub qpc_refresh_period: u64,
	pub rate_compose: UnsignedRatio,
	pub qpc_v_blank: u64,
	pub refresh: u64,
	pub dx_refresh: u32,
	pub qpc_compose: u64,
	pub frame: u64,
	pub dx_present: u32,
	pub refresh_frame: u64,
	pub frame_submitted: u64,
	pub dx_present_submitted: u32,
	pub frame_confirmed: u64,
	pub dx_present_confirmed: u32,
	pub refresh_confirmed: u64,
	pub dx_refresh_confirmed: u32,
	pub frames_late: u64,
	pub frames_outstanding: u32,
	pub frame_displayed: u64,
	pub qpc_frame_displayed: u64,
	pub refresh_frame_displayed: u64,
	pub frame_complete: u64,
	pub qpc_frame_complete: u64,
	pub frame_pending: u64,
	pub qpc_frame_pending: u64,
	pub frames_displayed: u64,
	pub frames_complete: u64,
	pub frames_pending: u64,
	pub frames_available: u64,
	pub frames_dropped: u64,
	pub frames_missed: u64,
	pub refresh_next_displayed: u64,
	pub refresh_next_presented: u64,
	pub refreshes_displayed: u64,
	pub refreshes_presented: u64,
	pub refresh_started: u64,
	pub pixels_received: u64,
	pub pixels_drawn: u64,
	pub buffers_empty: u64,
}

/// DWM_PRESENT_PARAMETERS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DwmPresentParameters {
	pub size: u32,
	pub f_queue: Bool,
	pub refresh_start: u64,
	pub buffer: u32,
	pub f_use_source_rate: Bool,
	pub rate_source: UnsignedRatio,
	pub refreshes_per_frame: u32,
	pub e_sampling: DwmSourceFrameSampling,
}

/// MilMatrix3x2D
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MilMatrix3x2D {
	pub s_11: f64,
	pub s_12: f64,
	pub s_21: f64,
	pub s_22: f64,
	pub dx: f64,
	pub dy: f64,
}

