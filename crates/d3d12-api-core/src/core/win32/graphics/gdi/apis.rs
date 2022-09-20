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

use crate::core::win32::graphics::gdi::*;
use crate::core::win32::foundation::*;


pub unsafe fn GetObjectA() { todo!() }

pub fn AddFontResourceA(param0: &str) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AddFontResourceA(param0: *const u8) -> i32;
		} 
		let _ret_ = AddFontResourceA(param0.to_null_terminated().as_ptr_or_null());
		_ret_
	}
}

pub fn AddFontResourceW(param0: &str) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AddFontResourceW(param0: *const u16) -> i32;
		} 
		let _ret_ = AddFontResourceW(param0.to_utf16().as_ptr_or_null());
		_ret_
	}
}

pub fn AnimatePalette(pal: HPalette, i_start_index: u32, ppe: &[PaletteEntry]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AnimatePalette(hPal: HPalette, iStartIndex: u32, cEntries: u32, ppe: *const PaletteEntry) -> Bool;
		} 
		let (ppe_ptr_, ppe_len_) = ppe.deconstruct();
		let _ret_ = AnimatePalette(pal, i_start_index, ppe_len_ as u32, ppe_ptr_);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn Arc(hdc: HDc, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn Arc(hdc: HDc, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Bool;
		} 
		let _ret_ = Arc(hdc, x1, y1, x2, y2, x3, y3, x4, y4);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn BitBlt(hdc: HDc, x: i32, y: i32, cx: i32, cy: i32, hdc_src: Option<HDc>, x1: i32, y1: i32, rop: RopCode) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn BitBlt(hdc: HDc, x: i32, y: i32, cx: i32, cy: i32, hdcSrc: *const c_void, x1: i32, y1: i32, rop: RopCode) -> Bool;
		} 
		let _ret_ = BitBlt(hdc, x, y, cx, cy, transmute(hdc_src), x1, y1, rop);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CancelDC(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CancelDC(hdc: HDc) -> Bool;
		} 
		let _ret_ = CancelDC(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn Chord(hdc: HDc, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn Chord(hdc: HDc, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Bool;
		} 
		let _ret_ = Chord(hdc, x1, y1, x2, y2, x3, y3, x4, y4);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CloseMetaFile(hdc: HDc) -> Result<HMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CloseMetaFile(hdc: HDc) -> *const c_void;
		} 
		let _ret_ = CloseMetaFile(hdc);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CombineRgn(hrgn_dst: Option<HRgn>, hrgn_src1: Option<HRgn>, hrgn_src2: Option<HRgn>, i_mode: RgnCombineMode) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CombineRgn(hrgnDst: *const c_void, hrgnSrc1: *const c_void, hrgnSrc2: *const c_void, iMode: RgnCombineMode) -> GdiRegionType;
		} 
		let _ret_ = CombineRgn(transmute(hrgn_dst), transmute(hrgn_src1), transmute(hrgn_src2), i_mode);
		_ret_
	}
}

pub fn CopyMetaFileA(param0: HMetaFile, param1: Option<&str>) -> Result<HMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CopyMetaFileA(param0: HMetaFile, param1: *const u8) -> *const c_void;
		} 
		let _ret_ = CopyMetaFileA(param0, param1.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CopyMetaFileW(param0: HMetaFile, param1: Option<&str>) -> Result<HMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CopyMetaFileW(param0: HMetaFile, param1: *const u16) -> *const c_void;
		} 
		let _ret_ = CopyMetaFileW(param0, param1.map(str::to_utf16).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateBitmap(width: i32, height: i32, planes: u32, bit_count: u32, bits: *const ()) -> Result<HBitmap, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateBitmap(nWidth: i32, nHeight: i32, nPlanes: u32, nBitCount: u32, lpBits: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateBitmap(width, height, planes, bit_count, bits as _);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateBitmapIndirect(pbm: &Bitmap) -> Result<HBitmap, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateBitmapIndirect(pbm: &Bitmap) -> *const c_void;
		} 
		let _ret_ = CreateBitmapIndirect(pbm);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateBrushIndirect(plbrush: &LogBrush) -> Result<HBrush, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateBrushIndirect(plbrush: &LogBrush) -> *const c_void;
		} 
		let _ret_ = CreateBrushIndirect(plbrush);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateCompatibleBitmap(hdc: HDc, cx: i32, cy: i32) -> Result<HBitmap, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateCompatibleBitmap(hdc: HDc, cx: i32, cy: i32) -> *const c_void;
		} 
		let _ret_ = CreateCompatibleBitmap(hdc, cx, cy);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDiscardableBitmap(hdc: HDc, cx: i32, cy: i32) -> Result<HBitmap, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateDiscardableBitmap(hdc: HDc, cx: i32, cy: i32) -> *const c_void;
		} 
		let _ret_ = CreateDiscardableBitmap(hdc, cx, cy);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateCompatibleDC(hdc: Option<HDc>) -> Result<CreatedHDC, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateCompatibleDC(hdc: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateCompatibleDC(transmute(hdc));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDCA(pwsz_driver: Option<&str>, pwsz_device: Option<&str>, psz_port: Option<&str>, pdm: Option<&DevModeA>) -> Result<CreatedHDC, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateDCA(pwszDriver: *const u8, pwszDevice: *const u8, pszPort: *const u8, pdm: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateDCA(pwsz_driver.map(str::to_null_terminated).as_ptr_or_null(), pwsz_device.map(str::to_null_terminated).as_ptr_or_null(), psz_port.map(str::to_null_terminated).as_ptr_or_null(), transmute(pdm));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDCW(pwsz_driver: Option<&str>, pwsz_device: Option<&str>, psz_port: Option<&str>, pdm: Option<&DevModeW>) -> Result<CreatedHDC, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateDCW(pwszDriver: *const u16, pwszDevice: *const u16, pszPort: *const u16, pdm: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateDCW(pwsz_driver.map(str::to_utf16).as_ptr_or_null(), pwsz_device.map(str::to_utf16).as_ptr_or_null(), psz_port.map(str::to_utf16).as_ptr_or_null(), transmute(pdm));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDIBitmap(hdc: HDc, pbmih: Option<&BitmapInfoHeader>, fl_init: u32, pj_bits: *const (), pbmi: Option<&BitmapInfo>, i_usage: DibUsage) -> Result<HBitmap, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateDIBitmap(hdc: HDc, pbmih: *const c_void, flInit: u32, pjBits: *const c_void, pbmi: *const c_void, iUsage: DibUsage) -> *const c_void;
		} 
		let _ret_ = CreateDIBitmap(hdc, transmute(pbmih), fl_init, pj_bits as _, transmute(pbmi), i_usage);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDIBPatternBrush(h: NonZeroUsize, i_usage: DibUsage) -> Result<HBrush, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateDIBPatternBrush(h: NonZeroUsize, iUsage: DibUsage) -> *const c_void;
		} 
		let _ret_ = CreateDIBPatternBrush(h, i_usage);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateDIBPatternBrushPt(packed_dib: *const impl Sized, i_usage: DibUsage) -> Result<HBrush, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateDIBPatternBrushPt(lpPackedDIB: *const c_void, iUsage: DibUsage) -> *const c_void;
		} 
		let _ret_ = CreateDIBPatternBrushPt(packed_dib as _, i_usage);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> *const c_void;
		} 
		let _ret_ = CreateEllipticRgn(x1, y1, x2, y2);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateEllipticRgnIndirect(lprect: &Rect) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateEllipticRgnIndirect(lprect: &Rect) -> *const c_void;
		} 
		let _ret_ = CreateEllipticRgnIndirect(lprect);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateFontIndirectA(lplf: &LogFontA) -> Result<HFont, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateFontIndirectA(lplf: &LogFontA) -> *const c_void;
		} 
		let _ret_ = CreateFontIndirectA(lplf);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateFontIndirectW(lplf: &LogFontW) -> Result<HFont, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateFontIndirectW(lplf: &LogFontW) -> *const c_void;
		} 
		let _ret_ = CreateFontIndirectW(lplf);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateFontA(height: i32, width: i32, escapement: i32, orientation: i32, weight: i32, italic: u32, underline: u32, strike_out: u32, i_char_set: u32, i_out_precision: FontOutputPrecision, i_clip_precision: FontClipPrecision, i_quality: FontQuality, i_pitch_and_family: FontPitchAndFamily, psz_face_name: Option<&str>) -> Result<HFont, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateFontA(cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32, cWeight: i32, bItalic: u32, bUnderline: u32, bStrikeOut: u32, iCharSet: u32, iOutPrecision: FontOutputPrecision, iClipPrecision: FontClipPrecision, iQuality: FontQuality, iPitchAndFamily: FontPitchAndFamily, pszFaceName: *const u8) -> *const c_void;
		} 
		let _ret_ = CreateFontA(height, width, escapement, orientation, weight, italic, underline, strike_out, i_char_set, i_out_precision, i_clip_precision, i_quality, i_pitch_and_family, psz_face_name.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateFontW(height: i32, width: i32, escapement: i32, orientation: i32, weight: i32, italic: u32, underline: u32, strike_out: u32, i_char_set: u32, i_out_precision: FontOutputPrecision, i_clip_precision: FontClipPrecision, i_quality: FontQuality, i_pitch_and_family: FontPitchAndFamily, psz_face_name: Option<&str>) -> Result<HFont, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateFontW(cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32, cWeight: i32, bItalic: u32, bUnderline: u32, bStrikeOut: u32, iCharSet: u32, iOutPrecision: FontOutputPrecision, iClipPrecision: FontClipPrecision, iQuality: FontQuality, iPitchAndFamily: FontPitchAndFamily, pszFaceName: *const u16) -> *const c_void;
		} 
		let _ret_ = CreateFontW(height, width, escapement, orientation, weight, italic, underline, strike_out, i_char_set, i_out_precision, i_clip_precision, i_quality, i_pitch_and_family, psz_face_name.map(str::to_utf16).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateHatchBrush(i_hatch: HatchBrushStyle, color: u32) -> Result<HBrush, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateHatchBrush(iHatch: HatchBrushStyle, color: u32) -> *const c_void;
		} 
		let _ret_ = CreateHatchBrush(i_hatch, color);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateICA(psz_driver: Option<&str>, psz_device: Option<&str>, psz_port: Option<&str>, pdm: Option<&DevModeA>) -> Result<CreatedHDC, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateICA(pszDriver: *const u8, pszDevice: *const u8, pszPort: *const u8, pdm: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateICA(psz_driver.map(str::to_null_terminated).as_ptr_or_null(), psz_device.map(str::to_null_terminated).as_ptr_or_null(), psz_port.map(str::to_null_terminated).as_ptr_or_null(), transmute(pdm));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateICW(psz_driver: Option<&str>, psz_device: Option<&str>, psz_port: Option<&str>, pdm: Option<&DevModeW>) -> Result<CreatedHDC, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateICW(pszDriver: *const u16, pszDevice: *const u16, pszPort: *const u16, pdm: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateICW(psz_driver.map(str::to_utf16).as_ptr_or_null(), psz_device.map(str::to_utf16).as_ptr_or_null(), psz_port.map(str::to_utf16).as_ptr_or_null(), transmute(pdm));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateMetaFileA(psz_file: Option<&str>) -> Result<HdcMetdataFileHandle, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateMetaFileA(pszFile: *const u8) -> *const c_void;
		} 
		let _ret_ = CreateMetaFileA(psz_file.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateMetaFileW(psz_file: Option<&str>) -> Result<HdcMetdataFileHandle, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateMetaFileW(pszFile: *const u16) -> *const c_void;
		} 
		let _ret_ = CreateMetaFileW(psz_file.map(str::to_utf16).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreatePalette(plpal: &LogPalette) -> Result<HPalette, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreatePalette(plpal: &LogPalette) -> *const c_void;
		} 
		let _ret_ = CreatePalette(plpal);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreatePen(i_style: PenStyle, width: i32, color: u32) -> Result<HPen, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreatePen(iStyle: PenStyle, cWidth: i32, color: u32) -> *const c_void;
		} 
		let _ret_ = CreatePen(i_style, width, color);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreatePenIndirect(plpen: &LogPen) -> Result<HPen, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreatePenIndirect(plpen: &LogPen) -> *const c_void;
		} 
		let _ret_ = CreatePenIndirect(plpen);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreatePolyPolygonRgn(pptl: &Point, pc: &[i32], i_mode: CreatePolygonRgnMode) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreatePolyPolygonRgn(pptl: &Point, pc: *const i32, cPoly: i32, iMode: CreatePolygonRgnMode) -> *const c_void;
		} 
		let (pc_ptr_, pc_len_) = pc.deconstruct();
		let _ret_ = CreatePolyPolygonRgn(pptl, pc_ptr_, pc_len_ as i32, i_mode);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreatePatternBrush(hbm: HBitmap) -> Result<HBrush, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreatePatternBrush(hbm: HBitmap) -> *const c_void;
		} 
		let _ret_ = CreatePatternBrush(hbm);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> *const c_void;
		} 
		let _ret_ = CreateRectRgn(x1, y1, x2, y2);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateRectRgnIndirect(lprect: &Rect) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateRectRgnIndirect(lprect: &Rect) -> *const c_void;
		} 
		let _ret_ = CreateRectRgnIndirect(lprect);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> *const c_void;
		} 
		let _ret_ = CreateRoundRectRgn(x1, y1, x2, y2, w, h);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateScalableFontResourceA(fdw_hidden: u32, font: &str, file: &str, path: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateScalableFontResourceA(fdwHidden: u32, lpszFont: *const u8, lpszFile: *const u8, lpszPath: *const u8) -> Bool;
		} 
		let _ret_ = CreateScalableFontResourceA(fdw_hidden, font.to_null_terminated().as_ptr_or_null(), file.to_null_terminated().as_ptr_or_null(), path.map(str::to_null_terminated).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CreateScalableFontResourceW(fdw_hidden: u32, font: &str, file: &str, path: Option<&str>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateScalableFontResourceW(fdwHidden: u32, lpszFont: *const u16, lpszFile: *const u16, lpszPath: *const u16) -> Bool;
		} 
		let _ret_ = CreateScalableFontResourceW(fdw_hidden, font.to_utf16().as_ptr_or_null(), file.to_utf16().as_ptr_or_null(), path.map(str::to_utf16).as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CreateSolidBrush(color: ColorRef) -> Result<HBrush, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateSolidBrush(color: ColorRef) -> *const c_void;
		} 
		let _ret_ = CreateSolidBrush(color);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DeleteDC(hdc: CreatedHDC) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn DeleteDC(hdc: CreatedHDC) -> Bool;
		} 
		let _ret_ = DeleteDC(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DeleteMetaFile(hmf: HMetaFile) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn DeleteMetaFile(hmf: HMetaFile) -> Bool;
		} 
		let _ret_ = DeleteMetaFile(hmf);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DeleteObject(ho: HGdiObj) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn DeleteObject(ho: HGdiObj) -> Bool;
		} 
		let _ret_ = DeleteObject(ho);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DrawEscape(hdc: HDc, i_escape: i32, r#in: Option<&str>) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn DrawEscape(hdc: HDc, iEscape: i32, cjIn: i32, lpIn: *const u8) -> i32;
		} 
		let (r#in_ptr_, r#in_len_) = r#in.deconstruct();
		let _ret_ = DrawEscape(hdc, i_escape, r#in_len_ as i32, r#in_ptr_);
		_ret_
	}
}

