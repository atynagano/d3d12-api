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
use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1CommandSink2(pub(crate) D2D1CommandSink1);

impl Deref for D2D1CommandSink2 {
	type Target = D2D1CommandSink1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1CommandSink2 {
	const IID: &'static GUID = &GUID::from_u128(0x3bab440e417e47dfa2e2bc0be6a00916u128);
}

impl Com for D2D1CommandSink2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1CommandSink2 {
	pub fn DrawInk(&self, ink: &D2D1Ink, brush: &D2D1Brush, ink_style: Option<&D2D1InkStyle>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, *const c_void) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, ink.vtable(), brush.vtable(), option_to_vtable(ink_style));
			_ret_.ok()
		}
	}

	pub fn DrawGradientMesh(&self, gradient_mesh: &D2D1GradientMesh) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, gradient_mesh.vtable());
			_ret_.ok()
		}
	}

	pub fn DrawGdiMetafile(&self, gdi_metafile: &D2D1GdiMetafile, destination_rectangle: Option<&D2DRectF>, source_rectangle: Option<&D2DRectF>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, *const c_void) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, gdi_metafile.vtable(), transmute(destination_rectangle), transmute(source_rectangle));
			_ret_.ok()
		}
	}
}

pub trait ID2D1CommandSink2: ID2D1CommandSink1 {
	fn as_command_sink2(&self) -> &D2D1CommandSink2;
	fn into_command_sink2(self) -> D2D1CommandSink2;
}

impl ID2D1CommandSink2 for D2D1CommandSink2 {
	fn as_command_sink2(&self) -> &D2D1CommandSink2 { self }
	fn into_command_sink2(self) -> D2D1CommandSink2 { self }
}
impl ID2D1CommandSink1 for D2D1CommandSink2 {
	fn as_command_sink1(&self) -> &D2D1CommandSink1 { &self.0.as_command_sink1() }
	fn into_command_sink1(self) -> D2D1CommandSink1 { self.0.into_command_sink1() }
}

impl ID2D1CommandSink for D2D1CommandSink2 {
	fn as_command_sink(&self) -> &D2D1CommandSink { &self.0.as_command_sink() }
	fn into_command_sink(self) -> D2D1CommandSink { self.0.into_command_sink() }
}

impl IUnknown for D2D1CommandSink2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1CommandSink2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1CommandSink1::from(v))
    }
}

