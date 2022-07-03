#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct3d::dxc::*;
#[repr(C)]
pub struct DxcExtraOutputs(pub(crate) Unknown);

impl Guid for DxcExtraOutputs {
	const IID: &'static GUID = &GUID::from_u128(0x319b37a2a5c2494aa5de4801b2faf989u128);
}

impl Clone for DxcExtraOutputs {
	fn clone(&self) -> Self { DxcExtraOutputs(self.0.clone()) }
}

pub trait IDxcExtraOutputs: IUnknown {
	fn as_extra_outputs(&self) -> &DxcExtraOutputs;
	fn into_extra_outputs(self) -> DxcExtraOutputs;

	fn GetOutputCount(&self, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u32
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetOutput<T: IUnknown>(&self, index: u32, object: Option<&mut Option<T>>, mut output_type: Option<&mut Option<DxcBlobUtf16>>,mut output_name: Option<&mut Option<DxcBlobUtf16>>,) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_object: Option<Unknown> = None;
		if let Some(ref mut output_type) = output_type { **output_type = None; }
		if let Some(ref mut output_name) = output_name { **output_name = None; }
		let f: extern "system" fn(Param<Self>, index: u32, iid: &GUID, object: Option<&mut Option<Unknown>>, output_type: Option<&mut Option<DxcBlobUtf16>>, output_name: Option<&mut Option<DxcBlobUtf16>>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, index, T::IID, if object.is_some() { Some(&mut out_object) } else { None }, output_type, output_name, );
		if let (Some(object), Some(out_object)) = (object, out_object) { *object = Some(T::from(out_object)); }
		ret.ok()
	}
}

impl IDxcExtraOutputs for DxcExtraOutputs {
	fn as_extra_outputs(&self) -> &DxcExtraOutputs { self }
	fn into_extra_outputs(self) -> DxcExtraOutputs { self }
}

impl From<Unknown> for DxcExtraOutputs {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcExtraOutputs {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

