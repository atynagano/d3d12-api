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
#[repr(C)]
pub struct D3D12Tools(pub(crate) Unknown);

impl Guid for D3D12Tools {
	const IID: &'static GUID = &GUID::from_u128(0x7071e1f0e84b4b33974f12fa49de65c5u128);
}

impl Clone for D3D12Tools {
	fn clone(&self) -> Self { D3D12Tools(self.0.clone()) }
}

pub trait ID3D12Tools: IUnknown {
	fn as_tools(&self) -> &D3D12Tools;
	fn into_tools(self) -> D3D12Tools;

	fn EnableShaderInstrumentation(&self, enable: bool, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, enable: Bool, ) -> ()
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, enable.to_bool(), );
	}

	fn ShaderInstrumentationEnabled(&self, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Bool
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, );
		return (ret.to_bool());
	}
}

impl ID3D12Tools for D3D12Tools {
	fn as_tools(&self) -> &D3D12Tools { self }
	fn into_tools(self) -> D3D12Tools { self }
}

impl From<Unknown> for D3D12Tools {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12Tools {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

