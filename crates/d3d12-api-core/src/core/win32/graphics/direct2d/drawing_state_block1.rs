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
pub struct D2D1DrawingStateBlock1(pub(crate) D2D1DrawingStateBlock);

impl Deref for D2D1DrawingStateBlock1 {
	type Target = D2D1DrawingStateBlock;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DrawingStateBlock1 {
	const IID: &'static GUID = &GUID::from_u128(0x689f1f85c72e4e338f1985754efd5aceu128);
}

impl Com for D2D1DrawingStateBlock1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DrawingStateBlock1 {
	pub fn GetDescription(&self) -> D2D1DrawingStateDescription1 {
		unsafe {
			let vt = self.as_param();
			let mut state_description_out_: MaybeUninit<D2D1DrawingStateDescription1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1DrawingStateDescription1) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, state_description_out_.as_mut_ptr());
			state_description_out_.assume_init()
		}
	}

	pub fn SetDescription(&self, state_description: &D2D1DrawingStateDescription1) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1DrawingStateDescription1) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, state_description);
		}
	}
}

pub trait ID2D1DrawingStateBlock1: ID2D1DrawingStateBlock {
	fn as_drawing_state_block1(&self) -> &D2D1DrawingStateBlock1;
	fn into_drawing_state_block1(self) -> D2D1DrawingStateBlock1;
}

impl ID2D1DrawingStateBlock1 for D2D1DrawingStateBlock1 {
	fn as_drawing_state_block1(&self) -> &D2D1DrawingStateBlock1 { self }
	fn into_drawing_state_block1(self) -> D2D1DrawingStateBlock1 { self }
}
impl ID2D1DrawingStateBlock for D2D1DrawingStateBlock1 {
	fn as_drawing_state_block(&self) -> &D2D1DrawingStateBlock { &self.0.as_drawing_state_block() }
	fn into_drawing_state_block(self) -> D2D1DrawingStateBlock { self.0.into_drawing_state_block() }
}

impl ID2D1Resource for D2D1DrawingStateBlock1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DrawingStateBlock1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DrawingStateBlock1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1DrawingStateBlock::from(v))
    }
}

