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
pub struct D3D12CommandList(pub(crate) D3D12DeviceChild);

impl Guid for D3D12CommandList {
	const IID: &'static GUID = &GUID::from_u128(0x7116d91ce7e447ceb8c6ec8168f437e5u128);
}

impl Clone for D3D12CommandList {
	fn clone(&self) -> Self { D3D12CommandList(self.0.clone()) }
}

pub trait ID3D12CommandList: ID3D12DeviceChild {
	fn as_command_list(&self) -> &D3D12CommandList;
	fn into_command_list(self) -> D3D12CommandList;

	fn GetType(&self, ) -> D3D12CommandListType {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> D3D12CommandListType
				= transmute(vt[8]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl ID3D12CommandList for D3D12CommandList {
	fn as_command_list(&self) -> &D3D12CommandList { self }
	fn into_command_list(self) -> D3D12CommandList { self }
}

impl ID3D12DeviceChild for D3D12CommandList {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0 }
}

impl ID3D12Object for D3D12CommandList {
	fn as_object(&self) -> &D3D12Object { &self.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0 }
}

impl From<Unknown> for D3D12CommandList {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

impl IUnknown for D3D12CommandList {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

