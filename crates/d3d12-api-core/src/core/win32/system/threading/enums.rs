#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateEvent
{
	InitialSet           = 0x2u32,
	ManualReset          = 0x1u32,
}

impl BitOr for CreateEvent {
	type Output = CreateEvent;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for CreateEvent {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl CreateEvent {
    pub fn contains(self, other: CreateEvent) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WorkerThreadFlags
{
	ExecuteDefault       = 0x0u32,
	ExecuteInIoThread    = 0x1u32,
	ExecuteInPersistentThread = 0x80u32,
	ExecuteInWaitThread  = 0x4u32,
	ExECutElongFunction  = 0x10u32,
	ExecuteOnlyOnce      = 0x8u32,
	TransferImpersonation = 0x100u32,
	ExecuteInTimerThread = 0x20u32,
}

impl BitOr for WorkerThreadFlags {
	type Output = WorkerThreadFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for WorkerThreadFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl WorkerThreadFlags {
    pub fn contains(self, other: WorkerThreadFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

