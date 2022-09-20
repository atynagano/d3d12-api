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

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HRgn(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HMonitor(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HBrush(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HDc(pub NonZeroUsize);

/// BLENDFUNCTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BlendFunction {
	pub blend_op: u8,
	pub blend_flags: u8,
	pub source_constant_alpha: u8,
	pub alpha_format: u8,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HBitmap(pub NonZeroUsize);

/// LOGFONTA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LogFontA {
	pub height: i32,
	pub width: i32,
	pub escapement: i32,
	pub orientation: i32,
	pub weight: i32,
	pub italic: u8,
	pub underline: u8,
	pub strike_out: u8,
	pub char_set: u8,
	pub out_precision: u8,
	pub clip_precision: u8,
	pub quality: u8,
	pub pitch_and_family: u8,
	pub face_name: [Char; 32],
}

/// LOGFONTW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LogFontW {
	pub height: i32,
	pub width: i32,
	pub escapement: i32,
	pub orientation: i32,
	pub weight: i32,
	pub italic: u8,
	pub underline: u8,
	pub strike_out: u8,
	pub char_set: u8,
	pub out_precision: u8,
	pub clip_precision: u8,
	pub quality: u8,
	pub pitch_and_family: u8,
	pub face_name: [u16; 32],
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HGdiObj(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HPalette(pub NonZeroUsize);

/// PALETTEENTRY
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PaletteEntry {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub flags: u8,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HMetaFile(pub NonZeroUsize);

/// BITMAP
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Bitmap<'a> {
	pub bm_type: i32,
	pub bm_width: i32,
	pub bm_height: i32,
	pub bm_width_bytes: i32,
	pub bm_planes: u16,
	pub bm_bits_pixel: u16,
	pub bm_bits: &'a c_void,
}

/// LOGBRUSH
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LogBrush {
	pub style: u32,
	pub color: u32,
	pub hatch: usize,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct CreatedHDC(pub NonZeroUsize);

/// DEVMODEA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DevModeA {
	pub device_name: [u8; 32],
	pub spec_version: u16,
	pub driver_version: u16,
	pub size: u16,
	pub driver_extra: u16,
	pub fields: u32,
	pub anonymous1: DevModeAAnonymous1Union,
	pub color: i16,
	pub duplex: i16,
	pub y_resolution: i16,
	pub tt_option: i16,
	pub collate: i16,
	pub form_name: [u8; 32],
	pub log_pixels: u16,
	pub bits_per_pel: u32,
	pub pels_width: u32,
	pub pels_height: u32,
	pub anonymous2: DevModeAAnonymous2Union,
	pub display_frequency: u32,
	pub icm_method: u32,
	pub icm_intent: u32,
	pub media_type: u32,
	pub dither_type: u32,
	pub reserved1: u32,
	pub reserved2: u32,
	pub panning_width: u32,
	pub panning_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DevModeAAnonymous1Union {
	pub anonymous1: DevModeAAnonymous1UnionAnonymous1Struct,
	pub anonymous2: DevModeAAnonymous1UnionAnonymous2Struct,
}

impl std::fmt::Debug for DevModeAAnonymous1Union {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// _Anonymous1_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DevModeAAnonymous1UnionAnonymous1Struct {
	pub orientation: i16,
	pub paper_size: i16,
	pub paper_length: i16,
	pub paper_width: i16,
	pub scale: i16,
	pub copies: i16,
	pub default_source: i16,
	pub print_quality: i16,
}

/// _Anonymous2_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DevModeAAnonymous1UnionAnonymous2Struct {
	pub position: POIntl,
	pub display_orientation: u32,
	pub display_fixed_output: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DevModeAAnonymous2Union {
	pub display_flags: u32,
	pub nup: u32,
}

impl std::fmt::Debug for DevModeAAnonymous2Union {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// DEVMODEW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DevModeW {
	pub device_name: [u16; 32],
	pub spec_version: u16,
	pub driver_version: u16,
	pub size: u16,
	pub driver_extra: u16,
	pub fields: u32,
	pub anonymous1: DevModeWAnonymous1Union,
	pub color: i16,
	pub duplex: i16,
	pub y_resolution: i16,
	pub tt_option: i16,
	pub collate: i16,
	pub form_name: [u16; 32],
	pub log_pixels: u16,
	pub bits_per_pel: u32,
	pub pels_width: u32,
	pub pels_height: u32,
	pub anonymous2: DevModeWAnonymous2Union,
	pub display_frequency: u32,
	pub icm_method: u32,
	pub icm_intent: u32,
	pub media_type: u32,
	pub dither_type: u32,
	pub reserved1: u32,
	pub reserved2: u32,
	pub panning_width: u32,
	pub panning_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DevModeWAnonymous1Union {
	pub anonymous1: DevModeWAnonymous1UnionAnonymous1Struct,
	pub anonymous2: DevModeWAnonymous1UnionAnonymous2Struct,
}

impl std::fmt::Debug for DevModeWAnonymous1Union {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// _Anonymous1_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DevModeWAnonymous1UnionAnonymous1Struct {
	pub orientation: i16,
	pub paper_size: i16,
	pub paper_length: i16,
	pub paper_width: i16,
	pub scale: i16,
	pub copies: i16,
	pub default_source: i16,
	pub print_quality: i16,
}

/// _Anonymous2_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DevModeWAnonymous1UnionAnonymous2Struct {
	pub position: POIntl,
	pub display_orientation: u32,
	pub display_fixed_output: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DevModeWAnonymous2Union {
	pub display_flags: u32,
	pub nup: u32,
}

impl std::fmt::Debug for DevModeWAnonymous2Union {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// BITMAPINFOHEADER
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BitmapInfoHeader {
	pub size: u32,
	pub width: i32,
	pub height: i32,
	pub planes: u16,
	pub bit_count: u16,
	pub compression: u32,
	pub size_image: u32,
	pub x_pels_per_meter: i32,
	pub y_pels_per_meter: i32,
	pub clr_used: u32,
	pub clr_important: u32,
}

/// BITMAPINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BitmapInfo {
	pub bmi_header: BitmapInfoHeader,
	pub bmi_colors: [RgbQuad; 1],
}

/// RGBQUAD
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RgbQuad {
	pub rgb_blue: u8,
	pub rgb_green: u8,
	pub rgb_red: u8,
	pub rgb_reserved: u8,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HFont(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HdcMetdataFileHandle(pub NonZeroUsize);

/// LOGPALETTE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LogPalette {
	pub pal_version: u16,
	pub pal_num_entries: u16,
	pub pal_pal_entry: [PaletteEntry; 1],
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HPen(pub NonZeroUsize);

/// LOGPEN
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LogPen {
	pub lopn_style: u32,
	pub lopn_width: Point,
	pub lopn_color: u32,
}

/// XFORM
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XForm {
	pub e_m11: f32,
	pub e_m12: f32,
	pub e_m21: f32,
	pub e_m22: f32,
	pub e_dx: f32,
	pub e_dy: f32,
}

/// RGNDATA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RgnData {
	pub rdh: RgnDataHeader,
	pub buffer: [Char; 1],
}

/// RGNDATAHEADER
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RgnDataHeader {
	pub size: u32,
	pub i_type: u32,
	pub count: u32,
	pub rgn_size: u32,
	pub rc_bound: Rect,
}

/// ABC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Abc {
	pub abc_a: i32,
	pub abc_b: u32,
	pub abc_c: i32,
}

/// ABCFLOAT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AbcFloat {
	pub abcf_a: f32,
	pub abcf_b: f32,
	pub abcf_c: f32,
}

/// GLYPHMETRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlyphMetrics {
	pub black_box_x: u32,
	pub black_box_y: u32,
	pub gmpt_glyph_origin: Point,
	pub cell_inc_x: i16,
	pub cell_inc_y: i16,
}

/// MAT2
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat2 {
	pub e_m11: Fixed,
	pub e_m12: Fixed,
	pub e_m21: Fixed,
	pub e_m22: Fixed,
}

/// FIXED
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Fixed {
	pub fract: u16,
	pub value: i16,
}

/// OUTLINETEXTMETRICA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct OutlineTextMetricA<'a> {
	pub size: u32,
	pub text_metrics: TextMetricA,
	pub filler: u8,
	pub panose_number: Panose,
	pub otmfs_selection: u32,
	pub otmfs_type: u32,
	pub otms_char_slope_rise: i32,
	pub otms_char_slope_run: i32,
	pub italic_angle: i32,
	pub em_square: u32,
	pub ascent: i32,
	pub descent: i32,
	pub line_gap: u32,
	pub otms_cap_em_height: u32,
	pub otms_x_height: u32,
	pub otmrc_font_box: Rect,
	pub mac_ascent: i32,
	pub mac_descent: i32,
	pub mac_line_gap: u32,
	pub otmus_minimum_ppem: u32,
	pub otmpt_subscript_size: Point,
	pub otmpt_subscript_offset: Point,
	pub otmpt_superscript_size: Point,
	pub otmpt_superscript_offset: Point,
	pub otms_strikeout_size: u32,
	pub otms_strikeout_position: i32,
	pub otms_underscore_size: i32,
	pub otms_underscore_position: i32,
	pub otmp_family_name: PStr<'a>,
	pub otmp_face_name: PStr<'a>,
	pub otmp_style_name: PStr<'a>,
	pub otmp_full_name: PStr<'a>,
}

/// TEXTMETRICA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextMetricA {
	pub height: i32,
	pub ascent: i32,
	pub descent: i32,
	pub internal_leading: i32,
	pub external_leading: i32,
	pub ave_char_width: i32,
	pub max_char_width: i32,
	pub weight: i32,
	pub overhang: i32,
	pub digitized_aspect_x: i32,
	pub digitized_aspect_y: i32,
	pub first_char: u8,
	pub last_char: u8,
	pub default_char: u8,
	pub break_char: u8,
	pub italic: u8,
	pub underlined: u8,
	pub struck_out: u8,
	pub pitch_and_family: u8,
	pub char_set: u8,
}

/// PANOSE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Panose {
	pub family_type: u8,
	pub serif_style: u8,
	pub weight: u8,
	pub proportion: u8,
	pub contrast: u8,
	pub stroke_variation: u8,
	pub arm_style: u8,
	pub letterform: u8,
	pub midline: u8,
	pub x_height: u8,
}

