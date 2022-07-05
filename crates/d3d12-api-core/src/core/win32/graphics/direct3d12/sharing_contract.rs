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
use crate::core::win32::foundation::*;

#[repr(C)]
pub struct D3D12SharingContract(pub(crate) Unknown);

impl Guid for D3D12SharingContract {
	const IID: &'static GUID = &GUID::from_u128(0x0adf7d52929c4e61addbffed30de66efu128);
}

impl Clone for D3D12SharingContract {
	fn clone(&self) -> Self { D3D12SharingContract(self.0.clone()) }
}

pub trait ID3D12SharingContract: IUnknown {
	fn as_sharing_contract(&self) -> &D3D12SharingContract;
	fn into_sharing_contract(self) -> D3D12SharingContract;

	fn Present(&self, resource: &(impl ID3D12Resource + ?Sized), subresource: u32, window: HWnd, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, resource: VTable, subresource: u32, window: HWnd, ) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, resource.vtable(), subresource, window, );
		}
	}

	fn SharedFenceSignal(&self, fence: &(impl ID3D12Fence + ?Sized), fence_value: u64, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, fence: VTable, fence_value: u64, ) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, fence.vtable(), fence_value, );
		}
	}

	fn BeginCapturableWork(&self, guid: &GUID, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, guid: &GUID, ) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, guid, );
		}
	}

	fn EndCapturableWork(&self, guid: &GUID, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, guid: &GUID, ) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, guid, );
		}
	}
}

impl ID3D12SharingContract for D3D12SharingContract {
	fn as_sharing_contract(&self) -> &D3D12SharingContract { self }
	fn into_sharing_contract(self) -> D3D12SharingContract { self }
}

impl From<Unknown> for D3D12SharingContract {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12SharingContract {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

