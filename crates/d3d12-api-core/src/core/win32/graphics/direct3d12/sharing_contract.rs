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

use crate::core::win32::graphics::direct3d12::*;
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12SharingContract(pub(crate) Unknown);

impl Deref for D3D12SharingContract {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12SharingContract {
	const IID: &'static GUID = &GUID::from_u128(0x0adf7d52929c4e61addbffed30de66efu128);
}

impl Com for D3D12SharingContract {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12SharingContract {
	pub fn Present(&self, resource: &D3D12Resource, subresource: u32, window: HWnd) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, HWnd) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, resource.vtable(), subresource, window);
		}
	}

	pub fn SharedFenceSignal(&self, fence: &D3D12Fence, fence_value: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u64) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, fence.vtable(), fence_value);
		}
	}

	pub fn BeginCapturableWork(&self, guid: &GUID) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &GUID) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, guid);
		}
	}

	pub fn EndCapturableWork(&self, guid: &GUID) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &GUID) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, guid);
		}
	}
}

pub trait ID3D12SharingContract: IUnknown {
	fn as_sharing_contract(&self) -> &D3D12SharingContract;
	fn into_sharing_contract(self) -> D3D12SharingContract;
}

impl ID3D12SharingContract for D3D12SharingContract {
	fn as_sharing_contract(&self) -> &D3D12SharingContract { self }
	fn into_sharing_contract(self) -> D3D12SharingContract { self }
}
impl IUnknown for D3D12SharingContract {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12SharingContract {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

