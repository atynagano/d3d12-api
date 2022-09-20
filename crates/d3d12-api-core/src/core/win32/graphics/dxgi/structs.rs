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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::gdi::*;
use crate::core::win32::graphics::dxgi::*;

/// DXGI_RGBA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiRgba {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

/// DXGI_FRAME_STATISTICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiFrameStatistics {
	pub present_count: u32,
	pub present_refresh_count: u32,
	pub sync_refresh_count: u32,
	pub sync_qpc_time: i64,
	pub sync_gpu_time: i64,
}

/// DXGI_MAPPED_RECT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiMappedRect<'a> {
	pub pitch: i32,
	pub bits: &'a u8,
}

/// DXGI_ADAPTER_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiAdapterDesc {
	pub description: [u16; 128],
	pub vendor_id: u32,
	pub device_id: u32,
	pub sub_sys_id: u32,
	pub revision: u32,
	pub dedicated_video_memory: usize,
	pub dedicated_system_memory: usize,
	pub shared_system_memory: usize,
	pub adapter_luid: Luid,
}

/// DXGI_OUTPUT_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiOutputDesc {
	pub device_name: [u16; 32],
	pub desktop_coordinates: Rect,
	pub attached_to_desktop: Bool,
	pub rotation: DxgiModeRotation,
	pub monitor: HMonitor,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct DxgiSharedResource(pub Handle);

/// DXGI_SURFACE_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiSurfaceDesc {
	pub width: u32,
	pub height: u32,
	pub format: DxgiFormat,
	pub sample_desc: DxgiSampleDesc,
}

/// DXGI_SWAP_CHAIN_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiSwapChainDesc {
	pub buffer_desc: DxgiModeDesc,
	pub sample_desc: DxgiSampleDesc,
	pub buffer_usage: DxgiUsage,
	pub buffer_count: u32,
	pub output_window: HWnd,
	pub windowed: Bool,
	pub swap_effect: DxgiSwapEffect,
	pub flags: DxgiSwapChainFlag,
}

/// DXGI_ADAPTER_DESC1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DxgiAdapterDesc1 {
	pub base: DxgiAdapterDesc,
	pub flags: DxgiAdapterFlag,
}

impl Deref for DxgiAdapterDesc1 {
	type Target = DxgiAdapterDesc;
	fn deref(&self) -> &Self::Target { &self.base }
}

impl DerefMut for DxgiAdapterDesc1 {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

/// DXGI_DISPLAY_COLOR_SPACE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiDisplayColorSpace {
	pub primary_coordinates: [f32; 16],
	pub white_points: [f32; 32],
}

/// DXGI_OUTDUPL_MOVE_RECT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiOutDuplMoveRect {
	pub source_point: Point,
	pub destination_rect: Rect,
}

/// DXGI_OUTDUPL_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiOutDuplDesc {
	pub mode_desc: DxgiModeDesc,
	pub rotation: DxgiModeRotation,
	pub desktop_image_in_system_memory: Bool,
}

/// DXGI_OUTDUPL_POINTER_POSITION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiOutDuplPointerPosition {
	pub position: Point,
	pub visible: Bool,
}

/// DXGI_OUTDUPL_POINTER_SHAPE_INFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiOutDuplPointerShapeInfo {
	pub ty: u32,
	pub width: u32,
	pub height: u32,
	pub pitch: u32,
	pub hot_spot: Point,
}

/// DXGI_OUTDUPL_FRAME_INFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiOutDuplFrameInfo {
	pub last_present_time: i64,
	pub last_mouse_update_time: i64,
	pub accumulated_frames: u32,
	pub rects_coalesced: Bool,
	pub protected_content_masked_out: Bool,
	pub pointer_position: DxgiOutDuplPointerPosition,
	pub total_metadata_buffer_size: u32,
	pub pointer_shape_buffer_size: u32,
}

/// DXGI_MODE_DESC1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiModeDesc1 {
	pub width: u32,
	pub height: u32,
	pub refresh_rate: DxgiRational,
	pub format: DxgiFormat,
	pub scanline_ordering: DxgiModeScanLineOrder,
	pub scaling: DxgiModeScaling,
	pub stereo: Bool,
}

/// DXGI_SWAP_CHAIN_DESC1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiSwapChainDesc1 {
	pub width: u32,
	pub height: u32,
	pub format: DxgiFormat,
	pub stereo: Bool,
	pub sample_desc: DxgiSampleDesc,
	pub buffer_usage: DxgiUsage,
	pub buffer_count: u32,
	pub scaling: DxgiScaling,
	pub swap_effect: DxgiSwapEffect,
	pub alpha_mode: DxgiAlphaMode,
	pub flags: DxgiSwapChainFlag,
}

/// DXGI_SWAP_CHAIN_FULLSCREEN_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiSwapChainFullScreenDesc {
	pub refresh_rate: DxgiRational,
	pub scanline_ordering: DxgiModeScanLineOrder,
	pub scaling: DxgiModeScaling,
	pub windowed: Bool,
}