pub fn Ellipse(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn Ellipse(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> Bool;
		} 
		let _ret_ = Ellipse(hdc, left, top, right, bottom);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumFontFamiliesExA(hdc: HDc, logfont: &LogFontA, proc: FontEnumProcA, l_param: LParam, flags: u32) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumFontFamiliesExA(hdc: HDc, lpLogfont: &LogFontA, lpProc: FontEnumProcA, lParam: LParam, dwFlags: u32) -> i32;
		} 
		let _ret_ = EnumFontFamiliesExA(hdc, logfont, proc, l_param, flags);
		_ret_
	}
}

pub fn EnumFontFamiliesExW(hdc: HDc, logfont: &LogFontW, proc: FontEnumProcW, l_param: LParam, flags: u32) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumFontFamiliesExW(hdc: HDc, lpLogfont: &LogFontW, lpProc: FontEnumProcW, lParam: LParam, dwFlags: u32) -> i32;
		} 
		let _ret_ = EnumFontFamiliesExW(hdc, logfont, proc, l_param, flags);
		_ret_
	}
}

pub fn EnumFontFamiliesA(hdc: HDc, logfont: Option<&str>, proc: FontEnumProcA, l_param: LParam) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumFontFamiliesA(hdc: HDc, lpLogfont: *const u8, lpProc: FontEnumProcA, lParam: LParam) -> i32;
		} 
		let _ret_ = EnumFontFamiliesA(hdc, logfont.map(str::to_null_terminated).as_ptr_or_null(), proc, l_param);
		_ret_
	}
}

pub fn EnumFontFamiliesW(hdc: HDc, logfont: Option<&str>, proc: FontEnumProcW, l_param: LParam) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumFontFamiliesW(hdc: HDc, lpLogfont: *const u16, lpProc: FontEnumProcW, lParam: LParam) -> i32;
		} 
		let _ret_ = EnumFontFamiliesW(hdc, logfont.map(str::to_utf16).as_ptr_or_null(), proc, l_param);
		_ret_
	}
}

pub fn EnumFontsA(hdc: HDc, logfont: Option<&str>, proc: FontEnumProcA, l_param: LParam) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumFontsA(hdc: HDc, lpLogfont: *const u8, lpProc: FontEnumProcA, lParam: LParam) -> i32;
		} 
		let _ret_ = EnumFontsA(hdc, logfont.map(str::to_null_terminated).as_ptr_or_null(), proc, l_param);
		_ret_
	}
}

pub fn EnumFontsW(hdc: HDc, logfont: Option<&str>, proc: FontEnumProcW, l_param: LParam) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumFontsW(hdc: HDc, lpLogfont: *const u16, lpProc: FontEnumProcW, lParam: LParam) -> i32;
		} 
		let _ret_ = EnumFontsW(hdc, logfont.map(str::to_utf16).as_ptr_or_null(), proc, l_param);
		_ret_
	}
}

pub fn EnumObjects(hdc: HDc, r#type: ObjType, func: GObjEnumProc, l_param: LParam) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumObjects(hdc: HDc, nType: ObjType, lpFunc: GObjEnumProc, lParam: LParam) -> i32;
		} 
		let _ret_ = EnumObjects(hdc, r#type, func, l_param);
		_ret_
	}
}

pub fn EqualRgn(hrgn1: HRgn, hrgn2: HRgn) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EqualRgn(hrgn1: HRgn, hrgn2: HRgn) -> Bool;
		} 
		let _ret_ = EqualRgn(hrgn1, hrgn2);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ExcludeClipRect(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ExcludeClipRect(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> GdiRegionType;
		} 
		let _ret_ = ExcludeClipRect(hdc, left, top, right, bottom);
		_ret_
	}
}

pub unsafe fn ExtCreateRegion() { todo!() }

pub fn ExtFloodFill(hdc: HDc, x: i32, y: i32, color: u32, r#type: ExtFloodFillType) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ExtFloodFill(hdc: HDc, x: i32, y: i32, color: u32, r#type: ExtFloodFillType) -> Bool;
		} 
		let _ret_ = ExtFloodFill(hdc, x, y, color, r#type);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FillRgn(hdc: HDc, hrgn: HRgn, hbr: HBrush) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn FillRgn(hdc: HDc, hrgn: HRgn, hbr: HBrush) -> Bool;
		} 
		let _ret_ = FillRgn(hdc, hrgn, hbr);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FloodFill(hdc: HDc, x: i32, y: i32, color: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn FloodFill(hdc: HDc, x: i32, y: i32, color: u32) -> Bool;
		} 
		let _ret_ = FloodFill(hdc, x, y, color);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FrameRgn(hdc: HDc, hrgn: HRgn, hbr: HBrush, w: i32, h: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn FrameRgn(hdc: HDc, hrgn: HRgn, hbr: HBrush, w: i32, h: i32) -> Bool;
		} 
		let _ret_ = FrameRgn(hdc, hrgn, hbr, w, h);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetROP2(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetROP2(hdc: HDc) -> i32;
		} 
		let _ret_ = GetROP2(hdc);
		_ret_
	}
}

pub fn GetAspectRatioFilterEx(hdc: HDc) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetAspectRatioFilterEx(hdc: HDc, lpsize: *mut Size) -> Bool;
		} 
		let mut lpsize_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetAspectRatioFilterEx(hdc, lpsize_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsize_out_.assume_init()) }
	}
}

pub fn GetBkColor(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetBkColor(hdc: HDc) -> u32;
		} 
		let _ret_ = GetBkColor(hdc);
		_ret_
	}
}

pub fn GetDCBrushColor(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetDCBrushColor(hdc: HDc) -> u32;
		} 
		let _ret_ = GetDCBrushColor(hdc);
		_ret_
	}
}

pub fn GetDCPenColor(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetDCPenColor(hdc: HDc) -> u32;
		} 
		let _ret_ = GetDCPenColor(hdc);
		_ret_
	}
}

pub fn GetBkMode(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetBkMode(hdc: HDc) -> i32;
		} 
		let _ret_ = GetBkMode(hdc);
		_ret_
	}
}

pub unsafe fn GetBitmapBits() { todo!() }

pub fn GetBitmapDimensionEx(hbit: HBitmap) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetBitmapDimensionEx(hbit: HBitmap, lpsize: *mut Size) -> Bool;
		} 
		let mut lpsize_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetBitmapDimensionEx(hbit, lpsize_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsize_out_.assume_init()) }
	}
}

pub fn GetBoundsRect(hdc: HDc, flags: u32) -> (u32, Rect) {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetBoundsRect(hdc: HDc, lprect: *mut Rect, flags: u32) -> u32;
		} 
		let mut lprect_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = GetBoundsRect(hdc, lprect_out_.as_mut_ptr(), flags);
		(_ret_, lprect_out_.assume_init())
	}
}

pub fn GetBrushOrgEx(hdc: HDc) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetBrushOrgEx(hdc: HDc, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetBrushOrgEx(hdc, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn GetCharWidthA(hdc: HDc, i_first: u32, i_last: u32) -> Result<i32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharWidthA(hdc: HDc, iFirst: u32, iLast: u32, lpBuffer: *mut i32) -> Bool;
		} 
		let mut buffer_out_: MaybeUninit<i32> = MaybeUninit::zeroed();
		let _ret_ = GetCharWidthA(hdc, i_first, i_last, buffer_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(buffer_out_.assume_init()) }
	}
}

pub fn GetCharWidthW(hdc: HDc, i_first: u32, i_last: u32) -> Result<i32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharWidthW(hdc: HDc, iFirst: u32, iLast: u32, lpBuffer: *mut i32) -> Bool;
		} 
		let mut buffer_out_: MaybeUninit<i32> = MaybeUninit::zeroed();
		let _ret_ = GetCharWidthW(hdc, i_first, i_last, buffer_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(buffer_out_.assume_init()) }
	}
}

pub fn GetCharWidth32A(hdc: HDc, i_first: u32, i_last: u32) -> Result<i32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharWidth32A(hdc: HDc, iFirst: u32, iLast: u32, lpBuffer: *mut i32) -> Bool;
		} 
		let mut buffer_out_: MaybeUninit<i32> = MaybeUninit::zeroed();
		let _ret_ = GetCharWidth32A(hdc, i_first, i_last, buffer_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(buffer_out_.assume_init()) }
	}
}

pub fn GetCharWidth32W(hdc: HDc, i_first: u32, i_last: u32) -> Result<i32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharWidth32W(hdc: HDc, iFirst: u32, iLast: u32, lpBuffer: *mut i32) -> Bool;
		} 
		let mut buffer_out_: MaybeUninit<i32> = MaybeUninit::zeroed();
		let _ret_ = GetCharWidth32W(hdc, i_first, i_last, buffer_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(buffer_out_.assume_init()) }
	}
}

pub fn GetCharWidthFloatA(hdc: HDc, i_first: u32, i_last: u32) -> Result<f32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharWidthFloatA(hdc: HDc, iFirst: u32, iLast: u32, lpBuffer: *mut f32) -> Bool;
		} 
		let mut buffer_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
		let _ret_ = GetCharWidthFloatA(hdc, i_first, i_last, buffer_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(buffer_out_.assume_init()) }
	}
}

pub fn GetCharWidthFloatW(hdc: HDc, i_first: u32, i_last: u32) -> Result<f32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharWidthFloatW(hdc: HDc, iFirst: u32, iLast: u32, lpBuffer: *mut f32) -> Bool;
		} 
		let mut buffer_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
		let _ret_ = GetCharWidthFloatW(hdc, i_first, i_last, buffer_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(buffer_out_.assume_init()) }
	}
}

pub fn GetCharABCWidthsA(hdc: HDc, first: u32, last: u32) -> Result<Abc, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharABCWidthsA(hdc: HDc, wFirst: u32, wLast: u32, lpABC: *mut Abc) -> Bool;
		} 
		let mut abc_out_: MaybeUninit<Abc> = MaybeUninit::zeroed();
		let _ret_ = GetCharABCWidthsA(hdc, first, last, abc_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(abc_out_.assume_init()) }
	}
}

pub fn GetCharABCWidthsW(hdc: HDc, first: u32, last: u32) -> Result<Abc, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharABCWidthsW(hdc: HDc, wFirst: u32, wLast: u32, lpABC: *mut Abc) -> Bool;
		} 
		let mut abc_out_: MaybeUninit<Abc> = MaybeUninit::zeroed();
		let _ret_ = GetCharABCWidthsW(hdc, first, last, abc_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(abc_out_.assume_init()) }
	}
}

pub fn GetCharABCWidthsFloatA(hdc: HDc, i_first: u32, i_last: u32) -> Result<AbcFloat, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharABCWidthsFloatA(hdc: HDc, iFirst: u32, iLast: u32, lpABC: *mut AbcFloat) -> Bool;
		} 
		let mut abc_out_: MaybeUninit<AbcFloat> = MaybeUninit::zeroed();
		let _ret_ = GetCharABCWidthsFloatA(hdc, i_first, i_last, abc_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(abc_out_.assume_init()) }
	}
}

pub fn GetCharABCWidthsFloatW(hdc: HDc, i_first: u32, i_last: u32) -> Result<AbcFloat, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCharABCWidthsFloatW(hdc: HDc, iFirst: u32, iLast: u32, lpABC: *mut AbcFloat) -> Bool;
		} 
		let mut abc_out_: MaybeUninit<AbcFloat> = MaybeUninit::zeroed();
		let _ret_ = GetCharABCWidthsFloatW(hdc, i_first, i_last, abc_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(abc_out_.assume_init()) }
	}
}

pub fn GetClipBox(hdc: HDc) -> (GdiRegionType, Rect) {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetClipBox(hdc: HDc, lprect: *mut Rect) -> GdiRegionType;
		} 
		let mut lprect_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = GetClipBox(hdc, lprect_out_.as_mut_ptr());
		(_ret_, lprect_out_.assume_init())
	}
}

pub fn GetClipRgn(hdc: HDc, hrgn: HRgn) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetClipRgn(hdc: HDc, hrgn: HRgn) -> i32;
		} 
		let _ret_ = GetClipRgn(hdc, hrgn);
		_ret_
	}
}

pub fn GetMetaRgn(hdc: HDc, hrgn: HRgn) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetMetaRgn(hdc: HDc, hrgn: HRgn) -> i32;
		} 
		let _ret_ = GetMetaRgn(hdc, hrgn);
		_ret_
	}
}

pub fn GetCurrentObject(hdc: HDc, r#type: ObjType) -> Result<HGdiObj, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCurrentObject(hdc: HDc, r#type: ObjType) -> *const c_void;
		} 
		let _ret_ = GetCurrentObject(hdc, r#type);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetCurrentPositionEx(hdc: HDc) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetCurrentPositionEx(hdc: HDc, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetCurrentPositionEx(hdc, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn GetDeviceCaps(hdc: Option<HDc>, index: GetDeviceCapsIndex) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetDeviceCaps(hdc: *const c_void, index: GetDeviceCapsIndex) -> i32;
		} 
		let _ret_ = GetDeviceCaps(transmute(hdc), index);
		_ret_
	}
}

pub fn GetDIBits(hdc: HDc, hbm: HBitmap, start: u32, lines: u32, lpv_bits: Option<&mut MaybeUninit<()>>, lpbmi: &mut BitmapInfo, usage: DibUsage) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetDIBits(hdc: HDc, hbm: HBitmap, start: u32, cLines: u32, lpvBits: Option<&mut MaybeUninit<()>>, lpbmi: &mut BitmapInfo, usage: DibUsage) -> i32;
		} 
		let _ret_ = GetDIBits(hdc, hbm, start, lines, lpv_bits, lpbmi, usage);
		_ret_
	}
}

pub unsafe fn get_di_bits() { todo!() }

pub unsafe fn GetFontData() { todo!() }

pub unsafe fn GetGlyphOutlineA() { todo!() }

pub unsafe fn GetGlyphOutlineW() { todo!() }

pub fn GetGraphicsMode(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetGraphicsMode(hdc: HDc) -> i32;
		} 
		let _ret_ = GetGraphicsMode(hdc);
		_ret_
	}
}

pub fn GetMapMode(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetMapMode(hdc: HDc) -> i32;
		} 
		let _ret_ = GetMapMode(hdc);
		_ret_
	}
}

