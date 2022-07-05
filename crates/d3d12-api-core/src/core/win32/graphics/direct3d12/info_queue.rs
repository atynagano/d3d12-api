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
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, message_count_limit: u64, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, message_count_limit, );
			_ret_.ok()
		}
	}

	fn ClearStoredMessages(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, );
		}
	}

	fn GetNumMessagesAllowedByStorageFilter(&self, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u64
				= transmute(vt[6]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetNumMessagesDeniedByStorageFilter(&self, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u64
				= transmute(vt[7]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetNumStoredMessages(&self, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u64
				= transmute(vt[8]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetNumStoredMessagesAllowedByRetrievalFilter(&self, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u64
				= transmute(vt[9]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetNumMessagesDiscardedByMessageCountLimit(&self, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u64
				= transmute(vt[10]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetMessageCountLimit(&self, ) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u64
				= transmute(vt[11]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn AddStorageFilterEntries(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, filter, );
			_ret_.ok()
		}
	}

	fn ClearStorageFilter(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[14]);
			let _ret_ = f(vt, );
		}
	}

	fn PushEmptyStorageFilter(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn PushCopyOfStorageFilter(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn PushStorageFilter(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, filter, );
			_ret_.ok()
		}
	}

	fn PopStorageFilter(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, );
		}
	}

	fn GetStorageFilterStackSize(&self, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u32
				= transmute(vt[19]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn AddRetrievalFilterEntries(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, filter, );
			_ret_.ok()
		}
	}

	fn ClearRetrievalFilter(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[22]);
			let _ret_ = f(vt, );
		}
	}

	fn PushEmptyRetrievalFilter(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn PushCopyOfRetrievalFilter(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn PushRetrievalFilter(&self, filter: &D3D12InfoQueueFilter, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, filter: &D3D12InfoQueueFilter, ) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, filter, );
			_ret_.ok()
		}
	}

	fn PopRetrievalFilter(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[26]);
			let _ret_ = f(vt, );
		}
	}

	fn GetRetrievalFilterStackSize(&self, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u32
				= transmute(vt[27]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn AddMessage(&self, category: D3D12MessageCategory, severity: D3D12MessageSeverity, id: D3D12MessageId, description: &str, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, category: D3D12MessageCategory, severity: D3D12MessageSeverity, id: D3D12MessageId, description: *const u8, ) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, category, severity, id, description.to_null_terminated().as_ptr_or_null(), );
			_ret_.ok()
		}
	}

	fn AddApplicationMessage(&self, severity: D3D12MessageSeverity, description: &str, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, severity: D3D12MessageSeverity, description: *const u8, ) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, severity, description.to_null_terminated().as_ptr_or_null(), );
			_ret_.ok()
		}
	}

	fn SetBreakOnCategory(&self, category: D3D12MessageCategory, enable: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, category: D3D12MessageCategory, enable: Bool, ) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, category, enable.to_bool(), );
			_ret_.ok()
		}
	}

	fn SetBreakOnSeverity(&self, severity: D3D12MessageSeverity, enable: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, severity: D3D12MessageSeverity, enable: Bool, ) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, severity, enable.to_bool(), );
			_ret_.ok()
		}
	}

	fn SetBreakOnID(&self, id: D3D12MessageId, enable: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, id: D3D12MessageId, enable: Bool, ) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, id, enable.to_bool(), );
			_ret_.ok()
		}
	}

	fn GetBreakOnCategory(&self, category: D3D12MessageCategory, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, category: D3D12MessageCategory, ) -> Bool
				= transmute(vt[33]);
			let _ret_ = f(vt, category, );
			_ret_.to_bool()
		}
	}

	fn GetBreakOnSeverity(&self, severity: D3D12MessageSeverity, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, severity: D3D12MessageSeverity, ) -> Bool
				= transmute(vt[34]);
			let _ret_ = f(vt, severity, );
			_ret_.to_bool()
		}
	}

	fn GetBreakOnID(&self, id: D3D12MessageId, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, id: D3D12MessageId, ) -> Bool
				= transmute(vt[35]);
			let _ret_ = f(vt, id, );
			_ret_.to_bool()
		}
	}

	fn SetMuteDebugOutput(&self, mute: bool, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, mute: Bool, ) -> ()
				= transmute(vt[36]);
			let _ret_ = f(vt, mute.to_bool(), );
		}
	}

	fn GetMuteDebugOutput(&self, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Bool
				= transmute(vt[37]);
			let _ret_ = f(vt, );
			_ret_.to_bool()
		}
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

