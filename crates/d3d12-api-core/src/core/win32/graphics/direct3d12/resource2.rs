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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Resource2(pub(crate) D3D12Resource1);

impl Deref for D3D12Resource2 {
	type Target = D3D12Resource1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Resource2 {
	const IID: &'static GUID = &GUID::from_u128(0xbe36ec3bea854aeba45ae9d76404a495u128);
}

impl Com for D3D12Resource2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Resource2 {
	pub fn GetDesc1(&self) -> D3D12ResourceDesc1 {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12ResourceDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12ResourceDesc1) -> ()
				= transmute(vt[16]);
			let _ret_ = f(vt, _out_.as_mut_ptr());
			_out_.assume_init()
		}
	}
}

pub trait ID3D12Resource2: ID3D12Resource1 {
	fn as_resource2(&self) -> &D3D12Resource2;
	fn into_resource2(self) -> D3D12Resource2;
}

impl ID3D12Resource2 for D3D12Resource2 {
	fn as_resource2(&self) -> &D3D12Resource2 { self }
	fn into_resource2(self) -> D3D12Resource2 { self }
}
impl ID3D12Resource1 for D3D12Resource2 {
	fn as_resource1(&self) -> &D3D12Resource1 { &self.0.as_resource1() }
	fn into_resource1(self) -> D3D12Resource1 { self.0.into_resource1() }
}

impl ID3D12Resource for D3D12Resource2 {
	fn as_resource(&self) -> &D3D12Resource { &self.0.as_resource() }
	fn into_resource(self) -> D3D12Resource { self.0.into_resource() }
}

impl ID3D12Pageable for D3D12Resource2 {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12Resource2 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12Resource2 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Resource2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Resource2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Resource1::from(v))
    }
}

