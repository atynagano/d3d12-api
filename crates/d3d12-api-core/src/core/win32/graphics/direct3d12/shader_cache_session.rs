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
pub struct D3D12ShaderCacheSession(pub(crate) D3D12DeviceChild);

impl Deref for D3D12ShaderCacheSession {
	type Target = D3D12DeviceChild;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12ShaderCacheSession {
	const IID: &'static GUID = &GUID::from_u128(0x28e2495d0f644ae4a6ec129255dc49a8u128);
}

impl Com for D3D12ShaderCacheSession {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12ShaderCacheSession {
	pub unsafe fn FindValue(&self) { todo!() }

	pub fn StoreValue(&self, key: &[u8], value: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (key_ptr_, key_len_) = key.deconstruct();
			let (value_ptr_, value_len_) = value.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u8, u32, *const u8, u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, key_ptr_, key_len_ as u32, value_ptr_, value_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn SetDeleteOnDestroy(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt);
		}
	}

	pub fn GetDesc(&self) -> D3D12ShaderCacheSessionDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12ShaderCacheSessionDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12ShaderCacheSessionDesc) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, _out_.as_mut_ptr());
			_out_.assume_init()
		}
	}
}

pub trait ID3D12ShaderCacheSession: ID3D12DeviceChild {
	fn as_shader_cache_session(&self) -> &D3D12ShaderCacheSession;
	fn into_shader_cache_session(self) -> D3D12ShaderCacheSession;
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

impl IUnknown for D3D12ShaderCacheSession {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12ShaderCacheSession {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

