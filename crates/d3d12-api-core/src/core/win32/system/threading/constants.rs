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

use std::ops::{BitOr, BitOrAssign};
pub const WAIT_OBJECT_0: u32 = 0x0;
pub const WAIT_ABANDONED: u32 = 0x80;
pub const WAIT_ABANDONED_0: u32 = 0x80;
pub const WAIT_IO_COMPLETION: u32 = 0xC0;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 0x1;
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 0x1;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 0x1;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 0x1;
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 0x1;
pub const PME_CURRENT_VERSION: u32 = 0x1;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0x0;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 0x1;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 0x1;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 0x1;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 0x4;
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 0x1;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 0x1;
pub const INIT_ONCE_CHECK_ONLY: u32 = 0x1;
pub const INIT_ONCE_ASYNC: u32 = 0x2;
pub const INIT_ONCE_INIT_FAILED: u32 = 0x4;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 0x2;
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 0x1;
pub const MUTEX_MODIFY_STATE: u32 = 0x1;
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 0x1;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 0x1;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 0x2;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 0x1;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 0x2;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 0x4;
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 0x20000;
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 0x20002;
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 0x30003;
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 0x20004;
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 0x30005;
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 0x30006;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 0x20007;
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 0x20009;
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 0x2000B;
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 0x20016;
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 0x20019;
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 0x3001B;
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 0x20010;
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 0x2000D;
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 0x2000E;
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 0x2000F;
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 0x20012;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 0x20018;
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 0x2001A;
