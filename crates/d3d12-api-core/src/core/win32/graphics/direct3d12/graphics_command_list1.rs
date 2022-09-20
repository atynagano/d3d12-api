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
use crate::core::win32::graphics::dxgi::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12GraphicsCommandList1(pub(crate) D3D12GraphicsCommandList);

impl Deref for D3D12GraphicsCommandList1 {
	type Target = D3D12GraphicsCommandList;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12GraphicsCommandList1 {
	const IID: &'static GUID = &GUID::from_u128(0x553103fb1fe74557bb38946d7d0e7ca7u128);
}

impl Com for D3D12GraphicsCommandList1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12GraphicsCommandList1 {
	pub unsafe fn AtomicCopyBufferUINT() { todo!() }

	pub unsafe fn AtomicCopyBufferUINT64() { todo!() }

	pub fn OMSetDepthBounds(&self, min: f32, max: f32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32, f32) -> ()
				= transmute(vt[62]);
			let _ret_ = f(vt, min, max);
		}
	}

	pub fn SetSamplePositions(&self, num_samples_per_pixel: u32, num_pixels: u32, sample_positions: &D3D12SamplePosition) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, &D3D12SamplePosition) -> ()
				= transmute(vt[63]);
			let _ret_ = f(vt, num_samples_per_pixel, num_pixels, sample_positions);
		}
	}

	pub fn ResolveSubresourceRegion(&self, dst_resource: &D3D12Resource, dst_subresource: u32, dst_x: u32, dst_y: u32, src_resource: &D3D12Resource, src_subresource: u32, src_rect: Option<&Rect>, format: DxgiFormat, resolve_mode: D3D12ResolveMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, u32, u32, VTable, u32, *const c_void, DxgiFormat, D3D12ResolveMode) -> ()
				= transmute(vt[64]);
			let _ret_ = f(vt, dst_resource.vtable(), dst_subresource, dst_x, dst_y, src_resource.vtable(), src_subresource, transmute(src_rect), format, resolve_mode);
		}
	}

	pub fn SetViewInstanceMask(&self, mask: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[65]);
			let _ret_ = f(vt, mask);
		}
	}
}

pub trait ID3D12GraphicsCommandList1: ID3D12GraphicsCommandList {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1;
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1;
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList1 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { self }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self }
}
impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList1 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.as_graphics_command_list() }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.into_graphics_command_list() }
}

impl ID3D12CommandList for D3D12GraphicsCommandList1 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.as_command_list() }
	fn into_command_list(self) -> D3D12CommandList { self.0.into_command_list() }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12GraphicsCommandList1 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12GraphicsCommandList1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12GraphicsCommandList1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12GraphicsCommandList::from(v))
    }
}

