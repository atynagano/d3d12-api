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
use crate::core::win32::graphics::direct3d12::*;
#[repr(C)]
pub struct D3D12ShaderCacheSession(pub(crate) D3D12DeviceChild);

impl Guid for D3D12ShaderCacheSession {
	const IID: &'static GUID = &GUID::from_u128(0x28e2495d0f644ae4a6ec129255dc49a8u128);
}

impl Clone for D3D12ShaderCacheSession {
	fn clone(&self) -> Self { D3D12ShaderCacheSession(self.0.clone()) }
}

pub trait ID3D12ShaderCacheSession: ID3D12DeviceChild {
	fn as_shader_cache_session(&self) -> &D3D12ShaderCacheSession;
	fn into_shader_cache_session(self) -> D3D12ShaderCacheSession;

	fn FindValue<'a>(&self, key: &[u8], mut value: &'a mut [u8], ) -> Result<(&'a mut [u8]), HResult> {
		let vt = self.as_param();
		let mut value_size = value.len() as u32;
		let f: extern "system" fn(Param<Self>, key: *const u8, key_size: u32, value: *mut u8, value_size: &mut u32, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, key.as_ptr_or_null(), key.len() as u32, value.as_mut_ptr_or_null(), &mut value_size, );
		if ret.is_ok() {
			return Ok((&mut value[..(value_size as usize)]));
		}
		Err(ret)
	}

	fn StoreValue(&self, key: &[u8], value: &[u8], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, key: *const u8, key_size: u32, value: *const u8, value_size: u32, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, key.as_ptr_or_null(), key.len() as u32, value.as_ptr_or_null(), value.len() as u32, );
		ret.ok()
	}

	fn SetDeleteOnDestroy(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, );
	}

	fn GetDesc(&self, ) -> (D3D12ShaderCacheSessionDesc) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> D3D12ShaderCacheSessionDesc
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3D12ShaderCacheSession for D3D12ShaderCacheSession {
	fn as_shader_cache_session(&self) -> &D3D12ShaderCacheSession { self }
	fn into_shader_cache_session(self) -> D3D12ShaderCacheSession { self }
}

impl ID3D12DeviceChild for D3D12ShaderCacheSession {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0 }
}

impl ID3D12Object for D3D12ShaderCacheSession {
	fn as_object(&self) -> &D3D12Object { &self.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0 }
}

impl From<Unknown> for D3D12ShaderCacheSession {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

impl IUnknown for D3D12ShaderCacheSession {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

