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

	fn GetRootSignatureDescAtVersion(&self, convert_to_version: D3DRootSignatureVersion, ) -> Result<(&D3D12VersionedRootSignatureDesc), HResult> {
		let vt = self.as_param();
		let mut _desc: Option<&D3D12VersionedRootSignatureDesc> = None;
		let f: extern "system" fn(Param<Self>, convert_to_version: D3DRootSignatureVersion, _desc: &mut Option<&D3D12VersionedRootSignatureDesc>, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, convert_to_version, &mut _desc, );
		if ret.is_ok() {
			if let (Some(_desc)) = (_desc) {
				return Ok((_desc));
			}
		}
		Err(ret)
	}

	fn GetUnconvertedRootSignatureDesc(&self, ) -> (&D3D12VersionedRootSignatureDesc) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> &D3D12VersionedRootSignatureDesc
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, );
		return (ret);
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