pub unsafe fn GetMetaFileBitsEx() { todo!() }

pub fn GetMetaFileA(name: &str) -> Result<HMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetMetaFileA(lpName: *const u8) -> *const c_void;
		} 
		let _ret_ = GetMetaFileA(name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetMetaFileW(name: &str) -> Result<HMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetMetaFileW(lpName: *const u16) -> *const c_void;
		} 
		let _ret_ = GetMetaFileW(name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetNearestColor(hdc: HDc, color: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetNearestColor(hdc: HDc, color: u32) -> u32;
		} 
		let _ret_ = GetNearestColor(hdc, color);
		_ret_
	}
}

pub fn GetNearestPaletteIndex(h: HPalette, color: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetNearestPaletteIndex(h: HPalette, color: u32) -> u32;
		} 
		let _ret_ = GetNearestPaletteIndex(h, color);
		_ret_
	}
}

pub fn GetObjectType(h: HGdiObj) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetObjectType(h: HGdiObj) -> u32;
		} 
		let _ret_ = GetObjectType(h);
		_ret_
	}
}

pub unsafe fn GetOutlineTextMetricsA() { todo!() }

pub unsafe fn GetOutlineTextMetricsW() { todo!() }

pub unsafe fn GetPaletteEntries() { todo!() }

pub fn GetPixel(hdc: HDc, x: i32, y: i32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetPixel(hdc: HDc, x: i32, y: i32) -> u32;
		} 
		let _ret_ = GetPixel(hdc, x, y);
		_ret_
	}
}

pub fn GetPolyFillMode(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetPolyFillMode(hdc: HDc) -> i32;
		} 
		let _ret_ = GetPolyFillMode(hdc);
		_ret_
	}
}

pub unsafe fn GetRasterizerCaps() { todo!() }

pub fn GetRandomRgn(hdc: HDc, hrgn: HRgn, i: i32) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetRandomRgn(hdc: HDc, hrgn: HRgn, i: i32) -> i32;
		} 
		let _ret_ = GetRandomRgn(hdc, hrgn, i);
		_ret_
	}
}

pub unsafe fn GetRegionData() { todo!() }

pub fn GetRgnBox(hrgn: HRgn) -> (GdiRegionType, Rect) {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetRgnBox(hrgn: HRgn, lprc: *mut Rect) -> GdiRegionType;
		} 
		let mut lprc_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = GetRgnBox(hrgn, lprc_out_.as_mut_ptr());
		(_ret_, lprc_out_.assume_init())
	}
}

pub fn GetStockObject(i: GetStockObjectFlags) -> Result<HGdiObj, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetStockObject(i: GetStockObjectFlags) -> *const c_void;
		} 
		let _ret_ = GetStockObject(i);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetStretchBltMode(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetStretchBltMode(hdc: HDc) -> i32;
		} 
		let _ret_ = GetStretchBltMode(hdc);
		_ret_
	}
}

pub unsafe fn GetSystemPaletteEntries() { todo!() }

pub fn GetSystemPaletteUse(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetSystemPaletteUse(hdc: HDc) -> u32;
		} 
		let _ret_ = GetSystemPaletteUse(hdc);
		_ret_
	}
}

pub fn GetTextCharacterExtra(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextCharacterExtra(hdc: HDc) -> i32;
		} 
		let _ret_ = GetTextCharacterExtra(hdc);
		_ret_
	}
}

pub fn GetTextAlign(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextAlign(hdc: HDc) -> u32;
		} 
		let _ret_ = GetTextAlign(hdc);
		_ret_
	}
}

pub fn GetTextColor(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextColor(hdc: HDc) -> u32;
		} 
		let _ret_ = GetTextColor(hdc);
		_ret_
	}
}

pub fn GetTextExtentPointA(hdc: HDc, string: &str) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextExtentPointA(hdc: HDc, lpString: *const u8, c: i32, lpsz: *mut Size) -> Bool;
		} 
		let (string_ptr_, string_len_) = string.deconstruct();
		let mut lpsz_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetTextExtentPointA(hdc, string_ptr_, string_len_ as i32, lpsz_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsz_out_.assume_init()) }
	}
}

pub fn GetTextExtentPointW(hdc: HDc, string: &str) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextExtentPointW(hdc: HDc, lpString: *const u16, c: i32, lpsz: *mut Size) -> Bool;
		} 
		let string_utf16_ = string.encode_utf16().collect::<Vec<_>>();
		let (string_ptr_, string_len_) = string_utf16_.deconstruct();
		let mut lpsz_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetTextExtentPointW(hdc, string_ptr_, string_len_ as i32, lpsz_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsz_out_.assume_init()) }
	}
}

pub fn GetTextExtentPoint32A(hdc: HDc, string: &str) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextExtentPoint32A(hdc: HDc, lpString: *const u8, c: i32, psizl: *mut Size) -> Bool;
		} 
		let (string_ptr_, string_len_) = string.deconstruct();
		let mut psizl_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetTextExtentPoint32A(hdc, string_ptr_, string_len_ as i32, psizl_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(psizl_out_.assume_init()) }
	}
}

pub fn GetTextExtentPoint32W(hdc: HDc, string: &str) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextExtentPoint32W(hdc: HDc, lpString: *const u16, c: i32, psizl: *mut Size) -> Bool;
		} 
		let string_utf16_ = string.encode_utf16().collect::<Vec<_>>();
		let (string_ptr_, string_len_) = string_utf16_.deconstruct();
		let mut psizl_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetTextExtentPoint32W(hdc, string_ptr_, string_len_ as i32, psizl_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(psizl_out_.assume_init()) }
	}
}

	pub unsafe fn GetTextExtentExPointA() { todo!() }

	pub unsafe fn GetTextExtentExPointW() { todo!() }

pub fn GetFontLanguageInfo(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetFontLanguageInfo(hdc: HDc) -> u32;
		} 
		let _ret_ = GetFontLanguageInfo(hdc);
		_ret_
	}
}

	pub unsafe fn GetGlyphIndicesA() { todo!() }

	pub unsafe fn GetGlyphIndicesW() { todo!() }

pub fn GetTextExtentPointI(hdc: HDc, pgi_in: &[u16]) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextExtentPointI(hdc: HDc, pgiIn: *const u16, cgi: i32, psize: *mut Size) -> Bool;
		} 
		let (pgi_in_ptr_, pgi_in_len_) = pgi_in.deconstruct();
		let mut psize_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetTextExtentPointI(hdc, pgi_in_ptr_, pgi_in_len_ as i32, psize_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(psize_out_.assume_init()) }
	}
}

	pub unsafe fn GetTextExtentExPointI() { todo!() }

	pub unsafe fn GetCharWidthI() { todo!() }

	pub unsafe fn GetCharABCWidthsI() { todo!() }

pub fn AddFontResourceExA(name: &str, fl: FontResourceCharacteristics, res: &mut ()) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AddFontResourceExA(name: *const u8, fl: FontResourceCharacteristics, res: &mut ()) -> i32;
		} 
		let _ret_ = AddFontResourceExA(name.to_null_terminated().as_ptr_or_null(), fl, res);
		_ret_
	}
}

pub fn AddFontResourceExW(name: &str, fl: FontResourceCharacteristics, res: &mut ()) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AddFontResourceExW(name: *const u16, fl: FontResourceCharacteristics, res: &mut ()) -> i32;
		} 
		let _ret_ = AddFontResourceExW(name.to_utf16().as_ptr_or_null(), fl, res);
		_ret_
	}
}

pub fn RemoveFontResourceExA(name: &str, fl: u32, pdv: &mut ()) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RemoveFontResourceExA(name: *const u8, fl: u32, pdv: &mut ()) -> Bool;
		} 
		let _ret_ = RemoveFontResourceExA(name.to_null_terminated().as_ptr_or_null(), fl, pdv);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RemoveFontResourceExW(name: &str, fl: u32, pdv: &mut ()) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RemoveFontResourceExW(name: *const u16, fl: u32, pdv: &mut ()) -> Bool;
		} 
		let _ret_ = RemoveFontResourceExW(name.to_utf16().as_ptr_or_null(), fl, pdv);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn AddFontMemResourceEx(file_view: &[u8], pv_resrved: &mut (), num_fonts: &u32) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AddFontMemResourceEx(pFileView: *const u8, cjSize: u32, pvResrved: &mut (), pNumFonts: &u32) -> *const c_void;
		} 
		let (file_view_ptr_, file_view_len_) = file_view.deconstruct();
		let _ret_ = AddFontMemResourceEx(file_view_ptr_, file_view_len_ as u32, pv_resrved, num_fonts);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn RemoveFontMemResourceEx(h: Handle) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RemoveFontMemResourceEx(h: Handle) -> Bool;
		} 
		let _ret_ = RemoveFontMemResourceEx(h);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CreateFontIndirectExA(param0: &EnumLogFonTexDVA) -> Result<HFont, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateFontIndirectExA(param0: &EnumLogFonTexDVA) -> *const c_void;
		} 
		let _ret_ = CreateFontIndirectExA(param0);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateFontIndirectExW(param0: &EnumLogFonTexDVW) -> Result<HFont, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateFontIndirectExW(param0: &EnumLogFonTexDVW) -> *const c_void;
		} 
		let _ret_ = CreateFontIndirectExW(param0);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetViewportExtEx(hdc: HDc) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetViewportExtEx(hdc: HDc, lpsize: *mut Size) -> Bool;
		} 
		let mut lpsize_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetViewportExtEx(hdc, lpsize_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsize_out_.assume_init()) }
	}
}

pub fn GetViewportOrgEx(hdc: HDc) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetViewportOrgEx(hdc: HDc, lppoint: *mut Point) -> Bool;
		} 
		let mut lppoint_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetViewportOrgEx(hdc, lppoint_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppoint_out_.assume_init()) }
	}
}

pub fn GetWindowExtEx(hdc: HDc) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetWindowExtEx(hdc: HDc, lpsize: *mut Size) -> Bool;
		} 
		let mut lpsize_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = GetWindowExtEx(hdc, lpsize_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsize_out_.assume_init()) }
	}
}

pub fn GetWindowOrgEx(hdc: HDc) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetWindowOrgEx(hdc: HDc, lppoint: *mut Point) -> Bool;
		} 
		let mut lppoint_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetWindowOrgEx(hdc, lppoint_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppoint_out_.assume_init()) }
	}
}

pub fn IntersectClipRect(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn IntersectClipRect(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> GdiRegionType;
		} 
		let _ret_ = IntersectClipRect(hdc, left, top, right, bottom);
		_ret_
	}
}

pub fn InvertRgn(hdc: HDc, hrgn: HRgn) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn InvertRgn(hdc: HDc, hrgn: HRgn) -> Bool;
		} 
		let _ret_ = InvertRgn(hdc, hrgn);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LineDDA(x_start: i32, y_start: i32, x_end: i32, y_end: i32, proc: LinEddaProc, data: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn LineDDA(xStart: i32, yStart: i32, xEnd: i32, yEnd: i32, lpProc: LinEddaProc, data: LParam) -> Bool;
		} 
		let _ret_ = LineDDA(x_start, y_start, x_end, y_end, proc, data);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LineTo(hdc: HDc, x: i32, y: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn LineTo(hdc: HDc, x: i32, y: i32) -> Bool;
		} 
		let _ret_ = LineTo(hdc, x, y);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn MaskBlt(hdc_dest: HDc, x_dest: i32, y_dest: i32, width: i32, height: i32, hdc_src: HDc, x_src: i32, y_src: i32, hbm_mask: HBitmap, x_mask: i32, y_mask: i32, rop: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn MaskBlt(hdcDest: HDc, xDest: i32, yDest: i32, width: i32, height: i32, hdcSrc: HDc, xSrc: i32, ySrc: i32, hbmMask: HBitmap, xMask: i32, yMask: i32, rop: u32) -> Bool;
		} 
		let _ret_ = MaskBlt(hdc_dest, x_dest, y_dest, width, height, hdc_src, x_src, y_src, hbm_mask, x_mask, y_mask, rop);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PlgBlt(hdc_dest: HDc, point: &[Point; 3], hdc_src: HDc, x_src: i32, y_src: i32, width: i32, height: i32, hbm_mask: Option<HBitmap>, x_mask: i32, y_mask: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PlgBlt(hdcDest: HDc, lpPoint: *const Point, hdcSrc: HDc, xSrc: i32, ySrc: i32, width: i32, height: i32, hbmMask: *const c_void, xMask: i32, yMask: i32) -> Bool;
		} 
		let _ret_ = PlgBlt(hdc_dest, point.as_ptr_or_null(), hdc_src, x_src, y_src, width, height, transmute(hbm_mask), x_mask, y_mask);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn OffsetClipRgn(hdc: HDc, x: i32, y: i32) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn OffsetClipRgn(hdc: HDc, x: i32, y: i32) -> GdiRegionType;
		} 
		let _ret_ = OffsetClipRgn(hdc, x, y);
		_ret_
	}
}

pub fn OffsetRgn(hrgn: HRgn, x: i32, y: i32) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn OffsetRgn(hrgn: HRgn, x: i32, y: i32) -> GdiRegionType;
		} 
		let _ret_ = OffsetRgn(hrgn, x, y);
		_ret_
	}
}

pub fn PatBlt(hdc: HDc, x: i32, y: i32, w: i32, h: i32, rop: RopCode) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PatBlt(hdc: HDc, x: i32, y: i32, w: i32, h: i32, rop: RopCode) -> Bool;
		} 
		let _ret_ = PatBlt(hdc, x, y, w, h, rop);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn Pie(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn Pie(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> Bool;
		} 
		let _ret_ = Pie(hdc, left, top, right, bottom, xr1, yr1, xr2, yr2);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PlayMetaFile(hdc: HDc, hmf: HMetaFile) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PlayMetaFile(hdc: HDc, hmf: HMetaFile) -> Bool;
		} 
		let _ret_ = PlayMetaFile(hdc, hmf);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PaintRgn(hdc: HDc, hrgn: HRgn) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PaintRgn(hdc: HDc, hrgn: HRgn) -> Bool;
		} 
		let _ret_ = PaintRgn(hdc, hrgn);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PolyPolygon(hdc: HDc, apt: &Point, asz: &[i32]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PolyPolygon(hdc: HDc, apt: &Point, asz: *const i32, csz: i32) -> Bool;
		} 
		let (asz_ptr_, asz_len_) = asz.deconstruct();
		let _ret_ = PolyPolygon(hdc, apt, asz_ptr_, asz_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PtInRegion(hrgn: HRgn, x: i32, y: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PtInRegion(hrgn: HRgn, x: i32, y: i32) -> Bool;
		} 
		let _ret_ = PtInRegion(hrgn, x, y);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PtVisible(hdc: HDc, x: i32, y: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PtVisible(hdc: HDc, x: i32, y: i32) -> Bool;
		} 
		let _ret_ = PtVisible(hdc, x, y);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RectInRegion(hrgn: HRgn, lprect: &Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RectInRegion(hrgn: HRgn, lprect: &Rect) -> Bool;
		} 
		let _ret_ = RectInRegion(hrgn, lprect);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RectVisible(hdc: HDc, lprect: &Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RectVisible(hdc: HDc, lprect: &Rect) -> Bool;
		} 
		let _ret_ = RectVisible(hdc, lprect);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn Rectangle(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn Rectangle(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32) -> Bool;
		} 
		let _ret_ = Rectangle(hdc, left, top, right, bottom);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RestoreDC(hdc: HDc, saved_dc: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RestoreDC(hdc: HDc, nSavedDC: i32) -> Bool;
		} 
		let _ret_ = RestoreDC(hdc, saved_dc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ResetDCA(hdc: HDc, lpdm: &DevModeA) -> Result<HDc, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ResetDCA(hdc: HDc, lpdm: &DevModeA) -> *const c_void;
		} 
		let _ret_ = ResetDCA(hdc, lpdm);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn ResetDCW(hdc: HDc, lpdm: &DevModeW) -> Result<HDc, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ResetDCW(hdc: HDc, lpdm: &DevModeW) -> *const c_void;
		} 
		let _ret_ = ResetDCW(hdc, lpdm);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn RealizePalette(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RealizePalette(hdc: HDc) -> u32;
		} 
		let _ret_ = RealizePalette(hdc);
		_ret_
	}
}

