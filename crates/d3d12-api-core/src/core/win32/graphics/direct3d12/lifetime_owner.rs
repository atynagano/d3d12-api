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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12LifetimeOwner(pub(crate) Unknown);

impl Deref for D3D12LifetimeOwner {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12LifetimeOwner {
	const IID: &'static GUID = &GUID::from_u128(0xe667af9fcd564f4683ce032e595d70a8u128);
}

impl Com for D3D12LifetimeOwner {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12LifetimeOwner {
	pub fn LifetimeStateUpdated(&self, new_state: D3D12LifetimeState) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12LifetimeState) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, new_state);
		}
	}
}

pub trait ID3D12LifetimeOwner: IUnknown {
	fn as_lifetime_owner(&self) -> &D3D12LifetimeOwner;
	fn into_lifetime_owner(self) -> D3D12LifetimeOwner;
}

impl ID3D12LifetimeOwner for D3D12LifetimeOwner {
	fn as_lifetime_owner(&self) -> &D3D12LifetimeOwner { self }
	fn into_lifetime_owner(self) -> D3D12LifetimeOwner { self }
}
impl IUnknown for D3D12LifetimeOwner {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12LifetimeOwner {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

