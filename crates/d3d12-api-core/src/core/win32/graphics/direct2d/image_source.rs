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
pub struct D2D1ImageSource(pub(crate) D2D1Image);

impl Deref for D2D1ImageSource {
	type Target = D2D1Image;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ImageSource {
	const IID: &'static GUID = &GUID::from_u128(0xc9b664e574a143789ac2eefc37a3f4d8u128);
}

impl Com for D2D1ImageSource {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ImageSource {
	pub fn OfferResources(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn TryReclaimResources(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resources_discarded_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, &mut resources_discarded_out_);
			if _ret_.is_ok() { Ok(resources_discarded_out_.to_bool()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1ImageSource: ID2D1Image {
	fn as_image_source(&self) -> &D2D1ImageSource;
	fn into_image_source(self) -> D2D1ImageSource;
}

impl ID2D1ImageSource for D2D1ImageSource {
	fn as_image_source(&self) -> &D2D1ImageSource { self }
	fn into_image_source(self) -> D2D1ImageSource { self }
}
impl ID2D1Image for D2D1ImageSource {
	fn as_image(&self) -> &D2D1Image { &self.0.as_image() }
	fn into_image(self) -> D2D1Image { self.0.into_image() }
}

impl ID2D1Resource for D2D1ImageSource {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1ImageSource {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ImageSource {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Image::from(v))
    }
}

