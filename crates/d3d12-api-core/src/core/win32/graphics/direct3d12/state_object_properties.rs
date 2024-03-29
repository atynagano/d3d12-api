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
pub struct D3D12StateObjectProperties(pub(crate) Unknown);

impl Deref for D3D12StateObjectProperties {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12StateObjectProperties {
	const IID: &'static GUID = &GUID::from_u128(0xde5fa8279bf94f2689ffd7f56fde3860u128);
}

impl Com for D3D12StateObjectProperties {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12StateObjectProperties {
	pub fn GetShaderIdentifier(&self, export_name: &str) -> Option<&D3D12ShaderIdentifier> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> Option<&D3D12ShaderIdentifier>
				= transmute(vt[3]);
			let _ret_ = f(vt, export_name.to_utf16().as_ptr_or_null());
			_ret_
		}
	}

	pub fn GetShaderStackSize(&self, export_name: &str) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> u64
				= transmute(vt[4]);
			let _ret_ = f(vt, export_name.to_utf16().as_ptr_or_null());
			_ret_
		}
	}

	pub fn GetPipelineStackSize(&self) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u64
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetPipelineStackSize(&self, pipeline_stack_size_in_bytes: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, pipeline_stack_size_in_bytes);
		}
	}
}

pub trait ID3D12StateObjectProperties: IUnknown {
	fn as_state_object_properties(&self) -> &D3D12StateObjectProperties;
	fn into_state_object_properties(self) -> D3D12StateObjectProperties;
}

impl ID3D12StateObjectProperties for D3D12StateObjectProperties {
	fn as_state_object_properties(&self) -> &D3D12StateObjectProperties { self }
	fn into_state_object_properties(self) -> D3D12StateObjectProperties { self }
}
impl IUnknown for D3D12StateObjectProperties {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12StateObjectProperties {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

