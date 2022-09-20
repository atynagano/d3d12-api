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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Mesh(pub(crate) D2D1Resource);

impl Deref for D2D1Mesh {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Mesh {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906c212e211dc9fed001143a055f9u128);
}

impl Com for D2D1Mesh {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Mesh {
	pub fn Open(&self) -> Result<D2D1TessellationSink, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut tessellation_sink_out_: Option<D2D1TessellationSink> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(&mut tessellation_sink_out_));
			if _ret_.is_ok() { if let Some(tessellation_sink_out_) = tessellation_sink_out_ { return Ok(tessellation_sink_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Mesh: ID2D1Resource {
	fn as_mesh(&self) -> &D2D1Mesh;
	fn into_mesh(self) -> D2D1Mesh;
}

impl ID2D1Mesh for D2D1Mesh {
	fn as_mesh(&self) -> &D2D1Mesh { self }
	fn into_mesh(self) -> D2D1Mesh { self }
}
impl ID2D1Resource for D2D1Mesh {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Mesh {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Mesh {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

