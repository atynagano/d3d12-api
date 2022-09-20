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
use crate::core::win32::graphics::direct_write::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteTextLayout(pub(crate) DWriteTextFormat);

impl Deref for DWriteTextLayout {
	type Target = DWriteTextFormat;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteTextLayout {
	const IID: &'static GUID = &GUID::from_u128(0x537370376d14410b9bfe0b182bb70961u128);
}

impl Com for DWriteTextLayout {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteTextLayout {
	pub fn SetMaxWidth(&self, max_width: f32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, max_width);
			_ret_.ok()
		}
	}

	pub fn SetMaxHeight(&self, max_height: f32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, max_height);
			_ret_.ok()
		}
	}

	pub fn SetFontCollection(&self, font_collection: &DWriteFontCollection, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, DWriteTextRange) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, font_collection.vtable(), text_range);
			_ret_.ok()
		}
	}

	pub fn SetFontFamilyName(&self, font_family_name: &str, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, DWriteTextRange) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, font_family_name.to_utf16().as_ptr_or_null(), text_range);
			_ret_.ok()
		}
	}

	pub fn SetFontWeight(&self, font_weight: DWriteFontWeight, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteFontWeight, DWriteTextRange) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, font_weight, text_range);
			_ret_.ok()
		}
	}

	pub fn SetFontStyle(&self, font_style: DWriteFontStyle, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteFontStyle, DWriteTextRange) -> HResult
				= transmute(vt[33]);
			let _ret_ = f(vt, font_style, text_range);
			_ret_.ok()
		}
	}

	pub fn SetFontStretch(&self, font_stretch: DWriteFontStretch, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteFontStretch, DWriteTextRange) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, font_stretch, text_range);
			_ret_.ok()
		}
	}

	pub fn SetFontSize(&self, font_size: f32, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32, DWriteTextRange) -> HResult
				= transmute(vt[35]);
			let _ret_ = f(vt, font_size, text_range);
			_ret_.ok()
		}
	}

	pub fn SetUnderline(&self, has_underline: bool, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool, DWriteTextRange) -> HResult
				= transmute(vt[36]);
			let _ret_ = f(vt, has_underline.to_bool(), text_range);
			_ret_.ok()
		}
	}

	pub fn SetStrikethrough(&self, has_strikethrough: bool, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool, DWriteTextRange) -> HResult
				= transmute(vt[37]);
			let _ret_ = f(vt, has_strikethrough.to_bool(), text_range);
			_ret_.ok()
		}
	}

	pub fn SetDrawingEffect(&self, drawing_effect: &Unknown, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, DWriteTextRange) -> HResult
				= transmute(vt[38]);
			let _ret_ = f(vt, drawing_effect.vtable(), text_range);
			_ret_.ok()
		}
	}

	pub fn SetInlineObject(&self, inline_object: &DWriteInlineObject, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, DWriteTextRange) -> HResult
				= transmute(vt[39]);
			let _ret_ = f(vt, inline_object.vtable(), text_range);
			_ret_.ok()
		}
	}

	pub fn SetTypography(&self, typography: &DWriteTypography, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, DWriteTextRange) -> HResult
				= transmute(vt[40]);
			let _ret_ = f(vt, typography.vtable(), text_range);
			_ret_.ok()
		}
	}

	pub fn SetLocaleName(&self, locale_name: &str, text_range: DWriteTextRange) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, DWriteTextRange) -> HResult
				= transmute(vt[41]);
			let _ret_ = f(vt, locale_name.to_utf16().as_ptr_or_null(), text_range);
			_ret_.ok()
		}
	}

	pub fn GetMaxWidth(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[42]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetMaxHeight(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[43]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFontCollection(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<DWriteFontCollection, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_collection_out_: Option<DWriteFontCollection> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[44]);
			let _ret_ = f(vt, current_position, transmute(&mut font_collection_out_), text_range);
			if _ret_.is_ok() { if let Some(font_collection_out_) = font_collection_out_ { return Ok(font_collection_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn get_font_collection(&self) { todo!() }

	pub fn GetFontFamilyNameLength(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut name_length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[45]);
			let _ret_ = f(vt, current_position, name_length_out_.as_mut_ptr(), text_range);
			if _ret_.is_ok() { Ok(name_length_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn get_font_family_name_length(&self, current_position: u32) -> Result<(u32, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut name_length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32, *mut DWriteTextRange) -> HResult
				= transmute(vt[45]);
			let _ret_ = f(vt, current_position, name_length_out_.as_mut_ptr(), text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((name_length_out_.assume_init(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetFontFamilyName(&self) { todo!() }

	pub fn GetFontWeight(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<DWriteFontWeight, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_weight_out_: MaybeUninit<DWriteFontWeight> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut DWriteFontWeight, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[47]);
			let _ret_ = f(vt, current_position, font_weight_out_.as_mut_ptr(), text_range);
			if _ret_.is_ok() { Ok(font_weight_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn get_font_weight(&self, current_position: u32) -> Result<(DWriteFontWeight, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_weight_out_: MaybeUninit<DWriteFontWeight> = MaybeUninit::zeroed();
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut DWriteFontWeight, *mut DWriteTextRange) -> HResult
				= transmute(vt[47]);
			let _ret_ = f(vt, current_position, font_weight_out_.as_mut_ptr(), text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((font_weight_out_.assume_init(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetFontStyle(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<DWriteFontStyle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_style_out_: MaybeUninit<DWriteFontStyle> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut DWriteFontStyle, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[48]);
			let _ret_ = f(vt, current_position, font_style_out_.as_mut_ptr(), text_range);
			if _ret_.is_ok() { Ok(font_style_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn get_font_style(&self, current_position: u32) -> Result<(DWriteFontStyle, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_style_out_: MaybeUninit<DWriteFontStyle> = MaybeUninit::zeroed();
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut DWriteFontStyle, *mut DWriteTextRange) -> HResult
				= transmute(vt[48]);
			let _ret_ = f(vt, current_position, font_style_out_.as_mut_ptr(), text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((font_style_out_.assume_init(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetFontStretch(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<DWriteFontStretch, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_stretch_out_: MaybeUninit<DWriteFontStretch> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut DWriteFontStretch, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[49]);
			let _ret_ = f(vt, current_position, font_stretch_out_.as_mut_ptr(), text_range);
			if _ret_.is_ok() { Ok(font_stretch_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn get_font_stretch(&self, current_position: u32) -> Result<(DWriteFontStretch, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_stretch_out_: MaybeUninit<DWriteFontStretch> = MaybeUninit::zeroed();
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut DWriteFontStretch, *mut DWriteTextRange) -> HResult
				= transmute(vt[49]);
			let _ret_ = f(vt, current_position, font_stretch_out_.as_mut_ptr(), text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((font_stretch_out_.assume_init(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetFontSize(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<f32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_size_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut f32, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[50]);
			let _ret_ = f(vt, current_position, font_size_out_.as_mut_ptr(), text_range);
			if _ret_.is_ok() { Ok(font_size_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn get_font_size(&self, current_position: u32) -> Result<(f32, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_size_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut f32, *mut DWriteTextRange) -> HResult
				= transmute(vt[50]);
			let _ret_ = f(vt, current_position, font_size_out_.as_mut_ptr(), text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((font_size_out_.assume_init(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetUnderline(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut has_underline_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, u32, &mut Bool, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[51]);
			let _ret_ = f(vt, current_position, &mut has_underline_out_, text_range);
			if _ret_.is_ok() { Ok(has_underline_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn get_underline(&self, current_position: u32) -> Result<(bool, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut has_underline_out_ = Bool::FALSE;
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, &mut Bool, *mut DWriteTextRange) -> HResult
				= transmute(vt[51]);
			let _ret_ = f(vt, current_position, &mut has_underline_out_, text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((has_underline_out_.to_bool(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetStrikethrough(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut has_strikethrough_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, u32, &mut Bool, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[52]);
			let _ret_ = f(vt, current_position, &mut has_strikethrough_out_, text_range);
			if _ret_.is_ok() { Ok(has_strikethrough_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn get_strikethrough(&self, current_position: u32) -> Result<(bool, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut has_strikethrough_out_ = Bool::FALSE;
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, &mut Bool, *mut DWriteTextRange) -> HResult
				= transmute(vt[52]);
			let _ret_ = f(vt, current_position, &mut has_strikethrough_out_, text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((has_strikethrough_out_.to_bool(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetDrawingEffect(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<Unknown, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut drawing_effect_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[53]);
			let _ret_ = f(vt, current_position, transmute(&mut drawing_effect_out_), text_range);
			if _ret_.is_ok() { if let Some(drawing_effect_out_) = drawing_effect_out_ { return Ok(drawing_effect_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn get_drawing_effect(&self) { todo!() }

	pub fn GetInlineObject(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<DWriteInlineObject, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut inline_object_out_: Option<DWriteInlineObject> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[54]);
			let _ret_ = f(vt, current_position, transmute(&mut inline_object_out_), text_range);
			if _ret_.is_ok() { if let Some(inline_object_out_) = inline_object_out_ { return Ok(inline_object_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn get_inline_object(&self) { todo!() }

	pub fn GetTypography(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<DWriteTypography, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut typography_out_: Option<DWriteTypography> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[55]);
			let _ret_ = f(vt, current_position, transmute(&mut typography_out_), text_range);
			if _ret_.is_ok() { if let Some(typography_out_) = typography_out_ { return Ok(typography_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn get_typography(&self) { todo!() }

	pub fn GetLocaleNameLength(&self, current_position: u32, text_range: Option<&mut MaybeUninit<DWriteTextRange>>) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut name_length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32, Option<&mut MaybeUninit<DWriteTextRange>>) -> HResult
				= transmute(vt[56]);
			let _ret_ = f(vt, current_position, name_length_out_.as_mut_ptr(), text_range);
			if _ret_.is_ok() { Ok(name_length_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn get_locale_name_length(&self, current_position: u32) -> Result<(u32, DWriteTextRange), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut name_length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut text_range_out_: MaybeUninit<DWriteTextRange> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32, *mut DWriteTextRange) -> HResult
				= transmute(vt[56]);
			let _ret_ = f(vt, current_position, name_length_out_.as_mut_ptr(), text_range_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((name_length_out_.assume_init(), text_range_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetLocaleName(&self) { todo!() }

	pub fn Draw(&self, client_drawing_context: *const (), renderer: &DWriteTextRenderer, origin_x: f32, origin_y: f32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, VTable, f32, f32) -> HResult
				= transmute(vt[58]);
			let _ret_ = f(vt, client_drawing_context as _, renderer.vtable(), origin_x, origin_y);
			_ret_.ok()
		}
	}

	pub unsafe fn GetLineMetrics(&self) { todo!() }

	pub fn GetMetrics(&self) -> Result<DWriteTextMetrics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut text_metrics_out_: MaybeUninit<DWriteTextMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteTextMetrics) -> HResult
				= transmute(vt[60]);
			let _ret_ = f(vt, text_metrics_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(text_metrics_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetOverhangMetrics(&self) -> Result<DWriteOverhangMetrics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut overhangs_out_: MaybeUninit<DWriteOverhangMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteOverhangMetrics) -> HResult
				= transmute(vt[61]);
			let _ret_ = f(vt, overhangs_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(overhangs_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetClusterMetrics(&self) { todo!() }

	pub fn DetermineMinWidth(&self) -> Result<f32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut min_width_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut f32) -> HResult
				= transmute(vt[63]);
			let _ret_ = f(vt, min_width_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(min_width_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn HitTestPoint(&self, point_x: f32, point_y: f32) -> Result<(bool, bool, DWriteHitTestMetrics), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut is_trailing_hit_out_ = Bool::FALSE;
			let mut is_inside_out_ = Bool::FALSE;
			let mut hit_test_metrics_out_: MaybeUninit<DWriteHitTestMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, f32, f32, &mut Bool, &mut Bool, *mut DWriteHitTestMetrics) -> HResult
				= transmute(vt[64]);
			let _ret_ = f(vt, point_x, point_y, &mut is_trailing_hit_out_, &mut is_inside_out_, hit_test_metrics_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((is_trailing_hit_out_.to_bool(), is_inside_out_.to_bool(), hit_test_metrics_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn HitTestTextPosition(&self, text_position: u32, is_trailing_hit: bool) -> Result<(f32, f32, DWriteHitTestMetrics), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut point_x_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut point_y_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut hit_test_metrics_out_: MaybeUninit<DWriteHitTestMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, Bool, *mut f32, *mut f32, *mut DWriteHitTestMetrics) -> HResult
				= transmute(vt[65]);
			let _ret_ = f(vt, text_position, is_trailing_hit.to_bool(), point_x_out_.as_mut_ptr(), point_y_out_.as_mut_ptr(), hit_test_metrics_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((point_x_out_.assume_init(), point_y_out_.assume_init(), hit_test_metrics_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub unsafe fn HitTestTextRange(&self) { todo!() }
}

pub trait IDWriteTextLayout: IDWriteTextFormat {
	fn as_text_layout(&self) -> &DWriteTextLayout;
	fn into_text_layout(self) -> DWriteTextLayout;
}

impl IDWriteTextLayout for DWriteTextLayout {
	fn as_text_layout(&self) -> &DWriteTextLayout { self }
	fn into_text_layout(self) -> DWriteTextLayout { self }
}
impl IDWriteTextFormat for DWriteTextLayout {
	fn as_text_format(&self) -> &DWriteTextFormat { &self.0.as_text_format() }
	fn into_text_format(self) -> DWriteTextFormat { self.0.into_text_format() }
}

impl IUnknown for DWriteTextLayout {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteTextLayout {
    fn from(v: UnknownWrapper) -> Self {
        Self(DWriteTextFormat::from(v))
    }
}

