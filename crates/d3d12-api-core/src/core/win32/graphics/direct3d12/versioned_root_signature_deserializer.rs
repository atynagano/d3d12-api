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
use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
pub struct D3D12VersionedRootSignatureDeserializer(pub(crate) Unknown);

impl Guid for D3D12VersionedRootSignatureDeserializer {
	const IID: &'static GUID = &GUID::from_u128(0x7f91ce67090c4bb7b78eed8ff2e31da0u128);
}

impl Clone for D3D12VersionedRootSignatureDeserializer {
	fn clone(&self) -> Self { D3D12VersionedRootSignatureDeserializer(self.0.clone()) }
}

pub trait ID3D12VersionedRootSignatureDeserializer: IUnknown {
	fn as_versioned_root_signature_deserializer(&self) -> &D3D12VersionedRootSignatureDeserializer;
	fn into_versioned_root_signature_deserializer(self) -> D3D12VersionedRootSignatureDeserializer;

	fn GetUnconvertedRootSignatureDesc(&self, ) -> &D3D12VersionedRootSignatureDesc {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> &D3D12VersionedRootSignatureDesc
				= transmute(vt[4]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl ID3D12VersionedRootSignatureDeserializer for D3D12VersionedRootSignatureDeserializer {
	fn as_versioned_root_signature_deserializer(&self) -> &D3D12VersionedRootSignatureDeserializer { self }
	fn into_versioned_root_signature_deserializer(self) -> D3D12VersionedRootSignatureDeserializer { self }
}

impl From<Unknown> for D3D12VersionedRootSignatureDeserializer {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12VersionedRootSignatureDeserializer {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

