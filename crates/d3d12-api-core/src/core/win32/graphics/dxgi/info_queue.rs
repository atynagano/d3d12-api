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
use crate::core::win32::graphics::dxgi::*;
#[repr(C)]
pub struct DxgiInfoQueue(pub(crate) Unknown);

impl Guid for DxgiInfoQueue {
	const IID: &'static GUID = &GUID::from_u128(0xd67441c7672a476f9e82cd55b44949ceu128);
}

impl Clone for DxgiInfoQueue {
	fn clone(&self) -> Self { DxgiInfoQueue(self.0.clone()) }
}

pub trait IDxgiInfoQueue: IUnknown {
	fn as_info_queue(&self) -> &DxgiInfoQueue;
	fn into_info_queue(self) -> DxgiInfoQueue;

	fn SetMessageCountLimit(&self, producer: GUID, message_count_limit: u64, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, message_count_limit: u64, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, producer, message_count_limit, );
		ret.ok()
	}

	fn ClearStoredMessages(&self, producer: GUID, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, producer, );
	}

	fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: GUID, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn GetNumStoredMessages(&self, producer: GUID, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: GUID, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn GetMessageCountLimit(&self, producer: GUID, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn GetNumMessagesAllowedByStorageFilter(&self, producer: GUID, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn GetNumMessagesDeniedByStorageFilter(&self, producer: GUID, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn AddStorageFilterEntries(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, producer, filter, );
		ret.ok()
	}

	fn ClearStorageFilter(&self, producer: GUID, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, producer, );
	}

	fn PushEmptyStorageFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, producer, );
		ret.ok()
	}

	fn PushDenyAllStorageFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, producer, );
		ret.ok()
	}

	fn PushCopyOfStorageFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, producer, );
		ret.ok()
	}

	fn PushStorageFilter(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, producer, filter, );
		ret.ok()
	}

	fn PopStorageFilter(&self, producer: GUID, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
			= unsafe { transmute(vt[19]) };
		let ret = f(vt, producer, );
	}

	fn GetStorageFilterStackSize(&self, producer: GUID, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u32
			= unsafe { transmute(vt[20]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn AddRetrievalFilterEntries(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[21]) };
		let ret = f(vt, producer, filter, );
		ret.ok()
	}

	fn ClearRetrievalFilter(&self, producer: GUID, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, producer, );
	}

	fn PushEmptyRetrievalFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, producer, );
		ret.ok()
	}

	fn PushDenyAllRetrievalFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
			= unsafe { transmute(vt[25]) };
		let ret = f(vt, producer, );
		ret.ok()
	}

	fn PushCopyOfRetrievalFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
			= unsafe { transmute(vt[26]) };
		let ret = f(vt, producer, );
		ret.ok()
	}

	fn PushRetrievalFilter(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[27]) };
		let ret = f(vt, producer, filter, );
		ret.ok()
	}

	fn PopRetrievalFilter(&self, producer: GUID, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
			= unsafe { transmute(vt[28]) };
		let ret = f(vt, producer, );
	}

	fn GetRetrievalFilterStackSize(&self, producer: GUID, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u32
			= unsafe { transmute(vt[29]) };
		let ret = f(vt, producer, );
		return (ret);
	}

	fn AddMessage(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, severity: DxgiInfoQueueMessageSeverity, id: i32, description: &str, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, category: DxgiInfoQueueMessageCategory, severity: DxgiInfoQueueMessageSeverity, id: i32, description: *const u8, ) -> HResult
			= unsafe { transmute(vt[30]) };
		let ret = f(vt, producer, category, severity, id, description.to_null_terminated().as_ptr_or_null(), );
		ret.ok()
	}

	fn AddApplicationMessage(&self, severity: DxgiInfoQueueMessageSeverity, description: &str, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, severity: DxgiInfoQueueMessageSeverity, description: *const u8, ) -> HResult
			= unsafe { transmute(vt[31]) };
		let ret = f(vt, severity, description.to_null_terminated().as_ptr_or_null(), );
		ret.ok()
	}

	fn SetBreakOnCategory(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, enable: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, category: DxgiInfoQueueMessageCategory, enable: Bool, ) -> HResult
			= unsafe { transmute(vt[32]) };
		let ret = f(vt, producer, category, enable.to_bool(), );
		ret.ok()
	}

	fn SetBreakOnSeverity(&self, producer: GUID, severity: DxgiInfoQueueMessageSeverity, enable: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, severity: DxgiInfoQueueMessageSeverity, enable: Bool, ) -> HResult
			= unsafe { transmute(vt[33]) };
		let ret = f(vt, producer, severity, enable.to_bool(), );
		ret.ok()
	}

	fn SetBreakOnID(&self, producer: GUID, id: i32, enable: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, id: i32, enable: Bool, ) -> HResult
			= unsafe { transmute(vt[34]) };
		let ret = f(vt, producer, id, enable.to_bool(), );
		ret.ok()
	}

	fn GetBreakOnCategory(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, category: DxgiInfoQueueMessageCategory, ) -> Bool
			= unsafe { transmute(vt[35]) };
		let ret = f(vt, producer, category, );
		return (ret.to_bool());
	}

	fn GetBreakOnSeverity(&self, producer: GUID, severity: DxgiInfoQueueMessageSeverity, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, severity: DxgiInfoQueueMessageSeverity, ) -> Bool
			= unsafe { transmute(vt[36]) };
		let ret = f(vt, producer, severity, );
		return (ret.to_bool());
	}

	fn GetBreakOnID(&self, producer: GUID, id: i32, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, id: i32, ) -> Bool
			= unsafe { transmute(vt[37]) };
		let ret = f(vt, producer, id, );
		return (ret.to_bool());
	}

	fn SetMuteDebugOutput(&self, producer: GUID, mute: bool, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, mute: Bool, ) -> ()
			= unsafe { transmute(vt[38]) };
		let ret = f(vt, producer, mute.to_bool(), );
	}

	fn GetMuteDebugOutput(&self, producer: GUID, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, producer: GUID, ) -> Bool
			= unsafe { transmute(vt[39]) };
		let ret = f(vt, producer, );
		return (ret.to_bool());
	}
}

impl IDxgiInfoQueue for DxgiInfoQueue {
	fn as_info_queue(&self) -> &DxgiInfoQueue { self }
	fn into_info_queue(self) -> DxgiInfoQueue { self }
}

impl From<Unknown> for DxgiInfoQueue {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiInfoQueue {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

