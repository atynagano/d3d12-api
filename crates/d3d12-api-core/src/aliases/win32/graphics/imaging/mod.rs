#![allow(unused_imports)]

use crate::core::win32::graphics::imaging::*;

pub type BitmapSource = WICBitmapSource;
pub trait IBitmapSource: IWICBitmapSource {}
impl<T: IWICBitmapSource> IBitmapSource for T {}
pub type Bitmap = WICBitmap;
pub trait IBitmap: IWICBitmap {}
impl<T: IWICBitmap> IBitmap for T {}
pub type ColorContext = WICColorContext;
pub trait IColorContext: IWICColorContext {}
impl<T: IWICColorContext> IColorContext for T {}
pub type ImagingFactory = WICImagingFactory;
pub trait IImagingFactory: IWICImagingFactory {}
impl<T: IWICImagingFactory> IImagingFactory for T {}
pub type Palette = WICPalette;
pub trait IPalette: IWICPalette {}
impl<T: IWICPalette> IPalette for T {}
pub type Rect = WicRect;
pub type BitmapLock = WICBitmapLock;
pub trait IBitmapLock: IWICBitmapLock {}
impl<T: IWICBitmapLock> IBitmapLock for T {}
pub type ColorContextType = WicColorContextType;
pub type DecodeOptions = WicDecodeOptions;
pub type BitmapDecoder = WICBitmapDecoder;
pub trait IBitmapDecoder: IWICBitmapDecoder {}
impl<T: IWICBitmapDecoder> IBitmapDecoder for T {}
pub type ComponentInfo = WICComponentInfo;
pub trait IComponentInfo: IWICComponentInfo {}
impl<T: IWICComponentInfo> IComponentInfo for T {}
pub type BitmapEncoder = WICBitmapEncoder;
pub trait IBitmapEncoder: IWICBitmapEncoder {}
impl<T: IWICBitmapEncoder> IBitmapEncoder for T {}
pub type FormatConverter = WICFormatConverter;
pub trait IFormatConverter: IWICFormatConverter {}
impl<T: IWICFormatConverter> IFormatConverter for T {}
pub type BitmapScaler = WICBitmapScaler;
pub trait IBitmapScaler: IWICBitmapScaler {}
impl<T: IWICBitmapScaler> IBitmapScaler for T {}
pub type BitmapClipper = WICBitmapClipper;
pub trait IBitmapClipper: IWICBitmapClipper {}
impl<T: IWICBitmapClipper> IBitmapClipper for T {}
pub type BitmapFlipRotator = WICBitmapFlipRotator;
pub trait IBitmapFlipRotator: IWICBitmapFlipRotator {}
impl<T: IWICBitmapFlipRotator> IBitmapFlipRotator for T {}
pub type Stream = WICStream;
pub trait IStream: IWICStream {}
impl<T: IWICStream> IStream for T {}
pub type ColorTransform = WICColorTransform;
pub trait IColorTransform: IWICColorTransform {}
impl<T: IWICColorTransform> IColorTransform for T {}
pub type BitmapCreateCacheOption = WicBitmapCreateCacheOption;
pub type BitmapAlphaChannelOption = WicBitmapAlphaChannelOption;
pub type FastMetadataEncoder = WICFastMetadataEncoder;
pub trait IFastMetadataEncoder: IWICFastMetadataEncoder {}
impl<T: IWICFastMetadataEncoder> IFastMetadataEncoder for T {}
pub type BitmapFrameDecode = WICBitmapFrameDecode;
pub trait IBitmapFrameDecode: IWICBitmapFrameDecode {}
impl<T: IWICBitmapFrameDecode> IBitmapFrameDecode for T {}
pub type MetadataQueryWriter = WICMetadataQueryWriter;
pub trait IMetadataQueryWriter: IWICMetadataQueryWriter {}
impl<T: IWICMetadataQueryWriter> IMetadataQueryWriter for T {}
pub type MetadataQueryReader = WICMetadataQueryReader;
pub trait IMetadataQueryReader: IWICMetadataQueryReader {}
impl<T: IWICMetadataQueryReader> IMetadataQueryReader for T {}
pub type BitmapPaletteType = WicBitmapPaletteType;
pub type BitmapDecoderInfo = WICBitmapDecoderInfo;
pub trait IBitmapDecoderInfo: IWICBitmapDecoderInfo {}
impl<T: IWICBitmapDecoderInfo> IBitmapDecoderInfo for T {}
pub type BitmapCodecInfo = WICBitmapCodecInfo;
pub trait IBitmapCodecInfo: IWICBitmapCodecInfo {}
impl<T: IWICBitmapCodecInfo> IBitmapCodecInfo for T {}
pub type ComponentType = WicComponentType;
pub type BitmapEncoderCacheOption = WicBitmapEncoderCacheOption;
pub type BitmapEncoderInfo = WICBitmapEncoderInfo;
pub trait IBitmapEncoderInfo: IWICBitmapEncoderInfo {}
impl<T: IWICBitmapEncoderInfo> IBitmapEncoderInfo for T {}
pub type BitmapFrameEncode = WICBitmapFrameEncode;
pub trait IBitmapFrameEncode: IWICBitmapFrameEncode {}
impl<T: IWICBitmapFrameEncode> IBitmapFrameEncode for T {}
pub type BitmapDitherType = WicBitmapDitherType;
pub type BitmapInterpolationMode = WicBitmapInterpolationMode;
pub type BitmapTransformOptions = WicBitmapTransformOptions;
pub type BitmapPattern<'a> = WicBitmapPattern<'a>;
