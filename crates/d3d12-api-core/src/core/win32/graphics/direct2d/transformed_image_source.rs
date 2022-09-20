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

use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1TransformedImageSource(pub(crate) D2D1Image);

impl Deref for D2D1TransformedImageSource {
	type Target = D2D1Image;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1TransformedImageSource {
	const IID: &'static GUID = &GUID::from_u128(0x7f1f79e52796416c8f55700f911445e5u128);
}

impl Com for D2D1TransformedImageSource {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1TransformedImageSource {
	pub fn GetSource(&self, mut image_source: Option<&mut Option<D2D1ImageSource>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut image_source) = image_source { **image_source = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(image_source));
		}
	}

	pub fn GetProperties(&self) -> D2D1TransformedImageSourceProperties {
		unsafe {
			let vt = self.as_param();
			let mut properties_out_: MaybeUninit<D2D1TransformedImageSourceProperties> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1TransformedImageSourceProperties) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, properties_out_.as_mut_ptr());
			properties_out_.assume_init()
		}
	}
}

pub trait ID2D1TransformedImageSource: ID2D1Image {
	fn as_transformed_image_source(&self) -> &D2D1TransformedImageSource;
	fn into_transformed_image_source(self) -> D2D1TransformedImageSource;
}

impl ID2D1TransformedImageSource for D2D1TransformedImageSource {
	fn as_transformed_image_source(&self) -> &D2D1TransformedImageSource { self }
	fn into_transformed_image_source(self) -> D2D1TransformedImageSource { self }
}
impl ID2D1Image for D2D1TransformedImageSource {
	fn as_image(&self) -> &D2D1Image { &self.0.as_image() }
	fn into_image(self) -> D2D1Image { self.0.into_image() }
}

impl ID2D1Resource for D2D1TransformedImageSource {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1TransformedImageSource {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1TransformedImageSource {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Image::from(v))
    }
}

