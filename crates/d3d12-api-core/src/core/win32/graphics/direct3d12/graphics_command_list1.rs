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
use crate::core::win32::graphics::dxgi::common::*;

#[repr(C)]
pub struct D3D12GraphicsCommandList1(pub(crate) D3D12GraphicsCommandList);

impl Guid for D3D12GraphicsCommandList1 {
	const IID: &'static GUID = &GUID::from_u128(0x553103fb1fe74557bb38946d7d0e7ca7u128);
}

impl Clone for D3D12GraphicsCommandList1 {
	fn clone(&self) -> Self { D3D12GraphicsCommandList1(self.0.clone()) }
}

pub trait ID3D12GraphicsCommandList1: ID3D12GraphicsCommandList {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1;
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1;

	fn OMSetDepthBounds(&self, min: f32, max: f32, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, min: f32, max: f32, ) -> ()
				= transmute(vt[62]);
			let _ret_ = f(vt, min, max, );
		}
	}

	fn SetSamplePositions(&self, num_samples_per_pixel: u32, num_pixels: u32, sample_positions: &D3D12SamplePosition, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, num_samples_per_pixel: u32, num_pixels: u32, sample_positions: &D3D12SamplePosition, ) -> ()
				= transmute(vt[63]);
			let _ret_ = f(vt, num_samples_per_pixel, num_pixels, sample_positions, );
		}
	}

	fn ResolveSubresourceRegion(&self, dst_resource: &(impl ID3D12Resource + ?Sized), dst_subresource: u32, dst_x: u32, dst_y: u32, src_resource: &(impl ID3D12Resource + ?Sized), src_subresource: u32, src_rect: Option<&Rect>, format: DxgiFormat, resolve_mode: D3D12ResolveMode, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, dst_resource: VTable, dst_subresource: u32, dst_x: u32, dst_y: u32, src_resource: VTable, src_subresource: u32, src_rect: *const c_void, format: DxgiFormat, resolve_mode: D3D12ResolveMode, ) -> ()
				= transmute(vt[64]);
			let _ret_ = f(vt, dst_resource.vtable(), dst_subresource, dst_x, dst_y, src_resource.vtable(), src_subresource, transmute(src_rect), format, resolve_mode, );
		}
	}

	fn SetViewInstanceMask(&self, mask: u32, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, mask: u32, ) -> ()
				= transmute(vt[65]);
			let _ret_ = f(vt, mask, );
		}
	}
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList1 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { self }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList1 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0 }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0 }
}

impl ID3D12CommandList for D3D12GraphicsCommandList1 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.0 }
	fn into_command_list(self) -> D3D12CommandList { self.0.0 }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0 }
}

impl ID3D12Object for D3D12GraphicsCommandList1 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0 }
}

impl From<Unknown> for D3D12GraphicsCommandList1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12GraphicsCommandList::from(v))
    }
}

impl IUnknown for D3D12GraphicsCommandList1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

