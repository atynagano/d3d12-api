#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// STREAM_SEEK
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StreamSeek
{
	/// STREAM_SEEK_SET = 0x0u32
	Set                  = 0x0u32,
	/// STREAM_SEEK_CUR = 0x1u32
	Cur                  = 0x1u32,
	/// STREAM_SEEK_END = 0x2u32
	End                  = 0x2u32,
}

bitflags::bitflags! {
	/// STGC
	#[repr(transparent)]
	pub struct StgC: u32 {
		/// STGC_DEFAULT = 0x0u32
		const Default              = 0x0u32;
		/// STGC_OVERWRITE = 0x1u32
		const Overwrite            = 0x1u32;
		/// STGC_ONLYIFCURRENT = 0x2u32
		const OnlyIfCurrent        = 0x2u32;
		/// STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE = 0x4u32
		const DangerouslyCommitMerelyToDiskCache = 0x4u32;
		/// STGC_CONSOLIDATE = 0x8u32
		const Consolidate          = 0x8u32;
	}
}