/// OUTLINETEXTMETRICW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct OutlineTextMetricW<'a> {
	pub size: u32,
	pub text_metrics: TextMetricW,
	pub filler: u8,
	pub panose_number: Panose,
	pub otmfs_selection: u32,
	pub otmfs_type: u32,
	pub otms_char_slope_rise: i32,
	pub otms_char_slope_run: i32,
	pub italic_angle: i32,
	pub em_square: u32,
	pub ascent: i32,
	pub descent: i32,
	pub line_gap: u32,
	pub otms_cap_em_height: u32,
	pub otms_x_height: u32,
	pub otmrc_font_box: Rect,
	pub mac_ascent: i32,
	pub mac_descent: i32,
	pub mac_line_gap: u32,
	pub otmus_minimum_ppem: u32,
	pub otmpt_subscript_size: Point,
	pub otmpt_subscript_offset: Point,
	pub otmpt_superscript_size: Point,
	pub otmpt_superscript_offset: Point,
	pub otms_strikeout_size: u32,
	pub otms_strikeout_position: i32,
	pub otms_underscore_size: i32,
	pub otms_underscore_position: i32,
	pub otmp_family_name: PStr<'a>,
	pub otmp_face_name: PStr<'a>,
	pub otmp_style_name: PStr<'a>,
	pub otmp_full_name: PStr<'a>,
}

