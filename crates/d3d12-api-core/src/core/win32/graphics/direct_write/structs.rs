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
use crate::core::win32::graphics::direct_write::*;
use crate::core::win32::foundation::*;

/// DWRITE_GLYPH_RUN
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteGlyphRun<'a> {
	pub font_face: Param<'a, DWriteFontFace>,
	pub font_em_size: f32,
	pub glyph_count: u32,
	pub glyph_indices: &'a u16,
	pub glyph_advances: &'a f32,
	pub glyph_offsets: &'a DWriteGlyphOffset,
	pub is_sideways: Bool,
	pub bidi_level: u32,
}

/// DWRITE_GLYPH_OFFSET
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteGlyphOffset {
	pub advance_offset: f32,
	pub ascender_offset: f32,
}

/// DWRITE_GLYPH_RUN_DESCRIPTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteGlyphRunDescription<'a> {
	pub locale_name: PWStr<'a>,
	pub string: PWStr<'a>,
	pub string_length: u32,
	pub cluster_map: &'a u16,
	pub text_position: u32,
}

/// DWRITE_TRIMMING
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteTrimming {
	pub granularity: DWriteTrimmingGranularity,
	pub delimiter: u32,
	pub delimiter_count: u32,
}

/// DWRITE_TEXT_RANGE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteTextRange {
	pub start_position: u32,
	pub length: u32,
}

/// DWRITE_LINE_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteLineMetrics {
	pub length: u32,
	pub trailing_whitespace_length: u32,
	pub newline_length: u32,
	pub height: f32,
	pub baseline: f32,
	pub is_trimmed: Bool,
}

/// DWRITE_TEXT_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteTextMetrics {
	pub left: f32,
	pub top: f32,
	pub width: f32,
	pub width_including_trailing_whitespace: f32,
	pub height: f32,
	pub layout_width: f32,
	pub layout_height: f32,
	pub max_bidi_reordering_depth: u32,
	pub line_count: u32,
}

/// DWRITE_OVERHANG_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteOverhangMetrics {
	pub left: f32,
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
}

/// DWRITE_CLUSTER_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteClusterMetrics {
	pub width: f32,
	pub length: u16,
	pub bitfield: u16,
}

/// DWRITE_HIT_TEST_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteHitTestMetrics {
	pub text_position: u32,
	pub length: u32,
	pub left: f32,
	pub top: f32,
	pub width: f32,
	pub height: f32,
	pub bidi_level: u32,
	pub is_text: Bool,
	pub is_trimmed: Bool,
}

/// DWRITE_FONT_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteFontMetrics {
	pub design_units_per_em: u16,
	pub ascent: u16,
	pub descent: u16,
	pub line_gap: i16,
	pub cap_height: u16,
	pub x_height: u16,
	pub underline_position: i16,
	pub underline_thickness: u16,
	pub strikethrough_position: i16,
	pub strikethrough_thickness: u16,
}

/// DWRITE_GLYPH_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteGlyphMetrics {
	pub left_side_bearing: i32,
	pub advance_width: u32,
	pub right_side_bearing: i32,
	pub top_side_bearing: i32,
	pub advance_height: u32,
	pub bottom_side_bearing: i32,
	pub vertical_origin_y: i32,
}

/// DWRITE_MATRIX
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteMatrix {
	pub m11: f32,
	pub m12: f32,
	pub m21: f32,
	pub m22: f32,
	pub dx: f32,
	pub dy: f32,
}

/// DWRITE_INLINE_OBJECT_METRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteInlineObjectMetrics {
	pub width: f32,
	pub height: f32,
	pub baseline: f32,
	pub supports_sideways: Bool,
}

/// DWRITE_FONT_FEATURE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteFontFeature {
	pub name_tag: DWriteFontFeatureTag,
	pub parameter: u32,
}

/// DWRITE_UNDERLINE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteUnderline<'a> {
	pub width: f32,
	pub thickness: f32,
	pub offset: f32,
	pub run_height: f32,
	pub reading_direction: DWriteReadingDirection,
	pub flow_direction: DWriteFlowDirection,
	pub locale_name: PWStr<'a>,
	pub measuring_mode: DWriteMeasuringMode,
}

/// DWRITE_STRIKETHROUGH
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DWriteStrikeThrough<'a> {
	pub width: f32,
	pub thickness: f32,
	pub offset: f32,
	pub reading_direction: DWriteReadingDirection,
	pub flow_direction: DWriteFlowDirection,
	pub locale_name: PWStr<'a>,
	pub measuring_mode: DWriteMeasuringMode,
}

