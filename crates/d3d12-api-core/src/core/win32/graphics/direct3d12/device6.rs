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
pub struct D3D12Device6(pub(crate) D3D12Device5);

impl Guid for D3D12Device6 {
	const IID: &'static GUID = &GUID::from_u128(0xc70b221b40e44a1789af025a0727a6dcu128);
}

impl Clone for D3D12Device6 {
	fn clone(&self) -> Self { D3D12Device6(self.0.clone()) }
}

pub trait ID3D12Device6: ID3D12Device5 {
	fn as_device6(&self) -> &D3D12Device6;
	fn into_device6(self) -> D3D12Device6;

	fn SetBackgroundProcessingMode(&self, mode: D3D12BackgroundProcessingMode, measurements_action: D3D12MeasurementsAction, event_to_signal_upon_completion: Option<Handle>, pb_further_measurements_desired: Option<&mut Bool>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, mode: D3D12BackgroundProcessingMode, measurements_action: D3D12MeasurementsAction, event_to_signal_upon_completion: Option<Handle>, pb_further_measurements_desired: Option<&mut Bool>, ) -> HResult
			= unsafe { transmute(vt[65]) };
		let ret = f(vt, mode, measurements_action, event_to_signal_upon_completion, pb_further_measurements_desired, );
		ret.ok()
	}
}

impl ID3D12Device6 for D3D12Device6 {
	fn as_device6(&self) -> &D3D12Device6 { self }
	fn into_device6(self) -> D3D12Device6 { self }
}

impl ID3D12Device5 for D3D12Device6 {
	fn as_device5(&self) -> &D3D12Device5 { &self.0 }
	fn into_device5(self) -> D3D12Device5 { self.0 }
}

impl ID3D12Device4 for D3D12Device6 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.0 }
	fn into_device4(self) -> D3D12Device4 { self.0.0 }
}

impl ID3D12Device3 for D3D12Device6 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.0.0 }
	fn into_device3(self) -> D3D12Device3 { self.0.0.0 }
}

impl ID3D12Device2 for D3D12Device6 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.0.0.0 }
	fn into_device2(self) -> D3D12Device2 { self.0.0.0.0 }
}

impl ID3D12Device1 for D3D12Device6 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.0.0.0.0 }
	fn into_device1(self) -> D3D12Device1 { self.0.0.0.0.0 }
}

impl ID3D12Device for D3D12Device6 {
	fn as_device(&self) -> &D3D12Device { &self.0.0.0.0.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0.0.0.0.0 }
}

impl ID3D12Object for D3D12Device6 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12Device6 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device5::from(v))
    }
}

impl IUnknown for D3D12Device6 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0 }
}

