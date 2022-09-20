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
pub struct D3D12LifetimeTracker(pub(crate) D3D12DeviceChild);

impl Deref for D3D12LifetimeTracker {
	type Target = D3D12DeviceChild;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12LifetimeTracker {
	const IID: &'static GUID = &GUID::from_u128(0x3fd03d364eb1424aa582494ecb8ba813u128);
}

impl Com for D3D12LifetimeTracker {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12LifetimeTracker {
	pub fn DestroyOwnedObject(&self, object: &D3D12DeviceChild) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, object.vtable());
			_ret_.ok()
		}
	}
}

pub trait ID3D12LifetimeTracker: ID3D12DeviceChild {
	fn as_lifetime_tracker(&self) -> &D3D12LifetimeTracker;
	fn into_lifetime_tracker(self) -> D3D12LifetimeTracker;
}

impl ID3D12LifetimeTracker for D3D12LifetimeTracker {
	fn as_lifetime_tracker(&self) -> &D3D12LifetimeTracker { self }
	fn into_lifetime_tracker(self) -> D3D12LifetimeTracker { self }
}
impl ID3D12DeviceChild for D3D12LifetimeTracker {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12LifetimeTracker {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12LifetimeTracker {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12LifetimeTracker {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

