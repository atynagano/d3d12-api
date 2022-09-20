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
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12CommandQueue(pub(crate) D3D12Pageable);

impl Deref for D3D12CommandQueue {
	type Target = D3D12Pageable;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12CommandQueue {
	const IID: &'static GUID = &GUID::from_u128(0x0ec870a65d7e4c228cfc5baae07616edu128);
}

impl Com for D3D12CommandQueue {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12CommandQueue {
	pub unsafe fn UpdateTileMappings() { todo!() }

	pub fn CopyTileMappings(&self, dst_resource: &D3D12Resource, dst_region_start_coordinate: &D3D12TiledResourceCoordinate, src_resource: &D3D12Resource, src_region_start_coordinate: &D3D12TiledResourceCoordinate, region_size: &D3D12TileRegionSize, flags: D3D12TileMappingFlags) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, &D3D12TiledResourceCoordinate, VTable, &D3D12TiledResourceCoordinate, &D3D12TileRegionSize, D3D12TileMappingFlags) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, dst_resource.vtable(), dst_region_start_coordinate, src_resource.vtable(), src_region_start_coordinate, region_size, flags);
		}
	}

	pub fn ExecuteCommandLists(&self, command_lists: &[Param<D3D12CommandList>]) -> () {
		unsafe {
			let vt = self.as_param();
			let (command_lists_ptr_, command_lists_len_) = command_lists.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<D3D12CommandList>) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, command_lists_len_ as u32, command_lists_ptr_);
		}
	}

	pub fn SetMarker(&self, metadata: u32, data: Option<&[u8]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const u8, u32) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, metadata, data_ptr_, data_len_ as u32);
		}
	}

	pub fn BeginEvent(&self, metadata: u32, data: Option<&[u8]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const u8, u32) -> ()
				= transmute(vt[12]);
			let _ret_ = f(vt, metadata, data_ptr_, data_len_ as u32);
		}
	}

	pub fn EndEvent(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt);
		}
	}

	pub fn Signal(&self, fence: &D3D12Fence, value: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u64) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, fence.vtable(), value);
			_ret_.ok()
		}
	}

	pub fn Wait(&self, fence: &D3D12Fence, value: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u64) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, fence.vtable(), value);
			_ret_.ok()
		}
	}

	pub fn GetTimestampFrequency(&self) -> Result<u64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut frequency_out_: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u64) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, frequency_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(frequency_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetClockCalibration(&self) -> Result<(u64, u64), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut gpu_timestamp_out_: MaybeUninit<u64> = MaybeUninit::zeroed();
			let mut cpu_timestamp_out_: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u64, *mut u64) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, gpu_timestamp_out_.as_mut_ptr(), cpu_timestamp_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((gpu_timestamp_out_.assume_init(), cpu_timestamp_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetDesc(&self) -> D3D12CommandQueueDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12CommandQueueDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12CommandQueueDesc) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, _out_.as_mut_ptr());
			_out_.assume_init()
		}
	}
}

pub trait ID3D12CommandQueue: ID3D12Pageable {
	fn as_command_queue(&self) -> &D3D12CommandQueue;
	fn into_command_queue(self) -> D3D12CommandQueue;
}

impl ID3D12CommandQueue for D3D12CommandQueue {
	fn as_command_queue(&self) -> &D3D12CommandQueue { self }
	fn into_command_queue(self) -> D3D12CommandQueue { self }
}
impl ID3D12Pageable for D3D12CommandQueue {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12CommandQueue {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12CommandQueue {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12CommandQueue {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12CommandQueue {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

