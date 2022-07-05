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

#[repr(C)]
pub struct D3D12SwapChainAssistant(pub(crate) Unknown);

impl Guid for D3D12SwapChainAssistant {
	const IID: &'static GUID = &GUID::from_u128(0xf1df64b657fd49cd8807c0eb88b45c8fu128);
}

impl Clone for D3D12SwapChainAssistant {
	fn clone(&self) -> Self { D3D12SwapChainAssistant(self.0.clone()) }
}

pub trait ID3D12SwapChainAssistant: IUnknown {
	fn as_swap_chain_assistant(&self) -> &D3D12SwapChainAssistant;
	fn into_swap_chain_assistant(self) -> D3D12SwapChainAssistant;

	fn GetLUID(&self, ) -> Luid {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Luid
				= transmute(vt[3]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetSwapChainObject<T: IUnknown>(&self, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_ppv: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, riid: &GUID, _out_ppv: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, T::IID, transmute(&mut _out_ppv), );
			if _ret_.is_ok() {
				if let Some(_out_ppv) = _out_ppv {
					return Ok(T::from(_out_ppv));
				}
			}
			Err(_ret_)
		}
	}

	fn InsertImplicitSync(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}
}

impl ID3D12SwapChainAssistant for D3D12SwapChainAssistant {
	fn as_swap_chain_assistant(&self) -> &D3D12SwapChainAssistant { self }
	fn into_swap_chain_assistant(self) -> D3D12SwapChainAssistant { self }
}

impl From<Unknown> for D3D12SwapChainAssistant {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12SwapChainAssistant {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

