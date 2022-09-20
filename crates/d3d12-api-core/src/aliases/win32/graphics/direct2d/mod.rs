#![allow(unused_imports)]

pub mod common;

use crate::core::win32::graphics::direct2d::*;

pub type InterpolationModeDefinition = D2D1InterpolationModeDefinition;
pub type Gamma = D2D1Gamma;
pub type OpacityMaskContent = D2D1OpacityMaskContent;
pub type ExtendMode = D2D1ExtendMode;
pub type AntiAliasMode = D2D1AntiAliasMode;
pub type TextAntiAliasMode = D2D1TextAntiAliasMode;
pub type BitmapInterpolationMode = D2D1BitmapInterpolationMode;
pub type DrawTextOptions = D2D1DrawTextOptions;
pub type BitmapProperties = D2D1BitmapProperties;
pub type GradientStop = D2D1GradientStop;
pub type BrushProperties = D2D1BrushProperties;
pub type BitmapBrushProperties = D2D1BitmapBrushProperties;
pub type LinearGradientBrushProperties = D2D1LinearGradientBrushProperties;
pub type RadialGradientBrushProperties = D2D1RadialGradientBrushProperties;
pub type ArcSize = D2D1ArcSize;
pub type CapStyle = D2D1CapStyle;
pub type DashStyle = D2D1DashStyle;
pub type LineJoin = D2D1LineJoin;
pub type CombineMode = D2D1CombineMode;
pub type GeometryRelation = D2D1GeometryRelation;
pub type GeometrySimplificationOption = D2D1GeometrySimplificationOption;
pub type Triangle = D2D1Triangle;
pub type SweepDirection = D2D1SweepDirection;
pub type ArcSegment = D2D1ArcSegment;
pub type QuadraticBezierSegment = D2D1QuadraticBezierSegment;
pub type Ellipse = D2D1Ellipse;
pub type RoundedRect = D2D1RoundedRect;
pub type StrokeStyleProperties = D2D1StrokeStyleProperties;
pub type LayerOptions = D2D1LayerOptions;
pub type LayerParameters<'a> = D2D1LayerParameters<'a>;
pub type WindowState = D2D1WindowState;
pub type RenderTargetType = D2D1RenderTargetType;
pub type FeatureLevel = D2D1FeatureLevel;
pub type RenderTargetUsage = D2D1RenderTargetUsage;
pub type PresentOptions = D2D1PresentOptions;
pub type RenderTargetProperties = D2D1RenderTargetProperties;
pub type HWndRenderTargetProperties = D2D1HWndRenderTargetProperties;
pub type CompatibleRenderTargetOptions = D2D1CompatibleRenderTargetOptions;
pub type DrawingStateDescription = D2D1DrawingStateDescription;
pub type DcInitializeMode = D2D1DcInitializeMode;
pub type DebugLevel = D2D1DebugLevel;
pub type FactoryType = D2D1FactoryType;
pub type FactoryOptions = D2D1FactoryOptions;
pub type Resource = D2D1Resource;
pub trait IResource: ID2D1Resource {}
impl<T: ID2D1Resource> IResource for T {}
pub type Image = D2D1Image;
pub trait IImage: ID2D1Image {}
impl<T: ID2D1Image> IImage for T {}
pub type Bitmap = D2D1Bitmap;
pub trait IBitmap: ID2D1Bitmap {}
impl<T: ID2D1Bitmap> IBitmap for T {}
pub type GradientStopCollection = D2D1GradientStopCollection;
pub trait IGradientStopCollection: ID2D1GradientStopCollection {}
impl<T: ID2D1GradientStopCollection> IGradientStopCollection for T {}
pub type Brush = D2D1Brush;
pub trait IBrush: ID2D1Brush {}
impl<T: ID2D1Brush> IBrush for T {}
pub type BitmapBrush = D2D1BitmapBrush;
pub trait IBitmapBrush: ID2D1BitmapBrush {}
impl<T: ID2D1BitmapBrush> IBitmapBrush for T {}
pub type SolidColorBrush = D2D1SolidColorBrush;
pub trait ISolidColorBrush: ID2D1SolidColorBrush {}
impl<T: ID2D1SolidColorBrush> ISolidColorBrush for T {}
pub type LinearGradientBrush = D2D1LinearGradientBrush;
pub trait ILinearGradientBrush: ID2D1LinearGradientBrush {}
impl<T: ID2D1LinearGradientBrush> ILinearGradientBrush for T {}
pub type RadialGradientBrush = D2D1RadialGradientBrush;
pub trait IRadialGradientBrush: ID2D1RadialGradientBrush {}
impl<T: ID2D1RadialGradientBrush> IRadialGradientBrush for T {}
pub type StrokeStyle = D2D1StrokeStyle;
pub trait IStrokeStyle: ID2D1StrokeStyle {}
impl<T: ID2D1StrokeStyle> IStrokeStyle for T {}
pub type Geometry = D2D1Geometry;
pub trait IGeometry: ID2D1Geometry {}
impl<T: ID2D1Geometry> IGeometry for T {}
pub type RectangleGeometry = D2D1RectangleGeometry;
pub trait IRectangleGeometry: ID2D1RectangleGeometry {}
impl<T: ID2D1RectangleGeometry> IRectangleGeometry for T {}
pub type RoundedRectangleGeometry = D2D1RoundedRectangleGeometry;
pub trait IRoundedRectangleGeometry: ID2D1RoundedRectangleGeometry {}
impl<T: ID2D1RoundedRectangleGeometry> IRoundedRectangleGeometry for T {}
pub type EllipseGeometry = D2D1EllipseGeometry;
pub trait IEllipseGeometry: ID2D1EllipseGeometry {}
impl<T: ID2D1EllipseGeometry> IEllipseGeometry for T {}
pub type GeometryGroup = D2D1GeometryGroup;
pub trait IGeometryGroup: ID2D1GeometryGroup {}
impl<T: ID2D1GeometryGroup> IGeometryGroup for T {}
pub type TransformedGeometry = D2D1TransformedGeometry;
pub trait ITransformedGeometry: ID2D1TransformedGeometry {}
impl<T: ID2D1TransformedGeometry> ITransformedGeometry for T {}
pub type GeometrySink = D2D1GeometrySink;
pub trait IGeometrySink: ID2D1GeometrySink {}
impl<T: ID2D1GeometrySink> IGeometrySink for T {}
pub type TessellationSink = D2D1TessellationSink;
pub trait ITessellationSink: ID2D1TessellationSink {}
impl<T: ID2D1TessellationSink> ITessellationSink for T {}
pub type PathGeometry = D2D1PathGeometry;
pub trait IPathGeometry: ID2D1PathGeometry {}
impl<T: ID2D1PathGeometry> IPathGeometry for T {}
pub type Mesh = D2D1Mesh;
pub trait IMesh: ID2D1Mesh {}
impl<T: ID2D1Mesh> IMesh for T {}
pub type Layer = D2D1Layer;
pub trait ILayer: ID2D1Layer {}
impl<T: ID2D1Layer> ILayer for T {}
pub type DrawingStateBlock = D2D1DrawingStateBlock;
pub trait IDrawingStateBlock: ID2D1DrawingStateBlock {}
impl<T: ID2D1DrawingStateBlock> IDrawingStateBlock for T {}
pub type RenderTarget = D2D1RenderTarget;
pub trait IRenderTarget: ID2D1RenderTarget {}
impl<T: ID2D1RenderTarget> IRenderTarget for T {}
pub type BitmapRenderTarget = D2D1BitmapRenderTarget;
pub trait IBitmapRenderTarget: ID2D1BitmapRenderTarget {}
impl<T: ID2D1BitmapRenderTarget> IBitmapRenderTarget for T {}
pub type HwndRenderTarget = D2D1HwndRenderTarget;
pub trait IHwndRenderTarget: ID2D1HwndRenderTarget {}
impl<T: ID2D1HwndRenderTarget> IHwndRenderTarget for T {}
pub type GdiInteropRenderTarget = D2D1GdiInteropRenderTarget;
pub trait IGdiInteropRenderTarget: ID2D1GdiInteropRenderTarget {}
impl<T: ID2D1GdiInteropRenderTarget> IGdiInteropRenderTarget for T {}
pub type DCRenderTarget = D2D1DCRenderTarget;
pub trait IDCRenderTarget: ID2D1DCRenderTarget {}
impl<T: ID2D1DCRenderTarget> IDCRenderTarget for T {}
pub type Factory = D2D1Factory;
pub trait IFactory: ID2D1Factory {}
impl<T: ID2D1Factory> IFactory for T {}
pub type ChannelSelector = D2D1ChannelSelector;
pub type BitmapSourceOrientation = D2D1BitmapSourceOrientation;
pub type GaussianBlurProp = D2D1GaussianBlurProp;
pub type GaussianBlurOptimization = D2D1GaussianBlurOptimization;
pub type DirectionalBlurProp = D2D1DirectionalBlurProp;
pub type DirectionalBlurOptimization = D2D1DirectionalBlurOptimization;
pub type ShadowProp = D2D1ShadowProp;
pub type ShadowOptimization = D2D1ShadowOptimization;
pub type BlendProp = D2D1BlendProp;
pub type SaturationProp = D2D1SaturationProp;
pub type HueRotationProp = D2D1HueRotationProp;
pub type ColorMatrixProp = D2D1ColorMatrixProp;
pub type BitmapSourceProp = D2D1BitmapSourceProp;
pub type BitmapSourceInterpolationMode = D2D1BitmapSourceInterpolationMode;
pub type BitmapSourceAlphaMode = D2D1BitmapSourceAlphaMode;
pub type CompositeProp = D2D1CompositeProp;
pub type _3dTransformProp = D2D1_3dTransformProp;
pub type _3dTransformInterpolationMode = D2D1_3dTransformInterpolationMode;
pub type _3dPerspectiveTransformProp = D2D1_3dPerspectiveTransformProp;
pub type _3dPerspectiveTransformInterpolationMode = D2D1_3dPerspectiveTransformInterpolationMode;
pub type _2dAffineTransformProp = D2D1_2dAffineTransformProp;
pub type DpiCompensationProp = D2D1DpiCompensationProp;
pub type DpiCompensationInterpolationMode = D2D1DpiCompensationInterpolationMode;
pub type ScaleProp = D2D1ScaleProp;
pub type ScaleInterpolationMode = D2D1ScaleInterpolationMode;
pub type TurbulenceProp = D2D1TurbulenceProp;
pub type DisplacementMapProp = D2D1DisplacementMapProp;
pub type ColorManagementProp = D2D1ColorManagementProp;
pub type ColorManagementAlphaMode = D2D1ColorManagementAlphaMode;
pub type ColorManagementQuality = D2D1ColorManagementQuality;
pub type ColorManagementRenderingIntent = D2D1ColorManagementRenderingIntent;
pub type HistogramProp = D2D1HistogramProp;
pub type PointSpecularProp = D2D1PointSpecularProp;
pub type PointSpecularScaleMode = D2D1PointSpecularScaleMode;
pub type SpotSpecularProp = D2D1SpotSpecularProp;
pub type SpotSpecularScaleMode = D2D1SpotSpecularScaleMode;
pub type DistantSpecularProp = D2D1DistantSpecularProp;
pub type DistantSpecularScaleMode = D2D1DistantSpecularScaleMode;
pub type PointDiffuseProp = D2D1PointDiffuseProp;
pub type PointDiffuseScaleMode = D2D1PointDiffuseScaleMode;
pub type SpotDiffuseProp = D2D1SpotDiffuseProp;
pub type SpotDiffuseScaleMode = D2D1SpotDiffuseScaleMode;
pub type DistantDiffuseProp = D2D1DistantDiffuseProp;
pub type DistantDiffuseScaleMode = D2D1DistantDiffuseScaleMode;
pub type FloodProp = D2D1FloodProp;
pub type LinearTransferProp = D2D1LinearTransferProp;
pub type GammaTransferProp = D2D1GammaTransferProp;
pub type TableTransferProp = D2D1TableTransferProp;
pub type DiscreteTransferProp = D2D1DiscreteTransferProp;
pub type ConvolveMatrixProp = D2D1ConvolveMatrixProp;
pub type ConvolveMatrixScaleMode = D2D1ConvolveMatrixScaleMode;
pub type BrightnessProp = D2D1BrightnessProp;
pub type ArithmeticCompositeProp = D2D1ArithmeticCompositeProp;
pub type CropProp = D2D1CropProp;
pub type BorderProp = D2D1BorderProp;
pub type BorderEdgeMode = D2D1BorderEdgeMode;
pub type MorphologyProp = D2D1MorphologyProp;
pub type MorphologyMode = D2D1MorphologyMode;
pub type TileProp = D2D1TileProp;
pub type AtlasProp = D2D1AtlasProp;
pub type OpacityMetaDataProp = D2D1OpacityMetaDataProp;
pub type PropertyType = D2D1PropertyType;
pub type Property = D2D1Property;
pub type SubProperty = D2D1SubProperty;
pub type BitmapOptions = D2D1BitmapOptions;
pub type BufferPrecision = D2D1BufferPrecision;
pub type MapOptions = D2D1MapOptions;
pub type InterpolationMode = D2D1InterpolationMode;
pub type UnitMode = D2D1UnitMode;
pub type ColorSpace = D2D1ColorSpace;
pub type DeviceContextOptions = D2D1DeviceContextOptions;
pub type StrokeTransformType = D2D1StrokeTransformType;
pub type PrimitiveBlend = D2D1PrimitiveBlend;
pub type ThreadingMode = D2D1ThreadingMode;
pub type ColorInterpolationMode = D2D1ColorInterpolationMode;
pub type BitmapProperties1<'a> = D2D1BitmapProperties1<'a>;
pub type MappedRect<'a> = D2D1MappedRect<'a>;
pub type RenderingControls = D2D1RenderingControls;
pub type EffectInputDescription<'a> = D2D1EffectInputDescription<'a>;
pub type PointDescription = D2D1PointDescription;
pub type ImageBrushProperties = D2D1ImageBrushProperties;
pub type BitmapBrushProperties1 = D2D1BitmapBrushProperties1;
pub type StrokeStyleProperties1 = D2D1StrokeStyleProperties1;
pub type LayerOptions1 = D2D1LayerOptions1;
pub type LayerParameters1<'a> = D2D1LayerParameters1<'a>;
pub type PrintFontSubsetMode = D2D1PrintFontSubsetMode;
pub type DrawingStateDescription1 = D2D1DrawingStateDescription1;
pub type PrintControlProperties = D2D1PrintControlProperties;
pub type CreationProperties = D2D1CreationProperties;
pub type GdiMetafileSink = D2D1GdiMetafileSink;
pub trait IGdiMetafileSink: ID2D1GdiMetafileSink {}
impl<T: ID2D1GdiMetafileSink> IGdiMetafileSink for T {}
pub type GdiMetafile = D2D1GdiMetafile;
pub trait IGdiMetafile: ID2D1GdiMetafile {}
impl<T: ID2D1GdiMetafile> IGdiMetafile for T {}
pub type CommandSink = D2D1CommandSink;
pub trait ICommandSink: ID2D1CommandSink {}
impl<T: ID2D1CommandSink> ICommandSink for T {}
pub type CommandList = D2D1CommandList;
pub trait ICommandList: ID2D1CommandList {}
impl<T: ID2D1CommandList> ICommandList for T {}
pub type PrintControl = D2D1PrintControl;
pub trait IPrintControl: ID2D1PrintControl {}
impl<T: ID2D1PrintControl> IPrintControl for T {}
pub type ImageBrush = D2D1ImageBrush;
pub trait IImageBrush: ID2D1ImageBrush {}
impl<T: ID2D1ImageBrush> IImageBrush for T {}
pub type BitmapBrush1 = D2D1BitmapBrush1;
pub trait IBitmapBrush1: ID2D1BitmapBrush1 {}
impl<T: ID2D1BitmapBrush1> IBitmapBrush1 for T {}
pub type StrokeStyle1 = D2D1StrokeStyle1;
pub trait IStrokeStyle1: ID2D1StrokeStyle1 {}
impl<T: ID2D1StrokeStyle1> IStrokeStyle1 for T {}
pub type PathGeometry1 = D2D1PathGeometry1;
pub trait IPathGeometry1: ID2D1PathGeometry1 {}
impl<T: ID2D1PathGeometry1> IPathGeometry1 for T {}
pub type Properties = D2D1Properties;
pub trait IProperties: ID2D1Properties {}
impl<T: ID2D1Properties> IProperties for T {}
pub type Effect = D2D1Effect;
pub trait IEffect: ID2D1Effect {}
impl<T: ID2D1Effect> IEffect for T {}
pub type Bitmap1 = D2D1Bitmap1;
pub trait IBitmap1: ID2D1Bitmap1 {}
impl<T: ID2D1Bitmap1> IBitmap1 for T {}
pub type ColorContext = D2D1ColorContext;
pub trait IColorContext: ID2D1ColorContext {}
impl<T: ID2D1ColorContext> IColorContext for T {}
pub type GradientStopCollection1 = D2D1GradientStopCollection1;
pub trait IGradientStopCollection1: ID2D1GradientStopCollection1 {}
impl<T: ID2D1GradientStopCollection1> IGradientStopCollection1 for T {}
pub type DrawingStateBlock1 = D2D1DrawingStateBlock1;
pub trait IDrawingStateBlock1: ID2D1DrawingStateBlock1 {}
impl<T: ID2D1DrawingStateBlock1> IDrawingStateBlock1 for T {}
pub type DeviceContext = D2D1DeviceContext;
pub trait IDeviceContext: ID2D1DeviceContext {}
impl<T: ID2D1DeviceContext> IDeviceContext for T {}
pub type Device = D2D1Device;
pub trait IDevice: ID2D1Device {}
impl<T: ID2D1Device> IDevice for T {}
pub type Factory1 = D2D1Factory1;
pub trait IFactory1: ID2D1Factory1 {}
impl<T: ID2D1Factory1> IFactory1 for T {}
pub type Multithread = D2D1Multithread;
pub trait IMultithread: ID2D1Multithread {}
impl<T: ID2D1Multithread> IMultithread for T {}
pub type ChangeType = D2D1ChangeType;
pub type PixelOptions = D2D1PixelOptions;
pub type VertexOptions = D2D1VertexOptions;
pub type VertexUsage = D2D1VertexUsage;
pub type BlendOperation = D2D1BlendOperation;
pub type Blend = D2D1Blend;
pub type ChannelDepth = D2D1ChannelDepth;
pub type Filter = D2D1Filter;
pub type Feature = D2D1Feature;
pub type PropertyBinding<'a> = D2D1PropertyBinding<'a>;
pub type ResourceTextureProperties<'a> = D2D1ResourceTextureProperties<'a>;
pub type InputElementDesc<'a> = D2D1InputElementDesc<'a>;
pub type VertexBufferProperties<'a> = D2D1VertexBufferProperties<'a>;
pub type CustomVertexBufferProperties<'a> = D2D1CustomVertexBufferProperties<'a>;
pub type VertexRange = D2D1VertexRange;
pub type BlendDescription = D2D1BlendDescription;
pub type InputDescription = D2D1InputDescription;
pub type FeatureDataDoubles = D2D1FeatureDataDoubles;
pub type FeatureDataD3D10XHardwareOptions = D2D1FeatureDataD3D10XHardwareOptions;
pub type VertexBuffer = D2D1VertexBuffer;
pub trait IVertexBuffer: ID2D1VertexBuffer {}
impl<T: ID2D1VertexBuffer> IVertexBuffer for T {}
pub type ResourceTexture = D2D1ResourceTexture;
pub trait IResourceTexture: ID2D1ResourceTexture {}
impl<T: ID2D1ResourceTexture> IResourceTexture for T {}
pub type RenderInfo = D2D1RenderInfo;
pub trait IRenderInfo: ID2D1RenderInfo {}
impl<T: ID2D1RenderInfo> IRenderInfo for T {}
pub type DrawInfo = D2D1DrawInfo;
pub trait IDrawInfo: ID2D1DrawInfo {}
impl<T: ID2D1DrawInfo> IDrawInfo for T {}
pub type ComputeInfo = D2D1ComputeInfo;
pub trait IComputeInfo: ID2D1ComputeInfo {}
impl<T: ID2D1ComputeInfo> IComputeInfo for T {}
pub type TransformNode = D2D1TransformNode;
pub trait ITransformNode: ID2D1TransformNode {}
impl<T: ID2D1TransformNode> ITransformNode for T {}
pub type TransformGraph = D2D1TransformGraph;
pub trait ITransformGraph: ID2D1TransformGraph {}
impl<T: ID2D1TransformGraph> ITransformGraph for T {}
pub type Transform = D2D1Transform;
pub trait ITransform: ID2D1Transform {}
impl<T: ID2D1Transform> ITransform for T {}
pub type DrawTransform = D2D1DrawTransform;
pub trait IDrawTransform: ID2D1DrawTransform {}
impl<T: ID2D1DrawTransform> IDrawTransform for T {}
pub type ComputeTransform = D2D1ComputeTransform;
pub trait IComputeTransform: ID2D1ComputeTransform {}
impl<T: ID2D1ComputeTransform> IComputeTransform for T {}
pub type AnalysisTransform = D2D1AnalysisTransform;
pub trait IAnalysisTransform: ID2D1AnalysisTransform {}
impl<T: ID2D1AnalysisTransform> IAnalysisTransform for T {}
pub type SourceTransform = D2D1SourceTransform;
pub trait ISourceTransform: ID2D1SourceTransform {}
impl<T: ID2D1SourceTransform> ISourceTransform for T {}
pub type ConcreteTransform = D2D1ConcreteTransform;
pub trait IConcreteTransform: ID2D1ConcreteTransform {}
impl<T: ID2D1ConcreteTransform> IConcreteTransform for T {}
pub type BlendTransform = D2D1BlendTransform;
pub trait IBlendTransform: ID2D1BlendTransform {}
impl<T: ID2D1BlendTransform> IBlendTransform for T {}
pub type BorderTransform = D2D1BorderTransform;
pub trait IBorderTransform: ID2D1BorderTransform {}
impl<T: ID2D1BorderTransform> IBorderTransform for T {}
pub type OffsetTransform = D2D1OffsetTransform;
pub trait IOffsetTransform: ID2D1OffsetTransform {}
impl<T: ID2D1OffsetTransform> IOffsetTransform for T {}
pub type BoundsAdjustmentTransform = D2D1BoundsAdjustmentTransform;
pub trait IBoundsAdjustmentTransform: ID2D1BoundsAdjustmentTransform {}
impl<T: ID2D1BoundsAdjustmentTransform> IBoundsAdjustmentTransform for T {}
pub type EffectImpl = D2D1EffectImpl;
pub trait IEffectImpl: ID2D1EffectImpl {}
impl<T: ID2D1EffectImpl> IEffectImpl for T {}
pub type EffectContext = D2D1EffectContext;
pub trait IEffectContext: ID2D1EffectContext {}
impl<T: ID2D1EffectContext> IEffectContext for T {}
pub type YCbCrProp = D2D1YCbCrProp;
pub type YCbCrChromaSubsampling = D2D1YCbCrChromaSubsampling;
pub type YCbCrInterpolationMode = D2D1YCbCrInterpolationMode;
pub type ContrastProp = D2D1ContrastProp;
pub type RgbToHueProp = D2D1RgbToHueProp;
pub type RgbToHueOutputColorSpace = D2D1RgbToHueOutputColorSpace;
pub type HueToRgbProp = D2D1HueToRgbProp;
pub type HueToRgbInputColorSpace = D2D1HueToRgbInputColorSpace;
pub type CHRomAkeyProp = D2D1CHRomAkeyProp;
pub type EmbossProp = D2D1EmbossProp;
pub type ExposureProp = D2D1ExposureProp;
pub type PosterizeProp = D2D1PosterizeProp;
pub type SepiaProp = D2D1SepiaProp;
pub type SharpenProp = D2D1SharpenProp;
pub type StraightenProp = D2D1StraightenProp;
pub type StraightenScaleMode = D2D1StraightenScaleMode;
pub type TemperatureAndTintProp = D2D1TemperatureAndTintProp;
pub type VignetteProp = D2D1VignetteProp;
pub type EdgeDetectionProp = D2D1EdgeDetectionProp;
pub type EdgeDetectionMode = D2D1EdgeDetectionMode;
pub type HighlightSandShadowsProp = D2D1HighlightSandShadowsProp;
pub type HighlightSandShadowsInputGamma = D2D1HighlightSandShadowsInputGamma;
pub type LookUptable3dProp = D2D1LookUptable3dProp;
pub type OpacityProp = D2D1OpacityProp;
pub type CrossFadeProp = D2D1CrossFadeProp;
pub type TintProp = D2D1TintProp;
pub type WhiteLevelAdjustmentProp = D2D1WhiteLevelAdjustmentProp;
pub type HdrToneMapProp = D2D1HdrToneMapProp;
pub type HdrToneMapDisplayMode = D2D1HdrToneMapDisplayMode;
pub type RenderingPriority = D2D1RenderingPriority;
pub type GeometryRealization = D2D1GeometryRealization;
pub trait IGeometryRealization: ID2D1GeometryRealization {}
impl<T: ID2D1GeometryRealization> IGeometryRealization for T {}
pub type DeviceContext1 = D2D1DeviceContext1;
pub trait IDeviceContext1: ID2D1DeviceContext1 {}
impl<T: ID2D1DeviceContext1> IDeviceContext1 for T {}
pub type Device1 = D2D1Device1;
pub trait IDevice1: ID2D1Device1 {}
impl<T: ID2D1Device1> IDevice1 for T {}
pub type Factory2 = D2D1Factory2;
pub trait IFactory2: ID2D1Factory2 {}
impl<T: ID2D1Factory2> IFactory2 for T {}
pub type CommandSink1 = D2D1CommandSink1;
pub trait ICommandSink1: ID2D1CommandSink1 {}
impl<T: ID2D1CommandSink1> ICommandSink1 for T {}
pub type SvgPaintType = D2D1SvgPaintType;
pub type SvgLengthUnits = D2D1SvgLengthUnits;
pub type SvgDisplay = D2D1SvgDisplay;
pub type SvgVisibility = D2D1SvgVisibility;
pub type SvgOverflow = D2D1SvgOverflow;
pub type SvgLineCap = D2D1SvgLineCap;
pub type SvgLineJoin = D2D1SvgLineJoin;
pub type SvgAspectAlign = D2D1SvgAspectAlign;
pub type SvgAspectScaling = D2D1SvgAspectScaling;
pub type SvgPathCommand = D2D1SvgPathCommand;
pub type SvgUnitType = D2D1SvgUnitType;
pub type SvgAttributeStringType = D2D1SvgAttributeStringType;
pub type SvgAttributePodType = D2D1SvgAttributePodType;
pub type SvgLength = D2D1SvgLength;
pub type SvgPreserveAspectRatio = D2D1SvgPreserveAspectRatio;
pub type SvgViewBox = D2D1SvgViewBox;
pub type SvgAttribute = D2D1SvgAttribute;
pub trait ISvgAttribute: ID2D1SvgAttribute {}
impl<T: ID2D1SvgAttribute> ISvgAttribute for T {}
pub type SvgPaint = D2D1SvgPaint;
pub trait ISvgPaint: ID2D1SvgPaint {}
impl<T: ID2D1SvgPaint> ISvgPaint for T {}
pub type SvgStrokeDashArray = D2D1SvgStrokeDashArray;
pub trait ISvgStrokeDashArray: ID2D1SvgStrokeDashArray {}
impl<T: ID2D1SvgStrokeDashArray> ISvgStrokeDashArray for T {}
pub type SvgPointCollection = D2D1SvgPointCollection;
pub trait ISvgPointCollection: ID2D1SvgPointCollection {}
impl<T: ID2D1SvgPointCollection> ISvgPointCollection for T {}
pub type SvgPathData = D2D1SvgPathData;
pub trait ISvgPathData: ID2D1SvgPathData {}
impl<T: ID2D1SvgPathData> ISvgPathData for T {}
pub type SvgElement = D2D1SvgElement;
pub trait ISvgElement: ID2D1SvgElement {}
impl<T: ID2D1SvgElement> ISvgElement for T {}
pub type SvgDocument = D2D1SvgDocument;
pub trait ISvgDocument: ID2D1SvgDocument {}
impl<T: ID2D1SvgDocument> ISvgDocument for T {}
pub type InkNibShape = D2D1InkNibShape;
pub type Orientation = D2D1Orientation;
pub type ImageSourceLoadingOptions = D2D1ImageSourceLoadingOptions;
pub type ImageSourceFromDxgiOptions = D2D1ImageSourceFromDxgiOptions;
pub type TransformedImageSourceOptions = D2D1TransformedImageSourceOptions;
pub type TransformedImageSourceProperties = D2D1TransformedImageSourceProperties;
pub type InkPoint = D2D1InkPoint;
pub type InkBezierSegment = D2D1InkBezierSegment;
pub type InkStyleProperties = D2D1InkStyleProperties;
pub type PatchEdgeMode = D2D1PatchEdgeMode;
pub type GradientMeshPatch = D2D1GradientMeshPatch;
pub type SpriteOptions = D2D1SpriteOptions;
pub type ColorBitmapGlyphSnapOption = D2D1ColorBitmapGlyphSnapOption;
pub type Gamma1 = D2D1Gamma1;
pub type SimpleColorProfile = D2D1SimpleColorProfile;
pub type ColorContextType = D2D1ColorContextType;
pub type InkStyle = D2D1InkStyle;
pub trait IInkStyle: ID2D1InkStyle {}
impl<T: ID2D1InkStyle> IInkStyle for T {}
pub type Ink = D2D1Ink;
pub trait IInk: ID2D1Ink {}
impl<T: ID2D1Ink> IInk for T {}
pub type GradientMesh = D2D1GradientMesh;
pub trait IGradientMesh: ID2D1GradientMesh {}
impl<T: ID2D1GradientMesh> IGradientMesh for T {}
pub type ImageSource = D2D1ImageSource;
pub trait IImageSource: ID2D1ImageSource {}
impl<T: ID2D1ImageSource> IImageSource for T {}
pub type ImageSourceFromWic = D2D1ImageSourceFromWic;
pub trait IImageSourceFromWic: ID2D1ImageSourceFromWic {}
impl<T: ID2D1ImageSourceFromWic> IImageSourceFromWic for T {}
pub type TransformedImageSource = D2D1TransformedImageSource;
pub trait ITransformedImageSource: ID2D1TransformedImageSource {}
impl<T: ID2D1TransformedImageSource> ITransformedImageSource for T {}
pub type LookupTable3D = D2D1LookupTable3D;
pub trait ILookupTable3D: ID2D1LookupTable3D {}
impl<T: ID2D1LookupTable3D> ILookupTable3D for T {}
pub type DeviceContext2 = D2D1DeviceContext2;
pub trait IDeviceContext2: ID2D1DeviceContext2 {}
impl<T: ID2D1DeviceContext2> IDeviceContext2 for T {}
pub type Device2 = D2D1Device2;
pub trait IDevice2: ID2D1Device2 {}
impl<T: ID2D1Device2> IDevice2 for T {}
pub type Factory3 = D2D1Factory3;
pub trait IFactory3: ID2D1Factory3 {}
impl<T: ID2D1Factory3> IFactory3 for T {}
pub type CommandSink2 = D2D1CommandSink2;
pub trait ICommandSink2: ID2D1CommandSink2 {}
impl<T: ID2D1CommandSink2> ICommandSink2 for T {}
pub type GdiMetafile1 = D2D1GdiMetafile1;
pub trait IGdiMetafile1: ID2D1GdiMetafile1 {}
impl<T: ID2D1GdiMetafile1> IGdiMetafile1 for T {}
pub type GdiMetafileSink1 = D2D1GdiMetafileSink1;
pub trait IGdiMetafileSink1: ID2D1GdiMetafileSink1 {}
impl<T: ID2D1GdiMetafileSink1> IGdiMetafileSink1 for T {}
pub type SpriteBatch = D2D1SpriteBatch;
pub trait ISpriteBatch: ID2D1SpriteBatch {}
impl<T: ID2D1SpriteBatch> ISpriteBatch for T {}
pub type DeviceContext3 = D2D1DeviceContext3;
pub trait IDeviceContext3: ID2D1DeviceContext3 {}
impl<T: ID2D1DeviceContext3> IDeviceContext3 for T {}
pub type Device3 = D2D1Device3;
pub trait IDevice3: ID2D1Device3 {}
impl<T: ID2D1Device3> IDevice3 for T {}
pub type Factory4 = D2D1Factory4;
pub trait IFactory4: ID2D1Factory4 {}
impl<T: ID2D1Factory4> IFactory4 for T {}
pub type CommandSink3 = D2D1CommandSink3;
pub trait ICommandSink3: ID2D1CommandSink3 {}
impl<T: ID2D1CommandSink3> ICommandSink3 for T {}
pub type SvgGlyphStyle = D2D1SvgGlyphStyle;
pub trait ISvgGlyphStyle: ID2D1SvgGlyphStyle {}
impl<T: ID2D1SvgGlyphStyle> ISvgGlyphStyle for T {}
pub type DeviceContext4 = D2D1DeviceContext4;
pub trait IDeviceContext4: ID2D1DeviceContext4 {}
impl<T: ID2D1DeviceContext4> IDeviceContext4 for T {}
pub type Device4 = D2D1Device4;
pub trait IDevice4: ID2D1Device4 {}
impl<T: ID2D1Device4> IDevice4 for T {}
pub type Factory5 = D2D1Factory5;
pub trait IFactory5: ID2D1Factory5 {}
impl<T: ID2D1Factory5> IFactory5 for T {}
pub type CommandSink4 = D2D1CommandSink4;
pub trait ICommandSink4: ID2D1CommandSink4 {}
impl<T: ID2D1CommandSink4> ICommandSink4 for T {}
pub type ColorContext1 = D2D1ColorContext1;
pub trait IColorContext1: ID2D1ColorContext1 {}
impl<T: ID2D1ColorContext1> IColorContext1 for T {}
pub type DeviceContext5 = D2D1DeviceContext5;
pub trait IDeviceContext5: ID2D1DeviceContext5 {}
impl<T: ID2D1DeviceContext5> IDeviceContext5 for T {}
pub type Device5 = D2D1Device5;
pub trait IDevice5: ID2D1Device5 {}
impl<T: ID2D1Device5> IDevice5 for T {}
pub type Factory6 = D2D1Factory6;
pub trait IFactory6: ID2D1Factory6 {}
impl<T: ID2D1Factory6> IFactory6 for T {}
pub type CommandSink5 = D2D1CommandSink5;
pub trait ICommandSink5: ID2D1CommandSink5 {}
impl<T: ID2D1CommandSink5> ICommandSink5 for T {}
pub type DeviceContext6 = D2D1DeviceContext6;
pub trait IDeviceContext6: ID2D1DeviceContext6 {}
impl<T: ID2D1DeviceContext6> IDeviceContext6 for T {}
pub type Device6 = D2D1Device6;
pub trait IDevice6: ID2D1Device6 {}
impl<T: ID2D1Device6> IDevice6 for T {}
pub type Factory7 = D2D1Factory7;
pub trait IFactory7: ID2D1Factory7 {}
impl<T: ID2D1Factory7> IFactory7 for T {}
pub type EffectContext1 = D2D1EffectContext1;
pub trait IEffectContext1: ID2D1EffectContext1 {}
impl<T: ID2D1EffectContext1> IEffectContext1 for T {}
pub type EffectContext2 = D2D1EffectContext2;
pub trait IEffectContext2: ID2D1EffectContext2 {}
impl<T: ID2D1EffectContext2> IEffectContext2 for T {}
