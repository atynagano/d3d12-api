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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Tools(pub(crate) Unknown);

impl Deref for D3D12Tools {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Tools {
	const IID: &'static GUID = &GUID::from_u128(0x7071e1f0e84b4b33974f12fa49de65c5u128);
}

impl Com for D3D12Tools {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Tools {
	pub fn EnableShaderInstrumentation(&self, enable: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, enable.to_bool());
		}
	}

	pub fn ShaderInstrumentationEnabled(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}
}

pub trait ID3D12Tools: IUnknown {
	fn as_tools(&self) -> &D3D12Tools;
	fn into_tools(self) -> D3D12Tools;
}

impl ID3D12Tools for D3D12Tools {
	fn as_tools(&self) -> &D3D12Tools { self }
	fn into_tools(self) -> D3D12Tools { self }
}
impl IUnknown for D3D12Tools {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Tools {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

