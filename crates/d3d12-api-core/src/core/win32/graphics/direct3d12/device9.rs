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
pub struct D3D12Device9(pub(crate) D3D12Device8);

impl Guid for D3D12Device9 {
	const IID: &'static GUID = &GUID::from_u128(0x4c80e962f0324f60bc9eebc2cfa1d83cu128);
}

impl Clone for D3D12Device9 {
	fn clone(&self) -> Self { D3D12Device9(self.0.clone()) }
}

pub trait ID3D12Device9: ID3D12Device8 {
	fn as_device9(&self) -> &D3D12Device9;
	fn into_device9(self) -> D3D12Device9;

	fn CreateShaderCacheSession<T: IUnknown>(&self, desc: &D3D12ShaderCacheSessionDesc, session: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_session: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ShaderCacheSessionDesc, riid: &GUID, session: *mut c_void, ) -> HResult
				= transmute(vt[73]);
			let _ret_ = f(vt, desc, T::IID, transmute(if session.is_some() { Some(&mut _out_session) } else { None }), );
			if let Some(_out_session) = _out_session { *session.unwrap_unchecked() = Some(T::from(_out_session)); }
			_ret_.ok()
		}
	}

	fn create_shader_cache_session<T: IUnknown>(&self, desc: &D3D12ShaderCacheSessionDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_session: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ShaderCacheSessionDesc, riid: &GUID, _out_session: *mut c_void, ) -> HResult
				= transmute(vt[73]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_session), );
			if _ret_.is_ok() {
				if let Some(_out_session) = _out_session {
					return Ok(T::from(_out_session));
				}
			}
			Err(_ret_)
		}
	}

	fn ShaderCacheControl(&self, kinds: D3D12ShaderCacheKindFlags, control: D3D12ShaderCacheControlFlags, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, kinds: D3D12ShaderCacheKindFlags, control: D3D12ShaderCacheControlFlags, ) -> HResult
				= transmute(vt[74]);
			let _ret_ = f(vt, kinds, control, );
			_ret_.ok()
		}
	}

	fn CreateCommandQueue1<T: IUnknown>(&self, desc: &D3D12CommandQueueDesc, creator_id: &GUID, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_command_queue: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12CommandQueueDesc, creator_id: &GUID, riid: &GUID, _out_command_queue: *mut c_void, ) -> HResult
				= transmute(vt[75]);
			let _ret_ = f(vt, desc, creator_id, T::IID, transmute(&mut _out_command_queue), );
			if _ret_.is_ok() {
				if let Some(_out_command_queue) = _out_command_queue {
					return Ok(T::from(_out_command_queue));
				}
			}
			Err(_ret_)
		}
	}
}

impl ID3D12Device9 for D3D12Device9 {
	fn as_device9(&self) -> &D3D12Device9 { self }
	fn into_device9(self) -> D3D12Device9 { self }
}

impl ID3D12Device8 for D3D12Device9 {
	fn as_device8(&self) -> &D3D12Device8 { &self.0.as_device8() }
	fn into_device8(self) -> D3D12Device8 { self.0.into_device8() }
}

impl ID3D12Device7 for D3D12Device9 {
	fn as_device7(&self) -> &D3D12Device7 { &self.0.as_device7() }
	fn into_device7(self) -> D3D12Device7 { self.0.into_device7() }
}

impl ID3D12Device6 for D3D12Device9 {
	fn as_device6(&self) -> &D3D12Device6 { &self.0.as_device6() }
	fn into_device6(self) -> D3D12Device6 { self.0.into_device6() }
}

impl ID3D12Device5 for D3D12Device9 {
	fn as_device5(&self) -> &D3D12Device5 { &self.0.as_device5() }
	fn into_device5(self) -> D3D12Device5 { self.0.into_device5() }
}

impl ID3D12Device4 for D3D12Device9 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.as_device4() }
	fn into_device4(self) -> D3D12Device4 { self.0.into_device4() }
}

impl ID3D12Device3 for D3D12Device9 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.as_device3() }
	fn into_device3(self) -> D3D12Device3 { self.0.into_device3() }
}

impl ID3D12Device2 for D3D12Device9 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D3D12Device2 { self.0.into_device2() }
}

impl ID3D12Device1 for D3D12Device9 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D3D12Device1 { self.0.into_device1() }
}

impl ID3D12Device for D3D12Device9 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device9 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12Device9 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device8::from(v))
    }
}

impl IUnknown for D3D12Device9 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