/// TEXTMETRICW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextMetricW {
	pub height: i32,
	pub ascent: i32,
	pub descent: i32,
	pub internal_leading: i32,
	pub external_leading: i32,
	pub ave_char_width: i32,
	pub max_char_width: i32,
	pub weight: i32,
	pub overhang: i32,
	pub digitized_aspect_x: i32,
	pub digitized_aspect_y: i32,
	pub first_char: u16,
	pub last_char: u16,
	pub default_char: u16,
	pub break_char: u16,
	pub italic: u8,
	pub underlined: u8,
	pub struck_out: u8,
	pub pitch_and_family: u8,
	pub char_set: u8,
}

/// RASTERIZER_STATUS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RasterizerStatus {
	pub size: i16,
	pub flags: i16,
	pub language_id: i16,
}

/// ENUMLOGFONTEXDVA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EnumLogFonTexDVA {
	pub enum_logfont_ex: EnumLogFonTexA,
	pub design_vector: DesignVector,
}

/// ENUMLOGFONTEXA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EnumLogFonTexA {
	pub log_font: LogFontA,
	pub full_name: [u8; 64],
	pub style: [u8; 32],
	pub script: [u8; 32],
}

/// DESIGNVECTOR
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DesignVector {
	pub reserved: u32,
	pub num_axes: u32,
	pub values: [i32; 16],
}

/// ENUMLOGFONTEXDVW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EnumLogFonTexDVW {
	pub enum_logfont_ex: EnumLogFonTexW,
	pub design_vector: DesignVector,
}

/// ENUMLOGFONTEXW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EnumLogFonTexW {
	pub log_font: LogFontW,
	pub full_name: [u16; 64],
	pub style: [u16; 32],
	pub script: [u16; 32],
}

/// TRIVERTEX
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TriVertex {
	pub x: i32,
	pub y: i32,
	pub red: u16,
	pub green: u16,
	pub blue: u16,
	pub alpha: u16,
}

/// HANDLETABLE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HandleTable {
	pub object_handle: [HGdiObj; 1],
}

