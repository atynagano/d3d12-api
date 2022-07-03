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
pub struct D3D12InfoQueue(pub(crate) Unknown);

impl Guid for D3D12InfoQueue {
	const IID: &'static GUID = &GUID::from_u128(0x0742a90bc387483fb94630a7e4e61458u128);
}

impl Clone for D3D12InfoQueue {
	fn clone(&self) -> Self { D3D12InfoQueue(self.0.clone()) }
}

pub trait ID3D12InfoQueue: IUnknown {
	fn as_info_queue(&self) -> &D3D12InfoQueue;
	fn into_info_queue(self) -> D3D12InfoQueue;

	fn SetMessageCountLimit(&self, message_count_limit: u64, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, message_count_limit: u64, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, message_count_limit, );
		ret.ok()
	}

	fn ClearStoredMessages(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, );
	}

	fn GetNumMessagesAllowedByStorageFilter(&self, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u64
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetNumMessagesDeniedByStorageFilter(&self, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u64
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetNumStoredMessages(&self, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u64
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetNumStoredMessagesAllowedByRetrievalFilter(&self, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u64
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetNumMessagesDiscardedByMessageCountLimit(&self, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u64
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetMessageCountLimit(&self, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u64
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, );
		return (ret);
	}

	fn AddStorageFilterEntries(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, filter, );
		ret.ok()
	}

	fn ClearStorageFilter(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, );
	}

	fn PushEmptyStorageFilter(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn PushCopyOfStorageFilter(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn PushStorageFilter(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, filter, );
		ret.ok()
	}

	fn PopStorageFilter(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, );
	}

	fn GetStorageFilterStackSize(&self, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u32
			= unsafe { transmute(vt[19]) };
		let ret = f(vt, );
		return (ret);
	}

	fn AddRetrievalFilterEntries(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[20]) };
		let ret = f(vt, filter, );
		ret.ok()
	}

	fn ClearRetrievalFilter(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[22]) };
		let ret = f(vt, );
	}

	fn PushEmptyRetrievalFilter(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn PushCopyOfRetrievalFilter(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn PushRetrievalFilter(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
			= unsafe { transmute(vt[25]) };
		let ret = f(vt, filter, );
		ret.ok()
	}

	fn PopRetrievalFilter(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[26]) };
		let ret = f(vt, );
	}

	fn GetRetrievalFilterStackSize(&self, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u32
			= unsafe { transmute(vt[27]) };
		let ret = f(vt, );
		return (ret);
	}

	fn AddMessage(&self, category: D3D12MessageCategory, severity: D3D12MessageSeverity, id: D3D12MessageId, description: &str, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, category: D3D12MessageCategory, severity: D3D12MessageSeverity, id: D3D12MessageId, description: *const u8, ) -> HResult
			= unsafe { transmute(vt[28]) };
		let ret = f(vt, category, severity, id, description.to_null_terminated().as_ptr_or_null(), );
		ret.ok()
	}

	fn AddApplicationMessage(&self, severity: D3D12MessageSeverity, description: &str, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, severity: D3D12MessageSeverity, description: *const u8, ) -> HResult
			= unsafe { transmute(vt[29]) };
		let ret = f(vt, severity, description.to_null_terminated().as_ptr_or_null(), );
		ret.ok()
	}

	fn SetBreakOnCategory(&self, category: D3D12MessageCategory, enable: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, category: D3D12MessageCategory, enable: Bool, ) -> HResult
			= unsafe { transmute(vt[30]) };
		let ret = f(vt, category, enable.to_bool(), );
		ret.ok()
	}

	fn SetBreakOnSeverity(&self, severity: D3D12MessageSeverity, enable: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, severity: D3D12MessageSeverity, enable: Bool, ) -> HResult
			= unsafe { transmute(vt[31]) };
		let ret = f(vt, severity, enable.to_bool(), );
		ret.ok()
	}

	fn SetBreakOnID(&self, id: D3D12MessageId, enable: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, id: D3D12MessageId, enable: Bool, ) -> HResult
			= unsafe { transmute(vt[32]) };
		let ret = f(vt, id, enable.to_bool(), );
		ret.ok()
	}

	fn GetBreakOnCategory(&self, category: D3D12MessageCategory, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, category: D3D12MessageCategory, ) -> Bool
			= unsafe { transmute(vt[33]) };
		let ret = f(vt, category, );
		return (ret.to_bool());
	}

	fn GetBreakOnSeverity(&self, severity: D3D12MessageSeverity, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, severity: D3D12MessageSeverity, ) -> Bool
			= unsafe { transmute(vt[34]) };
		let ret = f(vt, severity, );
		return (ret.to_bool());
	}

	fn GetBreakOnID(&self, id: D3D12MessageId, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, id: D3D12MessageId, ) -> Bool
			= unsafe { transmute(vt[35]) };
		let ret = f(vt, id, );
		return (ret.to_bool());
	}

	fn SetMuteDebugOutput(&self, mute: bool, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, mute: Bool, ) -> ()
			= unsafe { transmute(vt[36]) };
		let ret = f(vt, mute.to_bool(), );
	}

	fn GetMuteDebugOutput(&self, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Bool
			= unsafe { transmute(vt[37]) };
		let ret = f(vt, );
		return (ret.to_bool());
	}
}

impl ID3D12InfoQueue for D3D12InfoQueue {
	fn as_info_queue(&self) -> &D3D12InfoQueue { self }
	fn into_info_queue(self) -> D3D12InfoQueue { self }
}

impl From<Unknown> for D3D12InfoQueue {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12InfoQueue {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

