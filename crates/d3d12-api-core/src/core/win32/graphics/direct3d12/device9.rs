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
pub struct D3D12Device9(pub(crate) D3D12Device8);

impl Deref for D3D12Device9 {
	type Target = D3D12Device8;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device9 {
	const IID: &'static GUID = &GUID::from_u128(0x4c80e962f0324f60bc9eebc2cfa1d83cu128);
}

impl Com for D3D12Device9 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device9 {
	pub fn CreateShaderCacheSession<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ShaderCacheSessionDesc, session: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut session_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ShaderCacheSessionDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[73]);
			let _ret_ = f(vt, desc, T::IID, transmute(if session_out_.is_some() { Some(&mut session_out_) } else { None }));
			if let Some(session_out_) = session_out_ { *session.unwrap_unchecked() = Some(T::from(UnknownWrapper(session_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_shader_cache_session<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ShaderCacheSessionDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut session_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ShaderCacheSessionDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[73]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut session_out_));
			if _ret_.is_ok() { if let Some(session_out_) = session_out_ { return Ok(T::from(UnknownWrapper(session_out_))); } }
			Err(_ret_)
		}
	}

	pub fn ShaderCacheControl(&self, kinds: D3D12ShaderCacheKindFlags, control: D3D12ShaderCacheControlFlags) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12ShaderCacheKindFlags, D3D12ShaderCacheControlFlags) -> HResult
				= transmute(vt[74]);
			let _ret_ = f(vt, kinds, control);
			_ret_.ok()
		}
	}

	pub fn CreateCommandQueue1<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12CommandQueueDesc, creator_id: &GUID) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_queue_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12CommandQueueDesc, &GUID, &GUID, *mut c_void) -> HResult
				= transmute(vt[75]);
			let _ret_ = f(vt, desc, creator_id, T::IID, transmute(&mut command_queue_out_));
			if _ret_.is_ok() { if let Some(command_queue_out_) = command_queue_out_ { return Ok(T::from(UnknownWrapper(command_queue_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait ID3D12Device9: ID3D12Device8 {
	fn as_device9(&self) -> &D3D12Device9;
	fn into_device9(self) -> D3D12Device9;
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

impl IUnknown for D3D12Device9 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device9 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Device8::from(v))
    }
}

