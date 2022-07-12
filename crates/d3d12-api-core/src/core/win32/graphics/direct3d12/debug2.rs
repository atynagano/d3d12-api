#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
pub struct D3D12Debug2(pub(crate) Unknown);

impl Guid for D3D12Debug2 {
	const IID: &'static GUID = &GUID::from_u128(0x93a665c4a3b24e5db692a26ae14e3374u128);
}

impl Clone for D3D12Debug2 {
	fn clone(&self) -> Self { D3D12Debug2(self.0.clone()) }
}

pub trait ID3D12Debug2: IUnknown {
	fn as_debug2(&self) -> &D3D12Debug2;
	fn into_debug2(self) -> D3D12Debug2;

	fn SetGPUBasedValidationFlags(&self, flags: D3D12GpuBasedValidationFlags, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, flags: D3D12GpuBasedValidationFlags, ) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, flags, );
		}
	}
}

impl ID3D12Debug2 for D3D12Debug2 {
	fn as_debug2(&self) -> &D3D12Debug2 { self }
	fn into_debug2(self) -> D3D12Debug2 { self }
}

impl From<Unknown> for D3D12Debug2 {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12Debug2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

