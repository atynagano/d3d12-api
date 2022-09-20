#![allow(unused_imports)]

use crate::core::win32::graphics::direct2d::common::*;

pub type ColorF = D2DColorF;
pub type AlphaMode = D2D1AlphaMode;
pub type PixelFormat = D2D1PixelFormat;
pub type Point2U = D2DPoint2U;
pub type Point2F = D2DPoint2F;
pub type Vector2F = D2DVector2F;
pub type Vector3F = D2DVector3F;
pub type Vector4F = D2DVector4F;
pub type RectF = D2DRectF;
pub type RectU = D2DRectU;
pub type SizeF = D2DSizeF;
pub type SizeU = D2DSizeU;
pub type Matrix3x2FAnonymousUnionAnonymous1Struct = D2DMatrix3x2FAnonymousUnionAnonymous1Struct;
pub type Matrix3x2FAnonymousUnionAnonymous2Struct = D2DMatrix3x2FAnonymousUnionAnonymous2Struct;
pub type Matrix3x2FAnonymousUnion = D2DMatrix3x2FAnonymousUnion;
pub type Matrix3x2F = D2DMatrix3x2F;
pub type Matrix4x3FAnonymousUnionAnonymousStruct = D2DMatrix4x3FAnonymousUnionAnonymousStruct;
pub type Matrix4x3FAnonymousUnion = D2DMatrix4x3FAnonymousUnion;
pub type Matrix4x3F = D2DMatrix4x3F;
pub type Matrix4x4FAnonymousUnionAnonymousStruct = D2DMatrix4x4FAnonymousUnionAnonymousStruct;
pub type Matrix4x4FAnonymousUnion = D2DMatrix4x4FAnonymousUnion;
pub type Matrix4x4F = D2DMatrix4x4F;
pub type Matrix5x4FAnonymousUnionAnonymousStruct = D2DMatrix5x4FAnonymousUnionAnonymousStruct;
pub type Matrix5x4FAnonymousUnion = D2DMatrix5x4FAnonymousUnion;
pub type Matrix5x4F = D2DMatrix5x4F;
pub type FigureBegin = D2D1FigureBegin;
pub type FigureEnd = D2D1FigureEnd;
pub type BezierSegment = D2D1BezierSegment;
pub type PathSegment = D2D1PathSegment;
pub type FillMode = D2D1FillMode;
pub type SimplifiedGeometrySink = D2D1SimplifiedGeometrySink;
pub trait ISimplifiedGeometrySink: ID2D1SimplifiedGeometrySink {}
impl<T: ID2D1SimplifiedGeometrySink> ISimplifiedGeometrySink for T {}
pub type BorderMode = D2D1BorderMode;
pub type BlendMode = D2D1BlendMode;
pub type ColorMatrixAlphaMode = D2D1ColorMatrixAlphaMode;
pub type _2dAffineTransformInterpolationMode = D2D1_2dAffineTransformInterpolationMode;
pub type TurbulenceNoise = D2D1TurbulenceNoise;
pub type CompositeMode = D2D1CompositeMode;
