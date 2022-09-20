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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct Stream(pub(crate) SequentialStream);

impl Deref for Stream {
	type Target = SequentialStream;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for Stream {
	const IID: &'static GUID = &GUID::from_u128(0x0000000c00000000c000000000000046u128);
}

impl Com for Stream {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl Stream {
	pub fn Seek(&self, dlib_move: i64, origin: StreamSeek, plib_new_position: Option<&mut MaybeUninit<u64>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, i64, StreamSeek, Option<&mut MaybeUninit<u64>>) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, dlib_move, origin, plib_new_position);
			_ret_.ok()
		}
	}

	pub fn seek(&self, dlib_move: i64, origin: StreamSeek) -> Result<u64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut plib_new_position_out_: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, i64, StreamSeek, *mut u64) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, dlib_move, origin, plib_new_position_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(plib_new_position_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn SetSize(&self, lib_new_size: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, lib_new_size);
			_ret_.ok()
		}
	}

	pub fn CopyTo(&self, pstm: &Stream, cb: u64, pcb_read: Option<&mut MaybeUninit<u64>>, pcb_written: Option<&mut MaybeUninit<u64>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u64, Option<&mut MaybeUninit<u64>>, Option<&mut MaybeUninit<u64>>) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, pstm.vtable(), cb, pcb_read, pcb_written);
			_ret_.ok()
		}
	}

	pub fn Commit(&self, grf_commit_flags: StgC) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, StgC) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, grf_commit_flags);
			_ret_.ok()
		}
	}

	pub fn Revert(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn LockRegion(&self, lib_offset: u64, cb: u64, lock_type: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64, u64, u32) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, lib_offset, cb, lock_type);
			_ret_.ok()
		}
	}

	pub fn UnlockRegion(&self, lib_offset: u64, cb: u64, lock_type: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64, u64, u32) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, lib_offset, cb, lock_type);
			_ret_.ok()
		}
	}

	pub fn Stat(&self, grf_stat_flag: u32) -> Result<StatStg, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pstatstg_out_: MaybeUninit<StatStg> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut StatStg, u32) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, pstatstg_out_.as_mut_ptr(), grf_stat_flag);
			if _ret_.is_ok() { Ok(pstatstg_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn Clone(&self) -> Result<Stream, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut ppstm_out_: Option<Stream> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(&mut ppstm_out_));
			if _ret_.is_ok() { if let Some(ppstm_out_) = ppstm_out_ { return Ok(ppstm_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IStream: ISequentialStream {
	fn as_stream(&self) -> &Stream;
	fn into_stream(self) -> Stream;
}

impl IStream for Stream {
	fn as_stream(&self) -> &Stream { self }
	fn into_stream(self) -> Stream { self }
}
impl ISequentialStream for Stream {
	fn as_sequential_stream(&self) -> &SequentialStream { &self.0.as_sequential_stream() }
	fn into_sequential_stream(self) -> SequentialStream { self.0.into_sequential_stream() }
}

impl IUnknown for Stream {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for Stream {
    fn from(v: UnknownWrapper) -> Self {
        Self(SequentialStream::from(v))
    }
}

