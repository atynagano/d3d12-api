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
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dlib_move: i64, origin: StreamSeek, plib_new_position: Option<&mut u64>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, dlib_move, origin, plib_new_position, );
		ret.ok()
	}

	fn SetSize(&self, lib_new_size: u64, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, lib_new_size: u64, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, lib_new_size, );
		ret.ok()
	}

	fn CopyTo(&self, pstm: &(impl IStream + ?Sized), cb: u64, pcb_read: Option<&mut u64>, pcb_written: Option<&mut u64>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, pstm: VTable, cb: u64, pcb_read: Option<&mut u64>, pcb_written: Option<&mut u64>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, pstm.vtable(), cb, pcb_read, pcb_written, );
		ret.ok()
	}

	fn Commit(&self, grf_commit_flags: StgC, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, grf_commit_flags: StgC, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, grf_commit_flags, );
		ret.ok()
	}

	fn Revert(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn LockRegion(&self, lib_offset: u64, cb: u64, lock_type: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, lib_offset: u64, cb: u64, lock_type: u32, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, lib_offset, cb, lock_type, );
		ret.ok()
	}

	fn UnlockRegion(&self, lib_offset: u64, cb: u64, lock_type: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, lib_offset: u64, cb: u64, lock_type: u32, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, lib_offset, cb, lock_type, );
		ret.ok()
	}

	fn Stat(&self, grf_stat_flag: u32, ) -> Result<(StatStg), HResult> {
		let vt = self.as_param();
		let mut _pstatstg: StatStg = StatStg::zeroed();
		let f: extern "system" fn(Param<Self>, _pstatstg: &mut StatStg, grf_stat_flag: u32, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, &mut _pstatstg, grf_stat_flag, );
		if ret.is_ok() {
			return Ok((_pstatstg));
		}
		Err(ret)
	}

	fn Clone(&self, ) -> Result<(Stream), HResult> {
		let vt = self.as_param();
		let mut _ppstm: Option<Stream> = None;
		let f: extern "system" fn(Param<Self>, _ppstm: &mut Option<Stream>, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, &mut _ppstm, );
		if ret.is_ok() {
			if let (Some(_ppstm)) = (_ppstm) {
				return Ok((_ppstm));
			}
		}
		Err(ret)
	}
}

impl IStream for Stream {
	fn as_stream(&self) -> &Stream { self }
	fn into_stream(self) -> Stream { self }
}

impl ISequentialStream for Stream {
	fn as_sequential_stream(&self) -> &SequentialStream { &self.0 }
	fn into_sequential_stream(self) -> SequentialStream { self.0 }
}

impl From<Unknown> for Stream {
    fn from(v: Unknown) -> Self {
        Self(SequentialStream::from(v))
    }
}

impl IUnknown for Stream {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

