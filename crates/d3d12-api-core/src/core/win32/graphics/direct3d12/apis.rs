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
use crate::core::win32::graphics::direct3d::*;
use crate::core::win32::system::com::*;

pub fn D3D12SerializeRootSignature(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, mut error_blob: Option<&mut Option<D3DBlob>>,) -> Result<(D3DBlob), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12SerializeRootSignature(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, _blob: &mut Option<D3DBlob>, error_blob: Option<&mut Option<D3DBlob>>, ) -> HResult;
		}
		let mut _blob: Option<D3DBlob> = None;
		if let Some(ref mut error_blob) = error_blob { **error_blob = None; }
		let ret = D3D12SerializeRootSignature(root_signature, version, &mut _blob, error_blob, );
		if ret.is_ok() {
			if let (Some(_blob)) = (_blob) {
				return Ok((_blob));
			}
		}
		Err(ret)
	}
}

pub fn D3D12CreateRootSignatureDeserializer<T: IUnknown>(src_data: &[u8], ) -> Result<(T), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateRootSignatureDeserializer(src_data: *const u8, src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, _root_signature_deserializer: &mut Option<Unknown>, ) -> HResult;
		}
		let mut _root_signature_deserializer: Option<Unknown> = None;
		let ret = D3D12CreateRootSignatureDeserializer(src_data.as_ptr_or_null(), src_data.len() as usize, T::IID, &mut _root_signature_deserializer, );
		if ret.is_ok() {
			if let (Some(_root_signature_deserializer)) = (_root_signature_deserializer) {
				return Ok((T::from(_root_signature_deserializer)));
			}
		}
		Err(ret)
	}
}

pub fn D3D12SerializeVersionedRootSignature(root_signature: &D3D12VersionedRootSignatureDesc, mut error_blob: Option<&mut Option<D3DBlob>>,) -> Result<(D3DBlob), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12SerializeVersionedRootSignature(root_signature: &D3D12VersionedRootSignatureDesc, _blob: &mut Option<D3DBlob>, error_blob: Option<&mut Option<D3DBlob>>, ) -> HResult;
		}
		let mut _blob: Option<D3DBlob> = None;
		if let Some(ref mut error_blob) = error_blob { **error_blob = None; }
		let ret = D3D12SerializeVersionedRootSignature(root_signature, &mut _blob, error_blob, );
		if ret.is_ok() {
			if let (Some(_blob)) = (_blob) {
				return Ok((_blob));
			}
		}
		Err(ret)
	}
}

pub fn D3D12CreateVersionedRootSignatureDeserializer<T: IUnknown>(src_data: &[u8], ) -> Result<(T), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateVersionedRootSignatureDeserializer(src_data: *const u8, src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, _root_signature_deserializer: &mut Option<Unknown>, ) -> HResult;
		}
		let mut _root_signature_deserializer: Option<Unknown> = None;
		let ret = D3D12CreateVersionedRootSignatureDeserializer(src_data.as_ptr_or_null(), src_data.len() as usize, T::IID, &mut _root_signature_deserializer, );
		if ret.is_ok() {
			if let (Some(_root_signature_deserializer)) = (_root_signature_deserializer) {
				return Ok((T::from(_root_signature_deserializer)));
			}
		}
		Err(ret)
	}
}

pub fn D3D12CreateDevice<T: IUnknown>(adapter: Option<&Unknown>, minimum_feature_level: D3DFeatureLevel, device: Option<&mut Option<T>>, ) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12CreateDevice(adapter: Option<VTable>, minimum_feature_level: D3DFeatureLevel, riid: &GUID, device: Option<&mut Option<Unknown>>, ) -> HResult;
		}
		let mut out_device: Option<Unknown> = None;
		let ret = D3D12CreateDevice(option_to_vtable(adapter), minimum_feature_level, T::IID, if device.is_some() { Some(&mut out_device) } else { None }, );
		if let (Some(device), Some(out_device)) = (device, out_device) { *device = Some(T::from(out_device)); }
		ret.ok()
	}
}

