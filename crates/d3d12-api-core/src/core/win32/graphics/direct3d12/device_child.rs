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
#[repr(C)]
pub struct D3D12DeviceChild(pub(crate) D3D12Object);

impl Guid for D3D12DeviceChild {
	const IID: &'static GUID = &GUID::from_u128(0x905db94ba00c41409df52b64ca9ea357u128);
}

impl Clone for D3D12DeviceChild {
	fn clone(&self) -> Self { D3D12DeviceChild(self.0.clone()) }
}

pub trait ID3D12DeviceChild: ID3D12Object {
	fn as_device_child(&self) -> &D3D12DeviceChild;
	fn into_device_child(self) -> D3D12DeviceChild;

	fn GetDevice<T: IUnknown>(&self, device: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_device: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, riid: &GUID, device: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, T::IID, if device.is_some() { Some(&mut out_device) } else { None }, );
		if let (Some(device), Some(out_device)) = (device, out_device) { *device = Some(T::from(out_device)); }
		ret.ok()
	}
}

impl ID3D12DeviceChild for D3D12DeviceChild {
	fn as_device_child(&self) -> &D3D12DeviceChild { self }
	fn into_device_child(self) -> D3D12DeviceChild { self }
}

impl ID3D12Object for D3D12DeviceChild {
	fn as_object(&self) -> &D3D12Object { &self.0 }
	fn into_object(self) -> D3D12Object { self.0 }
}

impl From<Unknown> for D3D12DeviceChild {
    fn from(v: Unknown) -> Self {
        Self(D3D12Object::from(v))
    }
}

impl IUnknown for D3D12DeviceChild {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