pub fn RemoveFontResourceA(file_name: &str) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RemoveFontResourceA(lpFileName: *const u8) -> Bool;
		} 
		let _ret_ = RemoveFontResourceA(file_name.to_null_terminated().as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RemoveFontResourceW(file_name: &str) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RemoveFontResourceW(lpFileName: *const u16) -> Bool;
		} 
		let _ret_ = RemoveFontResourceW(file_name.to_utf16().as_ptr_or_null());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RoundRect(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn RoundRect(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> Bool;
		} 
		let _ret_ = RoundRect(hdc, left, top, right, bottom, width, height);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ResizePalette(hpal: HPalette, n: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ResizePalette(hpal: HPalette, n: u32) -> Bool;
		} 
		let _ret_ = ResizePalette(hpal, n);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SaveDC(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SaveDC(hdc: HDc) -> i32;
		} 
		let _ret_ = SaveDC(hdc);
		_ret_
	}
}

pub fn SelectClipRgn(hdc: HDc, hrgn: Option<HRgn>) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SelectClipRgn(hdc: HDc, hrgn: *const c_void) -> GdiRegionType;
		} 
		let _ret_ = SelectClipRgn(hdc, transmute(hrgn));
		_ret_
	}
}

pub fn ExtSelectClipRgn(hdc: HDc, hrgn: Option<HRgn>, mode: RgnCombineMode) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ExtSelectClipRgn(hdc: HDc, hrgn: *const c_void, mode: RgnCombineMode) -> GdiRegionType;
		} 
		let _ret_ = ExtSelectClipRgn(hdc, transmute(hrgn), mode);
		_ret_
	}
}

pub fn SetMetaRgn(hdc: HDc) -> GdiRegionType {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetMetaRgn(hdc: HDc) -> GdiRegionType;
		} 
		let _ret_ = SetMetaRgn(hdc);
		_ret_
	}
}

pub fn SelectObject(hdc: HDc, h: HGdiObj) -> Result<HGdiObj, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SelectObject(hdc: HDc, h: HGdiObj) -> *const c_void;
		} 
		let _ret_ = SelectObject(hdc, h);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SelectPalette(hdc: HDc, pal: HPalette, force_bkgd: bool) -> Result<HPalette, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SelectPalette(hdc: HDc, hPal: HPalette, bForceBkgd: Bool) -> *const c_void;
		} 
		let _ret_ = SelectPalette(hdc, pal, force_bkgd.to_bool());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetBkColor(hdc: HDc, color: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBkColor(hdc: HDc, color: u32) -> u32;
		} 
		let _ret_ = SetBkColor(hdc, color);
		_ret_
	}
}

pub fn SetDCBrushColor(hdc: HDc, color: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetDCBrushColor(hdc: HDc, color: u32) -> u32;
		} 
		let _ret_ = SetDCBrushColor(hdc, color);
		_ret_
	}
}

pub fn SetDCPenColor(hdc: HDc, color: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetDCPenColor(hdc: HDc, color: u32) -> u32;
		} 
		let _ret_ = SetDCPenColor(hdc, color);
		_ret_
	}
}

pub fn SetBkMode(hdc: HDc, mode: BackgroundMode) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBkMode(hdc: HDc, mode: BackgroundMode) -> i32;
		} 
		let _ret_ = SetBkMode(hdc, mode);
		_ret_
	}
}

pub fn SetBitmapBits(hbm: HBitmap, pv_bits: &[u8]) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBitmapBits(hbm: HBitmap, cb: u32, pvBits: *const u8) -> i32;
		} 
		let (pv_bits_ptr_, pv_bits_len_) = pv_bits.deconstruct();
		let _ret_ = SetBitmapBits(hbm, pv_bits_len_ as u32, pv_bits_ptr_);
		_ret_
	}
}

pub fn SetBoundsRect(hdc: HDc, lprect: Option<&Rect>, flags: SetBoundsRectFlags) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBoundsRect(hdc: HDc, lprect: *const c_void, flags: SetBoundsRectFlags) -> u32;
		} 
		let _ret_ = SetBoundsRect(hdc, transmute(lprect), flags);
		_ret_
	}
}

pub fn SetDIBits(hdc: Option<HDc>, hbm: HBitmap, start: u32, lines: u32, bits: *const impl Sized, lpbmi: &BitmapInfo, color_use: DibUsage) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetDIBits(hdc: *const c_void, hbm: HBitmap, start: u32, cLines: u32, lpBits: *const c_void, lpbmi: &BitmapInfo, ColorUse: DibUsage) -> i32;
		} 
		let _ret_ = SetDIBits(transmute(hdc), hbm, start, lines, bits as _, lpbmi, color_use);
		_ret_
	}
}

pub fn SetDIBitsToDevice(hdc: HDc, x_dest: i32, y_dest: i32, w: u32, h: u32, x_src: i32, y_src: i32, start_scan: u32, lines: u32, lpv_bits: *const impl Sized, lpbmi: &BitmapInfo, color_use: DibUsage) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetDIBitsToDevice(hdc: HDc, xDest: i32, yDest: i32, w: u32, h: u32, xSrc: i32, ySrc: i32, StartScan: u32, cLines: u32, lpvBits: *const c_void, lpbmi: &BitmapInfo, ColorUse: DibUsage) -> i32;
		} 
		let _ret_ = SetDIBitsToDevice(hdc, x_dest, y_dest, w, h, x_src, y_src, start_scan, lines, lpv_bits as _, lpbmi, color_use);
		_ret_
	}
}

pub fn SetMapperFlags(hdc: HDc, flags: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetMapperFlags(hdc: HDc, flags: u32) -> u32;
		} 
		let _ret_ = SetMapperFlags(hdc, flags);
		_ret_
	}
}

pub fn SetGraphicsMode(hdc: HDc, i_mode: GraphicsMode) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetGraphicsMode(hdc: HDc, iMode: GraphicsMode) -> i32;
		} 
		let _ret_ = SetGraphicsMode(hdc, i_mode);
		_ret_
	}
}

pub fn SetMapMode(hdc: HDc, i_mode: HDcMapMode) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetMapMode(hdc: HDc, iMode: HDcMapMode) -> i32;
		} 
		let _ret_ = SetMapMode(hdc, i_mode);
		_ret_
	}
}

pub fn SetLayout(hdc: HDc, l: DcLayout) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetLayout(hdc: HDc, l: DcLayout) -> u32;
		} 
		let _ret_ = SetLayout(hdc, l);
		_ret_
	}
}

pub fn GetLayout(hdc: HDc) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetLayout(hdc: HDc) -> u32;
		} 
		let _ret_ = GetLayout(hdc);
		_ret_
	}
}

pub fn SetMetaFileBitsEx(data: &[u8]) -> Result<HMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetMetaFileBitsEx(cbBuffer: u32, lpData: *const u8) -> *const c_void;
		} 
		let (data_ptr_, data_len_) = data.deconstruct();
		let _ret_ = SetMetaFileBitsEx(data_len_ as u32, data_ptr_);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn SetPaletteEntries(hpal: HPalette, i_start: u32, pal_entries: &[PaletteEntry]) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetPaletteEntries(hpal: HPalette, iStart: u32, cEntries: u32, pPalEntries: *const PaletteEntry) -> u32;
		} 
		let (pal_entries_ptr_, pal_entries_len_) = pal_entries.deconstruct();
		let _ret_ = SetPaletteEntries(hpal, i_start, pal_entries_len_ as u32, pal_entries_ptr_);
		_ret_
	}
}

pub fn SetPixel(hdc: HDc, x: i32, y: i32, color: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetPixel(hdc: HDc, x: i32, y: i32, color: u32) -> u32;
		} 
		let _ret_ = SetPixel(hdc, x, y, color);
		_ret_
	}
}

pub fn SetPixelV(hdc: HDc, x: i32, y: i32, color: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetPixelV(hdc: HDc, x: i32, y: i32, color: u32) -> Bool;
		} 
		let _ret_ = SetPixelV(hdc, x, y, color);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetPolyFillMode(hdc: HDc, mode: CreatePolygonRgnMode) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetPolyFillMode(hdc: HDc, mode: CreatePolygonRgnMode) -> i32;
		} 
		let _ret_ = SetPolyFillMode(hdc, mode);
		_ret_
	}
}

pub fn SetRectRgn(hrgn: HRgn, left: i32, top: i32, right: i32, bottom: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetRectRgn(hrgn: HRgn, left: i32, top: i32, right: i32, bottom: i32) -> Bool;
		} 
		let _ret_ = SetRectRgn(hrgn, left, top, right, bottom);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn StretchDIBits(hdc: HDc, x_dest: i32, y_dest: i32, dest_width: i32, dest_height: i32, x_src: i32, y_src: i32, src_width: i32, src_height: i32, bits: *const (), lpbmi: &BitmapInfo, i_usage: DibUsage, rop: RopCode) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn StretchDIBits(hdc: HDc, xDest: i32, yDest: i32, DestWidth: i32, DestHeight: i32, xSrc: i32, ySrc: i32, SrcWidth: i32, SrcHeight: i32, lpBits: *const c_void, lpbmi: &BitmapInfo, iUsage: DibUsage, rop: RopCode) -> i32;
		} 
		let _ret_ = StretchDIBits(hdc, x_dest, y_dest, dest_width, dest_height, x_src, y_src, src_width, src_height, bits as _, lpbmi, i_usage, rop);
		_ret_
	}
}

pub fn SetROP2(hdc: HDc, rop2: R2Mode) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetROP2(hdc: HDc, rop2: R2Mode) -> i32;
		} 
		let _ret_ = SetROP2(hdc, rop2);
		_ret_
	}
}

pub fn SetStretchBltMode(hdc: HDc, mode: StretchBltMode) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetStretchBltMode(hdc: HDc, mode: StretchBltMode) -> i32;
		} 
		let _ret_ = SetStretchBltMode(hdc, mode);
		_ret_
	}
}

pub fn SetSystemPaletteUse(hdc: HDc, r#use: SystemPaletteUse) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetSystemPaletteUse(hdc: HDc, r#use: SystemPaletteUse) -> u32;
		} 
		let _ret_ = SetSystemPaletteUse(hdc, r#use);
		_ret_
	}
}

pub fn SetTextCharacterExtra(hdc: HDc, extra: i32) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetTextCharacterExtra(hdc: HDc, extra: i32) -> i32;
		} 
		let _ret_ = SetTextCharacterExtra(hdc, extra);
		_ret_
	}
}

pub fn SetTextColor(hdc: HDc, color: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetTextColor(hdc: HDc, color: u32) -> u32;
		} 
		let _ret_ = SetTextColor(hdc, color);
		_ret_
	}
}

pub fn SetTextAlign(hdc: HDc, align: TextAlignOptions) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetTextAlign(hdc: HDc, align: TextAlignOptions) -> u32;
		} 
		let _ret_ = SetTextAlign(hdc, align);
		_ret_
	}
}

