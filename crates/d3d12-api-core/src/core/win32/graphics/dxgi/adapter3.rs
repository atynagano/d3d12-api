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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiAdapter3(pub(crate) DxgiAdapter2);

impl Deref for DxgiAdapter3 {
	type Target = DxgiAdapter2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiAdapter3 {
	const IID: &'static GUID = &GUID::from_u128(0x645967a413924310a7988053ce3e93fdu128);
}

impl Com for DxgiAdapter3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiAdapter3 {
	pub fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, event: Handle) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_cookie_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, Handle, *mut u32) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, event, pdw_cookie_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_cookie_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn UnregisterHardwareContentProtectionTeardownStatus(&self, cookie: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, cookie);
		}
	}

	pub fn QueryVideoMemoryInfo(&self, node_index: u32, memory_segment_group: DxgiMemorySegmentGroup) -> Result<DxgiQueryVideoMemoryInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut video_memory_info_out_: MaybeUninit<DxgiQueryVideoMemoryInfo> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, DxgiMemorySegmentGroup, *mut DxgiQueryVideoMemoryInfo) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, node_index, memory_segment_group, video_memory_info_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(video_memory_info_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn SetVideoMemoryReservation(&self, node_index: u32, memory_segment_group: DxgiMemorySegmentGroup, reservation: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, DxgiMemorySegmentGroup, u64) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, node_index, memory_segment_group, reservation);
			_ret_.ok()
		}
	}

	pub fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, event: Handle) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_cookie_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, Handle, *mut u32) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, event, pdw_cookie_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_cookie_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn UnregisterVideoMemoryBudgetChangeNotification(&self, cookie: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, cookie);
		}
	}
}

pub trait IDxgiAdapter3: IDxgiAdapter2 {
	fn as_adapter3(&self) -> &DxgiAdapter3;
	fn into_adapter3(self) -> DxgiAdapter3;
}

impl IDxgiAdapter3 for DxgiAdapter3 {
	fn as_adapter3(&self) -> &DxgiAdapter3 { self }
	fn into_adapter3(self) -> DxgiAdapter3 { self }
}
impl IDxgiAdapter2 for DxgiAdapter3 {
	fn as_adapter2(&self) -> &DxgiAdapter2 { &self.0.as_adapter2() }
	fn into_adapter2(self) -> DxgiAdapter2 { self.0.into_adapter2() }
}

impl IDxgiAdapter1 for DxgiAdapter3 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { &self.0.as_adapter1() }
	fn into_adapter1(self) -> DxgiAdapter1 { self.0.into_adapter1() }
}

impl IDxgiAdapter for DxgiAdapter3 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0.as_adapter() }
	fn into_adapter(self) -> DxgiAdapter { self.0.into_adapter() }
}

impl IDxgiObject for DxgiAdapter3 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiAdapter3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiAdapter3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiAdapter2::from(v))
    }
}

