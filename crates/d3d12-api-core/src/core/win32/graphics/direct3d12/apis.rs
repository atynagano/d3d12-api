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
use crate::core::win32::graphics::direct3d::*;
use crate::core::win32::system::com::*;


pub fn D3D12SerializeRootSignature(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, mut error_blob: Option<&mut Option<D3DBlob>>) -> Result<D3DBlob, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12SerializeRootSignature(pRootSignature: &D3D12RootSignatureDesc, Version: D3DRootSignatureVersion, ppBlob: *mut c_void, ppErrorBlob: *mut c_void) -> HResult;
		} 
		let mut blob_out_: Option<D3DBlob> = None;
		if let Some(ref mut error_blob) = error_blob { **error_blob = None; }
		let _ret_ = D3D12SerializeRootSignature(root_signature, version, transmute(&mut blob_out_), transmute(error_blob));
		if _ret_.is_ok() { if let Some(blob_out_) = blob_out_ { return Ok(blob_out_); } }
		Err(_ret_)
	}
}

pub fn D3D12CreateRootSignatureDeserializer<T: IUnknown + From<UnknownWrapper>>(src_data: &[u8]) -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateRootSignatureDeserializer(pSrcData: *const u8, SrcDataSizeInBytes: usize, pRootSignatureDeserializerInterface: &GUID, ppRootSignatureDeserializer: *mut c_void) -> HResult;
		} 
		let (src_data_ptr_, src_data_len_) = src_data.deconstruct();
		let mut root_signature_deserializer_out_: Option<Unknown> = None;
		let _ret_ = D3D12CreateRootSignatureDeserializer(src_data_ptr_, src_data_len_ as usize, T::IID, transmute(&mut root_signature_deserializer_out_));
		if _ret_.is_ok() { if let Some(root_signature_deserializer_out_) = root_signature_deserializer_out_ { return Ok(T::from(UnknownWrapper(root_signature_deserializer_out_))); } }
		Err(_ret_)
	}
}

pub fn D3D12SerializeVersionedRootSignature(root_signature: &D3D12VersionedRootSignatureDesc, mut error_blob: Option<&mut Option<D3DBlob>>) -> Result<D3DBlob, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12SerializeVersionedRootSignature(pRootSignature: &D3D12VersionedRootSignatureDesc, ppBlob: *mut c_void, ppErrorBlob: *mut c_void) -> HResult;
		} 
		let mut blob_out_: Option<D3DBlob> = None;
		if let Some(ref mut error_blob) = error_blob { **error_blob = None; }
		let _ret_ = D3D12SerializeVersionedRootSignature(root_signature, transmute(&mut blob_out_), transmute(error_blob));
		if _ret_.is_ok() { if let Some(blob_out_) = blob_out_ { return Ok(blob_out_); } }
		Err(_ret_)
	}
}

pub fn D3D12CreateVersionedRootSignatureDeserializer<T: IUnknown + From<UnknownWrapper>>(src_data: &[u8]) -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateVersionedRootSignatureDeserializer(pSrcData: *const u8, SrcDataSizeInBytes: usize, pRootSignatureDeserializerInterface: &GUID, ppRootSignatureDeserializer: *mut c_void) -> HResult;
		} 
		let (src_data_ptr_, src_data_len_) = src_data.deconstruct();
		let mut root_signature_deserializer_out_: Option<Unknown> = None;
		let _ret_ = D3D12CreateVersionedRootSignatureDeserializer(src_data_ptr_, src_data_len_ as usize, T::IID, transmute(&mut root_signature_deserializer_out_));
		if _ret_.is_ok() { if let Some(root_signature_deserializer_out_) = root_signature_deserializer_out_ { return Ok(T::from(UnknownWrapper(root_signature_deserializer_out_))); } }
		Err(_ret_)
	}
}

pub fn D3D12CreateDevice<T: IUnknown + From<UnknownWrapper>>(adapter: Option<&Unknown>, minimum_feature_level: D3DFeatureLevel, device: Option<&mut Option<T>>) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateDevice(pAdapter: *const c_void, MinimumFeatureLevel: D3DFeatureLevel, riid: &GUID, ppDevice: *mut c_void) -> HResult;
		} 
		let mut device_out_: Option<Unknown> = None;
		let _ret_ = D3D12CreateDevice(option_to_vtable(adapter), minimum_feature_level, T::IID, transmute(if device_out_.is_some() { Some(&mut device_out_) } else { None }));
		if let Some(device_out_) = device_out_ { *device.unwrap_unchecked() = Some(T::from(UnknownWrapper(device_out_))); }
		_ret_.ok()
	}
}

