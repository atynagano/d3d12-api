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
use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12VersionedRootSignatureDeserializer(pub(crate) Unknown);

impl Deref for D3D12VersionedRootSignatureDeserializer {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12VersionedRootSignatureDeserializer {
	const IID: &'static GUID = &GUID::from_u128(0x7f91ce67090c4bb7b78eed8ff2e31da0u128);
}

impl Com for D3D12VersionedRootSignatureDeserializer {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12VersionedRootSignatureDeserializer {
	pub unsafe fn GetRootSignatureDescAtVersion(&self) { todo!() }

	pub fn GetUnconvertedRootSignatureDesc(&self) -> &D3D12VersionedRootSignatureDesc {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> &D3D12VersionedRootSignatureDesc
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID3D12VersionedRootSignatureDeserializer: IUnknown {
	fn as_versioned_root_signature_deserializer(&self) -> &D3D12VersionedRootSignatureDeserializer;
	fn into_versioned_root_signature_deserializer(self) -> D3D12VersionedRootSignatureDeserializer;
}

impl ID3D12VersionedRootSignatureDeserializer for D3D12VersionedRootSignatureDeserializer {
	fn as_versioned_root_signature_deserializer(&self) -> &D3D12VersionedRootSignatureDeserializer { self }
	fn into_versioned_root_signature_deserializer(self) -> D3D12VersionedRootSignatureDeserializer { self }
}
impl IUnknown for D3D12VersionedRootSignatureDeserializer {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12VersionedRootSignatureDeserializer {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

