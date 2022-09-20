#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::direct2d::common::*;

/// D2D_COLOR_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DColorF {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

/// D2D1_PIXEL_FORMAT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1PixelFormat {
	pub format: DxgiFormat,
	pub alpha_mode: D2D1AlphaMode,
}

/// D2D_POINT_2U
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DPoint2U {
	pub x: u32,
	pub y: u32,
}

/// D2D_POINT_2F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DPoint2F {
	pub x: f32,
	pub y: f32,
}

/// D2D_VECTOR_2F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DVector2F {
	pub x: f32,
	pub y: f32,
}

/// D2D_VECTOR_3F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DVector3F {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

/// D2D_VECTOR_4F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DVector4F {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

/// D2D_RECT_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DRectF {
	pub left: f32,
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
}

/// D2D_RECT_U
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DRectU {
	pub left: u32,
	pub top: u32,
	pub right: u32,
	pub bottom: u32,
}

/// D2D_SIZE_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DSizeF {
	pub width: f32,
	pub height: f32,
}

/// D2D_SIZE_U
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DSizeU {
	pub width: u32,
	pub height: u32,
}

/// _Anonymous1_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix3x2FAnonymousUnionAnonymous1Struct {
	pub m11: f32,
	pub m12: f32,
	pub m21: f32,
	pub m22: f32,
	pub dx: f32,
	pub dy: f32,
}

/// _Anonymous2_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix3x2FAnonymousUnionAnonymous2Struct {
	pub _11: f32,
	pub _12: f32,
	pub _21: f32,
	pub _22: f32,
	pub _31: f32,
	pub _32: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D2DMatrix3x2FAnonymousUnion {
	pub anonymous1: D2DMatrix3x2FAnonymousUnionAnonymous1Struct,
	pub anonymous2: D2DMatrix3x2FAnonymousUnionAnonymous2Struct,
	pub m: [f32; 6],
}

impl std::fmt::Debug for D2DMatrix3x2FAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// D2D_MATRIX_3X2_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix3x2F {
	pub anonymous: D2DMatrix3x2FAnonymousUnion,
}

/// _Anonymous_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix4x3FAnonymousUnionAnonymousStruct {
	pub _11: f32,
	pub _12: f32,
	pub _13: f32,
	pub _21: f32,
	pub _22: f32,
	pub _23: f32,
	pub _31: f32,
	pub _32: f32,
	pub _33: f32,
	pub _41: f32,
	pub _42: f32,
	pub _43: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D2DMatrix4x3FAnonymousUnion {
	pub anonymous: D2DMatrix4x3FAnonymousUnionAnonymousStruct,
	pub m: [f32; 12],
}

impl std::fmt::Debug for D2DMatrix4x3FAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// D2D_MATRIX_4X3_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix4x3F {
	pub anonymous: D2DMatrix4x3FAnonymousUnion,
}

/// _Anonymous_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix4x4FAnonymousUnionAnonymousStruct {
	pub _11: f32,
	pub _12: f32,
	pub _13: f32,
	pub _14: f32,
	pub _21: f32,
	pub _22: f32,
	pub _23: f32,
	pub _24: f32,
	pub _31: f32,
	pub _32: f32,
	pub _33: f32,
	pub _34: f32,
	pub _41: f32,
	pub _42: f32,
	pub _43: f32,
	pub _44: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D2DMatrix4x4FAnonymousUnion {
	pub anonymous: D2DMatrix4x4FAnonymousUnionAnonymousStruct,
	pub m: [f32; 16],
}

impl std::fmt::Debug for D2DMatrix4x4FAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// D2D_MATRIX_4X4_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix4x4F {
	pub anonymous: D2DMatrix4x4FAnonymousUnion,
}

/// _Anonymous_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix5x4FAnonymousUnionAnonymousStruct {
	pub _11: f32,
	pub _12: f32,
	pub _13: f32,
	pub _14: f32,
	pub _21: f32,
	pub _22: f32,
	pub _23: f32,
	pub _24: f32,
	pub _31: f32,
	pub _32: f32,
	pub _33: f32,
	pub _34: f32,
	pub _41: f32,
	pub _42: f32,
	pub _43: f32,
	pub _44: f32,
	pub _51: f32,
	pub _52: f32,
	pub _53: f32,
	pub _54: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D2DMatrix5x4FAnonymousUnion {
	pub anonymous: D2DMatrix5x4FAnonymousUnionAnonymousStruct,
	pub m: [f32; 20],
}

impl std::fmt::Debug for D2DMatrix5x4FAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// D2D_MATRIX_5X4_F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2DMatrix5x4F {
	pub anonymous: D2DMatrix5x4FAnonymousUnion,
}

/// D2D1_BEZIER_SEGMENT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1BezierSegment {
	pub point1: D2DPoint2F,
	pub point2: D2DPoint2F,
	pub point3: D2DPoint2F,
}

