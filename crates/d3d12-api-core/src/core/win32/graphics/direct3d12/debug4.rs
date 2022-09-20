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


#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Debug4(pub(crate) D3D12Debug3);

impl Deref for D3D12Debug4 {
	type Target = D3D12Debug3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Debug4 {
	const IID: &'static GUID = &GUID::from_u128(0x014b816e9ec54a2fa845ffbe441ce13au128);
}

impl Com for D3D12Debug4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Debug4 {
	pub fn DisableDebugLayer(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt);
		}
	}
}

pub trait ID3D12Debug4: ID3D12Debug3 {
	fn as_debug4(&self) -> &D3D12Debug4;
	fn into_debug4(self) -> D3D12Debug4;
}

impl ID3D12Debug4 for D3D12Debug4 {
	fn as_debug4(&self) -> &D3D12Debug4 { self }
	fn into_debug4(self) -> D3D12Debug4 { self }
}
impl ID3D12Debug3 for D3D12Debug4 {
	fn as_debug3(&self) -> &D3D12Debug3 { &self.0.as_debug3() }
	fn into_debug3(self) -> D3D12Debug3 { self.0.into_debug3() }
}

impl ID3D12Debug for D3D12Debug4 {
	fn as_debug(&self) -> &D3D12Debug { &self.0.as_debug() }
	fn into_debug(self) -> D3D12Debug { self.0.into_debug() }
}

impl IUnknown for D3D12Debug4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Debug4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Debug3::from(v))
    }
}