/// DXGI_PRESENT_PARAMETERS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiPresentParameters<'a> {
	pub dirty_rects_count: u32,
	pub dirty_rects: &'a Rect,
	pub scroll_rect: &'a Rect,
	pub scroll_offset: &'a Point,
}

/// DXGI_ADAPTER_DESC2
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiAdapterDesc2 {
	pub description: [u16; 128],
	pub vendor_id: u32,
	pub device_id: u32,
	pub sub_sys_id: u32,
	pub revision: u32,
	pub dedicated_video_memory: usize,
	pub dedicated_system_memory: usize,
	pub shared_system_memory: usize,
	pub adapter_luid: Luid,
	pub flags: u32,
	pub graphics_preemption_granularity: DxgiGraphicsPreemptionGranularity,
	pub compute_preemption_granularity: DxgiComputePreemptionGranularity,
}

/// DXGI_MATRIX_3X2_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiMatrix3x2F {
	pub _11: f32,
	pub _12: f32,
	pub _21: f32,
	pub _22: f32,
	pub _31: f32,
	pub _32: f32,
}

/// DXGI_DECODE_SWAP_CHAIN_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiDecodeSwapChainDesc {
	pub flags: u32,
}

/// DXGI_FRAME_STATISTICS_MEDIA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiFrameStatisticsMedia {
	pub present_count: u32,
	pub present_refresh_count: u32,
	pub sync_refresh_count: u32,
	pub sync_qpc_time: i64,
	pub sync_gpu_time: i64,
	pub composition_mode: DxgiFramePresentationMode,
	pub approved_present_duration: u32,
}

/// DXGI_QUERY_VIDEO_MEMORY_INFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiQueryVideoMemoryInfo {
	pub budget: u64,
	pub current_usage: u64,
	pub available_for_reservation: u64,
	pub current_reservation: u64,
}

/// DXGI_HDR_METADATA_HDR10
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiHdrMetaDataHdr10 {
	pub red_primary: [u16; 2],
	pub green_primary: [u16; 2],
	pub blue_primary: [u16; 2],
	pub white_point: [u16; 2],
	pub max_mastering_luminance: u32,
	pub min_mastering_luminance: u32,
	pub max_content_light_level: u16,
	pub max_frame_average_light_level: u16,
}

/// DXGI_HDR_METADATA_HDR10PLUS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiHdrMetaDataHdr10Plus {
	pub data: [u8; 72],
}

/// DXGI_ADAPTER_DESC3
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiAdapterDesc3 {
	pub description: [u16; 128],
	pub vendor_id: u32,
	pub device_id: u32,
	pub sub_sys_id: u32,
	pub revision: u32,
	pub dedicated_video_memory: usize,
	pub dedicated_system_memory: usize,
	pub shared_system_memory: usize,
	pub adapter_luid: Luid,
	pub flags: DxgiAdapterFlag3,
	pub graphics_preemption_granularity: DxgiGraphicsPreemptionGranularity,
	pub compute_preemption_granularity: DxgiComputePreemptionGranularity,
}

/// DXGI_OUTPUT_DESC1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiOutputDesc1 {
	pub base: DxgiOutputDesc,
	pub bits_per_color: u32,
	pub color_space: DxgiColorSpaceType,
	pub red_primary: [f32; 2],
	pub green_primary: [f32; 2],
	pub blue_primary: [f32; 2],
	pub white_point: [f32; 2],
	pub min_luminance: f32,
	pub max_luminance: f32,
	pub max_full_frame_luminance: f32,
}

impl Deref for DxgiOutputDesc1 {
	type Target = DxgiOutputDesc;
	fn deref(&self) -> &Self::Target { &self.base }
}

impl DerefMut for DxgiOutputDesc1 {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

/// DXGI_INFO_QUEUE_MESSAGE
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct RawDxgiInfoQueueMessage<'a> {
	pub producer: GUID,
	pub category: DxgiInfoQueueMessageCategory,
	pub severity: DxgiInfoQueueMessageSeverity,
	pub id: i32,
	pub description: &'a u8,
	pub description_byte_length: usize,
}

/// DXGI_INFO_QUEUE_FILTER_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiInfoQueueFilterDesc<'a> {
	pub num_categories: u32,
	pub category_list: &'a DxgiInfoQueueMessageCategory,
	pub num_severities: u32,
	pub severity_list: &'a DxgiInfoQueueMessageSeverity,
	pub num_i_ds: u32,
	pub id_list: &'a i32,
}

/// DXGI_INFO_QUEUE_FILTER
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxgiInfoQueueFilter<'a> {
	pub allow_list: DxgiInfoQueueFilterDesc<'a>,
	pub deny_list: DxgiInfoQueueFilterDesc<'a>,
}

