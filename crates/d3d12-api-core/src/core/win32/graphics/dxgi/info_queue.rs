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
pub struct DxgiInfoQueue(pub(crate) Unknown);

impl Deref for DxgiInfoQueue {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiInfoQueue {
	const IID: &'static GUID = &GUID::from_u128(0xd67441c7672a476f9e82cd55b44949ceu128);
}

impl Com for DxgiInfoQueue {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiInfoQueue {
	pub fn SetMessageCountLimit(&self, producer: GUID, message_count_limit: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, u64) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, producer, message_count_limit);
			_ret_.ok()
		}
	}

	pub fn ClearStoredMessages(&self, producer: GUID) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, producer);
		}
	}

	pub fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: GUID) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u64
				= transmute(vt[6]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn GetNumStoredMessages(&self, producer: GUID) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u64
				= transmute(vt[7]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: GUID) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u64
				= transmute(vt[8]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn GetMessageCountLimit(&self, producer: GUID) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u64
				= transmute(vt[9]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn GetNumMessagesAllowedByStorageFilter(&self, producer: GUID) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u64
				= transmute(vt[10]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn GetNumMessagesDeniedByStorageFilter(&self, producer: GUID) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u64
				= transmute(vt[11]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn AddStorageFilterEntries(&self, producer: GUID, filter: &DxgiInfoQueueFilter) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, &DxgiInfoQueueFilter) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, producer, filter);
			_ret_.ok()
		}
	}

	pub fn ClearStorageFilter(&self, producer: GUID) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> ()
				= transmute(vt[14]);
			let _ret_ = f(vt, producer);
		}
	}

	pub fn PushEmptyStorageFilter(&self, producer: GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, producer);
			_ret_.ok()
		}
	}

	pub fn PushDenyAllStorageFilter(&self, producer: GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, producer);
			_ret_.ok()
		}
	}

	pub fn PushCopyOfStorageFilter(&self, producer: GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, producer);
			_ret_.ok()
		}
	}

	pub fn PushStorageFilter(&self, producer: GUID, filter: &DxgiInfoQueueFilter) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, &DxgiInfoQueueFilter) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, producer, filter);
			_ret_.ok()
		}
	}

	pub fn PopStorageFilter(&self, producer: GUID) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> ()
				= transmute(vt[19]);
			let _ret_ = f(vt, producer);
		}
	}

	pub fn GetStorageFilterStackSize(&self, producer: GUID) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u32
				= transmute(vt[20]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn AddRetrievalFilterEntries(&self, producer: GUID, filter: &DxgiInfoQueueFilter) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, &DxgiInfoQueueFilter) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, producer, filter);
			_ret_.ok()
		}
	}

	pub fn ClearRetrievalFilter(&self, producer: GUID) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> ()
				= transmute(vt[23]);
			let _ret_ = f(vt, producer);
		}
	}

	pub fn PushEmptyRetrievalFilter(&self, producer: GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, producer);
			_ret_.ok()
		}
	}

	pub fn PushDenyAllRetrievalFilter(&self, producer: GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, producer);
			_ret_.ok()
		}
	}

	pub fn PushCopyOfRetrievalFilter(&self, producer: GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, producer);
			_ret_.ok()
		}
	}

	pub fn PushRetrievalFilter(&self, producer: GUID, filter: &DxgiInfoQueueFilter) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, &DxgiInfoQueueFilter) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, producer, filter);
			_ret_.ok()
		}
	}

	pub fn PopRetrievalFilter(&self, producer: GUID) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> ()
				= transmute(vt[28]);
			let _ret_ = f(vt, producer);
		}
	}

	pub fn GetRetrievalFilterStackSize(&self, producer: GUID) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> u32
				= transmute(vt[29]);
			let _ret_ = f(vt, producer);
			_ret_
		}
	}

	pub fn AddMessage(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, severity: DxgiInfoQueueMessageSeverity, id: i32, description: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, DxgiInfoQueueMessageCategory, DxgiInfoQueueMessageSeverity, i32, *const u8) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, producer, category, severity, id, description.to_null_terminated().as_ptr_or_null());
			_ret_.ok()
		}
	}

	pub fn AddApplicationMessage(&self, severity: DxgiInfoQueueMessageSeverity, description: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DxgiInfoQueueMessageSeverity, *const u8) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, severity, description.to_null_terminated().as_ptr_or_null());
			_ret_.ok()
		}
	}

	pub fn SetBreakOnCategory(&self, producer: GUID, category: DxgiInfoQueueMessageCategory, enable: bool) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, DxgiInfoQueueMessageCategory, Bool) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, producer, category, enable.to_bool());
			_ret_.ok()
		}
	}

	pub fn SetBreakOnSeverity(&self, producer: GUID, severity: DxgiInfoQueueMessageSeverity, enable: bool) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, DxgiInfoQueueMessageSeverity, Bool) -> HResult
				= transmute(vt[33]);
			let _ret_ = f(vt, producer, severity, enable.to_bool());
			_ret_.ok()
		}
	}

	pub fn SetBreakOnID(&self, producer: GUID, id: i32, enable: bool) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, i32, Bool) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, producer, id, enable.to_bool());
			_ret_.ok()
		}
	}

	pub fn GetBreakOnCategory(&self, producer: GUID, category: DxgiInfoQueueMessageCategory) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, DxgiInfoQueueMessageCategory) -> Bool
				= transmute(vt[35]);
			let _ret_ = f(vt, producer, category);
			_ret_.to_bool()
		}
	}

	pub fn GetBreakOnSeverity(&self, producer: GUID, severity: DxgiInfoQueueMessageSeverity) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, DxgiInfoQueueMessageSeverity) -> Bool
				= transmute(vt[36]);
			let _ret_ = f(vt, producer, severity);
			_ret_.to_bool()
		}
	}

	pub fn GetBreakOnID(&self, producer: GUID, id: i32) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, i32) -> Bool
				= transmute(vt[37]);
			let _ret_ = f(vt, producer, id);
			_ret_.to_bool()
		}
	}

	pub fn SetMuteDebugOutput(&self, producer: GUID, mute: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, Bool) -> ()
				= transmute(vt[38]);
			let _ret_ = f(vt, producer, mute.to_bool());
		}
	}

	pub fn GetMuteDebugOutput(&self, producer: GUID) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID) -> Bool
				= transmute(vt[39]);
			let _ret_ = f(vt, producer);
			_ret_.to_bool()
		}
	}
}

pub trait IDxgiInfoQueue: IUnknown {
	fn as_info_queue(&self) -> &DxgiInfoQueue;
	fn into_info_queue(self) -> DxgiInfoQueue;
}

impl IDxgiInfoQueue for DxgiInfoQueue {
	fn as_info_queue(&self) -> &DxgiInfoQueue { self }
	fn into_info_queue(self) -> DxgiInfoQueue { self }
}
impl IUnknown for DxgiInfoQueue {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiInfoQueue {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

