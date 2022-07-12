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

use crate::core::win32::graphics::direct3d12::*;
use crate::core::win32::foundation::*;

#[repr(C)]
pub struct D3D12CommandQueue(pub(crate) D3D12Pageable);

impl Guid for D3D12CommandQueue {
	const IID: &'static GUID = &GUID::from_u128(0x0ec870a65d7e4c228cfc5baae07616edu128);
}

impl Clone for D3D12CommandQueue {
	fn clone(&self) -> Self { D3D12CommandQueue(self.0.clone()) }
}

pub trait ID3D12CommandQueue: ID3D12Pageable {
	fn as_command_queue(&self) -> &D3D12CommandQueue;
	fn into_command_queue(self) -> D3D12CommandQueue;

	fn CopyTileMappings(&self, dst_resource: &(impl ID3D12Resource + ?Sized), dst_region_start_coordinate: &D3D12TiledResourceCoordinate, src_resource: &(impl ID3D12Resource + ?Sized), src_region_start_coordinate: &D3D12TiledResourceCoordinate, region_size: &D3D12TileRegionSize, flags: D3D12TileMappingFlags, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, dst_resource: VTable, dst_region_start_coordinate: &D3D12TiledResourceCoordinate, src_resource: VTable, src_region_start_coordinate: &D3D12TiledResourceCoordinate, region_size: &D3D12TileRegionSize, flags: D3D12TileMappingFlags, ) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, dst_resource.vtable(), dst_region_start_coordinate, src_resource.vtable(), src_region_start_coordinate, region_size, flags, );
		}
	}

	fn ExecuteCommandLists(&self, command_lists: &[Param<D3D12CommandList>], ) -> () {
		unsafe {
			let vt = self.as_param();
			let (_ptr_command_lists, _len_command_lists) = command_lists.deconstruct();
			let f: extern "system" fn(Param<Self>, num_command_lists: u32, command_lists: *const Param<D3D12CommandList>, ) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, _len_command_lists as u32, _ptr_command_lists, );
		}
	}

	fn EndEvent(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, );
		}
	}

	fn Signal(&self, fence: &(impl ID3D12Fence + ?Sized), value: u64, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, fence: VTable, value: u64, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, fence.vtable(), value, );
			_ret_.ok()
		}
	}

	fn Wait(&self, fence: &(impl ID3D12Fence + ?Sized), value: u64, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, fence: VTable, value: u64, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, fence.vtable(), value, );
			_ret_.ok()
		}
	}

	fn GetTimestampFrequency(&self, ) -> Result<u64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_frequency: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_frequency: *mut u64, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, _out_frequency.as_mut_ptr(), );
			Ok(_out_frequency.assume_init())
		}
	}

	fn GetClockCalibration(&self, ) -> Result<(u64, u64, ), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_gpu_timestamp: MaybeUninit<u64> = MaybeUninit::zeroed();
			let mut _out_cpu_timestamp: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_gpu_timestamp: *mut u64, _out_cpu_timestamp: *mut u64, ) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, _out_gpu_timestamp.as_mut_ptr(), _out_cpu_timestamp.as_mut_ptr(), );
			if _ret_.is_ok() {
				return Ok((_out_gpu_timestamp.assume_init(), _out_cpu_timestamp.assume_init(), ));
			}
			Err(_ret_)
		}
	}

	fn GetDesc(&self, ) -> D3D12CommandQueueDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out__out_desc: MaybeUninit<D3D12CommandQueueDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out__out_desc: *mut D3D12CommandQueueDesc, ) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, _out__out_desc.as_mut_ptr(), );
			_out__out_desc.assume_init()
		}
	}
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

impl From<Unknown> for D3D12CommandQueue {
    fn from(v: Unknown) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

impl IUnknown for D3D12CommandQueue {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

