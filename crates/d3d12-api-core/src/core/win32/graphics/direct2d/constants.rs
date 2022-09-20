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

use std::ops::{BitOr, BitOrAssign};

pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25f32;
pub const D2D1_SCENE_REFERRED_SDR_WHITE_LEVEL: f32 = 80f32;
pub const D2D1_APPEND_ALIGNED_ELEMENT: u32 = 0xFFFFFFFFu32;
pub const FACILITY_D2D: u32 = 0x899u32;
pub const CLSID_D2D12DAffineTransform: GUID = GUID::from_u128(0x6aa9748563544cfc908ce4a74f62c96cu128);
pub const CLSID_D2D13DPerspectiveTransform: GUID = GUID::from_u128(0xc2844d0b3d8646e785ba526c9240f3fbu128);
pub const CLSID_D2D13DTransform: GUID = GUID::from_u128(0xe8467b04ec614b8ab5ded4d73debea5au128);
pub const CLSID_D2D1ArithmeticComposite: GUID = GUID::from_u128(0xfc151437049a4784a24af1c4daf20987u128);
pub const CLSID_D2D1Atlas: GUID = GUID::from_u128(0x913e2be4fdcf4fe2a5f02454f14ff408u128);
pub const CLSID_D2D1BitmapSource: GUID = GUID::from_u128(0x5fb6c24dc6dd4231940450f4d5c3252du128);
pub const CLSID_D2D1Blend: GUID = GUID::from_u128(0x81c5b77b13f84cddad20c890547ac65du128);
pub const CLSID_D2D1Border: GUID = GUID::from_u128(0x2a2d49c04acf43c78c6a7c4a27874d27u128);
pub const CLSID_D2D1Brightness: GUID = GUID::from_u128(0x8cea8d1e77b04986b3b92f0c0eae7887u128);
pub const CLSID_D2D1ColorManagement: GUID = GUID::from_u128(0x1a28524cfdd64aa4ae8f837eb8267b37u128);
pub const CLSID_D2D1ColorMatrix: GUID = GUID::from_u128(0x921f03d6641c47df852db4bb6153ae11u128);
pub const CLSID_D2D1Composite: GUID = GUID::from_u128(0x48fc9f51f6ac48f18b583b28ac46f76du128);
pub const CLSID_D2D1ConvolveMatrix: GUID = GUID::from_u128(0x407f8c0855334331a34123cc3877843eu128);
pub const CLSID_D2D1Crop: GUID = GUID::from_u128(0xe23f71100e9a4324af476a2c0c46f35bu128);
pub const CLSID_D2D1DirectionalBlur: GUID = GUID::from_u128(0x174319a658e949b2bb63caf2c811a3dbu128);
pub const CLSID_D2D1DiscreteTransfer: GUID = GUID::from_u128(0x90866fcd488e454baf06e5041b66c36cu128);
pub const CLSID_D2D1DisplacementMap: GUID = GUID::from_u128(0xedc4836404174111945043845fa9f890u128);
pub const CLSID_D2D1DistantDiffuse: GUID = GUID::from_u128(0x3e7efd62a32d46d4a83c5278889ac954u128);
pub const CLSID_D2D1DistantSpecular: GUID = GUID::from_u128(0x428c1ee577b844508ab572219c21abdau128);
pub const CLSID_D2D1DpiCompensation: GUID = GUID::from_u128(0x6c26c5c734e046fc9cfde5823706e228u128);
pub const CLSID_D2D1Flood: GUID = GUID::from_u128(0x61c23c20ae694d8e94cf50078df638f2u128);
pub const CLSID_D2D1GammaTransfer: GUID = GUID::from_u128(0x409444c4c41941a0b0c18cd0c0a18e42u128);
pub const CLSID_D2D1GaussianBlur: GUID = GUID::from_u128(0x1feb6d692fe64ac98c581d7f93e7a6a5u128);
pub const CLSID_D2D1Scale: GUID = GUID::from_u128(0x9daf936938464d0ea44e0c607934a5d7u128);
pub const CLSID_D2D1Histogram: GUID = GUID::from_u128(0x881db7d0f7ee4d4da6d24697acc66ee8u128);
pub const CLSID_D2D1HueRotation: GUID = GUID::from_u128(0x0f4458ec4b32491b9e85bd73f44d3eb6u128);
pub const CLSID_D2D1LinearTransfer: GUID = GUID::from_u128(0xad47c8fd63ef4acc9b5167979c036c06u128);
pub const CLSID_D2D1LuminanceToAlpha: GUID = GUID::from_u128(0x41251ab70beb46f89da759e93fcce5deu128);
pub const CLSID_D2D1Morphology: GUID = GUID::from_u128(0xeae6c40d626a4c2dbfcb391001abe202u128);
pub const CLSID_D2D1OpacityMetadata: GUID = GUID::from_u128(0x6c53006a44504199aa5bad1656fece5eu128);
pub const CLSID_D2D1PointDiffuse: GUID = GUID::from_u128(0xb9e303c3c08c4f918b7b38656bc48c20u128);
pub const CLSID_D2D1PointSpecular: GUID = GUID::from_u128(0x09c3ca263ae24f099ebced3865d53f22u128);
pub const CLSID_D2D1Premultiply: GUID = GUID::from_u128(0x06eab419deed401880d23e1d471adeb2u128);
pub const CLSID_D2D1Saturation: GUID = GUID::from_u128(0x5cb2d9cf327d459fa0ce40c0b2086bf7u128);
pub const CLSID_D2D1Shadow: GUID = GUID::from_u128(0xc67ea36118634e6989db695d3e9a5b6bu128);
pub const CLSID_D2D1SpotDiffuse: GUID = GUID::from_u128(0x818a1105793244f4aa8608ae7b2f2c93u128);
pub const CLSID_D2D1SpotSpecular: GUID = GUID::from_u128(0xedae421e76544a379db871acc1beb3c1u128);
pub const CLSID_D2D1TableTransfer: GUID = GUID::from_u128(0x5bf818c35e4348cbb631868396d6a1d4u128);
pub const CLSID_D2D1Tile: GUID = GUID::from_u128(0xb07841383b764bc5b13b0fa2ad02659fu128);
pub const CLSID_D2D1Turbulence: GUID = GUID::from_u128(0xcf2bb6ae889a4ad7ba29a2fd732c9fc9u128);
pub const CLSID_D2D1UnPremultiply: GUID = GUID::from_u128(0xfb9ac489ad8d41ed9999bb6347d110f7u128);
pub const CLSID_D2D1YCbCr: GUID = GUID::from_u128(0x99503cc166c745c9a8758ad8a7914401u128);
pub const CLSID_D2D1Contrast: GUID = GUID::from_u128(0xb648a78a0ed54f80a94a8e825aca6b77u128);
pub const CLSID_D2D1RgbToHue: GUID = GUID::from_u128(0x23f3e5ec91e84d3dad0aafadc1004aa1u128);
pub const CLSID_D2D1HueToRgb: GUID = GUID::from_u128(0x7b78a6bd01414def8a526356ee0cbdd5u128);
pub const CLSID_D2D1ChromaKey: GUID = GUID::from_u128(0x74c01f5b2a0d408c88e2c7a3c7197742u128);
pub const CLSID_D2D1Emboss: GUID = GUID::from_u128(0xb1c5eb2b034843f081074957cacba2aeu128);
pub const CLSID_D2D1Exposure: GUID = GUID::from_u128(0xb56c8cfaf63441eebee0ffa617106004u128);
pub const CLSID_D2D1Grayscale: GUID = GUID::from_u128(0x36dde0eb372542e0836d52fb20aee644u128);
pub const CLSID_D2D1Invert: GUID = GUID::from_u128(0xe0c3784dcb394e84b6fd6b72f0810263u128);
pub const CLSID_D2D1Posterize: GUID = GUID::from_u128(0x2188945e33a34366b7bc086bd02d0884u128);
pub const CLSID_D2D1Sepia: GUID = GUID::from_u128(0x3a1af4105f1d4dbe84df915da79b7153u128);
pub const CLSID_D2D1Sharpen: GUID = GUID::from_u128(0xc9b887cbc5ff4dc59779273dcf417c7du128);
pub const CLSID_D2D1Straighten: GUID = GUID::from_u128(0x4da47b1279a34fb08237bbc3b2a4de08u128);
pub const CLSID_D2D1TemperatureTint: GUID = GUID::from_u128(0x891760878af94a08aeb1895f38db1766u128);
pub const CLSID_D2D1Vignette: GUID = GUID::from_u128(0xc00c40be5e674ca395b4f4b02c115135u128);
pub const CLSID_D2D1EdgeDetection: GUID = GUID::from_u128(0xeff583cacb074aa9ac5d2cc44c76460fu128);
pub const CLSID_D2D1HighlightsShadows: GUID = GUID::from_u128(0xcadc8384323f4c7ea3612e2b24df6ee4u128);
pub const CLSID_D2D1LookupTable3D: GUID = GUID::from_u128(0x349e0eda00884a799ca3c7e300202020u128);
pub const CLSID_D2D1Opacity: GUID = GUID::from_u128(0x811d79a4de2844548094c64685f8bd4cu128);
pub const CLSID_D2D1AlphaMask: GUID = GUID::from_u128(0xc80ecff03fd54f058328c5d1724b4f0au128);
pub const CLSID_D2D1CrossFade: GUID = GUID::from_u128(0x12f575e84db1485f9a8403a07dd3829fu128);
pub const CLSID_D2D1Tint: GUID = GUID::from_u128(0x36312b17f7dd4014915dffca768cf211u128);
pub const CLSID_D2D1WhiteLevelAdjustment: GUID = GUID::from_u128(0x44a1cadb6cdd48188ff426c1cfe95bdbu128);
pub const CLSID_D2D1HdrToneMap: GUID = GUID::from_u128(0x7b0b748d46104486a90c999d9a2e2b11u128);

