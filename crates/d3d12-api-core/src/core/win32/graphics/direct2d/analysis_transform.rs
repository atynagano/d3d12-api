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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1AnalysisTransform(pub(crate) Unknown);

impl Deref for D2D1AnalysisTransform {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1AnalysisTransform {
	const IID: &'static GUID = &GUID::from_u128(0x0359dc3095e64568905527720d130e93u128);
}

impl Com for D2D1AnalysisTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1AnalysisTransform {
	pub fn ProcessAnalysisResults(&self, analysis_data: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (analysis_data_ptr_, analysis_data_len_) = analysis_data.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u8, u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, analysis_data_ptr_, analysis_data_len_ as u32);
			_ret_.ok()
		}
	}
}

pub trait ID2D1AnalysisTransform: IUnknown {
	fn as_analysis_transform(&self) -> &D2D1AnalysisTransform;
	fn into_analysis_transform(self) -> D2D1AnalysisTransform;
}

impl ID2D1AnalysisTransform for D2D1AnalysisTransform {
	fn as_analysis_transform(&self) -> &D2D1AnalysisTransform { self }
	fn into_analysis_transform(self) -> D2D1AnalysisTransform { self }
}
impl IUnknown for D2D1AnalysisTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1AnalysisTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

