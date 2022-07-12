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

	fn SetDeleteOnDestroy(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, );
		}
	}

	fn GetDesc(&self, ) -> D3D12ShaderCacheSessionDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out__out_desc: MaybeUninit<D3D12ShaderCacheSessionDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out__out_desc: *mut D3D12ShaderCacheSessionDesc, ) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, _out__out_desc.as_mut_ptr(), );
			_out__out_desc.assume_init()
		}
	}
}

impl ID3D12ShaderCacheSession for D3D12ShaderCacheSession {
	fn as_shader_cache_session(&self) -> &D3D12ShaderCacheSession { self }
	fn into_shader_cache_session(self) -> D3D12ShaderCacheSession { self }
}

impl ID3D12DeviceChild for D3D12ShaderCacheSession {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12ShaderCacheSession {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12ShaderCacheSession {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

impl IUnknown for D3D12ShaderCacheSession {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