pub fn SetTextJustification(hdc: HDc, extra: i32, count: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetTextJustification(hdc: HDc, extra: i32, count: i32) -> Bool;
		} 
		let _ret_ = SetTextJustification(hdc, extra, count);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn UpdateColors(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn UpdateColors(hdc: HDc) -> Bool;
		} 
		let _ret_ = UpdateColors(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GdiGradientFill(hdc: HDc, vertex: &[TriVertex], mesh: *const impl Sized, count: u32, ul_mode: GradientFill) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GdiGradientFill(hdc: HDc, pVertex: *const TriVertex, nVertex: u32, pMesh: *const c_void, nCount: u32, ulMode: GradientFill) -> Bool;
		} 
		let (vertex_ptr_, vertex_len_) = vertex.deconstruct();
		let _ret_ = GdiGradientFill(hdc, vertex_ptr_, vertex_len_ as u32, mesh as _, count, ul_mode);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PlayMetaFileRecord(hdc: HDc, handle_table: &[HandleTable], mr: &MetaRecord) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PlayMetaFileRecord(hdc: HDc, lpHandleTable: *const HandleTable, lpMR: &MetaRecord, noObjs: u32) -> Bool;
		} 
		let (handle_table_ptr_, handle_table_len_) = handle_table.deconstruct();
		let _ret_ = PlayMetaFileRecord(hdc, handle_table_ptr_, mr, handle_table_len_ as u32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumMetaFile(hdc: HDc, hmf: HMetaFile, proc: MFEnumProc, param3: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumMetaFile(hdc: HDc, hmf: HMetaFile, proc: MFEnumProc, param3: LParam) -> Bool;
		} 
		let _ret_ = EnumMetaFile(hdc, hmf, proc, param3);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CloseEnhMetaFile(hdc: HDc) -> Result<HEnhMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CloseEnhMetaFile(hdc: HDc) -> *const c_void;
		} 
		let _ret_ = CloseEnhMetaFile(hdc);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CopyEnhMetaFileA(enh: HEnhMetaFile, file_name: Option<&str>) -> Result<HEnhMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CopyEnhMetaFileA(hEnh: HEnhMetaFile, lpFileName: *const u8) -> *const c_void;
		} 
		let _ret_ = CopyEnhMetaFileA(enh, file_name.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CopyEnhMetaFileW(enh: HEnhMetaFile, file_name: Option<&str>) -> Result<HEnhMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CopyEnhMetaFileW(hEnh: HEnhMetaFile, lpFileName: *const u16) -> *const c_void;
		} 
		let _ret_ = CopyEnhMetaFileW(enh, file_name.map(str::to_utf16).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateEnhMetaFileA(hdc: Option<HDc>, filename: Option<&str>, lprc: Option<&Rect>, desc: Option<&str>) -> Result<HdcMetdataEnhFileHandle, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateEnhMetaFileA(hdc: *const c_void, lpFilename: *const u8, lprc: *const c_void, lpDesc: *const u8) -> *const c_void;
		} 
		let _ret_ = CreateEnhMetaFileA(transmute(hdc), filename.map(str::to_null_terminated).as_ptr_or_null(), transmute(lprc), desc.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateEnhMetaFileW(hdc: Option<HDc>, filename: Option<&str>, lprc: Option<&Rect>, desc: Option<&str>) -> Result<HdcMetdataEnhFileHandle, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateEnhMetaFileW(hdc: *const c_void, lpFilename: *const u16, lprc: *const c_void, lpDesc: *const u16) -> *const c_void;
		} 
		let _ret_ = CreateEnhMetaFileW(transmute(hdc), filename.map(str::to_utf16).as_ptr_or_null(), transmute(lprc), desc.map(str::to_utf16).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DeleteEnhMetaFile(hmf: Option<HEnhMetaFile>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn DeleteEnhMetaFile(hmf: *const c_void) -> Bool;
		} 
		let _ret_ = DeleteEnhMetaFile(transmute(hmf));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumEnhMetaFile(hdc: Option<HDc>, hmf: HEnhMetaFile, proc: EnhMFEnumProc, param3: *const (), rect: Option<&Rect>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EnumEnhMetaFile(hdc: *const c_void, hmf: HEnhMetaFile, proc: EnhMFEnumProc, param3: *const c_void, lpRect: *const c_void) -> Bool;
		} 
		let _ret_ = EnumEnhMetaFile(transmute(hdc), hmf, proc, param3 as _, transmute(rect));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetEnhMetaFileA(name: &str) -> Result<HEnhMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetEnhMetaFileA(lpName: *const u8) -> *const c_void;
		} 
		let _ret_ = GetEnhMetaFileA(name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetEnhMetaFileW(name: &str) -> Result<HEnhMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetEnhMetaFileW(lpName: *const u16) -> *const c_void;
		} 
		let _ret_ = GetEnhMetaFileW(name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub unsafe fn GetEnhMetaFileBits() { todo!() }

pub unsafe fn GetEnhMetaFileDescriptionA() { todo!() }

pub unsafe fn GetEnhMetaFileDescriptionW() { todo!() }

pub unsafe fn GetEnhMetaFileHeader() { todo!() }

pub unsafe fn GetEnhMetaFilePaletteEntries() { todo!() }

pub unsafe fn GetWinMetaFileBits() { todo!() }

pub fn PlayEnhMetaFile(hdc: HDc, hmf: HEnhMetaFile, lprect: &Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PlayEnhMetaFile(hdc: HDc, hmf: HEnhMetaFile, lprect: &Rect) -> Bool;
		} 
		let _ret_ = PlayEnhMetaFile(hdc, hmf, lprect);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PlayEnhMetaFileRecord(hdc: HDc, pht: &[HandleTable], pmr: &EnhMetaRecord) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PlayEnhMetaFileRecord(hdc: HDc, pht: *const HandleTable, pmr: &EnhMetaRecord, cht: u32) -> Bool;
		} 
		let (pht_ptr_, pht_len_) = pht.deconstruct();
		let _ret_ = PlayEnhMetaFileRecord(hdc, pht_ptr_, pmr, pht_len_ as u32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetEnhMetaFileBits(pb: &[u8]) -> Result<HEnhMetaFile, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetEnhMetaFileBits(nSize: u32, pb: *const u8) -> *const c_void;
		} 
		let (pb_ptr_, pb_len_) = pb.deconstruct();
		let _ret_ = SetEnhMetaFileBits(pb_len_ as u32, pb_ptr_);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GdiComment(hdc: HDc, data: &[u8]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GdiComment(hdc: HDc, nSize: u32, lpData: *const u8) -> Bool;
		} 
		let (data_ptr_, data_len_) = data.deconstruct();
		let _ret_ = GdiComment(hdc, data_len_ as u32, data_ptr_);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetTextMetricsA(hdc: HDc) -> Result<TextMetricA, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextMetricsA(hdc: HDc, lptm: *mut TextMetricA) -> Bool;
		} 
		let mut lptm_out_: MaybeUninit<TextMetricA> = MaybeUninit::zeroed();
		let _ret_ = GetTextMetricsA(hdc, lptm_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lptm_out_.assume_init()) }
	}
}

pub fn GetTextMetricsW(hdc: HDc) -> Result<TextMetricW, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetTextMetricsW(hdc: HDc, lptm: *mut TextMetricW) -> Bool;
		} 
		let mut lptm_out_: MaybeUninit<TextMetricW> = MaybeUninit::zeroed();
		let _ret_ = GetTextMetricsW(hdc, lptm_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lptm_out_.assume_init()) }
	}
}

pub fn AngleArc(hdc: HDc, x: i32, y: i32, r: u32, start_angle: f32, sweep_angle: f32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AngleArc(hdc: HDc, x: i32, y: i32, r: u32, StartAngle: f32, SweepAngle: f32) -> Bool;
		} 
		let _ret_ = AngleArc(hdc, x, y, r, start_angle, sweep_angle);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PolyPolyline(hdc: HDc, apt: &Point, asz: &[u32]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PolyPolyline(hdc: HDc, apt: &Point, asz: *const u32, csz: u32) -> Bool;
		} 
		let (asz_ptr_, asz_len_) = asz.deconstruct();
		let _ret_ = PolyPolyline(hdc, apt, asz_ptr_, asz_len_ as u32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetWorldTransform(hdc: HDc) -> Result<XForm, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetWorldTransform(hdc: HDc, lpxf: *mut XForm) -> Bool;
		} 
		let mut lpxf_out_: MaybeUninit<XForm> = MaybeUninit::zeroed();
		let _ret_ = GetWorldTransform(hdc, lpxf_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpxf_out_.assume_init()) }
	}
}

pub fn SetWorldTransform(hdc: HDc, lpxf: &XForm) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetWorldTransform(hdc: HDc, lpxf: &XForm) -> Bool;
		} 
		let _ret_ = SetWorldTransform(hdc, lpxf);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ModifyWorldTransform(hdc: HDc, lpxf: Option<&XForm>, mode: ModifyWorldTransformMode) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ModifyWorldTransform(hdc: HDc, lpxf: *const c_void, mode: ModifyWorldTransformMode) -> Bool;
		} 
		let _ret_ = ModifyWorldTransform(hdc, transmute(lpxf), mode);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CombineTransform(lpxf1: &XForm, lpxf2: &XForm) -> Result<XForm, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CombineTransform(lpxfOut: *mut XForm, lpxf1: &XForm, lpxf2: &XForm) -> Bool;
		} 
		let mut lpxf_out_out_: MaybeUninit<XForm> = MaybeUninit::zeroed();
		let _ret_ = CombineTransform(lpxf_out_out_.as_mut_ptr(), lpxf1, lpxf2);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpxf_out_out_.assume_init()) }
	}
}

pub unsafe fn CreateDIBSection() { todo!() }

pub unsafe fn GetDIBColorTable() { todo!() }

pub fn SetDIBColorTable(hdc: HDc, i_start: u32, prgbq: &[RgbQuad]) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetDIBColorTable(hdc: HDc, iStart: u32, cEntries: u32, prgbq: *const RgbQuad) -> u32;
		} 
		let (prgbq_ptr_, prgbq_len_) = prgbq.deconstruct();
		let _ret_ = SetDIBColorTable(hdc, i_start, prgbq_len_ as u32, prgbq_ptr_);
		_ret_
	}
}

pub fn SetColorAdjustment(hdc: HDc, lpca: &ColorAdjustment) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetColorAdjustment(hdc: HDc, lpca: &ColorAdjustment) -> Bool;
		} 
		let _ret_ = SetColorAdjustment(hdc, lpca);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetColorAdjustment(hdc: HDc) -> Result<ColorAdjustment, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetColorAdjustment(hdc: HDc, lpca: *mut ColorAdjustment) -> Bool;
		} 
		let mut lpca_out_: MaybeUninit<ColorAdjustment> = MaybeUninit::zeroed();
		let _ret_ = GetColorAdjustment(hdc, lpca_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpca_out_.assume_init()) }
	}
}

pub fn CreateHalftonePalette(hdc: Option<HDc>) -> Result<HPalette, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreateHalftonePalette(hdc: *const c_void) -> *const c_void;
		} 
		let _ret_ = CreateHalftonePalette(transmute(hdc));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn AbortPath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn AbortPath(hdc: HDc) -> Bool;
		} 
		let _ret_ = AbortPath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ArcTo(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ArcTo(hdc: HDc, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> Bool;
		} 
		let _ret_ = ArcTo(hdc, left, top, right, bottom, xr1, yr1, xr2, yr2);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn BeginPath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn BeginPath(hdc: HDc) -> Bool;
		} 
		let _ret_ = BeginPath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CloseFigure(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CloseFigure(hdc: HDc) -> Bool;
		} 
		let _ret_ = CloseFigure(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EndPath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn EndPath(hdc: HDc) -> Bool;
		} 
		let _ret_ = EndPath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FillPath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn FillPath(hdc: HDc) -> Bool;
		} 
		let _ret_ = FillPath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FlattenPath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn FlattenPath(hdc: HDc) -> Bool;
		} 
		let _ret_ = FlattenPath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

	pub unsafe fn GetPath() { todo!() }

pub fn PathToRegion(hdc: HDc) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PathToRegion(hdc: HDc) -> *const c_void;
		} 
		let _ret_ = PathToRegion(hdc);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

	pub unsafe fn PolyDraw() { todo!() }

pub fn SelectClipPath(hdc: HDc, mode: RgnCombineMode) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SelectClipPath(hdc: HDc, mode: RgnCombineMode) -> Bool;
		} 
		let _ret_ = SelectClipPath(hdc, mode);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetArcDirection(hdc: HDc, dir: ArcDirection) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetArcDirection(hdc: HDc, dir: ArcDirection) -> i32;
		} 
		let _ret_ = SetArcDirection(hdc, dir);
		_ret_
	}
}

pub fn SetMiterLimit(hdc: HDc, limit: f32, old: Option<&mut MaybeUninit<f32>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetMiterLimit(hdc: HDc, limit: f32, old: Option<&mut MaybeUninit<f32>>) -> Bool;
		} 
		let _ret_ = SetMiterLimit(hdc, limit, old);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn set_miter_limit(hdc: HDc, limit: f32) -> Result<f32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetMiterLimit(hdc: HDc, limit: f32, old: *mut f32) -> Bool;
		} 
		let mut old_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
		let _ret_ = SetMiterLimit(hdc, limit, old_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(old_out_.assume_init()) }
	}
}

pub fn StrokeAndFillPath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn StrokeAndFillPath(hdc: HDc) -> Bool;
		} 
		let _ret_ = StrokeAndFillPath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn StrokePath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn StrokePath(hdc: HDc) -> Bool;
		} 
		let _ret_ = StrokePath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn WidenPath(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn WidenPath(hdc: HDc) -> Bool;
		} 
		let _ret_ = WidenPath(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ExtCreatePen(i_pen_style: PenStyle, width: u32, plbrush: &LogBrush, pstyle: Option<&[u32]>) -> Result<HPen, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ExtCreatePen(iPenStyle: PenStyle, cWidth: u32, plbrush: &LogBrush, cStyle: u32, pstyle: *const u32) -> *const c_void;
		} 
		let (pstyle_ptr_, pstyle_len_) = pstyle.deconstruct();
		let _ret_ = ExtCreatePen(i_pen_style, width, plbrush, pstyle_len_ as u32, pstyle_ptr_);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetMiterLimit(hdc: HDc) -> Result<f32, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetMiterLimit(hdc: HDc, plimit: *mut f32) -> Bool;
		} 
		let mut plimit_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
		let _ret_ = GetMiterLimit(hdc, plimit_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(plimit_out_.assume_init()) }
	}
}

pub fn GetArcDirection(hdc: HDc) -> i32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetArcDirection(hdc: HDc) -> i32;
		} 
		let _ret_ = GetArcDirection(hdc);
		_ret_
	}
}

pub unsafe fn GetObjectW() { todo!() }

