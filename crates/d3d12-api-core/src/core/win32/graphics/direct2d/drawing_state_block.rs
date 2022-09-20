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
use crate::core::win32::graphics::direct_write::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1DrawingStateBlock(pub(crate) D2D1Resource);

impl Deref for D2D1DrawingStateBlock {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DrawingStateBlock {
	const IID: &'static GUID = &GUID::from_u128(0x28506e39ebf646a1bb47fd85565ab957u128);
}

impl Com for D2D1DrawingStateBlock {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DrawingStateBlock {
	pub fn GetDescription(&self) -> D2D1DrawingStateDescription {
		unsafe {
			let vt = self.as_param();
			let mut state_description_out_: MaybeUninit<D2D1DrawingStateDescription> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1DrawingStateDescription) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, state_description_out_.as_mut_ptr());
			state_description_out_.assume_init()
		}
	}

	pub fn SetDescription(&self, state_description: &D2D1DrawingStateDescription) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1DrawingStateDescription) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, state_description);
		}
	}

	pub fn SetTextRenderingParams(&self, text_rendering_params: Option<&DWriteRenderingParams>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, option_to_vtable(text_rendering_params));
		}
	}

	pub fn GetTextRenderingParams(&self, mut text_rendering_params: Option<&mut Option<DWriteRenderingParams>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut text_rendering_params) = text_rendering_params { **text_rendering_params = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, transmute(text_rendering_params));
		}
	}
}

pub trait ID2D1DrawingStateBlock: ID2D1Resource {
	fn as_drawing_state_block(&self) -> &D2D1DrawingStateBlock;
	fn into_drawing_state_block(self) -> D2D1DrawingStateBlock;
}

impl ID2D1DrawingStateBlock for D2D1DrawingStateBlock {
	fn as_drawing_state_block(&self) -> &D2D1DrawingStateBlock { self }
	fn into_drawing_state_block(self) -> D2D1DrawingStateBlock { self }
}
impl ID2D1Resource for D2D1DrawingStateBlock {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DrawingStateBlock {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DrawingStateBlock {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

