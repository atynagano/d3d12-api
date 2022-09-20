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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteFontFile(pub(crate) Unknown);

impl Deref for DWriteFontFile {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFontFile {
	const IID: &'static GUID = &GUID::from_u128(0x739d886acef547dc87691a8b41bebbb0u128);
}

impl Com for DWriteFontFile {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFontFile {
	pub unsafe fn GetReferenceKey(&self) { todo!() }

	pub fn GetLoader(&self) -> Result<DWriteFontFileLoader, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_file_loader_out_: Option<DWriteFontFileLoader> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(&mut font_file_loader_out_));
			if _ret_.is_ok() { if let Some(font_file_loader_out_) = font_file_loader_out_ { return Ok(font_file_loader_out_); } }
			Err(_ret_)
		}
	}

	pub fn Analyze(&self, font_face_type: Option<&mut MaybeUninit<DWriteFontFaceType>>) -> Result<(bool, DWriteFontFileType, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut is_supported_font_type_out_ = Bool::FALSE;
			let mut font_file_type_out_: MaybeUninit<DWriteFontFileType> = MaybeUninit::zeroed();
			let mut number_of_faces_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &mut Bool, *mut DWriteFontFileType, Option<&mut MaybeUninit<DWriteFontFaceType>>, *mut u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, &mut is_supported_font_type_out_, font_file_type_out_.as_mut_ptr(), font_face_type, number_of_faces_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((is_supported_font_type_out_.to_bool(), font_file_type_out_.assume_init(), number_of_faces_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn analyze(&self) -> Result<(bool, DWriteFontFileType, DWriteFontFaceType, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut is_supported_font_type_out_ = Bool::FALSE;
			let mut font_file_type_out_: MaybeUninit<DWriteFontFileType> = MaybeUninit::zeroed();
			let mut font_face_type_out_: MaybeUninit<DWriteFontFaceType> = MaybeUninit::zeroed();
			let mut number_of_faces_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &mut Bool, *mut DWriteFontFileType, *mut DWriteFontFaceType, *mut u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, &mut is_supported_font_type_out_, font_file_type_out_.as_mut_ptr(), font_face_type_out_.as_mut_ptr(), number_of_faces_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((is_supported_font_type_out_.to_bool(), font_file_type_out_.assume_init(), font_face_type_out_.assume_init(), number_of_faces_out_.assume_init())) } else { Err(_ret_) }
		}
	}
}

pub trait IDWriteFontFile: IUnknown {
	fn as_font_file(&self) -> &DWriteFontFile;
	fn into_font_file(self) -> DWriteFontFile;
}

impl IDWriteFontFile for DWriteFontFile {
	fn as_font_file(&self) -> &DWriteFontFile { self }
	fn into_font_file(self) -> DWriteFontFile { self }
}
impl IUnknown for DWriteFontFile {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFontFile {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

