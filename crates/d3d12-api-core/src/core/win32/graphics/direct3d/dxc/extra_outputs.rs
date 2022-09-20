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
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcExtraOutputs(pub(crate) Unknown);

impl Deref for DxcExtraOutputs {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcExtraOutputs {
	const IID: &'static GUID = &GUID::from_u128(0x319b37a2a5c2494aa5de4801b2faf989u128);
}

impl Com for DxcExtraOutputs {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcExtraOutputs {
	pub fn GetOutputCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetOutput<T: IUnknown + From<UnknownWrapper>>(&self, u_index: u32, object: Option<&mut Option<T>>, mut output_type: Option<&mut Option<DxcBlobUtf16>>, mut output_name: Option<&mut Option<DxcBlobUtf16>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut object_out_: Option<Unknown> = None;
			if let Some(ref mut output_type) = output_type { **output_type = None; }
			if let Some(ref mut output_name) = output_name { **output_name = None; }
			let f: extern "system" fn(Param<Self>, u32, &GUID, *mut c_void, *mut c_void, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, u_index, T::IID, transmute(if object_out_.is_some() { Some(&mut object_out_) } else { None }), transmute(output_type), transmute(output_name));
			if let Some(object_out_) = object_out_ { *object.unwrap_unchecked() = Some(T::from(UnknownWrapper(object_out_))); }
			_ret_.ok()
		}
	}
}

pub trait IDxcExtraOutputs: IUnknown {
	fn as_extra_outputs(&self) -> &DxcExtraOutputs;
	fn into_extra_outputs(self) -> DxcExtraOutputs;
}

impl IDxcExtraOutputs for DxcExtraOutputs {
	fn as_extra_outputs(&self) -> &DxcExtraOutputs { self }
	fn into_extra_outputs(self) -> DxcExtraOutputs { self }
}
impl IUnknown for DxcExtraOutputs {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcExtraOutputs {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

