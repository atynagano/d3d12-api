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
pub struct D3D12LifetimeTracker(pub(crate) D3D12DeviceChild);

impl Guid for D3D12LifetimeTracker {
	const IID: &'static GUID = &GUID::from_u128(0x3fd03d364eb1424aa582494ecb8ba813u128);
}

impl Clone for D3D12LifetimeTracker {
	fn clone(&self) -> Self { D3D12LifetimeTracker(self.0.clone()) }
}

pub trait ID3D12LifetimeTracker: ID3D12DeviceChild {
	fn as_lifetime_tracker(&self) -> &D3D12LifetimeTracker;
	fn into_lifetime_tracker(self) -> D3D12LifetimeTracker;

	fn DestroyOwnedObject(&self, object: &(impl ID3D12DeviceChild + ?Sized), ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, object: VTable, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, object.vtable(), );
		ret.ok()
	}
}

impl ID3D12LifetimeTracker for D3D12LifetimeTracker {
	fn as_lifetime_tracker(&self) -> &D3D12LifetimeTracker { self }
	fn into_lifetime_tracker(self) -> D3D12LifetimeTracker { self }
}

impl ID3D12DeviceChild for D3D12LifetimeTracker {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0 }
}

impl ID3D12Object for D3D12LifetimeTracker {
	fn as_object(&self) -> &D3D12Object { &self.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0 }
}

impl From<Unknown> for D3D12LifetimeTracker {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

impl IUnknown for D3D12LifetimeTracker {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

