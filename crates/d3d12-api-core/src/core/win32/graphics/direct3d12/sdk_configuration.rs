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
pub struct D3D12SDKConfiguration(pub(crate) Unknown);

impl Deref for D3D12SDKConfiguration {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12SDKConfiguration {
	const IID: &'static GUID = &GUID::from_u128(0xe9eb531433aa42b2a718d77f58b1f1c7u128);
}

impl Com for D3D12SDKConfiguration {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12SDKConfiguration {
	pub fn SetSDKVersion(&self, sdk_version: u32, sdk_path: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, *const u8) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, sdk_version, sdk_path.to_null_terminated().as_ptr_or_null());
			_ret_.ok()
		}
	}
}

pub trait ID3D12SDKConfiguration: IUnknown {
	fn as_sdk_configuration(&self) -> &D3D12SDKConfiguration;
	fn into_sdk_configuration(self) -> D3D12SDKConfiguration;
}

impl ID3D12SDKConfiguration for D3D12SDKConfiguration {
	fn as_sdk_configuration(&self) -> &D3D12SDKConfiguration { self }
	fn into_sdk_configuration(self) -> D3D12SDKConfiguration { self }
}
impl IUnknown for D3D12SDKConfiguration {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12SDKConfiguration {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

