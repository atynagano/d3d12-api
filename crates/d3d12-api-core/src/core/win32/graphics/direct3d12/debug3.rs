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
use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Debug3(pub(crate) D3D12Debug);

impl Deref for D3D12Debug3 {
	type Target = D3D12Debug;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Debug3 {
	const IID: &'static GUID = &GUID::from_u128(0x5cf4e58ff6714ff1a5423686e3d153d1u128);
}

impl Com for D3D12Debug3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Debug3 {
	pub fn SetEnableGPUBasedValidation(&self, enable: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, enable.to_bool());
		}
	}

	pub fn SetEnableSynchronizedCommandQueueValidation(&self, enable: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, enable.to_bool());
		}
	}

	pub fn SetGPUBasedValidationFlags(&self, flags: D3D12GpuBasedValidationFlags) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12GpuBasedValidationFlags) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, flags);
		}
	}
}

pub trait ID3D12Debug3: ID3D12Debug {
	fn as_debug3(&self) -> &D3D12Debug3;
	fn into_debug3(self) -> D3D12Debug3;
}

impl ID3D12Debug3 for D3D12Debug3 {
	fn as_debug3(&self) -> &D3D12Debug3 { self }
	fn into_debug3(self) -> D3D12Debug3 { self }
}
impl ID3D12Debug for D3D12Debug3 {
	fn as_debug(&self) -> &D3D12Debug { &self.0.as_debug() }
	fn into_debug(self) -> D3D12Debug { self.0.into_debug() }
}

impl IUnknown for D3D12Debug3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Debug3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Debug::from(v))
    }
}

