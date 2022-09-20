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
pub struct D3D12Device6(pub(crate) D3D12Device5);

impl Deref for D3D12Device6 {
	type Target = D3D12Device5;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device6 {
	const IID: &'static GUID = &GUID::from_u128(0xc70b221b40e44a1789af025a0727a6dcu128);
}

impl Com for D3D12Device6 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device6 {
	pub fn SetBackgroundProcessingMode(&self, mode: D3D12BackgroundProcessingMode, measurements_action: D3D12MeasurementsAction, event_to_signal_upon_completion: Option<Handle>, pb_further_measurements_desired: Option<&mut MaybeUninit<Bool>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12BackgroundProcessingMode, D3D12MeasurementsAction, *const c_void, Option<&mut MaybeUninit<Bool>>) -> HResult
				= transmute(vt[65]);
			let _ret_ = f(vt, mode, measurements_action, transmute(event_to_signal_upon_completion), pb_further_measurements_desired);
			_ret_.ok()
		}
	}

	pub fn set_background_processing_mode(&self, mode: D3D12BackgroundProcessingMode, measurements_action: D3D12MeasurementsAction, event_to_signal_upon_completion: Option<Handle>) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pb_further_measurements_desired_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, D3D12BackgroundProcessingMode, D3D12MeasurementsAction, *const c_void, &mut Bool) -> HResult
				= transmute(vt[65]);
			let _ret_ = f(vt, mode, measurements_action, transmute(event_to_signal_upon_completion), &mut pb_further_measurements_desired_out_);
			if _ret_.is_ok() { Ok(pb_further_measurements_desired_out_.to_bool()) } else { Err(_ret_) }
		}
	}
}

pub trait ID3D12Device6: ID3D12Device5 {
	fn as_device6(&self) -> &D3D12Device6;
	fn into_device6(self) -> D3D12Device6;
}

impl ID3D12Device6 for D3D12Device6 {
	fn as_device6(&self) -> &D3D12Device6 { self }
	fn into_device6(self) -> D3D12Device6 { self }
}
impl ID3D12Device5 for D3D12Device6 {
	fn as_device5(&self) -> &D3D12Device5 { &self.0.as_device5() }
	fn into_device5(self) -> D3D12Device5 { self.0.into_device5() }
}

impl ID3D12Device4 for D3D12Device6 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.as_device4() }
	fn into_device4(self) -> D3D12Device4 { self.0.into_device4() }
}

impl ID3D12Device3 for D3D12Device6 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.as_device3() }
	fn into_device3(self) -> D3D12Device3 { self.0.into_device3() }
}

impl ID3D12Device2 for D3D12Device6 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D3D12Device2 { self.0.into_device2() }
}

impl ID3D12Device1 for D3D12Device6 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D3D12Device1 { self.0.into_device1() }
}

impl ID3D12Device for D3D12Device6 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device6 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Device6 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device6 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Device5::from(v))
    }
}

