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
pub struct D3D12SwapChainAssistant(pub(crate) Unknown);

impl Deref for D3D12SwapChainAssistant {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12SwapChainAssistant {
	const IID: &'static GUID = &GUID::from_u128(0xf1df64b657fd49cd8807c0eb88b45c8fu128);
}

impl Com for D3D12SwapChainAssistant {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12SwapChainAssistant {
	pub fn GetLUID(&self) -> Luid {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Luid
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetSwapChainObject<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut ppv_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, T::IID, transmute(&mut ppv_out_));
			if _ret_.is_ok() { if let Some(ppv_out_) = ppv_out_ { return Ok(T::from(UnknownWrapper(ppv_out_))); } }
			Err(_ret_)
		}
	}

	pub fn InsertImplicitSync(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait ID3D12SwapChainAssistant: IUnknown {
	fn as_swap_chain_assistant(&self) -> &D3D12SwapChainAssistant;
	fn into_swap_chain_assistant(self) -> D3D12SwapChainAssistant;
}

impl ID3D12SwapChainAssistant for D3D12SwapChainAssistant {
	fn as_swap_chain_assistant(&self) -> &D3D12SwapChainAssistant { self }
	fn into_swap_chain_assistant(self) -> D3D12SwapChainAssistant { self }
}
impl IUnknown for D3D12SwapChainAssistant {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12SwapChainAssistant {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

