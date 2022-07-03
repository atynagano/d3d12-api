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

	fn UpdateTileMappings(&self, resource: &(impl ID3D12Resource + ?Sized), resource_region_start_coordinates: Option<&[D3D12TiledResourceCoordinate]>, resource_region_sizes: Option<&[D3D12TileRegionSize]>, heap: Option<&D3D12Heap>, range_flags: Option<&[D3D12TileRangeFlags]>, heap_range_start_offsets: Option<&[u32]>, range_tile_counts: Option<&[u32]>, flags: D3D12TileMappingFlags, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource: VTable, num_resource_regions: u32, resource_region_start_coordinates: *const D3D12TiledResourceCoordinate, resource_region_sizes: *const D3D12TileRegionSize, heap: Option<VTable>, num_ranges: u32, range_flags: *const D3D12TileRangeFlags, heap_range_start_offsets: *const u32, range_tile_counts: *const u32, flags: D3D12TileMappingFlags, ) -> ()
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, resource.vtable(), resource_region_start_coordinates.len() as u32, resource_region_start_coordinates.as_ptr_or_null(), resource_region_sizes.as_ptr_or_null(), option_to_vtable(heap), range_flags.len() as u32, range_flags.as_ptr_or_null(), heap_range_start_offsets.as_ptr_or_null(), range_tile_counts.as_ptr_or_null(), flags, );
	}

	fn CopyTileMappings(&self, dst_resource: &(impl ID3D12Resource + ?Sized), dst_region_start_coordinate: &D3D12TiledResourceCoordinate, src_resource: &(impl ID3D12Resource + ?Sized), src_region_start_coordinate: &D3D12TiledResourceCoordinate, region_size: &D3D12TileRegionSize, flags: D3D12TileMappingFlags, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dst_resource: VTable, dst_region_start_coordinate: &D3D12TiledResourceCoordinate, src_resource: VTable, src_region_start_coordinate: &D3D12TiledResourceCoordinate, region_size: &D3D12TileRegionSize, flags: D3D12TileMappingFlags, ) -> ()
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, dst_resource.vtable(), dst_region_start_coordinate, src_resource.vtable(), src_region_start_coordinate, region_size, flags, );
	}

	fn ExecuteCommandLists(&self, command_lists: &[Param<D3D12CommandList>], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_command_lists: u32, command_lists: *const Param<D3D12CommandList>, ) -> ()
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, command_lists.len() as u32, command_lists.as_ptr_or_null(), );
	}

	fn SetMarker(&self, metadata: u32, data: Option<&[u8]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, metadata: u32, data: *const u8, size: u32, ) -> ()
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, metadata, data.as_ptr_or_null(), data.len() as u32, );
	}

	fn BeginEvent(&self, metadata: u32, data: Option<&[u8]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, metadata: u32, data: *const u8, size: u32, ) -> ()
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, metadata, data.as_ptr_or_null(), data.len() as u32, );
	}

	fn EndEvent(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, );
	}

	fn Signal(&self, fence: &(impl ID3D12Fence + ?Sized), value: u64, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, fence: VTable, value: u64, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, fence.vtable(), value, );
		ret.ok()
	}

	fn Wait(&self, fence: &(impl ID3D12Fence + ?Sized), value: u64, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, fence: VTable, value: u64, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, fence.vtable(), value, );
		ret.ok()
	}

	fn GetTimestampFrequency(&self, ) -> Result<(u64), HResult> {
		let vt = self.as_param();
		let mut _frequency: u64 = u64::zeroed();
		let f: extern "system" fn(Param<Self>, _frequency: &mut u64, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, &mut _frequency, );
		if ret.is_ok() {
			return Ok((_frequency));
		}
		Err(ret)
	}

	fn GetClockCalibration(&self, ) -> Result<(u64, u64), HResult> {
		let vt = self.as_param();
		let mut _gpu_timestamp: u64 = u64::zeroed();
		let mut _cpu_timestamp: u64 = u64::zeroed();
		let f: extern "system" fn(Param<Self>, _gpu_timestamp: &mut u64, _cpu_timestamp: &mut u64, ) -> HResult
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, &mut _gpu_timestamp, &mut _cpu_timestamp, );
		if ret.is_ok() {
			return Ok((_gpu_timestamp, _cpu_timestamp));
		}
		Err(ret)
	}

	fn GetDesc(&self, ) -> (D3D12CommandQueueDesc) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> D3D12CommandQueueDesc
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3D12CommandQueue for D3D12CommandQueue {
	fn as_command_queue(&self) -> &D3D12CommandQueue { self }
	fn into_command_queue(self) -> D3D12CommandQueue { self }
}

impl ID3D12Pageable for D3D12CommandQueue {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0 }
}

impl ID3D12DeviceChild for D3D12CommandQueue {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12CommandQueue {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12CommandQueue {
    fn from(v: Unknown) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

impl IUnknown for D3D12CommandQueue {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