/// METARECORD
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MetaRecord {
	pub rd_size: u32,
	pub rd_function: u16,
	pub rd_parm: [u16; 1],
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HEnhMetaFile(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HdcMetdataEnhFileHandle(pub NonZeroUsize);

/// ENHMETAHEADER
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EnhMetaHeader {
	pub i_type: u32,
	pub size: u32,
	pub rcl_bounds: RECtl,
	pub rcl_frame: RECtl,
	pub d_signature: u32,
	pub version: u32,
	pub bytes: u32,
	pub records: u32,
	pub handles: u16,
	pub s_reserved: u16,
	pub description: u32,
	pub off_description: u32,
	pub pal_entries: u32,
	pub szl_device: Size,
	pub szl_millimeters: Size,
	pub pixel_format: u32,
	pub off_pixel_format: u32,
	pub open_gl: u32,
	pub szl_micrometers: Size,
}

/// ENHMETARECORD
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EnhMetaRecord {
	pub i_type: u32,
	pub size: u32,
	pub d_parm: [u32; 1],
}

/// COLORADJUSTMENT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ColorAdjustment {
	pub size: u16,
	pub flags: u16,
	pub illuminant_index: u16,
	pub red_gamma: u16,
	pub green_gamma: u16,
	pub blue_gamma: u16,
	pub reference_black: u16,
	pub reference_white: u16,
	pub contrast: i16,
	pub brightness: i16,
	pub colorfulness: i16,
	pub red_green_tint: i16,
}

/// POLYTEXTA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PolyTextA<'a> {
	pub x: i32,
	pub y: i32,
	pub n: u32,
	pub lpstr: PStr<'a>,
	pub ui_flags: u32,
	pub rcl: Rect,
	pub pdx: &'a i32,
}

/// POLYTEXTW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PolyTextW<'a> {
	pub x: i32,
	pub y: i32,
	pub n: u32,
	pub lpstr: PWStr<'a>,
	pub ui_flags: u32,
	pub rcl: Rect,
	pub pdx: &'a i32,
}

/// KERNINGPAIR
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct KerningPair {
	pub first: u16,
	pub second: u16,
	pub i_kern_amount: i32,
}

/// WGLSWAP
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WglSwap {
	pub hdc: HDc,
	pub ui_flags: u32,
}

/// TTEMBEDINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TtEmbedInfo<'a> {
	pub us_struct_size: u16,
	pub us_root_str_size: u16,
	pub pus_root_str: &'a u16,
}

/// TTLOADINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TtLoadinfo<'a> {
	pub us_struct_size: u16,
	pub us_ref_str_size: u16,
	pub pus_ref_str: &'a u16,
}

/// TTVALIDATIONTESTSPARAMS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TtValidationTestsParams<'a> {
	pub ul_struct_size: u32,
	pub test_from_size: i32,
	pub test_to_size: i32,
	pub ul_char_set: u32,
	pub us_reserved1: u16,
	pub us_char_code_count: u16,
	pub pus_char_code_set: &'a u16,
}

/// TTVALIDATIONTESTSPARAMSEX
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TtValidationTestsParamsEx<'a> {
	pub ul_struct_size: u32,
	pub test_from_size: i32,
	pub test_to_size: i32,
	pub ul_char_set: u32,
	pub us_reserved1: u16,
	pub us_char_code_count: u16,
	pub pul_char_code_set: &'a u32,
}

/// DRAWTEXTPARAMS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DrawTextParams {
	pub size: u32,
	pub i_tab_length: i32,
	pub i_left_margin: i32,
	pub i_right_margin: i32,
	pub ui_length_drawn: u32,
}

/// PAINTSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PaintStruct {
	pub hdc: HDc,
	pub f_erase: Bool,
	pub rc_paint: Rect,
	pub f_restore: Bool,
	pub f_inc_update: Bool,
	pub rgb_reserved: [u8; 32],
}

/// DISPLAY_DEVICEA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayDeviceA {
	pub cb: u32,
	pub device_name: [Char; 32],
	pub device_string: [Char; 128],
	pub state_flags: u32,
	pub device_id: [Char; 128],
	pub device_key: [Char; 128],
}

/// DISPLAY_DEVICEW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayDeviceW {
	pub cb: u32,
	pub device_name: [u16; 32],
	pub device_string: [u16; 128],
	pub state_flags: u32,
	pub device_id: [u16; 128],
	pub device_key: [u16; 128],
}

/// MONITORINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MonitorInfo {
	pub size: u32,
	pub rc_monitor: Rect,
	pub rc_work: Rect,
	pub flags: u32,
}

