#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StreamSeek
{
	Set                  = 0x0u32,
	Cur                  = 0x1u32,
	End                  = 0x2u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StgC
{
	Default              = 0x0u32,
	Overwrite            = 0x1u32,
	OnlyIfCurrent        = 0x2u32,
	DangerouslyCommitMerelyToDiskCache = 0x4u32,
	Consolidate          = 0x8u32,
}

impl BitOr for StgC {
	type Output = StgC;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for StgC {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl StgC {
    pub fn contains(self, other: StgC) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

