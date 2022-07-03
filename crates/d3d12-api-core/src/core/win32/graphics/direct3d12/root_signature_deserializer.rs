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

use crate::core::win32::graphics::direct3d12::*;
#[repr(C)]
pub struct D3D12RootSignatureDeserializer(pub(crate) Unknown);

impl Guid for D3D12RootSignatureDeserializer {
	const IID: &'static GUID = &GUID::from_u128(0x34ab647b3cc846ac841bc0965645c046u128);
}

impl Clone for D3D12RootSignatureDeserializer {
	fn clone(&self) -> Self { D3D12RootSignatureDeserializer(self.0.clone()) }
}

pub trait ID3D12RootSignatureDeserializer: IUnknown {
	fn as_root_signature_deserializer(&self) -> &D3D12RootSignatureDeserializer;
	fn into_root_signature_deserializer(self) -> D3D12RootSignatureDeserializer;

	fn GetRootSignatureDesc(&self, ) -> (&D3D12RootSignatureDesc) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> &D3D12RootSignatureDesc
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3D12RootSignatureDeserializer for D3D12RootSignatureDeserializer {
	fn as_root_signature_deserializer(&self) -> &D3D12RootSignatureDeserializer { self }
	fn into_root_signature_deserializer(self) -> D3D12RootSignatureDeserializer { self }
}

impl From<Unknown> for D3D12RootSignatureDeserializer {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12RootSignatureDeserializer {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

