#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

bitflags::bitflags! {
	/// SYNCHRONIZATION_ACCESS_RIGHTS
	#[repr(transparent)]
	pub struct SynchronizationAccessRights: u32 {
		/// EVENT_ALL_ACCESS = 0x1F0003u32
		const AllAccess            = 0x1F0003u32;
		/// EVENT_MODIFY_STATE = 0x2u32
		const ModifyState          = 0x2u32;
		/// TIMER_QUERY_STATE = 0x1u32
		const QueryState           = 0x1u32;
		/// SYNCHRONIZATION_DELETE = 0x10000u32
		const Delete               = 0x10000u32;
		/// SYNCHRONIZATION_READ_CONTROL = 0x20000u32
		const ReadControl          = 0x20000u32;
		/// SYNCHRONIZATION_WRITE_DAC = 0x40000u32
		const WriteDac             = 0x40000u32;
		/// SYNCHRONIZATION_WRITE_OWNER = 0x80000u32
		const WriteOwner           = 0x80000u32;
		/// SYNCHRONIZATION_SYNCHRONIZE = 0x100000u32
		const Synchronize          = 0x100000u32;
	}
}

bitflags::bitflags! {
	/// CREATE_EVENT
	#[repr(transparent)]
	pub struct CreateEvent: u32 {
		/// CREATE_EVENT_INITIAL_SET = 0x2u32
		const InitialSet           = 0x2u32;
		/// CREATE_EVENT_MANUAL_RESET = 0x1u32
		const ManualReset          = 0x1u32;
	}
}

bitflags::bitflags! {
	/// WORKER_THREAD_FLAGS
	#[repr(transparent)]
	pub struct WorkerThreadFlags: u32 {
		/// WT_EXECUTEDEFAULT = 0x0u32
		const ExecuteDefault       = 0x0u32;
		/// WT_EXECUTEINIOTHREAD = 0x1u32
		const ExecuteInIoThread    = 0x1u32;
		/// WT_EXECUTEINPERSISTENTTHREAD = 0x80u32
		const ExecuteInPersistentThread = 0x80u32;
		/// WT_EXECUTEINWAITTHREAD = 0x4u32
		const ExecuteInWaitThread  = 0x4u32;
		/// WT_EXECUTELONGFUNCTION = 0x10u32
		const ExECutElongFunction  = 0x10u32;
		/// WT_EXECUTEONLYONCE = 0x8u32
		const ExecuteOnlyOnce      = 0x8u32;
		/// WT_TRANSFER_IMPERSONATION = 0x100u32
		const TransferImpersonation = 0x100u32;
		/// WT_EXECUTEINTIMERTHREAD = 0x20u32
		const ExecuteInTimerThread = 0x20u32;
	}
}