pub fn MoveToEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn MoveToEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Bool;
		} 
		let _ret_ = MoveToEx(hdc, x, y, lppt);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn move_to_ex(hdc: HDc, x: i32, y: i32) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn MoveToEx(hdc: HDc, x: i32, y: i32, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = MoveToEx(hdc, x, y, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn TextOutA(hdc: HDc, x: i32, y: i32, string: &str) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn TextOutA(hdc: HDc, x: i32, y: i32, lpString: *const u8, c: i32) -> Bool;
		} 
		let (string_ptr_, string_len_) = string.deconstruct();
		let _ret_ = TextOutA(hdc, x, y, string_ptr_, string_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn TextOutW(hdc: HDc, x: i32, y: i32, string: &str) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn TextOutW(hdc: HDc, x: i32, y: i32, lpString: *const u16, c: i32) -> Bool;
		} 
		let string_utf16_ = string.encode_utf16().collect::<Vec<_>>();
		let (string_ptr_, string_len_) = string_utf16_.deconstruct();
		let _ret_ = TextOutW(hdc, x, y, string_ptr_, string_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

	pub unsafe fn ExtTextOutA() { todo!() }

	pub unsafe fn ExtTextOutW() { todo!() }

pub fn PolyTextOutA(hdc: HDc, ppt: &[PolyTextA]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PolyTextOutA(hdc: HDc, ppt: *const PolyTextA, nstrings: i32) -> Bool;
		} 
		let (ppt_ptr_, ppt_len_) = ppt.deconstruct();
		let _ret_ = PolyTextOutA(hdc, ppt_ptr_, ppt_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PolyTextOutW(hdc: HDc, ppt: &[PolyTextW]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PolyTextOutW(hdc: HDc, ppt: *const PolyTextW, nstrings: i32) -> Bool;
		} 
		let (ppt_ptr_, ppt_len_) = ppt.deconstruct();
		let _ret_ = PolyTextOutW(hdc, ppt_ptr_, ppt_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn CreatePolygonRgn(pptl: &[Point], i_mode: CreatePolygonRgnMode) -> Result<HRgn, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn CreatePolygonRgn(pptl: *const Point, cPoint: i32, iMode: CreatePolygonRgnMode) -> *const c_void;
		} 
		let (pptl_ptr_, pptl_len_) = pptl.deconstruct();
		let _ret_ = CreatePolygonRgn(pptl_ptr_, pptl_len_ as i32, i_mode);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DPtoLP(hdc: HDc, lppt: &mut [Point]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn DPtoLP(hdc: HDc, lppt: *mut Point, c: i32) -> Bool;
		} 
		let (lppt_ptr_, lppt_len_) = lppt.deconstruct();
		let _ret_ = DPtoLP(hdc, lppt_ptr_, lppt_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LPtoDP(hdc: HDc, lppt: &mut [Point]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn LPtoDP(hdc: HDc, lppt: *mut Point, c: i32) -> Bool;
		} 
		let (lppt_ptr_, lppt_len_) = lppt.deconstruct();
		let _ret_ = LPtoDP(hdc, lppt_ptr_, lppt_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn Polygon(hdc: HDc, apt: &[Point]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn Polygon(hdc: HDc, apt: *const Point, cpt: i32) -> Bool;
		} 
		let (apt_ptr_, apt_len_) = apt.deconstruct();
		let _ret_ = Polygon(hdc, apt_ptr_, apt_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn Polyline(hdc: HDc, apt: &[Point]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn Polyline(hdc: HDc, apt: *const Point, cpt: i32) -> Bool;
		} 
		let (apt_ptr_, apt_len_) = apt.deconstruct();
		let _ret_ = Polyline(hdc, apt_ptr_, apt_len_ as i32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PolyBezier(hdc: HDc, apt: &[Point]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PolyBezier(hdc: HDc, apt: *const Point, cpt: u32) -> Bool;
		} 
		let (apt_ptr_, apt_len_) = apt.deconstruct();
		let _ret_ = PolyBezier(hdc, apt_ptr_, apt_len_ as u32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PolyBezierTo(hdc: HDc, apt: &[Point]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PolyBezierTo(hdc: HDc, apt: *const Point, cpt: u32) -> Bool;
		} 
		let (apt_ptr_, apt_len_) = apt.deconstruct();
		let _ret_ = PolyBezierTo(hdc, apt_ptr_, apt_len_ as u32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PolylineTo(hdc: HDc, apt: &[Point]) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn PolylineTo(hdc: HDc, apt: *const Point, cpt: u32) -> Bool;
		} 
		let (apt_ptr_, apt_len_) = apt.deconstruct();
		let _ret_ = PolylineTo(hdc, apt_ptr_, apt_len_ as u32);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetViewportExtEx(hdc: HDc, x: i32, y: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetViewportExtEx(hdc: HDc, x: i32, y: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Bool;
		} 
		let _ret_ = SetViewportExtEx(hdc, x, y, lpsz);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn set_viewport_ext_ex(hdc: HDc, x: i32, y: i32) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetViewportExtEx(hdc: HDc, x: i32, y: i32, lpsz: *mut Size) -> Bool;
		} 
		let mut lpsz_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = SetViewportExtEx(hdc, x, y, lpsz_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsz_out_.assume_init()) }
	}
}

pub fn SetViewportOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetViewportOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Bool;
		} 
		let _ret_ = SetViewportOrgEx(hdc, x, y, lppt);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn set_viewport_org_ex(hdc: HDc, x: i32, y: i32) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetViewportOrgEx(hdc: HDc, x: i32, y: i32, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = SetViewportOrgEx(hdc, x, y, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn SetWindowExtEx(hdc: HDc, x: i32, y: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetWindowExtEx(hdc: HDc, x: i32, y: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Bool;
		} 
		let _ret_ = SetWindowExtEx(hdc, x, y, lpsz);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn set_window_ext_ex(hdc: HDc, x: i32, y: i32) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetWindowExtEx(hdc: HDc, x: i32, y: i32, lpsz: *mut Size) -> Bool;
		} 
		let mut lpsz_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = SetWindowExtEx(hdc, x, y, lpsz_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsz_out_.assume_init()) }
	}
}

pub fn SetWindowOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetWindowOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Bool;
		} 
		let _ret_ = SetWindowOrgEx(hdc, x, y, lppt);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn set_window_org_ex(hdc: HDc, x: i32, y: i32) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetWindowOrgEx(hdc: HDc, x: i32, y: i32, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = SetWindowOrgEx(hdc, x, y, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn OffsetViewportOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn OffsetViewportOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Bool;
		} 
		let _ret_ = OffsetViewportOrgEx(hdc, x, y, lppt);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn offset_viewport_org_ex(hdc: HDc, x: i32, y: i32) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn OffsetViewportOrgEx(hdc: HDc, x: i32, y: i32, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = OffsetViewportOrgEx(hdc, x, y, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn OffsetWindowOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn OffsetWindowOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Bool;
		} 
		let _ret_ = OffsetWindowOrgEx(hdc, x, y, lppt);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn offset_window_org_ex(hdc: HDc, x: i32, y: i32) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn OffsetWindowOrgEx(hdc: HDc, x: i32, y: i32, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = OffsetWindowOrgEx(hdc, x, y, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn ScaleViewportExtEx(hdc: HDc, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ScaleViewportExtEx(hdc: HDc, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Bool;
		} 
		let _ret_ = ScaleViewportExtEx(hdc, xn, dx, yn, yd, lpsz);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn scale_viewport_ext_ex(hdc: HDc, xn: i32, dx: i32, yn: i32, yd: i32) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ScaleViewportExtEx(hdc: HDc, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: *mut Size) -> Bool;
		} 
		let mut lpsz_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = ScaleViewportExtEx(hdc, xn, dx, yn, yd, lpsz_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsz_out_.assume_init()) }
	}
}

pub fn ScaleWindowExtEx(hdc: HDc, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ScaleWindowExtEx(hdc: HDc, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Bool;
		} 
		let _ret_ = ScaleWindowExtEx(hdc, xn, xd, yn, yd, lpsz);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn scale_window_ext_ex(hdc: HDc, xn: i32, xd: i32, yn: i32, yd: i32) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn ScaleWindowExtEx(hdc: HDc, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: *mut Size) -> Bool;
		} 
		let mut lpsz_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = ScaleWindowExtEx(hdc, xn, xd, yn, yd, lpsz_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsz_out_.assume_init()) }
	}
}

pub fn SetBitmapDimensionEx(hbm: HBitmap, w: i32, h: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBitmapDimensionEx(hbm: HBitmap, w: i32, h: i32, lpsz: Option<&mut MaybeUninit<Size>>) -> Bool;
		} 
		let _ret_ = SetBitmapDimensionEx(hbm, w, h, lpsz);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn set_bitmap_dimension_ex(hbm: HBitmap, w: i32, h: i32) -> Result<Size, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBitmapDimensionEx(hbm: HBitmap, w: i32, h: i32, lpsz: *mut Size) -> Bool;
		} 
		let mut lpsz_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = SetBitmapDimensionEx(hbm, w, h, lpsz_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lpsz_out_.assume_init()) }
	}
}

pub fn SetBrushOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBrushOrgEx(hdc: HDc, x: i32, y: i32, lppt: Option<&mut MaybeUninit<Point>>) -> Bool;
		} 
		let _ret_ = SetBrushOrgEx(hdc, x, y, lppt);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn set_brush_org_ex(hdc: HDc, x: i32, y: i32) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn SetBrushOrgEx(hdc: HDc, x: i32, y: i32, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = SetBrushOrgEx(hdc, x, y, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub unsafe fn GetTextFaceA() { todo!() }

pub unsafe fn GetTextFaceW() { todo!() }

pub unsafe fn GetKerningPairsA() { todo!() }

pub unsafe fn GetKerningPairsW() { todo!() }

pub fn GetDCOrgEx(hdc: HDc) -> Result<Point, Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GetDCOrgEx(hdc: HDc, lppt: *mut Point) -> Bool;
		} 
		let mut lppt_out_: MaybeUninit<Point> = MaybeUninit::zeroed();
		let _ret_ = GetDCOrgEx(hdc, lppt_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lppt_out_.assume_init()) }
	}
}

pub fn FixBrushOrgEx(hdc: HDc, x: i32, y: i32, ptl: Option<&Point>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn FixBrushOrgEx(hdc: HDc, x: i32, y: i32, ptl: *const c_void) -> Bool;
		} 
		let _ret_ = FixBrushOrgEx(hdc, x, y, transmute(ptl));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn UnrealizeObject(h: HGdiObj) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn UnrealizeObject(h: HGdiObj) -> Bool;
		} 
		let _ret_ = UnrealizeObject(h);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GdiFlush() -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GdiFlush() -> Bool;
		} 
		let _ret_ = GdiFlush();
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GdiSetBatchLimit(dw: u32) -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GdiSetBatchLimit(dw: u32) -> u32;
		} 
		let _ret_ = GdiSetBatchLimit(dw);
		_ret_
	}
}

pub fn GdiGetBatchLimit() -> u32 {
	unsafe {
		#[link(name = "GDI32")]
		extern "system" {
			fn GdiGetBatchLimit() -> u32;
		} 
		let _ret_ = GdiGetBatchLimit();
		_ret_
	}
}

pub fn WglSwapMultipleBuffers(param0: u32, param1: &WglSwap) -> u32 {
	unsafe {
		#[link(name = "OPENGL32")]
		extern "system" {
			fn wglSwapMultipleBuffers(param0: u32, param1: &WglSwap) -> u32;
		} 
		let _ret_ = wglSwapMultipleBuffers(param0, param1);
		_ret_
	}
}

pub fn TTEmbedFont(dc: HDc, ul_flags: TtEmbedFlags, ul_char_set: EmbedFontCharSet, write_to_stream: WriteEmbedProc, lpv_write_stream: *const impl Sized, pus_char_code_set: &[u16], us_language: u16, tt_embed_info: Option<&TtEmbedInfo>) -> (i32, EmbeddedFontPrivStatus, u32) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTEmbedFont(hDC: HDc, ulFlags: TtEmbedFlags, ulCharSet: EmbedFontCharSet, pulPrivStatus: *mut EmbeddedFontPrivStatus, pulStatus: *mut u32, lpfnWriteToStream: WriteEmbedProc, lpvWriteStream: *const c_void, pusCharCodeSet: *const u16, usCharCodeCount: u16, usLanguage: u16, pTTEmbedInfo: *const c_void) -> i32;
		} 
		let mut pul_priv_status_out_: MaybeUninit<EmbeddedFontPrivStatus> = MaybeUninit::zeroed();
		let mut pul_status_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let (pus_char_code_set_ptr_, pus_char_code_set_len_) = pus_char_code_set.deconstruct();
		let _ret_ = TTEmbedFont(dc, ul_flags, ul_char_set, pul_priv_status_out_.as_mut_ptr(), pul_status_out_.as_mut_ptr(), write_to_stream, lpv_write_stream as _, pus_char_code_set_ptr_, pus_char_code_set_len_ as u16, us_language, transmute(tt_embed_info));
		(_ret_, pul_priv_status_out_.assume_init(), pul_status_out_.assume_init())
	}
}

pub fn TTEmbedFontFromFileA(dc: HDc, sz_font_file_name: &str, us_ttc_index: u16, ul_flags: TtEmbedFlags, ul_char_set: EmbedFontCharSet, write_to_stream: WriteEmbedProc, lpv_write_stream: *const impl Sized, pus_char_code_set: &[u16], us_language: u16, tt_embed_info: Option<&TtEmbedInfo>) -> (i32, EmbeddedFontPrivStatus, u32) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTEmbedFontFromFileA(hDC: HDc, szFontFileName: *const u8, usTTCIndex: u16, ulFlags: TtEmbedFlags, ulCharSet: EmbedFontCharSet, pulPrivStatus: *mut EmbeddedFontPrivStatus, pulStatus: *mut u32, lpfnWriteToStream: WriteEmbedProc, lpvWriteStream: *const c_void, pusCharCodeSet: *const u16, usCharCodeCount: u16, usLanguage: u16, pTTEmbedInfo: *const c_void) -> i32;
		} 
		let mut pul_priv_status_out_: MaybeUninit<EmbeddedFontPrivStatus> = MaybeUninit::zeroed();
		let mut pul_status_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let (pus_char_code_set_ptr_, pus_char_code_set_len_) = pus_char_code_set.deconstruct();
		let _ret_ = TTEmbedFontFromFileA(dc, sz_font_file_name.to_null_terminated().as_ptr_or_null(), us_ttc_index, ul_flags, ul_char_set, pul_priv_status_out_.as_mut_ptr(), pul_status_out_.as_mut_ptr(), write_to_stream, lpv_write_stream as _, pus_char_code_set_ptr_, pus_char_code_set_len_ as u16, us_language, transmute(tt_embed_info));
		(_ret_, pul_priv_status_out_.assume_init(), pul_status_out_.assume_init())
	}
}

pub unsafe fn TTLoadEmbeddedFont() { todo!() }

pub fn TTGetEmbeddedFontInfo(ul_flags: TtEmbedFlags, ul_privs: FontLicensePRIVs, read_from_stream: ReadEmbedProc, lpv_read_stream: *const impl Sized, tt_load_info: Option<&TtLoadinfo>) -> (i32, u32, u32) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTGetEmbeddedFontInfo(ulFlags: TtEmbedFlags, pulPrivStatus: *mut u32, ulPrivs: FontLicensePRIVs, pulStatus: *mut u32, lpfnReadFromStream: ReadEmbedProc, lpvReadStream: *const c_void, pTTLoadInfo: *const c_void) -> i32;
		} 
		let mut pul_priv_status_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let mut pul_status_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let _ret_ = TTGetEmbeddedFontInfo(ul_flags, pul_priv_status_out_.as_mut_ptr(), ul_privs, pul_status_out_.as_mut_ptr(), read_from_stream, lpv_read_stream as _, transmute(tt_load_info));
		(_ret_, pul_priv_status_out_.assume_init(), pul_status_out_.assume_init())
	}
}

pub fn TTDeleteEmbeddedFont(font_reference: Handle, ul_flags: u32) -> (i32, u32) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTDeleteEmbeddedFont(hFontReference: Handle, ulFlags: u32, pulStatus: *mut u32) -> i32;
		} 
		let mut pul_status_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let _ret_ = TTDeleteEmbeddedFont(font_reference, ul_flags, pul_status_out_.as_mut_ptr());
		(_ret_, pul_status_out_.assume_init())
	}
}

pub fn TTGetEmbeddingType(dc: HDc) -> (i32, EmbeddedFontPrivStatus) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTGetEmbeddingType(hDC: HDc, pulEmbedType: *mut EmbeddedFontPrivStatus) -> i32;
		} 
		let mut pul_embed_type_out_: MaybeUninit<EmbeddedFontPrivStatus> = MaybeUninit::zeroed();
		let _ret_ = TTGetEmbeddingType(dc, pul_embed_type_out_.as_mut_ptr());
		(_ret_, pul_embed_type_out_.assume_init())
	}
}

pub unsafe fn TTCharToUnicode() { todo!() }

pub fn TTRunValidationTests(dc: HDc, test_param: &TtValidationTestsParams) -> i32 {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTRunValidationTests(hDC: HDc, pTestParam: &TtValidationTestsParams) -> i32;
		} 
		let _ret_ = TTRunValidationTests(dc, test_param);
		_ret_
	}
}

