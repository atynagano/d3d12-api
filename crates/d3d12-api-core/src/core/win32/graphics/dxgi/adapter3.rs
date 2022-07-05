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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
pub struct DxgiAdapter3(pub(crate) DxgiAdapter2);

impl Guid for DxgiAdapter3 {
	const IID: &'static GUID = &GUID::from_u128(0x645967a413924310a7988053ce3e93fdu128);
}

impl Clone for DxgiAdapter3 {
	fn clone(&self) -> Self { DxgiAdapter3(self.0.clone()) }
}

pub trait IDxgiAdapter3: IDxgiAdapter2 {
	fn as_adapter3(&self) -> &DxgiAdapter3;
	fn into_adapter3(self) -> DxgiAdapter3;

	fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, event: Handle, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pdw_cookie: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, event: Handle, _out_pdw_cookie: *mut u32, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, event, _out_pdw_cookie.as_mut_ptr(), );
			Ok(_out_pdw_cookie.assume_init())
		}
	}

	fn UnregisterHardwareContentProtectionTeardownStatus(&self, cookie: u32, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, cookie: u32, ) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, cookie, );
		}
	}

	fn QueryVideoMemoryInfo(&self, node_index: u32, memory_segment_group: DxgiMemorySegmentGroup, ) -> Result<DxgiQueryVideoMemoryInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_video_memory_info: MaybeUninit<DxgiQueryVideoMemoryInfo> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, node_index: u32, memory_segment_group: DxgiMemorySegmentGroup, _out_video_memory_info: *mut DxgiQueryVideoMemoryInfo, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, node_index, memory_segment_group, _out_video_memory_info.as_mut_ptr(), );
			Ok(_out_video_memory_info.assume_init())
		}
	}

	fn SetVideoMemoryReservation(&self, node_index: u32, memory_segment_group: DxgiMemorySegmentGroup, reservation: u64, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, node_index: u32, memory_segment_group: DxgiMemorySegmentGroup, reservation: u64, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, node_index, memory_segment_group, reservation, );
			_ret_.ok()
		}
	}

	fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, event: Handle, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pdw_cookie: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, event: Handle, _out_pdw_cookie: *mut u32, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, event, _out_pdw_cookie.as_mut_ptr(), );
			Ok(_out_pdw_cookie.assume_init())
		}
	}

	fn UnregisterVideoMemoryBudgetChangeNotification(&self, cookie: u32, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, cookie: u32, ) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, cookie, );
		}
	}
}

impl IDxgiAdapter3 for DxgiAdapter3 {
	fn as_adapter3(&self) -> &DxgiAdapter3 { self }
	fn into_adapter3(self) -> DxgiAdapter3 { self }
}

impl IDxgiAdapter2 for DxgiAdapter3 {
	fn as_adapter2(&self) -> &DxgiAdapter2 { &self.0 }
	fn into_adapter2(self) -> DxgiAdapter2 { self.0 }
}

impl IDxgiAdapter1 for DxgiAdapter3 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { &self.0.0 }
	fn into_adapter1(self) -> DxgiAdapter1 { self.0.0 }
}

impl IDxgiAdapter for DxgiAdapter3 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0.0.0 }
	fn into_adapter(self) -> DxgiAdapter { self.0.0.0 }
}

impl IDxgiObject for DxgiAdapter3 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0 }
}

impl From<Unknown> for DxgiAdapter3 {
    fn from(v: Unknown) -> Self {
        Self(DxgiAdapter2::from(v))
    }
}

impl IUnknown for DxgiAdapter3 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

