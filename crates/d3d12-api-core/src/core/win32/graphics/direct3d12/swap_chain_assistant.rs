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

	fn GetLUID(&self, ) -> (Luid) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Luid
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetSwapChainObject<T: IUnknown>(&self, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _ppv: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, riid: &GUID, _ppv: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, T::IID, &mut _ppv, );
		if ret.is_ok() {
			if let (Some(_ppv)) = (_ppv) {
				return Ok((T::from(_ppv)));
			}
		}
		Err(ret)
	}

	fn InsertImplicitSync(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, );
		ret.ok()
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