pub fn TTIsEmbeddingEnabled(dc: HDc) -> (i32, bool) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTIsEmbeddingEnabled(hDC: HDc, pbEnabled: &mut Bool) -> i32;
		} 
		let mut pb_enabled_out_ = Bool::FALSE;
		let _ret_ = TTIsEmbeddingEnabled(dc, &mut pb_enabled_out_);
		(_ret_, pb_enabled_out_.to_bool())
	}
}

pub fn TTIsEmbeddingEnabledForFacename(facename: &str) -> (i32, bool) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTIsEmbeddingEnabledForFacename(lpszFacename: *const u8, pbEnabled: &mut Bool) -> i32;
		} 
		let mut pb_enabled_out_ = Bool::FALSE;
		let _ret_ = TTIsEmbeddingEnabledForFacename(facename.to_null_terminated().as_ptr_or_null(), &mut pb_enabled_out_);
		(_ret_, pb_enabled_out_.to_bool())
	}
}

pub fn TTEnableEmbeddingForFacename(facename: &str, enable: bool) -> i32 {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTEnableEmbeddingForFacename(lpszFacename: *const u8, bEnable: Bool) -> i32;
		} 
		let _ret_ = TTEnableEmbeddingForFacename(facename.to_null_terminated().as_ptr_or_null(), enable.to_bool());
		_ret_
	}
}

pub fn TTEmbedFontEx(dc: HDc, ul_flags: TtEmbedFlags, ul_char_set: EmbedFontCharSet, write_to_stream: WriteEmbedProc, lpv_write_stream: *const impl Sized, pul_char_code_set: &[u32], us_language: u16, tt_embed_info: Option<&TtEmbedInfo>) -> (i32, EmbeddedFontPrivStatus, u32) {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTEmbedFontEx(hDC: HDc, ulFlags: TtEmbedFlags, ulCharSet: EmbedFontCharSet, pulPrivStatus: *mut EmbeddedFontPrivStatus, pulStatus: *mut u32, lpfnWriteToStream: WriteEmbedProc, lpvWriteStream: *const c_void, pulCharCodeSet: *const u32, usCharCodeCount: u16, usLanguage: u16, pTTEmbedInfo: *const c_void) -> i32;
		} 
		let mut pul_priv_status_out_: MaybeUninit<EmbeddedFontPrivStatus> = MaybeUninit::zeroed();
		let mut pul_status_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let (pul_char_code_set_ptr_, pul_char_code_set_len_) = pul_char_code_set.deconstruct();
		let _ret_ = TTEmbedFontEx(dc, ul_flags, ul_char_set, pul_priv_status_out_.as_mut_ptr(), pul_status_out_.as_mut_ptr(), write_to_stream, lpv_write_stream as _, pul_char_code_set_ptr_, pul_char_code_set_len_ as u16, us_language, transmute(tt_embed_info));
		(_ret_, pul_priv_status_out_.assume_init(), pul_status_out_.assume_init())
	}
}

pub fn TTRunValidationTestsEx(dc: HDc, test_param: &TtValidationTestsParamsEx) -> i32 {
	unsafe {
		#[link(name = "t2embed")]
		extern "system" {
			fn TTRunValidationTestsEx(hDC: HDc, pTestParam: &TtValidationTestsParamsEx) -> i32;
		} 
		let _ret_ = TTRunValidationTestsEx(dc, test_param);
		_ret_
	}
}

pub unsafe fn TTGetNewFontName() { todo!() }

pub fn DrawEdge(hdc: HDc, qrc: &mut Rect, edge: DrawedgeFlags, grf_flags: DrawEdgeFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawEdge(hdc: HDc, qrc: &mut Rect, edge: DrawedgeFlags, grfFlags: DrawEdgeFlags) -> Bool;
		} 
		let _ret_ = DrawEdge(hdc, qrc, edge, grf_flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DrawFrameControl(param0: HDc, param1: &mut Rect, param2: DfcType, param3: DfcsState) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawFrameControl(param0: HDc, param1: &mut Rect, param2: DfcType, param3: DfcsState) -> Bool;
		} 
		let _ret_ = DrawFrameControl(param0, param1, param2, param3);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DrawCaption(hwnd: HWnd, hdc: HDc, lprect: &Rect, flags: DrawCaptionFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawCaption(hwnd: HWnd, hdc: HDc, lprect: &Rect, flags: DrawCaptionFlags) -> Bool;
		} 
		let _ret_ = DrawCaption(hwnd, hdc, lprect, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DrawAnimatedRects(hwnd: Option<HWnd>, id_ani: i32, lprc_from: &Rect, lprc_to: &Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawAnimatedRects(hwnd: *const c_void, idAni: i32, lprcFrom: &Rect, lprcTo: &Rect) -> Bool;
		} 
		let _ret_ = DrawAnimatedRects(transmute(hwnd), id_ani, lprc_from, lprc_to);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn DrawTextA(hdc: HDc, lpch_text: &str, lprc: &mut Rect, format: DrawTextFormat) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawTextA(hdc: HDc, lpchText: *const u8, cchText: i32, lprc: &mut Rect, format: DrawTextFormat) -> i32;
		} 
		let (lpch_text_ptr_, lpch_text_len_) = lpch_text.deconstruct();
		let _ret_ = DrawTextA(hdc, lpch_text_ptr_, lpch_text_len_ as i32, lprc, format);
		_ret_
	}
}

pub fn DrawTextW(hdc: HDc, lpch_text: &str, lprc: &mut Rect, format: DrawTextFormat) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawTextW(hdc: HDc, lpchText: *const u16, cchText: i32, lprc: &mut Rect, format: DrawTextFormat) -> i32;
		} 
		let lpch_text_utf16_ = lpch_text.encode_utf16().collect::<Vec<_>>();
		let (lpch_text_ptr_, lpch_text_len_) = lpch_text_utf16_.deconstruct();
		let _ret_ = DrawTextW(hdc, lpch_text_ptr_, lpch_text_len_ as i32, lprc, format);
		_ret_
	}
}

pub unsafe fn DrawTextExA() { todo!() }

pub unsafe fn DrawTextExW() { todo!() }

pub fn GrayStringA(dc: HDc, brush: Option<HBrush>, output_func: Option<GrayStringProc>, data: LParam, count: i32, x: i32, y: i32, width: i32, height: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GrayStringA(hDC: HDc, hBrush: *const c_void, lpOutputFunc: *const c_void, lpData: LParam, nCount: i32, X: i32, Y: i32, nWidth: i32, nHeight: i32) -> Bool;
		} 
		let _ret_ = GrayStringA(dc, transmute(brush), transmute(output_func), data, count, x, y, width, height);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GrayStringW(dc: HDc, brush: Option<HBrush>, output_func: Option<GrayStringProc>, data: LParam, count: i32, x: i32, y: i32, width: i32, height: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GrayStringW(hDC: HDc, hBrush: *const c_void, lpOutputFunc: *const c_void, lpData: LParam, nCount: i32, X: i32, Y: i32, nWidth: i32, nHeight: i32) -> Bool;
		} 
		let _ret_ = GrayStringW(dc, transmute(brush), transmute(output_func), data, count, x, y, width, height);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn TabbedTextOutA(hdc: HDc, x: i32, y: i32, string: &str, lpn_tab_stop_positions: Option<&[i32]>, tab_origin: i32) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TabbedTextOutA(hdc: HDc, x: i32, y: i32, lpString: *const u8, chCount: i32, nTabPositions: i32, lpnTabStopPositions: *const i32, nTabOrigin: i32) -> i32;
		} 
		let (string_ptr_, string_len_) = string.deconstruct();
		let (lpn_tab_stop_positions_ptr_, lpn_tab_stop_positions_len_) = lpn_tab_stop_positions.deconstruct();
		let _ret_ = TabbedTextOutA(hdc, x, y, string_ptr_, string_len_ as i32, lpn_tab_stop_positions_len_ as i32, lpn_tab_stop_positions_ptr_, tab_origin);
		_ret_
	}
}

pub fn TabbedTextOutW(hdc: HDc, x: i32, y: i32, string: &str, lpn_tab_stop_positions: Option<&[i32]>, tab_origin: i32) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn TabbedTextOutW(hdc: HDc, x: i32, y: i32, lpString: *const u16, chCount: i32, nTabPositions: i32, lpnTabStopPositions: *const i32, nTabOrigin: i32) -> i32;
		} 
		let string_utf16_ = string.encode_utf16().collect::<Vec<_>>();
		let (string_ptr_, string_len_) = string_utf16_.deconstruct();
		let (lpn_tab_stop_positions_ptr_, lpn_tab_stop_positions_len_) = lpn_tab_stop_positions.deconstruct();
		let _ret_ = TabbedTextOutW(hdc, x, y, string_ptr_, string_len_ as i32, lpn_tab_stop_positions_len_ as i32, lpn_tab_stop_positions_ptr_, tab_origin);
		_ret_
	}
}

pub fn GetTabbedTextExtentA(hdc: HDc, string: &str, lpn_tab_stop_positions: Option<&[i32]>) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetTabbedTextExtentA(hdc: HDc, lpString: *const u8, chCount: i32, nTabPositions: i32, lpnTabStopPositions: *const i32) -> u32;
		} 
		let (string_ptr_, string_len_) = string.deconstruct();
		let (lpn_tab_stop_positions_ptr_, lpn_tab_stop_positions_len_) = lpn_tab_stop_positions.deconstruct();
		let _ret_ = GetTabbedTextExtentA(hdc, string_ptr_, string_len_ as i32, lpn_tab_stop_positions_len_ as i32, lpn_tab_stop_positions_ptr_);
		_ret_
	}
}

pub fn GetTabbedTextExtentW(hdc: HDc, string: &str, lpn_tab_stop_positions: Option<&[i32]>) -> u32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetTabbedTextExtentW(hdc: HDc, lpString: *const u16, chCount: i32, nTabPositions: i32, lpnTabStopPositions: *const i32) -> u32;
		} 
		let string_utf16_ = string.encode_utf16().collect::<Vec<_>>();
		let (string_ptr_, string_len_) = string_utf16_.deconstruct();
		let (lpn_tab_stop_positions_ptr_, lpn_tab_stop_positions_len_) = lpn_tab_stop_positions.deconstruct();
		let _ret_ = GetTabbedTextExtentW(hdc, string_ptr_, string_len_ as i32, lpn_tab_stop_positions_len_ as i32, lpn_tab_stop_positions_ptr_);
		_ret_
	}
}

pub fn UpdateWindow(wnd: HWnd) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UpdateWindow(hWnd: HWnd) -> Bool;
		} 
		let _ret_ = UpdateWindow(wnd);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn PaintDesktop(hdc: HDc) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PaintDesktop(hdc: HDc) -> Bool;
		} 
		let _ret_ = PaintDesktop(hdc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn WindowFromDC(dc: HDc) -> Result<HWnd, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn WindowFromDC(hDC: HDc) -> *const c_void;
		} 
		let _ret_ = WindowFromDC(dc);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetDC(wnd: Option<HWnd>) -> Result<HDc, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDC(hWnd: *const c_void) -> *const c_void;
		} 
		let _ret_ = GetDC(transmute(wnd));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetDCEx(wnd: Option<HWnd>, hrgn_clip: Option<HRgn>, flags: GetDcxFlags) -> Result<HDc, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetDCEx(hWnd: *const c_void, hrgnClip: *const c_void, flags: GetDcxFlags) -> *const c_void;
		} 
		let _ret_ = GetDCEx(transmute(wnd), transmute(hrgn_clip), flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetWindowDC(wnd: Option<HWnd>) -> Result<HDc, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowDC(hWnd: *const c_void) -> *const c_void;
		} 
		let _ret_ = GetWindowDC(transmute(wnd));
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn ReleaseDC(wnd: Option<HWnd>, dc: HDc) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ReleaseDC(hWnd: *const c_void, hDC: HDc) -> i32;
		} 
		let _ret_ = ReleaseDC(transmute(wnd), dc);
		_ret_
	}
}

pub fn GetUpdateRect(wnd: HWnd, rect: Option<&mut MaybeUninit<Rect>>, erase: bool) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetUpdateRect(hWnd: HWnd, lpRect: Option<&mut MaybeUninit<Rect>>, bErase: Bool) -> Bool;
		} 
		let _ret_ = GetUpdateRect(wnd, rect, erase.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn get_update_rect(wnd: HWnd, erase: bool) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetUpdateRect(hWnd: HWnd, lpRect: *mut Rect, bErase: Bool) -> Bool;
		} 
		let mut rect_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = GetUpdateRect(wnd, rect_out_.as_mut_ptr(), erase.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(rect_out_.assume_init()) }
	}
}

pub fn GetUpdateRgn(wnd: HWnd, rgn: HRgn, erase: bool) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetUpdateRgn(hWnd: HWnd, hRgn: HRgn, bErase: Bool) -> i32;
		} 
		let _ret_ = GetUpdateRgn(wnd, rgn, erase.to_bool());
		_ret_
	}
}

pub fn SetWindowRgn(wnd: HWnd, rgn: Option<HRgn>, redraw: bool) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetWindowRgn(hWnd: HWnd, hRgn: *const c_void, bRedraw: Bool) -> i32;
		} 
		let _ret_ = SetWindowRgn(wnd, transmute(rgn), redraw.to_bool());
		_ret_
	}
}

pub fn GetWindowRgn(wnd: HWnd, rgn: HRgn) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowRgn(hWnd: HWnd, hRgn: HRgn) -> i32;
		} 
		let _ret_ = GetWindowRgn(wnd, rgn);
		_ret_
	}
}

pub fn GetWindowRgnBox(wnd: HWnd) -> (i32, Rect) {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetWindowRgnBox(hWnd: HWnd, lprc: *mut Rect) -> i32;
		} 
		let mut lprc_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = GetWindowRgnBox(wnd, lprc_out_.as_mut_ptr());
		(_ret_, lprc_out_.assume_init())
	}
}

pub fn ExcludeUpdateRgn(dc: HDc, wnd: HWnd) -> i32 {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ExcludeUpdateRgn(hDC: HDc, hWnd: HWnd) -> i32;
		} 
		let _ret_ = ExcludeUpdateRgn(dc, wnd);
		_ret_
	}
}

