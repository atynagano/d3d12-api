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
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct Stream(pub(crate) SequentialStream);

impl Guid for Stream {
	const IID: &'static GUID = &GUID::from_u128(0x0000000c00000000c000000000000046u128);
}

impl Clone for Stream {
	fn clone(&self) -> Self { Stream(self.0.clone()) }
}

pub trait IStream: ISequentialStream {
	fn as_stream(&self) -> &Stream;
	fn into_stream(self) -> Stream;

	fn Seek(&self, dlib_move: i64, origin: StreamSeek, plib_new_position: Option<&mut u64>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, dlib_move: i64, origin: StreamSeek, plib_new_position: Option<&mut u64>, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, dlib_move, origin, plib_new_position, );
			_ret_.ok()
		}
	}

	fn seek(&self, dlib_move: i64, origin: StreamSeek, ) -> Result<u64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_plib_new_position: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, dlib_move: i64, origin: StreamSeek, _out_plib_new_position: *mut u64, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, dlib_move, origin, _out_plib_new_position.as_mut_ptr(), );
			Ok(_out_plib_new_position.assume_init())
		}
	}

	fn SetSize(&self, lib_new_size: u64, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, lib_new_size: u64, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, lib_new_size, );
			_ret_.ok()
		}
	}

	fn CopyTo(&self, pstm: &(impl IStream + ?Sized), cb: u64, pcb_read: Option<&mut u64>, pcb_written: Option<&mut u64>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, pstm: VTable, cb: u64, pcb_read: Option<&mut u64>, pcb_written: Option<&mut u64>, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, pstm.vtable(), cb, pcb_read, pcb_written, );
			_ret_.ok()
		}
	}

	fn Commit(&self, grf_commit_flags: StgC, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, grf_commit_flags: StgC, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, grf_commit_flags, );
			_ret_.ok()
		}
	}

	fn Revert(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn LockRegion(&self, lib_offset: u64, cb: u64, lock_type: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, lib_offset: u64, cb: u64, lock_type: u32, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, lib_offset, cb, lock_type, );
			_ret_.ok()
		}
	}

	fn UnlockRegion(&self, lib_offset: u64, cb: u64, lock_type: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, lib_offset: u64, cb: u64, lock_type: u32, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, lib_offset, cb, lock_type, );
			_ret_.ok()
		}
	}

	fn Stat(&self, grf_stat_flag: u32, ) -> Result<StatStg, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pstatstg: MaybeUninit<StatStg> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_pstatstg: *mut StatStg, grf_stat_flag: u32, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, _out_pstatstg.as_mut_ptr(), grf_stat_flag, );
			Ok(_out_pstatstg.assume_init())
		}
	}

	fn Clone(&self, ) -> Result<Stream, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_ppstm: Option<Stream> = None;
			let f: extern "system" fn(Param<Self>, ppstm: *mut c_void, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(&mut _out_ppstm), );
			if _ret_.is_ok() {
				if let Some(_out_ppstm) = _out_ppstm {
					return Ok(_out_ppstm);
				}
			}
			Err(_ret_)
		}
	}
}

impl IStream for Stream {
	fn as_stream(&self) -> &Stream { self }
	fn into_stream(self) -> Stream { self }
}

impl ISequentialStream for Stream {
	fn as_sequential_stream(&self) -> &SequentialStream { &self.0.as_sequential_stream() }
	fn into_sequential_stream(self) -> SequentialStream { self.0.into_sequential_stream() }
}

impl From<Unknown> for Stream {
    fn from(v: Unknown) -> Self {
        Self(SequentialStream::from(v))
    }
}

impl IUnknown for Stream {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

