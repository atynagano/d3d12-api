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
use crate::core::win32::graphics::direct3d::*;
use crate::core::win32::system::com::*;


pub fn D3D12SerializeRootSignature(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, mut error_blob: Option<&mut Option<D3DBlob>>, ) -> Result<D3DBlob, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12SerializeRootSignature(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, blob: *mut c_void, error_blob: *mut c_void, ) -> HResult;
		}
		let mut _out_blob: Option<D3DBlob> = None;
		if let Some(ref mut error_blob) = error_blob { **error_blob = None; }
		let _ret_ = D3D12SerializeRootSignature(root_signature, version, transmute(&mut _out_blob), transmute(error_blob), );
		if _ret_.is_ok() {
			if let Some(_out_blob) = _out_blob {
				return Ok(_out_blob);
			}
		}
		Err(_ret_)
	}
}

pub fn D3D12SerializeVersionedRootSignature(root_signature: &D3D12VersionedRootSignatureDesc, mut error_blob: Option<&mut Option<D3DBlob>>, ) -> Result<D3DBlob, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12SerializeVersionedRootSignature(root_signature: &D3D12VersionedRootSignatureDesc, blob: *mut c_void, error_blob: *mut c_void, ) -> HResult;
		}
		let mut _out_blob: Option<D3DBlob> = None;
		if let Some(ref mut error_blob) = error_blob { **error_blob = None; }
		let _ret_ = D3D12SerializeVersionedRootSignature(root_signature, transmute(&mut _out_blob), transmute(error_blob), );
		if _ret_.is_ok() {
			if let Some(_out_blob) = _out_blob {
				return Ok(_out_blob);
			}
		}
		Err(_ret_)
	}
}

pub fn D3D12CreateDevice<T: IUnknown>(adapter: Option<&Unknown>, minimum_feature_level: D3DFeatureLevel, device: Option<&mut Option<T>>, ) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateDevice(adapter: *const c_void, minimum_feature_level: D3DFeatureLevel, riid: &GUID, device: *mut c_void, ) -> HResult;
		}
		let mut _out_device: Option<Unknown> = None;
		let _ret_ = D3D12CreateDevice(option_to_vtable(adapter), minimum_feature_level, T::IID, transmute(if device.is_some() { Some(&mut _out_device) } else { None }), );
		if let Some(_out_device) = _out_device { *device.unwrap_unchecked() = Some(T::from(_out_device)); }
		_ret_.ok()
	}
}

pub fn d3d12create_device<T: IUnknown>(adapter: Option<&Unknown>, minimum_feature_level: D3DFeatureLevel, ) -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn d3d12create_device(adapter: *const c_void, minimum_feature_level: D3DFeatureLevel, riid: &GUID, _out_device: *mut c_void, ) -> HResult;
		}
		let mut _out_device: Option<Unknown> = None;
		let _ret_ = d3d12create_device(option_to_vtable(adapter), minimum_feature_level, T::IID, transmute(&mut _out_device), );
		if _ret_.is_ok() {
			if let Some(_out_device) = _out_device {
				return Ok(T::from(_out_device));
			}
		}
		Err(_ret_)
	}
}

pub fn D3D12GetDebugInterface<T: IUnknown>(debug: Option<&mut Option<T>>, ) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetDebugInterface(riid: &GUID, debug: *mut c_void, ) -> HResult;
		}
		let mut _out_debug: Option<Unknown> = None;
		let _ret_ = D3D12GetDebugInterface(T::IID, transmute(if debug.is_some() { Some(&mut _out_debug) } else { None }), );
		if let Some(_out_debug) = _out_debug { *debug.unwrap_unchecked() = Some(T::from(_out_debug)); }
		_ret_.ok()
	}
}

pub fn d3d12get_debug_interface<T: IUnknown>() -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn d3d12get_debug_interface(riid: &GUID, _out_debug: *mut c_void, ) -> HResult;
		}
		let mut _out_debug: Option<Unknown> = None;
		let _ret_ = d3d12get_debug_interface(T::IID, transmute(&mut _out_debug), );
		if _ret_.is_ok() {
			if let Some(_out_debug) = _out_debug {
				return Ok(T::from(_out_debug));
			}
		}
		Err(_ret_)
	}
}

pub fn D3D12GetInterface<T: IUnknown>(rclsid: &GUID, debug: Option<&mut Option<T>>, ) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetInterface(rclsid: &GUID, riid: &GUID, debug: *mut c_void, ) -> HResult;
		}
		let mut _out_debug: Option<Unknown> = None;
		let _ret_ = D3D12GetInterface(rclsid, T::IID, transmute(if debug.is_some() { Some(&mut _out_debug) } else { None }), );
		if let Some(_out_debug) = _out_debug { *debug.unwrap_unchecked() = Some(T::from(_out_debug)); }
		_ret_.ok()
	}
}

pub fn d3d12get_interface<T: IUnknown>(rclsid: &GUID, ) -> Result<T, HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn d3d12get_interface(rclsid: &GUID, riid: &GUID, _out_debug: *mut c_void, ) -> HResult;
		}
		let mut _out_debug: Option<Unknown> = None;
		let _ret_ = d3d12get_interface(rclsid, T::IID, transmute(&mut _out_debug), );
		if _ret_.is_ok() {
			if let Some(_out_debug) = _out_debug {
				return Ok(T::from(_out_debug));
			}
		}
		Err(_ret_)
	}
}


pub type PFnD3D12SerializeRootSignature 
	= unsafe extern "system" fn(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, blob: &mut D3DBlob, error_blob: Option<&mut D3DBlob>, ) -> HResult;

pub type PFnD3D12CreateRootSignatureDeserializer 
	= unsafe extern "system" fn(src_data: &(), src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, root_signature_deserializer: &mut &(), ) -> HResult;

pub type PFnD3D12SerializeVersionedRootSignature 
	= unsafe extern "system" fn(root_signature: &D3D12VersionedRootSignatureDesc, blob: &mut D3DBlob, error_blob: Option<&mut D3DBlob>, ) -> HResult;

pub type PFnD3D12CreateVersionedRootSignatureDeserializer 
	= unsafe extern "system" fn(src_data: &(), src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, root_signature_deserializer: &mut &(), ) -> HResult;

pub type D3D12MessageFunc 
	= unsafe extern "system" fn(category: D3D12MessageCategory, severity: D3D12MessageSeverity, id: D3D12MessageId, description: PStr, context: &mut (), ) -> ();

pub type PFnD3D12CreateDevice 
	= unsafe extern "system" fn(param0: Unknown, param1: D3DFeatureLevel, param2: &GUID, param3: Option<&mut &()>, ) -> HResult;

pub type PFnD3D12GetDebugInterface 
	= unsafe extern "system" fn(param0: &GUID, param1: Option<&mut &()>, ) -> HResult;

pub type PFnD3D12GetInterface 
	= unsafe extern "system" fn(param0: &GUID, param1: &GUID, param2: Option<&mut &()>, ) -> HResult;