pub fn InvalidateRect(wnd: Option<HWnd>, rect: Option<&Rect>, erase: bool) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InvalidateRect(hWnd: *const c_void, lpRect: *const c_void, bErase: Bool) -> Bool;
		} 
		let _ret_ = InvalidateRect(transmute(wnd), transmute(rect), erase.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ValidateRect(wnd: Option<HWnd>, rect: Option<&Rect>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ValidateRect(hWnd: *const c_void, lpRect: *const c_void) -> Bool;
		} 
		let _ret_ = ValidateRect(transmute(wnd), transmute(rect));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn InvalidateRgn(wnd: HWnd, rgn: Option<HRgn>, erase: bool) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InvalidateRgn(hWnd: HWnd, hRgn: *const c_void, bErase: Bool) -> Bool;
		} 
		let _ret_ = InvalidateRgn(wnd, transmute(rgn), erase.to_bool());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ValidateRgn(wnd: HWnd, rgn: Option<HRgn>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ValidateRgn(hWnd: HWnd, hRgn: *const c_void) -> Bool;
		} 
		let _ret_ = ValidateRgn(wnd, transmute(rgn));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn RedrawWindow(wnd: Option<HWnd>, lprc_update: Option<&Rect>, hrgn_update: Option<HRgn>, flags: RedrawWindowFlags) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn RedrawWindow(hWnd: *const c_void, lprcUpdate: *const c_void, hrgnUpdate: *const c_void, flags: RedrawWindowFlags) -> Bool;
		} 
		let _ret_ = RedrawWindow(transmute(wnd), transmute(lprc_update), transmute(hrgn_update), flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LockWindowUpdate(wnd_lock: Option<HWnd>) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LockWindowUpdate(hWndLock: *const c_void) -> Bool;
		} 
		let _ret_ = LockWindowUpdate(transmute(wnd_lock));
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ClientToScreen(wnd: HWnd, point: &mut Point) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ClientToScreen(hWnd: HWnd, lpPoint: &mut Point) -> Bool;
		} 
		let _ret_ = ClientToScreen(wnd, point);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn ScreenToClient(wnd: HWnd, point: &mut Point) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ScreenToClient(hWnd: HWnd, lpPoint: &mut Point) -> Bool;
		} 
		let _ret_ = ScreenToClient(wnd, point);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetSysColorBrush(index: super::super::super::ui::windows_and_messaging::SysColorIndex) -> Result<HBrush, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetSysColorBrush(nIndex: super::super::super::ui::windows_and_messaging::SysColorIndex) -> *const c_void;
		} 
		let _ret_ = GetSysColorBrush(index);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn DrawFocusRect(dc: HDc, lprc: &Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn DrawFocusRect(hDC: HDc, lprc: &Rect) -> Bool;
		} 
		let _ret_ = DrawFocusRect(dc, lprc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn FrameRect(dc: HDc, lprc: &Rect, hbr: HBrush) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn FrameRect(hDC: HDc, lprc: &Rect, hbr: HBrush) -> i32;
		} 
		let _ret_ = FrameRect(dc, lprc, hbr);
		match _ret_ { 0 => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn InvertRect(dc: HDc, lprc: &Rect) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InvertRect(hDC: HDc, lprc: &Rect) -> Bool;
		} 
		let _ret_ = InvertRect(dc, lprc);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn SetRect(x_left: i32, y_top: i32, x_right: i32, y_bottom: i32) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetRect(lprc: *mut Rect, xLeft: i32, yTop: i32, xRight: i32, yBottom: i32) -> Bool;
		} 
		let mut lprc_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = SetRect(lprc_out_.as_mut_ptr(), x_left, y_top, x_right, y_bottom);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_out_.assume_init()) }
	}
}

pub fn SetRectEmpty() -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetRectEmpty(lprc: *mut Rect) -> Bool;
		} 
		let mut lprc_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = SetRectEmpty(lprc_out_.as_mut_ptr());
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_out_.assume_init()) }
	}
}

pub fn CopyRect(lprc_src: &Rect) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn CopyRect(lprcDst: *mut Rect, lprcSrc: &Rect) -> Bool;
		} 
		let mut lprc_dst_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = CopyRect(lprc_dst_out_.as_mut_ptr(), lprc_src);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_dst_out_.assume_init()) }
	}
}

pub fn InflateRect(lprc: &mut Rect, dx: i32, dy: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn InflateRect(lprc: &mut Rect, dx: i32, dy: i32) -> Bool;
		} 
		let _ret_ = InflateRect(lprc, dx, dy);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn IntersectRect(lprc_src1: &Rect, lprc_src2: &Rect) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IntersectRect(lprcDst: *mut Rect, lprcSrc1: &Rect, lprcSrc2: &Rect) -> Bool;
		} 
		let mut lprc_dst_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = IntersectRect(lprc_dst_out_.as_mut_ptr(), lprc_src1, lprc_src2);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_dst_out_.assume_init()) }
	}
}

pub fn UnionRect(lprc_src1: &Rect, lprc_src2: &Rect) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn UnionRect(lprcDst: *mut Rect, lprcSrc1: &Rect, lprcSrc2: &Rect) -> Bool;
		} 
		let mut lprc_dst_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = UnionRect(lprc_dst_out_.as_mut_ptr(), lprc_src1, lprc_src2);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_dst_out_.assume_init()) }
	}
}

pub fn SubtractRect(lprc_src1: &Rect, lprc_src2: &Rect) -> Result<Rect, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SubtractRect(lprcDst: *mut Rect, lprcSrc1: &Rect, lprcSrc2: &Rect) -> Bool;
		} 
		let mut lprc_dst_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
		let _ret_ = SubtractRect(lprc_dst_out_.as_mut_ptr(), lprc_src1, lprc_src2);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(lprc_dst_out_.assume_init()) }
	}
}

pub fn OffsetRect(lprc: &mut Rect, dx: i32, dy: i32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn OffsetRect(lprc: &mut Rect, dx: i32, dy: i32) -> Bool;
		} 
		let _ret_ = OffsetRect(lprc, dx, dy);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn IsRectEmpty(lprc: &Rect) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn IsRectEmpty(lprc: &Rect) -> Bool;
		} 
		let _ret_ = IsRectEmpty(lprc);
		_ret_.to_bool()
	}
}

pub fn EqualRect(lprc1: &Rect, lprc2: &Rect) -> bool {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EqualRect(lprc1: &Rect, lprc2: &Rect) -> Bool;
		} 
		let _ret_ = EqualRect(lprc1, lprc2);
		_ret_.to_bool()
	}
}

pub fn PtInRect(lprc: &Rect, pt: Point) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn PtInRect(lprc: &Rect, pt: Point) -> Bool;
		} 
		let _ret_ = PtInRect(lprc, pt);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn LoadBitmapA(instance: Option<HInstance>, bitmap_name: &str) -> Result<HBitmap, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadBitmapA(hInstance: *const c_void, lpBitmapName: *const u8) -> *const c_void;
		} 
		let _ret_ = LoadBitmapA(transmute(instance), bitmap_name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn LoadBitmapW(instance: Option<HInstance>, bitmap_name: &str) -> Result<HBitmap, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn LoadBitmapW(hInstance: *const c_void, lpBitmapName: *const u16) -> *const c_void;
		} 
		let _ret_ = LoadBitmapW(transmute(instance), bitmap_name.to_utf16().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn ChangeDisplaySettingsA(dev_mode: Option<&DevModeA>, flags: CdsType) -> DispChange {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChangeDisplaySettingsA(lpDevMode: *const c_void, dwFlags: CdsType) -> DispChange;
		} 
		let _ret_ = ChangeDisplaySettingsA(transmute(dev_mode), flags);
		_ret_
	}
}

pub fn ChangeDisplaySettingsW(dev_mode: Option<&DevModeW>, flags: CdsType) -> DispChange {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChangeDisplaySettingsW(lpDevMode: *const c_void, dwFlags: CdsType) -> DispChange;
		} 
		let _ret_ = ChangeDisplaySettingsW(transmute(dev_mode), flags);
		_ret_
	}
}

pub fn ChangeDisplaySettingsExA(device_name: Option<&str>, dev_mode: Option<&DevModeA>, hwnd: HWnd, dwflags: CdsType, l_param: *const ()) -> DispChange {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChangeDisplaySettingsExA(lpszDeviceName: *const u8, lpDevMode: *const c_void, hwnd: HWnd, dwflags: CdsType, lParam: *const c_void) -> DispChange;
		} 
		let _ret_ = ChangeDisplaySettingsExA(device_name.map(str::to_null_terminated).as_ptr_or_null(), transmute(dev_mode), hwnd, dwflags, l_param as _);
		_ret_
	}
}

pub fn ChangeDisplaySettingsExW(device_name: Option<&str>, dev_mode: Option<&DevModeW>, hwnd: HWnd, dwflags: CdsType, l_param: *const ()) -> DispChange {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn ChangeDisplaySettingsExW(lpszDeviceName: *const u16, lpDevMode: *const c_void, hwnd: HWnd, dwflags: CdsType, lParam: *const c_void) -> DispChange;
		} 
		let _ret_ = ChangeDisplaySettingsExW(device_name.map(str::to_utf16).as_ptr_or_null(), transmute(dev_mode), hwnd, dwflags, l_param as _);
		_ret_
	}
}

pub fn EnumDisplaySettingsA(device_name: Option<&str>, i_mode_num: EnumDisplaySettingsMode, dev_mode: &mut DevModeA) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumDisplaySettingsA(lpszDeviceName: *const u8, iModeNum: EnumDisplaySettingsMode, lpDevMode: &mut DevModeA) -> Bool;
		} 
		let _ret_ = EnumDisplaySettingsA(device_name.map(str::to_null_terminated).as_ptr_or_null(), i_mode_num, dev_mode);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumDisplaySettingsW(device_name: Option<&str>, i_mode_num: EnumDisplaySettingsMode, dev_mode: &mut DevModeW) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumDisplaySettingsW(lpszDeviceName: *const u16, iModeNum: EnumDisplaySettingsMode, lpDevMode: &mut DevModeW) -> Bool;
		} 
		let _ret_ = EnumDisplaySettingsW(device_name.map(str::to_utf16).as_ptr_or_null(), i_mode_num, dev_mode);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumDisplaySettingsExA(device_name: Option<&str>, i_mode_num: EnumDisplaySettingsMode, dev_mode: &mut DevModeA, flags: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumDisplaySettingsExA(lpszDeviceName: *const u8, iModeNum: EnumDisplaySettingsMode, lpDevMode: &mut DevModeA, dwFlags: u32) -> Bool;
		} 
		let _ret_ = EnumDisplaySettingsExA(device_name.map(str::to_null_terminated).as_ptr_or_null(), i_mode_num, dev_mode, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumDisplaySettingsExW(device_name: Option<&str>, i_mode_num: EnumDisplaySettingsMode, dev_mode: &mut DevModeW, flags: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumDisplaySettingsExW(lpszDeviceName: *const u16, iModeNum: EnumDisplaySettingsMode, lpDevMode: &mut DevModeW, dwFlags: u32) -> Bool;
		} 
		let _ret_ = EnumDisplaySettingsExW(device_name.map(str::to_utf16).as_ptr_or_null(), i_mode_num, dev_mode, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumDisplayDevicesA(device: Option<&str>, i_dev_num: u32, display_device: &mut DisplayDeviceA, flags: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumDisplayDevicesA(lpDevice: *const u8, iDevNum: u32, lpDisplayDevice: &mut DisplayDeviceA, dwFlags: u32) -> Bool;
		} 
		let _ret_ = EnumDisplayDevicesA(device.map(str::to_null_terminated).as_ptr_or_null(), i_dev_num, display_device, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumDisplayDevicesW(device: Option<&str>, i_dev_num: u32, display_device: &mut DisplayDeviceW, flags: u32) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumDisplayDevicesW(lpDevice: *const u16, iDevNum: u32, lpDisplayDevice: &mut DisplayDeviceW, dwFlags: u32) -> Bool;
		} 
		let _ret_ = EnumDisplayDevicesW(device.map(str::to_utf16).as_ptr_or_null(), i_dev_num, display_device, flags);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn MonitorFromPoint(pt: Point, flags: MonitorFromFlags) -> Result<HMonitor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MonitorFromPoint(pt: Point, dwFlags: MonitorFromFlags) -> *const c_void;
		} 
		let _ret_ = MonitorFromPoint(pt, flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn MonitorFromRect(lprc: &Rect, flags: MonitorFromFlags) -> Result<HMonitor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MonitorFromRect(lprc: &Rect, dwFlags: MonitorFromFlags) -> *const c_void;
		} 
		let _ret_ = MonitorFromRect(lprc, flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn MonitorFromWindow(hwnd: HWnd, flags: MonitorFromFlags) -> Result<HMonitor, Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn MonitorFromWindow(hwnd: HWnd, dwFlags: MonitorFromFlags) -> *const c_void;
		} 
		let _ret_ = MonitorFromWindow(hwnd, flags);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn GetMonitorInfoA(monitor: HMonitor, lpmi: &mut MonitorInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMonitorInfoA(hMonitor: HMonitor, lpmi: &mut MonitorInfo) -> Bool;
		} 
		let _ret_ = GetMonitorInfoA(monitor, lpmi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn GetMonitorInfoW(monitor: HMonitor, lpmi: &mut MonitorInfo) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn GetMonitorInfoW(hMonitor: HMonitor, lpmi: &mut MonitorInfo) -> Bool;
		} 
		let _ret_ = GetMonitorInfoW(monitor, lpmi);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}

pub fn EnumDisplayMonitors(hdc: Option<HDc>, lprc_clip: Option<&Rect>, r#enum: MonitorEnumProc, data: LParam) -> Result<(), Win32Error> {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn EnumDisplayMonitors(hdc: *const c_void, lprcClip: *const c_void, lpfnEnum: MonitorEnumProc, dwData: LParam) -> Bool;
		} 
		let _ret_ = EnumDisplayMonitors(transmute(hdc), transmute(lprc_clip), r#enum, data);
		match _ret_.to_bool() { false => Err(GetLastError()), _ => Ok(()) }
	}
}


pub type FontEnumProcA 
	= unsafe extern "system" fn(param0: &LogFontA, param1: &TextMetricA, param2: u32, param3: LParam, ) -> i32;

pub type FontEnumProcW 
	= unsafe extern "system" fn(param0: &LogFontW, param1: &TextMetricW, param2: u32, param3: LParam, ) -> i32;

pub type GObjEnumProc 
	= unsafe extern "system" fn(param0: &mut (), param1: LParam, ) -> i32;

pub type LinEddaProc 
	= unsafe extern "system" fn(param0: i32, param1: i32, param2: LParam, ) -> ();

pub type MFEnumProc 
	= unsafe extern "system" fn(hdc: HDc, lpht: &HandleTable, mr: &MetaRecord, obj: i32, param4: LParam, ) -> i32;

pub type EnhMFEnumProc 
	= unsafe extern "system" fn(hdc: HDc, lpht: &HandleTable, lpmr: &EnhMetaRecord, handles: i32, data: LParam, ) -> i32;

pub type WriteEmbedProc 
	= unsafe extern "system" fn(param0: &mut (), param1: &(), param2: u32, ) -> u32;

pub type ReadEmbedProc 
	= unsafe extern "system" fn(param0: &mut (), param1: &mut (), param2: u32, ) -> u32;

pub type GrayStringProc 
	= unsafe extern "system" fn(param0: HDc, param1: LParam, param2: i32, ) -> Bool;

pub type MonitorEnumProc 
	= unsafe extern "system" fn(param0: HMonitor, param1: HDc, param2: &mut Rect, param3: LParam, ) -> Bool;