pub fn d3d12_create_device<T: IUnknown + From<UnknownWrapper>>(adapter: Option<&Unknown>, minimum_feature_level: D3DFeatureLevel) -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateDevice(pAdapter: *const c_void, MinimumFeatureLevel: D3DFeatureLevel, riid: &GUID, ppDevice: *mut c_void) -> HResult;
		} 
		let mut device_out_: Option<Unknown> = None;
		let _ret_ = D3D12CreateDevice(option_to_vtable(adapter), minimum_feature_level, T::IID, transmute(&mut device_out_));
		if _ret_.is_ok() { if let Some(device_out_) = device_out_ { return Ok(T::from(UnknownWrapper(device_out_))); } }
		Err(_ret_)
	}
}

pub fn D3D12GetDebugInterface<T: IUnknown + From<UnknownWrapper>>(debug: Option<&mut Option<T>>) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetDebugInterface(riid: &GUID, ppvDebug: *mut c_void) -> HResult;
		} 
		let mut debug_out_: Option<Unknown> = None;
		let _ret_ = D3D12GetDebugInterface(T::IID, transmute(if debug_out_.is_some() { Some(&mut debug_out_) } else { None }));
		if let Some(debug_out_) = debug_out_ { *debug.unwrap_unchecked() = Some(T::from(UnknownWrapper(debug_out_))); }
		_ret_.ok()
	}
}

pub fn d3d12_get_debug_interface<T: IUnknown + From<UnknownWrapper>>() -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetDebugInterface(riid: &GUID, ppvDebug: *mut c_void) -> HResult;
		} 
		let mut debug_out_: Option<Unknown> = None;
		let _ret_ = D3D12GetDebugInterface(T::IID, transmute(&mut debug_out_));
		if _ret_.is_ok() { if let Some(debug_out_) = debug_out_ { return Ok(T::from(UnknownWrapper(debug_out_))); } }
		Err(_ret_)
	}
}

	pub unsafe fn D3D12EnableExperimentalFeatures() { todo!() }

pub fn D3D12GetInterface<T: IUnknown + From<UnknownWrapper>>(rclsid: &GUID, debug: Option<&mut Option<T>>) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetInterface(rclsid: &GUID, riid: &GUID, ppvDebug: *mut c_void) -> HResult;
		} 
		let mut debug_out_: Option<Unknown> = None;
		let _ret_ = D3D12GetInterface(rclsid, T::IID, transmute(if debug_out_.is_some() { Some(&mut debug_out_) } else { None }));
		if let Some(debug_out_) = debug_out_ { *debug.unwrap_unchecked() = Some(T::from(UnknownWrapper(debug_out_))); }
		_ret_.ok()
	}
}

pub fn d3d12_get_interface<T: IUnknown + From<UnknownWrapper>>(rclsid: &GUID) -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetInterface(rclsid: &GUID, riid: &GUID, ppvDebug: *mut c_void) -> HResult;
		} 
		let mut debug_out_: Option<Unknown> = None;
		let _ret_ = D3D12GetInterface(rclsid, T::IID, transmute(&mut debug_out_));
		if _ret_.is_ok() { if let Some(debug_out_) = debug_out_ { return Ok(T::from(UnknownWrapper(debug_out_))); } }
		Err(_ret_)
	}
}


pub type PfnD3D12SerializeRootSignature 
	= unsafe extern "system" fn(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, blob: &mut D3DBlob, error_blob: Option<&mut D3DBlob>, ) -> HResult;

pub type PfnD3D12CreateRootSignatureDeserializer 
	= unsafe extern "system" fn(src_data: &(), src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, root_signature_deserializer: &mut &(), ) -> HResult;

pub type PfnD3D12SerializeVersionedRootSignature 
	= unsafe extern "system" fn(root_signature: &D3D12VersionedRootSignatureDesc, blob: &mut D3DBlob, error_blob: Option<&mut D3DBlob>, ) -> HResult;

pub type PfnD3D12CreateVersionedRootSignatureDeserializer 
	= unsafe extern "system" fn(src_data: &(), src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, root_signature_deserializer: &mut &(), ) -> HResult;

pub type D3D12MessageFunc 
	= unsafe extern "system" fn(category: D3D12MessageCategory, severity: D3D12MessageSeverity, id: D3D12MessageId, description: PStr, context: &mut (), ) -> ();

pub type PfnD3D12CreateDevice 
	= unsafe extern "system" fn(param0: Unknown, param1: D3DFeatureLevel, param2: &GUID, param3: Option<&mut &()>, ) -> HResult;

pub type PfnD3D12GetDebugInterface 
	= unsafe extern "system" fn(param0: &GUID, param1: Option<&mut &()>, ) -> HResult;

pub type PfnD3D12GetInterface 
	= unsafe extern "system" fn(param0: &GUID, param1: &GUID, param2: Option<&mut &()>, ) -> HResult;

