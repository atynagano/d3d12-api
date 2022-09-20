#![allow(unused_imports)]

use crate::core::win32::graphics::direct_write::*;

pub type RenderingParams = DWriteRenderingParams;
pub trait IRenderingParams: IDWriteRenderingParams {}
impl<T: IDWriteRenderingParams> IRenderingParams for T {}
pub type TextFormat = DWriteTextFormat;
pub trait ITextFormat: IDWriteTextFormat {}
impl<T: IDWriteTextFormat> ITextFormat for T {}
pub type MeasuringMode = DWriteMeasuringMode;
pub type TextLayout = DWriteTextLayout;
pub trait ITextLayout: IDWriteTextLayout {}
impl<T: IDWriteTextLayout> ITextLayout for T {}
pub type GlyphRun<'a> = DWriteGlyphRun<'a>;
pub type FontFace = DWriteFontFace;
pub trait IFontFace: IDWriteFontFace {}
impl<T: IDWriteFontFace> IFontFace for T {}
pub type GlyphOffset = DWriteGlyphOffset;
pub type GlyphRunDescription<'a> = DWriteGlyphRunDescription<'a>;
pub type GlyphImageFormats = DWriteGlyphImageFormats;
pub type PixelGeometry = DWritePixelGeometry;
pub type RenderingMode = DWriteRenderingMode;
pub type TextAlignment = DWriteTextAlignment;
pub type ParagraphAlignment = DWriteParagraphAlignment;
pub type WordWrapping = DWriteWordWrapping;
pub type ReadingDirection = DWriteReadingDirection;
pub type FlowDirection = DWriteFlowDirection;
pub type Trimming = DWriteTrimming;
pub type TrimmingGranularity = DWriteTrimmingGranularity;
pub type InlineObject = DWriteInlineObject;
pub trait IInlineObject: IDWriteInlineObject {}
impl<T: IDWriteInlineObject> IInlineObject for T {}
pub type LineSpacingMethod = DWriteLineSpacingMethod;
pub type FontCollection = DWriteFontCollection;
pub trait IFontCollection: IDWriteFontCollection {}
impl<T: IDWriteFontCollection> IFontCollection for T {}
pub type FontWeight = DWriteFontWeight;
pub type FontStyle = DWriteFontStyle;
pub type FontStretch = DWriteFontStretch;
pub type TextRange = DWriteTextRange;
pub type Typography = DWriteTypography;
pub trait ITypography: IDWriteTypography {}
impl<T: IDWriteTypography> ITypography for T {}
pub type TextRenderer = DWriteTextRenderer;
pub trait ITextRenderer: IDWriteTextRenderer {}
impl<T: IDWriteTextRenderer> ITextRenderer for T {}
pub type PixelSnapping = DWritePixelSnapping;
pub trait IPixelSnapping: IDWritePixelSnapping {}
impl<T: IDWritePixelSnapping> IPixelSnapping for T {}
pub type LineMetrics = DWriteLineMetrics;
pub type TextMetrics = DWriteTextMetrics;
pub type OverhangMetrics = DWriteOverhangMetrics;
pub type ClusterMetrics = DWriteClusterMetrics;
pub type HitTestMetrics = DWriteHitTestMetrics;
pub type FontFaceType = DWriteFontFaceType;
pub type FontFile = DWriteFontFile;
pub trait IFontFile: IDWriteFontFile {}
impl<T: IDWriteFontFile> IFontFile for T {}
pub type FontSimulations = DWriteFontSimulations;
pub type FontMetrics = DWriteFontMetrics;
pub type GlyphMetrics = DWriteGlyphMetrics;
pub type Matrix = DWriteMatrix;
pub type InlineObjectMetrics = DWriteInlineObjectMetrics;
pub type BreakCondition = DWriteBreakCondition;
pub type FontFamily = DWriteFontFamily;
pub trait IFontFamily: IDWriteFontFamily {}
impl<T: IDWriteFontFamily> IFontFamily for T {}
pub type FontList = DWriteFontList;
pub trait IFontList: IDWriteFontList {}
impl<T: IDWriteFontList> IFontList for T {}
pub type Font = DWriteFont;
pub trait IFont: IDWriteFont {}
impl<T: IDWriteFont> IFont for T {}
pub type FontFeature = DWriteFontFeature;
pub type FontFeatureTag = DWriteFontFeatureTag;
pub type Underline<'a> = DWriteUnderline<'a>;
pub type StrikeThrough<'a> = DWriteStrikeThrough<'a>;
pub type FontFileLoader = DWriteFontFileLoader;
pub trait IFontFileLoader: IDWriteFontFileLoader {}
impl<T: IDWriteFontFileLoader> IFontFileLoader for T {}
pub type FontFileType = DWriteFontFileType;
pub type LocalizedStrings = DWriteLocalizedStrings;
pub trait ILocalizedStrings: IDWriteLocalizedStrings {}
impl<T: IDWriteLocalizedStrings> ILocalizedStrings for T {}
pub type InformationalStringId = DWriteInformationalStringId;
pub type FontFileStream = DWriteFontFileStream;
pub trait IFontFileStream: IDWriteFontFileStream {}
impl<T: IDWriteFontFileStream> IFontFileStream for T {}