pub fn D3D12GetDebugInterface<T: IUnknown>(debug: Option<&mut Option<T>>, ) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetDebugInterface(riid: &GUID, debug: Option<&mut Option<Unknown>>, ) -> HResult;
		}
		let mut out_debug: Option<Unknown> = None;
		let ret = D3D12GetDebugInterface(T::IID, if debug.is_some() { Some(&mut out_debug) } else { None }, );
		if let (Some(debug), Some(out_debug)) = (debug, out_debug) { *debug = Some(T::from(out_debug)); }
		ret.ok()
	}
}

pub fn D3D12EnableExperimentalFeatures(ii_ds: &[GUID], configuration_structs: Option<&[u8]>, configuration_struct_sizes: Option<&[u32]>, ) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12EnableExperimentalFeatures(num_features: u32, ii_ds: *const GUID, configuration_structs: *const u8, configuration_struct_sizes: *const u32, ) -> HResult;
		}
		let ret = D3D12EnableExperimentalFeatures(ii_ds.len() as u32, ii_ds.as_ptr_or_null(), configuration_structs.as_ptr_or_null(), configuration_struct_sizes.as_ptr_or_null(), );
		ret.ok()
	}
}

pub fn D3D12GetInterface<T: IUnknown>(rclsid: &GUID, debug: Option<&mut Option<T>>, ) -> Result<(), HResult> {
	unsafe {
		#[link(name = "d3d12")]
		extern "system" {
			fn D3D12GetInterface(rclsid: &GUID, riid: &GUID, debug: Option<&mut Option<Unknown>>, ) -> HResult;
		}
		let mut out_debug: Option<Unknown> = None;
		let ret = D3D12GetInterface(rclsid, T::IID, if debug.is_some() { Some(&mut out_debug) } else { None }, );
		if let (Some(debug), Some(out_debug)) = (debug, out_debug) { *debug = Some(T::from(out_debug)); }
		ret.ok()
	}
}


pub type PFnD3D12SerializeRootSignature 
	= unsafe extern "system" fn(root_signature: &D3D12RootSignatureDesc, version: D3DRootSignatureVersion, blob: &mut D3DBlob, error_blob: Option<&mut D3DBlob>, ) -> HResult;

pub type PFnD3D12CreateRootSignatureDeserializer 
	= unsafe extern "system" fn(src_data: &(), src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, root_signature_deserializer: &mut *const c_void, ) -> HResult;

pub type PFnD3D12SerializeVersionedRootSignature 
	= unsafe extern "system" fn(root_signature: &D3D12VersionedRootSignatureDesc, blob: &mut D3DBlob, error_blob: Option<&mut D3DBlob>, ) -> HResult;

pub type PFnD3D12CreateVersionedRootSignatureDeserializer 
	= unsafe extern "system" fn(src_data: &(), src_data_size_in_bytes: usize, root_signature_deserializer_interface: &GUID, root_signature_deserializer: &mut *const c_void, ) -> HResult;

pub type D3D12MessageFunc 
	= unsafe extern "system" fn(category: D3D12MessageCategory, severity: D3D12MessageSeverity, id: D3D12MessageId, description: PStr, context: &mut (), ) -> ();

pub type PFnD3D12CreateDevice 
	= unsafe extern "system" fn(param0: Unknown, param1: D3DFeatureLevel, param2: &GUID, param3: Option<&mut *const c_void>, ) -> HResult;

pub type PFnD3D12GetDebugInterface 
	= unsafe extern "system" fn(param0: &GUID, param1: Option<&mut *const c_void>, ) -> HResult;

pub type PFnD3D12GetInterface 
	= unsafe extern "system" fn(param0: &GUID, param1: &GUID, param2: Option<&mut *const c_void>, ) -> HResult;

