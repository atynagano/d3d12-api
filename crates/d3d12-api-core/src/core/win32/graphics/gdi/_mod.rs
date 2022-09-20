use std::ffi::c_void;
use std::mem::{MaybeUninit, transmute};
use std::ptr::{null, null_mut};
use std::slice;
use crate::core::win32::foundation::*;
use crate::helpers::{Deconstruct, low_high_word, NullTerminated};
use super::*;

pub fn MapWindowPoints(wnd_from: Option<HWnd>, wnd_to: Option<HWnd>, points: &mut [Point]) -> Result<(i32, i32), Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn MapWindowPoints(wnd_from: *const c_void, wnd_to: *const c_void, points: *mut Point, len: u32) -> i32;
        }
        let (_ptr_points, _len_points) = points.deconstruct();

        SetLastError(Win32Error::NoError);
        let _ret_ = MapWindowPoints(transmute(wnd_from), transmute(wnd_to), _ptr_points, _len_points as u32);
        if _ret_ == 0 {
            let err = GetLastError();
            if err != Win32Error::NoError {
                return Err(err);
            }
        }
        // todo: 負にならないか確認
        let (x, y) = low_high_word(_ret_ as u32);
        Ok((x as i16 as i32, y as i16 as i32))
    }
}


pub fn MapWindowRects(wnd_from: Option<HWnd>, wnd_to: Option<HWnd>, points: &mut [Rect]) -> Result<(i32, i32), Win32Error> {
    let points = unsafe { slice::from_raw_parts_mut(points.as_mut_ptr() as *mut Point, points.len() * 2) };
    MapWindowPoints(wnd_from, wnd_to, points)
}

// todo: 現状、Optionを返して.ok_or_last_err()呼び出しをする奴とResultを返すやつが混在していてよろしくない。後者に統一するのがベター。
pub fn BeginPaint(wnd: HWnd) -> Result<(HDc, PaintStruct), Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn BeginPaint(wnd: HWnd, _out_paint: *mut PaintStruct) -> *const c_void;
        }
        let mut _out_paint: MaybeUninit<PaintStruct> = MaybeUninit::zeroed();
        let _ret_ = BeginPaint(wnd, _out_paint.as_mut_ptr());
        if _ret_ == null() {
            Err(GetLastError())
        } else {
            Ok((transmute(_ret_), _out_paint.assume_init(), ))
        }
    }
}

pub fn EndPaint(wnd: HWnd, paint: &PaintStruct) -> () {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn EndPaint(wnd: HWnd, paint: &PaintStruct) -> Bool;
        }
        // note: The return value is always nonzero.
        EndPaint(wnd, paint);
    }
}

pub fn FillRect(dc: HDc, lprc: &Rect, hbr: HBrush) -> Result<(), Win32Error> {
    unsafe {
        #[link(name = "USER32")]
        extern "system" {
            fn FillRect(dc: HDc, lprc: &Rect, hbr: HBrush) -> i32;
        }
        let _ret_ = FillRect(dc, lprc, hbr);
        match _ret_ { 0 => Err(GetLastError()), _ => Ok(()) }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ColorRef(pub u32);

impl ColorRef {
    pub const fn new(r: u8, g: u8, b: u8) -> Self { Self(r as u32 | (g as u32) << 8 | (b as u32) << 16) }
    pub const fn r(self) -> u8 { self.0 as u8 }
    pub const fn g(self) -> u8 { (self.0 >> 8) as u8 }
    pub const fn b(self) -> u8 { (self.0 >> 16) as u8 }
    pub const BLACK: Self = Self(0);
    pub const WHITE: Self = Self(0x00FFFFFF);
}

pub fn GetEnhMetaFileBits(hemf: HEnhMetaFile) -> Result<NullTerminated<'static>, Box<dyn std::fmt::Debug>> {
    unsafe {
        #[link(name = "GDI32")]
        extern "system" {
            fn GetEnhMetaFileBits(hemf: HEnhMetaFile, size: u32, data: *mut u8) -> u32;
        }
        let _len1_ = GetEnhMetaFileBits(hemf, 0, null_mut());
        if _len1_ == 0 { return Err(Box::new(GetLastError())); }
        let mut _buf_ = Vec::with_capacity(_len1_ as usize);
        _buf_.set_len(_len1_ as usize);
        let _len2_ = GetEnhMetaFileBits(hemf, _len1_, _buf_.as_mut_ptr());
        if _len2_ != _len1_ { return Err(Box::new(GetLastError())); }
        match String::from_utf8(_buf_) {
            Ok(s) => Ok(NullTerminated(std::borrow::Cow::Owned(s))),
            Err(e) => Err(Box::new(e)),
        }
    }
}