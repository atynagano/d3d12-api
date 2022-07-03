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
		let vt = self.as_param();
		let mut out_session: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12ShaderCacheSessionDesc, riid: &GUID, session: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[73]) };
		let ret = f(vt, desc, T::IID, if session.is_some() { Some(&mut out_session) } else { None }, );
		if let (Some(session), Some(out_session)) = (session, out_session) { *session = Some(T::from(out_session)); }
		ret.ok()
	}

	fn ShaderCacheControl(&self, kinds: D3D12ShaderCacheKindFlags, control: D3D12ShaderCacheControlFlags, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, kinds: D3D12ShaderCacheKindFlags, control: D3D12ShaderCacheControlFlags, ) -> HResult
			= unsafe { transmute(vt[74]) };
		let ret = f(vt, kinds, control, );
		ret.ok()
	}

	fn CreateCommandQueue1<T: IUnknown>(&self, desc: &D3D12CommandQueueDesc, creator_id: &GUID, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _command_queue: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12CommandQueueDesc, creator_id: &GUID, riid: &GUID, _command_queue: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[75]) };
		let ret = f(vt, desc, creator_id, T::IID, &mut _command_queue, );
		if ret.is_ok() {
			if let (Some(_command_queue)) = (_command_queue) {
				return Ok((T::from(_command_queue)));
			}
		}
		Err(ret)
	}
}

impl ID3D12Device9 for D3D12Device9 {
	fn as_device9(&self) -> &D3D12Device9 { self }
	fn into_device9(self) -> D3D12Device9 { self }
}

impl ID3D12Device8 for D3D12Device9 {
	fn as_device8(&self) -> &D3D12Device8 { &self.0 }
	fn into_device8(self) -> D3D12Device8 { self.0 }
}

impl ID3D12Device7 for D3D12Device9 {
	fn as_device7(&self) -> &D3D12Device7 { &self.0.0 }
	fn into_device7(self) -> D3D12Device7 { self.0.0 }
}

impl ID3D12Device6 for D3D12Device9 {
	fn as_device6(&self) -> &D3D12Device6 { &self.0.0.0 }
	fn into_device6(self) -> D3D12Device6 { self.0.0.0 }
}

impl ID3D12Device5 for D3D12Device9 {
	fn as_device5(&self) -> &D3D12Device5 { &self.0.0.0.0 }
	fn into_device5(self) -> D3D12Device5 { self.0.0.0.0 }
}

impl ID3D12Device4 for D3D12Device9 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.0.0.0.0 }
	fn into_device4(self) -> D3D12Device4 { self.0.0.0.0.0 }
}

impl ID3D12Device3 for D3D12Device9 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.0.0.0.0.0 }
	fn into_device3(self) -> D3D12Device3 { self.0.0.0.0.0.0 }
}

impl ID3D12Device2 for D3D12Device9 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.0.0.0.0.0.0 }
	fn into_device2(self) -> D3D12Device2 { self.0.0.0.0.0.0.0 }
}

impl ID3D12Device1 for D3D12Device9 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.0.0.0.0.0.0.0 }
	fn into_device1(self) -> D3D12Device1 { self.0.0.0.0.0.0.0.0 }
}

impl ID3D12Device for D3D12Device9 {
	fn as_device(&self) -> &D3D12Device { &self.0.0.0.0.0.0.0.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0.0.0.0.0.0.0.0 }
}

impl ID3D12Object for D3D12Device9 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12Device9 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device8::from(v))
    }
}

impl IUnknown for D3D12Device9 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0.0.0.0 }
}

