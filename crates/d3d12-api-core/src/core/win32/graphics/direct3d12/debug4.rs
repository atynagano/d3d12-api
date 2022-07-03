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

#[repr(C)]
pub struct D3D12Debug4(pub(crate) D3D12Debug3);

impl Guid for D3D12Debug4 {
	const IID: &'static GUID = &GUID::from_u128(0x014b816e9ec54a2fa845ffbe441ce13au128);
}

impl Clone for D3D12Debug4 {
	fn clone(&self) -> Self { D3D12Debug4(self.0.clone()) }
}

pub trait ID3D12Debug4: ID3D12Debug3 {
	fn as_debug4(&self) -> &D3D12Debug4;
	fn into_debug4(self) -> D3D12Debug4;

	fn DisableDebugLayer(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, );
	}
}

impl ID3D12Debug4 for D3D12Debug4 {
	fn as_debug4(&self) -> &D3D12Debug4 { self }
	fn into_debug4(self) -> D3D12Debug4 { self }
}

impl ID3D12Debug3 for D3D12Debug4 {
	fn as_debug3(&self) -> &D3D12Debug3 { &self.0 }
	fn into_debug3(self) -> D3D12Debug3 { self.0 }
}

impl ID3D12Debug for D3D12Debug4 {
	fn as_debug(&self) -> &D3D12Debug { &self.0.0 }
	fn into_debug(self) -> D3D12Debug { self.0.0 }
}

impl From<Unknown> for D3D12Debug4 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Debug3::from(v))
    }
}

impl IUnknown for D3D12Debug4 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

