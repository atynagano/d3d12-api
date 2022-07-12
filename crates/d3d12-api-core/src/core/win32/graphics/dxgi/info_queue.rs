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
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, message_count_limit: u64, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, producer, message_count_limit, );
			_ret_.ok()
		}
	}

	fn ClearStoredMessages(&self, producer: GUID, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, producer, );
		}
	}

	fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: GUID, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
				= transmute(vt[6]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn GetNumStoredMessages(&self, producer: GUID, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
				= transmute(vt[7]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: GUID, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
				= transmute(vt[8]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn GetMessageCountLimit(&self, producer: GUID, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
				= transmute(vt[9]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn GetNumMessagesAllowedByStorageFilter(&self, producer: GUID, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
				= transmute(vt[10]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn GetNumMessagesDeniedByStorageFilter(&self, producer: GUID, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u64
				= transmute(vt[11]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn AddStorageFilterEntries(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, producer, filter, );
			_ret_.ok()
		}
	}

	fn ClearStorageFilter(&self, producer: GUID, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
				= transmute(vt[14]);
			let _ret_ = f(vt, producer, );
		}
	}

	fn PushEmptyStorageFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, producer, );
			_ret_.ok()
		}
	}

	fn PushDenyAllStorageFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, producer, );
			_ret_.ok()
		}
	}

	fn PushCopyOfStorageFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, producer, );
			_ret_.ok()
		}
	}

	fn PushStorageFilter(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, producer, filter, );
			_ret_.ok()
		}
	}

	fn PopStorageFilter(&self, producer: GUID, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
				= transmute(vt[19]);
			let _ret_ = f(vt, producer, );
		}
	}

	fn GetStorageFilterStackSize(&self, producer: GUID, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u32
				= transmute(vt[20]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn AddRetrievalFilterEntries(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, producer, filter, );
			_ret_.ok()
		}
	}

	fn ClearRetrievalFilter(&self, producer: GUID, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
				= transmute(vt[23]);
			let _ret_ = f(vt, producer, );
		}
	}

	fn PushEmptyRetrievalFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, producer, );
			_ret_.ok()
		}
	}

	fn PushDenyAllRetrievalFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, producer, );
			_ret_.ok()
		}
	}

	fn PushCopyOfRetrievalFilter(&self, producer: GUID, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, producer, );
			_ret_.ok()
		}
	}

	fn PushRetrievalFilter(&self, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, filter: &DxgiInfoQueueFilter, ) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, producer, filter, );
			_ret_.ok()
		}
	}

	fn PopRetrievalFilter(&self, producer: GUID, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> ()
				= transmute(vt[28]);
			let _ret_ = f(vt, producer, );
		}
	}

	fn GetRetrievalFilterStackSize(&self, producer: GUID, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> u32
				= transmute(vt[29]);
			let _ret_ = f(vt, producer, );
			_ret_
		}
	}

	fn AddMessage(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, severity: DxgiInfoQueueMessageSeverity, id: i32, description: &str, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, category: DxgiInfoQueueMessageCategory, severity: DxgiInfoQueueMessageSeverity, id: i32, description: *const u8, ) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, producer, category, severity, id, description.to_null_terminated().as_ptr_or_null(), );
			_ret_.ok()
		}
	}

	fn AddApplicationMessage(&self, severity: DxgiInfoQueueMessageSeverity, description: &str, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, severity: DxgiInfoQueueMessageSeverity, description: *const u8, ) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, severity, description.to_null_terminated().as_ptr_or_null(), );
			_ret_.ok()
		}
	}

	fn SetBreakOnCategory(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, enable: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, category: DxgiInfoQueueMessageCategory, enable: Bool, ) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, producer, category, enable.to_bool(), );
			_ret_.ok()
		}
	}

	fn SetBreakOnSeverity(&self, producer: GUID, severity: DxgiInfoQueueMessageSeverity, enable: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, severity: DxgiInfoQueueMessageSeverity, enable: Bool, ) -> HResult
				= transmute(vt[33]);
			let _ret_ = f(vt, producer, severity, enable.to_bool(), );
			_ret_.ok()
		}
	}

	fn SetBreakOnID(&self, producer: GUID, id: i32, enable: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, id: i32, enable: Bool, ) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, producer, id, enable.to_bool(), );
			_ret_.ok()
		}
	}

	fn GetBreakOnCategory(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, category: DxgiInfoQueueMessageCategory, ) -> Bool
				= transmute(vt[35]);
			let _ret_ = f(vt, producer, category, );
			_ret_.to_bool()
		}
	}

	fn GetBreakOnSeverity(&self, producer: GUID, severity: DxgiInfoQueueMessageSeverity, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, severity: DxgiInfoQueueMessageSeverity, ) -> Bool
				= transmute(vt[36]);
			let _ret_ = f(vt, producer, severity, );
			_ret_.to_bool()
		}
	}

	fn GetBreakOnID(&self, producer: GUID, id: i32, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, id: i32, ) -> Bool
				= transmute(vt[37]);
			let _ret_ = f(vt, producer, id, );
			_ret_.to_bool()
		}
	}

	fn SetMuteDebugOutput(&self, producer: GUID, mute: bool, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, mute: Bool, ) -> ()
				= transmute(vt[38]);
			let _ret_ = f(vt, producer, mute.to_bool(), );
		}
	}

	fn GetMuteDebugOutput(&self, producer: GUID, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, producer: GUID, ) -> Bool
				= transmute(vt[39]);
			let _ret_ = f(vt, producer, );
			_ret_.to_bool()
		}
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
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

