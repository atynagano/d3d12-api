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

#[repr(C)]
pub struct SequentialStream(pub(crate) Unknown);

impl Guid for SequentialStream {
	const IID: &'static GUID = &GUID::from_u128(0x0c733a302a1c11ceade500aa0044773du128);
}

impl Clone for SequentialStream {
	fn clone(&self) -> Self { SequentialStream(self.0.clone()) }
}

pub trait ISequentialStream: IUnknown {
	fn as_sequential_stream(&self) -> &SequentialStream;
	fn into_sequential_stream(self) -> SequentialStream;
}

impl ISequentialStream for SequentialStream {
	fn as_sequential_stream(&self) -> &SequentialStream { self }
	fn into_sequential_stream(self) -> SequentialStream { self }
}

impl From<Unknown> for SequentialStream {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for SequentialStream {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

