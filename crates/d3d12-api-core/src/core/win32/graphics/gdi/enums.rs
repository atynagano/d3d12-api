#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

bitflags::bitflags! {
	/// ROP_CODE
	#[repr(transparent)]
	pub struct RopCode: u32 {
		/// BLACKNESS = 0x42u32
		const Blackness            = 0x42u32;
		/// NOTSRCERASE = 0x1100A6u32
		const NotSrcErase          = 0x1100A6u32;
		/// NOTSRCCOPY = 0x330008u32
		const NotSrcCopy           = 0x330008u32;
		/// SRCERASE = 0x440328u32
		const SrcErase             = 0x440328u32;
		/// DSTINVERT = 0x550009u32
		const DstInvert            = 0x550009u32;
		/// PATINVERT = 0x5A0049u32
		const PatInvert            = 0x5A0049u32;
		/// SRCINVERT = 0x660046u32
		const SrcInvert            = 0x660046u32;
		/// SRCAND = 0x8800C6u32
		const SrcAnd               = 0x8800C6u32;
		/// MERGEPAINT = 0xBB0226u32
		const MergePaint           = 0xBB0226u32;
		/// MERGECOPY = 0xC000CAu32
		const MergeCopy            = 0xC000CAu32;
		/// SRCCOPY = 0xCC0020u32
		const SrcCopy              = 0xCC0020u32;
		/// SRCPAINT = 0xEE0086u32
		const SrcPaint             = 0xEE0086u32;
		/// PATCOPY = 0xF00021u32
		const PatCopy              = 0xF00021u32;
		/// PATPAINT = 0xFB0A09u32
		const PatPaint             = 0xFB0A09u32;
		/// WHITENESS = 0xFF0062u32
		const Whiteness            = 0xFF0062u32;
		/// CAPTUREBLT = 0x40000000u32
		const CaptureBlt           = 0x40000000u32;
		/// NOMIRRORBITMAP = 0x80000000u32
		const NoMirrorBitmap       = 0x80000000u32;
	}
}

/// GDI_REGION_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GdiRegionType
{
	/// RGN_ERROR = 0x0i32
	RgnError             = 0x0i32,
	/// NULLREGION = 0x1i32
	NullRegion           = 0x1i32,
	/// SIMPLEREGION = 0x2i32
	SimpleRegion         = 0x2i32,
	/// COMPLEXREGION = 0x3i32
	ComplexRegion        = 0x3i32,
}

/// RGN_COMBINE_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RgnCombineMode
{
	/// RGN_AND = 0x1i32
	And                  = 0x1i32,
	/// RGN_OR = 0x2i32
	Or                   = 0x2i32,
	/// RGN_XOR = 0x3i32
	Xor                  = 0x3i32,
	/// RGN_DIFF = 0x4i32
	Diff                 = 0x4i32,
	/// RGN_COPY = 0x5i32
	Copy                 = 0x5i32,
}

impl RgnCombineMode {
	/// RGN_MIN = 0x1i32
	pub const Min                 : Self = unsafe { transmute(0x1i32) };
	/// RGN_MAX = 0x5i32
	pub const Max                 : Self = unsafe { transmute(0x5i32) };
}

/// DIB_USAGE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DibUsage
{
	/// DIB_RGB_COLORS = 0x0u32
	RgbColors            = 0x0u32,
	/// DIB_PAL_COLORS = 0x1u32
	PalColors            = 0x1u32,
}

/// FONT_OUTPUT_PRECISION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FontOutputPrecision
{
	/// OUT_CHARACTER_PRECIS = 0x2u32
	CharacterPrecis      = 0x2u32,
	/// OUT_DEFAULT_PRECIS = 0x0u32
	DefaultPrecis        = 0x0u32,
	/// OUT_DEVICE_PRECIS = 0x5u32
	DevicePrecis         = 0x5u32,
	/// OUT_OUTLINE_PRECIS = 0x8u32
	OutlinePrecis        = 0x8u32,
	/// OUT_PS_ONLY_PRECIS = 0xAu32
	PsOnlyPrecis         = 0xAu32,
	/// OUT_RASTER_PRECIS = 0x6u32
	RasterPrecis         = 0x6u32,
	/// OUT_STRING_PRECIS = 0x1u32
	StringPrecis         = 0x1u32,
	/// OUT_STROKE_PRECIS = 0x3u32
	StrokePrecis         = 0x3u32,
	/// OUT_TT_ONLY_PRECIS = 0x7u32
	TtOnlyPrecis         = 0x7u32,
	/// OUT_TT_PRECIS = 0x4u32
	TtPrecis             = 0x4u32,
}

bitflags::bitflags! {
	/// FONT_CLIP_PRECISION
	#[repr(transparent)]
	pub struct FontClipPrecision: u32 {
		/// CLIP_CHARACTER_PRECIS = 0x1u32
		const CharacterPrecis      = 0x1u32;
		/// CLIP_DEFAULT_PRECIS = 0x0u32
		const DefaultPrecis        = 0x0u32;
		/// CLIP_DFA_DISABLE = 0x40u32
		const DfaDisable           = 0x40u32;
		/// CLIP_EMBEDDED = 0x80u32
		const Embedded             = 0x80u32;
		/// CLIP_LH_ANGLES = 0x10u32
		const LhAngles             = 0x10u32;
		/// CLIP_MASK = 0xFu32
		const Mask                 = 0xFu32;
		/// CLIP_STROKE_PRECIS = 0x2u32
		const StrokePrecis         = 0x2u32;
		/// CLIP_TT_ALWAYS = 0x20u32
		const TtAlways             = 0x20u32;
	}
}

/// FONT_QUALITY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FontQuality
{
	/// ANTIALIASED_QUALITY = 0x4u32
	AntialiasedQuality   = 0x4u32,
	/// CLEARTYPE_QUALITY = 0x5u32
	ClearTypeQuality     = 0x5u32,
	/// DEFAULT_QUALITY = 0x0u32
	DefaultQuality       = 0x0u32,
	/// DRAFT_QUALITY = 0x1u32
	DraftQuality         = 0x1u32,
	/// NONANTIALIASED_QUALITY = 0x3u32
	NonAntialiasedQuality = 0x3u32,
	/// PROOF_QUALITY = 0x2u32
	ProofQuality         = 0x2u32,
}

/// FONT_PITCH_AND_FAMILY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FontPitchAndFamily
{
	/// FF_DECORATIVE = 0x50u32
	Decorative           = 0x50u32,
	/// FF_DONTCARE = 0x0u32
	DontCare             = 0x0u32,
	/// FF_MODERN = 0x30u32
	Modern               = 0x30u32,
	/// FF_ROMAN = 0x10u32
	Roman                = 0x10u32,
	/// FF_SCRIPT = 0x40u32
	Script               = 0x40u32,
	/// FF_SWISS = 0x20u32
	Swiss                = 0x20u32,
}

/// HATCH_BRUSH_STYLE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HatchBrushStyle
{
	/// HS_BDIAGONAL = 0x3u32
	BDiagonal            = 0x3u32,
	/// HS_CROSS = 0x4u32
	Cross                = 0x4u32,
	/// HS_DIAGCROSS = 0x5u32
	DiagCross            = 0x5u32,
	/// HS_FDIAGONAL = 0x2u32
	FDiagonal            = 0x2u32,
	/// HS_HORIZONTAL = 0x0u32
	Horizontal           = 0x0u32,
	/// HS_VERTICAL = 0x1u32
	Vertical             = 0x1u32,
}

bitflags::bitflags! {
	/// PEN_STYLE
	#[repr(transparent)]
	pub struct PenStyle: u32 {
		/// PS_GEOMETRIC = 0x10000u32
		const Geometric            = 0x10000u32;
		/// PS_COSMETIC = 0x0u32
		const Cosmetic             = 0x0u32;
		/// PS_SOLID = 0x0u32
		const Solid                = 0x0u32;
		/// PS_DASH = 0x1u32
		const Dash                 = 0x1u32;
		/// PS_DOT = 0x2u32
		const Dot                  = 0x2u32;
		/// PS_DASHDOT = 0x3u32
		const DashDot              = 0x3u32;
		/// PS_DASHDOTDOT = 0x4u32
		const DashDotDot           = 0x4u32;
		/// PS_NULL = 0x5u32
		const Null                 = 0x5u32;
		/// PS_INSIDEFRAME = 0x6u32
		const InsideFrame          = 0x6u32;
		/// PS_USERSTYLE = 0x7u32
		const UserStyle            = 0x7u32;
		/// PS_ALTERNATE = 0x8u32
		const Alternate            = 0x8u32;
		/// PS_STYLE_MASK = 0xFu32
		const StyleMask            = 0xFu32;
		/// PS_ENDCAP_ROUND = 0x0u32
		const EndCapRound          = 0x0u32;
		/// PS_ENDCAP_SQUARE = 0x100u32
		const EndCapSquare         = 0x100u32;
		/// PS_ENDCAP_FLAT = 0x200u32
		const EndCapFlat           = 0x200u32;
		/// PS_ENDCAP_MASK = 0xF00u32
		const EndCapMask           = 0xF00u32;
		/// PS_JOIN_ROUND = 0x0u32
		const JoinRound            = 0x0u32;
		/// PS_JOIN_BEVEL = 0x1000u32
		const JoinBevel            = 0x1000u32;
		/// PS_JOIN_MITER = 0x2000u32
		const JoinMiter            = 0x2000u32;
		/// PS_JOIN_MASK = 0xF000u32
		const JoinMask             = 0xF000u32;
		/// PS_TYPE_MASK = 0xF0000u32
		const TypeMask             = 0xF0000u32;
	}
}

/// CREATE_POLYGON_RGN_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CreatePolygonRgnMode
{
	/// ALTERNATE = 0x1u32
	Alternate            = 0x1u32,
	/// WINDING = 0x2u32
	Winding              = 0x2u32,
}

/// OBJ_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObjType
{
	/// OBJ_PEN = 0x1i32
	Pen                  = 0x1i32,
	/// OBJ_BRUSH = 0x2i32
	Brush                = 0x2i32,
	/// OBJ_DC = 0x3i32
	Dc                   = 0x3i32,
	/// OBJ_METADC = 0x4i32
	MetAdc               = 0x4i32,
	/// OBJ_PAL = 0x5i32
	Pal                  = 0x5i32,
	/// OBJ_FONT = 0x6i32
	Font                 = 0x6i32,
	/// OBJ_BITMAP = 0x7i32
	Bitmap               = 0x7i32,
	/// OBJ_REGION = 0x8i32
	Region               = 0x8i32,
	/// OBJ_METAFILE = 0x9i32
	MetaFile             = 0x9i32,
	/// OBJ_MEMDC = 0xAi32
	MemDc                = 0xAi32,
	/// OBJ_EXTPEN = 0xBi32
	ExtPen               = 0xBi32,
	/// OBJ_ENHMETADC = 0xCi32
	EnhMetAdc            = 0xCi32,
	/// OBJ_ENHMETAFILE = 0xDi32
	EnhMetaFile          = 0xDi32,
	/// OBJ_COLORSPACE = 0xEi32
	ColorSpace           = 0xEi32,
}

/// EXT_FLOOD_FILL_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExtFloodFillType
{
	/// FLOODFILLBORDER = 0x0u32
	Border               = 0x0u32,
	/// FLOODFILLSURFACE = 0x1u32
	Surface              = 0x1u32,
}

/// GET_DEVICE_CAPS_INDEX
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetDeviceCapsIndex
{
	/// DRIVERVERSION = 0x0u32
	DriverVersion        = 0x0u32,
	/// TECHNOLOGY = 0x2u32
	Technology           = 0x2u32,
	/// HORZSIZE = 0x4u32
	HorZSize             = 0x4u32,
	/// VERTSIZE = 0x6u32
	VertSize             = 0x6u32,
	/// HORZRES = 0x8u32
	HorZRes              = 0x8u32,
	/// VERTRES = 0xAu32
	VerTres              = 0xAu32,
	/// BITSPIXEL = 0xCu32
	BitsPixel            = 0xCu32,
	/// PLANES = 0xEu32
	Planes               = 0xEu32,
	/// NUMBRUSHES = 0x10u32
	NumBrushes           = 0x10u32,
	/// NUMPENS = 0x12u32
	NumPens              = 0x12u32,
	/// NUMMARKERS = 0x14u32
	NumMarkers           = 0x14u32,
	/// NUMFONTS = 0x16u32
	NumFonts             = 0x16u32,
	/// NUMCOLORS = 0x18u32
	NumColors            = 0x18u32,
	/// PDEVICESIZE = 0x1Au32
	PDeviceSize          = 0x1Au32,
	/// CURVECAPS = 0x1Cu32
	CurveCaps            = 0x1Cu32,
	/// LINECAPS = 0x1Eu32
	LineCaps             = 0x1Eu32,
	/// POLYGONALCAPS = 0x20u32
	PolygonalCaps        = 0x20u32,
	/// TEXTCAPS = 0x22u32
	TextCaps             = 0x22u32,
	/// CLIPCAPS = 0x24u32
	ClipCaps             = 0x24u32,
	/// RASTERCAPS = 0x26u32
	RasterCaps           = 0x26u32,
	/// ASPECTX = 0x28u32
	AspectX              = 0x28u32,
	/// ASPECTY = 0x2Au32
	AspectY              = 0x2Au32,
	/// ASPECTXY = 0x2Cu32
	AspectXY             = 0x2Cu32,
	/// LOGPIXELSX = 0x58u32
	LogPixelsX           = 0x58u32,
	/// LOGPIXELSY = 0x5Au32
	LogPixelsY           = 0x5Au32,
	/// SIZEPALETTE = 0x68u32
	SizePalette          = 0x68u32,
	/// NUMRESERVED = 0x6Au32
	NumReserved          = 0x6Au32,
	/// COLORRES = 0x6Cu32
	ColorRes             = 0x6Cu32,
	/// PHYSICALWIDTH = 0x6Eu32
	PhysicalWidth        = 0x6Eu32,
	/// PHYSICALHEIGHT = 0x6Fu32
	PhysicalHeight       = 0x6Fu32,
	/// PHYSICALOFFSETX = 0x70u32
	PhysicalOffsetX      = 0x70u32,
	/// PHYSICALOFFSETY = 0x71u32
	PhysicalOffsEty      = 0x71u32,
	/// SCALINGFACTORX = 0x72u32
	ScalingFactorX       = 0x72u32,
	/// SCALINGFACTORY = 0x73u32
	ScalingFactory       = 0x73u32,
	/// VREFRESH = 0x74u32
	VRefresh             = 0x74u32,
	/// DESKTOPVERTRES = 0x75u32
	DesktopVerTres       = 0x75u32,
	/// DESKTOPHORZRES = 0x76u32
	DesktopHorZRes       = 0x76u32,
	/// BLTALIGNMENT = 0x77u32
	BltAlignment         = 0x77u32,
	/// SHADEBLENDCAPS = 0x78u32
	ShadeBlendCaps       = 0x78u32,
	/// COLORMGMTCAPS = 0x79u32
	ColorMgmtCaps        = 0x79u32,
}

/// GET_GLYPH_OUTLINE_FORMAT
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetGlyphOutlineFormat
{
	/// GGO_BEZIER = 0x3u32
	Bezier               = 0x3u32,
	/// GGO_BITMAP = 0x1u32
	Bitmap               = 0x1u32,
	/// GGO_GLYPH_INDEX = 0x80u32
	GlyphIndex           = 0x80u32,
	/// GGO_GRAY2_BITMAP = 0x4u32
	Gray2Bitmap          = 0x4u32,
	/// GGO_GRAY4_BITMAP = 0x5u32
	Gray4Bitmap          = 0x5u32,
	/// GGO_GRAY8_BITMAP = 0x6u32
	Gray8Bitmap          = 0x6u32,
	/// GGO_METRICS = 0x0u32
	Metrics              = 0x0u32,
	/// GGO_NATIVE = 0x2u32
	Native               = 0x2u32,
	/// GGO_UNHINTED = 0x100u32
	Unhinted             = 0x100u32,
}

/// GET_STOCK_OBJECT_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetStockObjectFlags
{
	/// BLACK_BRUSH = 0x4u32
	BlackBrush           = 0x4u32,
	/// DKGRAY_BRUSH = 0x3u32
	DKGrayBrush          = 0x3u32,
	/// DC_BRUSH = 0x12u32
	DcBrush              = 0x12u32,
	/// GRAY_BRUSH = 0x2u32
	GrayBrush            = 0x2u32,
	/// HOLLOW_BRUSH = 0x5u32
	HollowBrush          = 0x5u32,
	/// LTGRAY_BRUSH = 0x1u32
	LTGrayBrush          = 0x1u32,
	/// WHITE_BRUSH = 0x0u32
	WhiteBrush           = 0x0u32,
	/// BLACK_PEN = 0x7u32
	BlackPen             = 0x7u32,
	/// DC_PEN = 0x13u32
	DcPen                = 0x13u32,
	/// NULL_PEN = 0x8u32
	NullPen              = 0x8u32,
	/// WHITE_PEN = 0x6u32
	WhitePen             = 0x6u32,
	/// ANSI_FIXED_FONT = 0xBu32
	AnsiFixedFont        = 0xBu32,
	/// ANSI_VAR_FONT = 0xCu32
	AnsiVarFont          = 0xCu32,
	/// DEVICE_DEFAULT_FONT = 0xEu32
	DeviceDefaultFont    = 0xEu32,
	/// DEFAULT_GUI_FONT = 0x11u32
	DefaultGuiFont       = 0x11u32,
	/// OEM_FIXED_FONT = 0xAu32
	OemFixedFont         = 0xAu32,
	/// SYSTEM_FONT = 0xDu32
	SystemFont           = 0xDu32,
	/// SYSTEM_FIXED_FONT = 0x10u32
	SystemFixedFont      = 0x10u32,
	/// DEFAULT_PALETTE = 0xFu32
	DefaultPalette       = 0xFu32,
}

impl GetStockObjectFlags {
	/// NULL_BRUSH = 0x5u32
	pub const NullBrush           : Self = unsafe { transmute(0x5u32) };
}

/// FONT_RESOURCE_CHARACTERISTICS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FontResourceCharacteristics
{
	/// FR_PRIVATE = 0x10u32
	Private              = 0x10u32,
	/// FR_NOT_ENUM = 0x20u32
	NotEnum              = 0x20u32,
}

/// BACKGROUND_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BackgroundMode
{
	/// OPAQUE = 0x2u32
	Opaque               = 0x2u32,
	/// TRANSPARENT = 0x1u32
	Transparent          = 0x1u32,
}

/// SET_BOUNDS_RECT_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SetBoundsRectFlags
{
	/// DCB_ACCUMULATE = 0x2u32
	Accumulate           = 0x2u32,
	/// DCB_DISABLE = 0x8u32
	Disable              = 0x8u32,
	/// DCB_ENABLE = 0x4u32
	Enable               = 0x4u32,
	/// DCB_RESET = 0x1u32
	Reset                = 0x1u32,
}

/// GRAPHICS_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphicsMode
{
	/// GM_COMPATIBLE = 0x1u32
	Compatible           = 0x1u32,
	/// GM_ADVANCED = 0x2u32
	Advanced             = 0x2u32,
}

/// HDC_MAP_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HDcMapMode
{
	/// MM_ANISOTROPIC = 0x8u32
	Anisotropic          = 0x8u32,
	/// MM_HIENGLISH = 0x5u32
	HiEnglish            = 0x5u32,
	/// MM_HIMETRIC = 0x3u32
	HiMetric             = 0x3u32,
	/// MM_ISOTROPIC = 0x7u32
	Isotropic            = 0x7u32,
	/// MM_LOENGLISH = 0x4u32
	LoEnglish            = 0x4u32,
	/// MM_LOMETRIC = 0x2u32
	LoMetric             = 0x2u32,
	/// MM_TEXT = 0x1u32
	Text                 = 0x1u32,
	/// MM_TWIPS = 0x6u32
	TWips                = 0x6u32,
}

bitflags::bitflags! {
	/// DC_LAYOUT
	#[repr(transparent)]
	pub struct DcLayout: u32 {
		/// LAYOUT_BITMAPORIENTATIONPRESERVED = 0x8u32
		const BitmapOrientationPreserved = 0x8u32;
		/// LAYOUT_RTL = 0x1u32
		const Rtl                  = 0x1u32;
	}
}

/// R2_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum R2Mode
{
	/// R2_BLACK = 0x1i32
	Black                = 0x1i32,
	/// R2_NOTMERGEPEN = 0x2i32
	NotMergePen          = 0x2i32,
	/// R2_MASKNOTPEN = 0x3i32
	MasKnotPen           = 0x3i32,
	/// R2_NOTCOPYPEN = 0x4i32
	NotCopyPen           = 0x4i32,
	/// R2_MASKPENNOT = 0x5i32
	MaskPenNot           = 0x5i32,
	/// R2_NOT = 0x6i32
	Not                  = 0x6i32,
	/// R2_XORPEN = 0x7i32
	XorPen               = 0x7i32,
	/// R2_NOTMASKPEN = 0x8i32
	NotMaskPen           = 0x8i32,
	/// R2_MASKPEN = 0x9i32
	MaskPen              = 0x9i32,
	/// R2_NOTXORPEN = 0xAi32
	NotXorPen            = 0xAi32,
	/// R2_NOP = 0xBi32
	Nop                  = 0xBi32,
	/// R2_MERGENOTPEN = 0xCi32
	MergeNotPen          = 0xCi32,
	/// R2_COPYPEN = 0xDi32
	CopyPen              = 0xDi32,
	/// R2_MERGEPENNOT = 0xEi32
	MergePenNot          = 0xEi32,
	/// R2_MERGEPEN = 0xFi32
	MergePen             = 0xFi32,
	/// R2_WHITE = 0x10i32
	White                = 0x10i32,
}

impl R2Mode {
	/// R2_LAST = 0x10i32
	pub const Last                : Self = unsafe { transmute(0x10i32) };
}

/// STRETCH_BLT_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StretchBltMode
{
	/// BLACKONWHITE = 0x1u32
	BLacKonWhite         = 0x1u32,
	/// COLORONCOLOR = 0x3u32
	ColorOnColor         = 0x3u32,
	/// HALFTONE = 0x4u32
	Halftone             = 0x4u32,
	/// STRETCH_ORSCANS = 0x2u32
	OrScans              = 0x2u32,
}

impl StretchBltMode {
	/// STRETCH_ANDSCANS = 0x1u32
	pub const AndScans            : Self = unsafe { transmute(0x1u32) };
	/// STRETCH_DELETESCANS = 0x3u32
	pub const DeleteScans         : Self = unsafe { transmute(0x3u32) };
	/// WHITEONBLACK = 0x2u32
	pub const WhiteOnBlack        : Self = unsafe { transmute(0x2u32) };
}

/// SYSTEM_PALETTE_USE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SystemPaletteUse
{
	/// SYSPAL_NOSTATIC = 0x2u32
	NoStatic             = 0x2u32,
	/// SYSPAL_NOSTATIC256 = 0x3u32
	NoStatic256          = 0x3u32,
	/// SYSPAL_STATIC = 0x1u32
	Static               = 0x1u32,
}

/// TEXT_ALIGN_OPTIONS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TextAlignOptions
{
	/// TA_NOUPDATECP = 0x0u32
	NoUpdateCp           = 0x0u32,
	/// TA_UPDATECP = 0x1u32
	UpdateCp             = 0x1u32,
	/// TA_RIGHT = 0x2u32
	Right                = 0x2u32,
	/// TA_CENTER = 0x6u32
	Center               = 0x6u32,
	/// TA_BOTTOM = 0x8u32
	Bottom               = 0x8u32,
	/// TA_BASELINE = 0x18u32
	Baseline             = 0x18u32,
	/// TA_RTLREADING = 0x100u32
	RtlReading           = 0x100u32,
	/// TA_MASK = 0x11Fu32
	Mask                 = 0x11Fu32,
}

impl TextAlignOptions {
	/// TA_LEFT = 0x0u32
	pub const Left                : Self = unsafe { transmute(0x0u32) };
	/// TA_TOP = 0x0u32
	pub const Top                 : Self = unsafe { transmute(0x0u32) };
}

/// GRADIENT_FILL
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GradientFill
{
	/// GRADIENT_FILL_RECT_H = 0x0u32
	RectH                = 0x0u32,
	/// GRADIENT_FILL_RECT_V = 0x1u32
	RectV                = 0x1u32,
	/// GRADIENT_FILL_TRIANGLE = 0x2u32
	Triangle             = 0x2u32,
}

/// MODIFY_WORLD_TRANSFORM_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModifyWorldTransformMode
{
	/// MWT_IDENTITY = 0x1u32
	Identity             = 0x1u32,
	/// MWT_LEFTMULTIPLY = 0x2u32
	LeftMultiply         = 0x2u32,
	/// MWT_RIGHTMULTIPLY = 0x3u32
	RightMultiply        = 0x3u32,
}

/// ARC_DIRECTION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ArcDirection
{
	/// AD_COUNTERCLOCKWISE = 0x1u32
	Counterclockwise     = 0x1u32,
	/// AD_CLOCKWISE = 0x2u32
	Clockwise            = 0x2u32,
}

bitflags::bitflags! {
	/// ETO_OPTIONS
	#[repr(transparent)]
	pub struct EtoOptions: u32 {
		/// ETO_OPAQUE = 0x2u32
		const Opaque               = 0x2u32;
		/// ETO_CLIPPED = 0x4u32
		const Clipped              = 0x4u32;
		/// ETO_GLYPH_INDEX = 0x10u32
		const GlyphIndex           = 0x10u32;
		/// ETO_RTLREADING = 0x80u32
		const RtlReading           = 0x80u32;
		/// ETO_NUMERICSLOCAL = 0x400u32
		const NumericsLocal        = 0x400u32;
		/// ETO_NUMERICSLATIN = 0x800u32
		const NumericsLatin        = 0x800u32;
		/// ETO_IGNORELANGUAGE = 0x1000u32
		const IgnoreLanguage       = 0x1000u32;
		/// ETO_PDY = 0x2000u32
		const Pdy                  = 0x2000u32;
		/// ETO_REVERSE_INDEX_MAP = 0x10000u32
		const ReverseIndexMap      = 0x10000u32;
	}
}

bitflags::bitflags! {
	/// TTEMBED_FLAGS
	#[repr(transparent)]
	pub struct TtEmbedFlags: u32 {
		/// TTEMBED_EMBEDEUDC = 0x20u32
		const EMBedeUDc            = 0x20u32;
		/// TTEMBED_RAW = 0x0u32
		const Raw                  = 0x0u32;
		/// TTEMBED_SUBSET = 0x1u32
		const Subset               = 0x1u32;
		/// TTEMBED_TTCOMPRESSED = 0x4u32
		const TtCompressed         = 0x4u32;
	}
}

/// EMBED_FONT_CHARSET
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EmbedFontCharSet
{
	/// CHARSET_UNICODE = 0x1u32
	Unicode              = 0x1u32,
	/// CHARSET_SYMBOL = 0x2u32
	Symbol               = 0x2u32,
}

/// EMBEDDED_FONT_PRIV_STATUS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EmbeddedFontPrivStatus
{
	/// EMBED_PREVIEWPRINT = 0x1u32
	PreviewPrint         = 0x1u32,
	/// EMBED_EDITABLE = 0x2u32
	Editable             = 0x2u32,
	/// EMBED_INSTALLABLE = 0x3u32
	InstallAble          = 0x3u32,
	/// EMBED_NOEMBEDDING = 0x4u32
	NoEmbedding          = 0x4u32,
}

/// FONT_LICENSE_PRIVS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FontLicensePRIVs
{
	/// LICENSE_PREVIEWPRINT = 0x4u32
	PreviewPrint         = 0x4u32,
	/// LICENSE_EDITABLE = 0x8u32
	Editable             = 0x8u32,
	/// LICENSE_INSTALLABLE = 0x0u32
	InstallAble          = 0x0u32,
	/// LICENSE_NOEMBEDDING = 0x2u32
	NoEmbedding          = 0x2u32,
}

impl FontLicensePRIVs {
	/// LICENSE_DEFAULT = 0x0u32
	pub const Default             : Self = unsafe { transmute(0x0u32) };
}

bitflags::bitflags! {
	/// TTLOAD_EMBEDDED_FONT_STATUS
	#[repr(transparent)]
	pub struct TtLoadEmbeddedFontStatus: u32 {
		/// TTLOAD_FONT_SUBSETTED = 0x1u32
		const Subsetted            = 0x1u32;
		/// TTLOAD_FONT_IN_SYSSTARTUP = 0x2u32
		const InSysStartup         = 0x2u32;
	}
}

bitflags::bitflags! {
	/// DRAWEDGE_FLAGS
	#[repr(transparent)]
	pub struct DrawedgeFlags: u32 {
		/// BDR_RAISEDOUTER = 0x1u32
		const RaisedOuter          = 0x1u32;
		/// BDR_SUNKENOUTER = 0x2u32
		const SunkenOuter          = 0x2u32;
		/// BDR_RAISEDINNER = 0x4u32
		const RaiseDinner          = 0x4u32;
		/// BDR_SUNKENINNER = 0x8u32
		const SunkenInner          = 0x8u32;
		/// BDR_OUTER = 0x3u32
		const Outer                = 0x3u32;
		/// BDR_INNER = 0xCu32
		const Inner                = 0xCu32;
		/// BDR_RAISED = 0x5u32
		const Raised               = 0x5u32;
		/// BDR_SUNKEN = 0xAu32
		const Sunken               = 0xAu32;
		/// EDGE_ETCHED = 0x6u32
		const Etched               = 0x6u32;
		/// EDGE_BUMP = 0x9u32
		const Bump                 = 0x9u32;
	}
}

bitflags::bitflags! {
	/// DRAW_EDGE_FLAGS
	#[repr(transparent)]
	pub struct DrawEdgeFlags: u32 {
		/// BF_ADJUST = 0x2000u32
		const Adjust               = 0x2000u32;
		/// BF_BOTTOM = 0x8u32
		const Bottom               = 0x8u32;
		/// BF_BOTTOMLEFT = 0x9u32
		const BottomLeft           = 0x9u32;
		/// BF_BOTTOMRIGHT = 0xCu32
		const BottomRight          = 0xCu32;
		/// BF_DIAGONAL = 0x10u32
		const Diagonal             = 0x10u32;
		/// BF_DIAGONAL_ENDBOTTOMLEFT = 0x19u32
		const DiagonalEndBottomLeft = 0x19u32;
		/// BF_DIAGONAL_ENDBOTTOMRIGHT = 0x1Cu32
		const DiagonalEndBottomRight = 0x1Cu32;
		/// BF_DIAGONAL_ENDTOPLEFT = 0x13u32
		const DiagonalEndTopLeft   = 0x13u32;
		/// BF_DIAGONAL_ENDTOPRIGHT = 0x16u32
		const DiagonalEndTopRight  = 0x16u32;
		/// BF_FLAT = 0x4000u32
		const Flat                 = 0x4000u32;
		/// BF_LEFT = 0x1u32
		const Left                 = 0x1u32;
		/// BF_MIDDLE = 0x800u32
		const Middle               = 0x800u32;
		/// BF_MONO = 0x8000u32
		const Mono                 = 0x8000u32;
		/// BF_RECT = 0xFu32
		const Rect                 = 0xFu32;
		/// BF_RIGHT = 0x4u32
		const Right                = 0x4u32;
		/// BF_SOFT = 0x1000u32
		const Soft                 = 0x1000u32;
		/// BF_TOP = 0x2u32
		const Top                  = 0x2u32;
		/// BF_TOPLEFT = 0x3u32
		const TopLeft              = 0x3u32;
		/// BF_TOPRIGHT = 0x6u32
		const TopRight             = 0x6u32;
	}
}

/// DFC_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DfcType
{
	/// DFC_CAPTION = 0x1u32
	Caption              = 0x1u32,
	/// DFC_MENU = 0x2u32
	Menu                 = 0x2u32,
	/// DFC_SCROLL = 0x3u32
	Scroll               = 0x3u32,
	/// DFC_BUTTON = 0x4u32
	Button               = 0x4u32,
	/// DFC_POPUPMENU = 0x5u32
	PopupMenu            = 0x5u32,
}

bitflags::bitflags! {
	/// DFCS_STATE
	#[repr(transparent)]
	pub struct DfcsState: u32 {
		/// DFCS_CAPTIONCLOSE = 0x0u32
		const CaptionClose         = 0x0u32;
		/// DFCS_CAPTIONMIN = 0x1u32
		const CaptionMin           = 0x1u32;
		/// DFCS_CAPTIONMAX = 0x2u32
		const CaptionMax           = 0x2u32;
		/// DFCS_CAPTIONRESTORE = 0x3u32
		const CaptionRestore       = 0x3u32;
		/// DFCS_CAPTIONHELP = 0x4u32
		const CaptionHelp          = 0x4u32;
		/// DFCS_MENUARROW = 0x0u32
		const MenuArrow            = 0x0u32;
		/// DFCS_MENUCHECK = 0x1u32
		const MenuCheck            = 0x1u32;
		/// DFCS_MENUBULLET = 0x2u32
		const MenuBullet           = 0x2u32;
		/// DFCS_MENUARROWRIGHT = 0x4u32
		const MenuArrOWright       = 0x4u32;
		/// DFCS_SCROLLUP = 0x0u32
		const ScrollUp             = 0x0u32;
		/// DFCS_SCROLLDOWN = 0x1u32
		const ScrollDown           = 0x1u32;
		/// DFCS_SCROLLLEFT = 0x2u32
		const ScrollLeft           = 0x2u32;
		/// DFCS_SCROLLRIGHT = 0x3u32
		const ScrollRight          = 0x3u32;
		/// DFCS_SCROLLCOMBOBOX = 0x5u32
		const ScrollComboBox       = 0x5u32;
		/// DFCS_SCROLLSIZEGRIP = 0x8u32
		const ScrollSizeGrip       = 0x8u32;
		/// DFCS_SCROLLSIZEGRIPRIGHT = 0x10u32
		const ScrollSizeGripRight  = 0x10u32;
		/// DFCS_BUTTONCHECK = 0x0u32
		const ButtonCheck          = 0x0u32;
		/// DFCS_BUTTONRADIOIMAGE = 0x1u32
		const ButtonRadioImage     = 0x1u32;
		/// DFCS_BUTTONRADIOMASK = 0x2u32
		const ButtonRadioMask      = 0x2u32;
		/// DFCS_BUTTONRADIO = 0x4u32
		const ButtonRadio          = 0x4u32;
		/// DFCS_BUTTON3STATE = 0x8u32
		const Button3State         = 0x8u32;
		/// DFCS_BUTTONPUSH = 0x10u32
		const ButtonPush           = 0x10u32;
		/// DFCS_INACTIVE = 0x100u32
		const Inactive             = 0x100u32;
		/// DFCS_PUSHED = 0x200u32
		const Pushed               = 0x200u32;
		/// DFCS_CHECKED = 0x400u32
		const Checked              = 0x400u32;
		/// DFCS_TRANSPARENT = 0x800u32
		const Transparent          = 0x800u32;
		/// DFCS_HOT = 0x1000u32
		const Hot                  = 0x1000u32;
		/// DFCS_ADJUSTRECT = 0x2000u32
		const AdjustRect           = 0x2000u32;
		/// DFCS_FLAT = 0x4000u32
		const Flat                 = 0x4000u32;
		/// DFCS_MONO = 0x8000u32
		const Mono                 = 0x8000u32;
	}
}

bitflags::bitflags! {
	/// DRAW_CAPTION_FLAGS
	#[repr(transparent)]
	pub struct DrawCaptionFlags: u32 {
		/// DC_ACTIVE = 0x1u32
		const Active               = 0x1u32;
		/// DC_BUTTONS = 0x1000u32
		const Buttons              = 0x1000u32;
		/// DC_GRADIENT = 0x20u32
		const Gradient             = 0x20u32;
		/// DC_ICON = 0x4u32
		const Icon                 = 0x4u32;
		/// DC_INBUTTON = 0x10u32
		const InButton             = 0x10u32;
		/// DC_SMALLCAP = 0x2u32
		const SmallCap             = 0x2u32;
		/// DC_TEXT = 0x8u32
		const Text                 = 0x8u32;
	}
}

bitflags::bitflags! {
	/// DRAW_TEXT_FORMAT
	#[repr(transparent)]
	pub struct DrawTextFormat: u32 {
		/// DT_BOTTOM = 0x8u32
		const Bottom               = 0x8u32;
		/// DT_CALCRECT = 0x400u32
		const CalcRect             = 0x400u32;
		/// DT_CENTER = 0x1u32
		const Center               = 0x1u32;
		/// DT_EDITCONTROL = 0x2000u32
		const EditControl          = 0x2000u32;
		/// DT_END_ELLIPSIS = 0x8000u32
		const EndEllipsis          = 0x8000u32;
		/// DT_EXPANDTABS = 0x40u32
		const ExpandTabs           = 0x40u32;
		/// DT_EXTERNALLEADING = 0x200u32
		const ExternalLeading      = 0x200u32;
		/// DT_HIDEPREFIX = 0x100000u32
		const HidePrefix           = 0x100000u32;
		/// DT_INTERNAL = 0x1000u32
		const Internal             = 0x1000u32;
		/// DT_LEFT = 0x0u32
		const Left                 = 0x0u32;
		/// DT_MODIFYSTRING = 0x10000u32
		const ModifyString         = 0x10000u32;
		/// DT_NOCLIP = 0x100u32
		const NoClip               = 0x100u32;
		/// DT_NOFULLWIDTHCHARBREAK = 0x80000u32
		const NoFullWidthCharBreak = 0x80000u32;
		/// DT_NOPREFIX = 0x800u32
		const NoPrefix             = 0x800u32;
		/// DT_PATH_ELLIPSIS = 0x4000u32
		const PathEllipsis         = 0x4000u32;
		/// DT_PREFIXONLY = 0x200000u32
		const PrefixOnly           = 0x200000u32;
		/// DT_RIGHT = 0x2u32
		const Right                = 0x2u32;
		/// DT_RTLREADING = 0x20000u32
		const RtlReading           = 0x20000u32;
		/// DT_SINGLELINE = 0x20u32
		const SingleLine           = 0x20u32;
		/// DT_TABSTOP = 0x80u32
		const TabStop              = 0x80u32;
		/// DT_TOP = 0x0u32
		const Top                  = 0x0u32;
		/// DT_VCENTER = 0x4u32
		const VCenter              = 0x4u32;
		/// DT_WORDBREAK = 0x10u32
		const Wordbreak            = 0x10u32;
		/// DT_WORD_ELLIPSIS = 0x40000u32
		const WordEllipsis         = 0x40000u32;
	}
}

bitflags::bitflags! {
	/// GET_DCX_FLAGS
	#[repr(transparent)]
	pub struct GetDcxFlags: u32 {
		/// DCX_WINDOW = 0x1u32
		const Window               = 0x1u32;
		/// DCX_CACHE = 0x2u32
		const Cache                = 0x2u32;
		/// DCX_PARENTCLIP = 0x20u32
		const ParentClip           = 0x20u32;
		/// DCX_CLIPSIBLINGS = 0x10u32
		const ClipSiblings         = 0x10u32;
		/// DCX_CLIPCHILDREN = 0x8u32
		const ClipChildren         = 0x8u32;
		/// DCX_NORESETATTRS = 0x4u32
		const NoResetAttrs         = 0x4u32;
		/// DCX_LOCKWINDOWUPDATE = 0x400u32
		const LockWindowUpdate     = 0x400u32;
		/// DCX_EXCLUDERGN = 0x40u32
		const ExcludeRgn           = 0x40u32;
		/// DCX_INTERSECTRGN = 0x80u32
		const IntersectRgn         = 0x80u32;
		/// DCX_INTERSECTUPDATE = 0x200u32
		const IntersectUpdate      = 0x200u32;
		/// DCX_VALIDATE = 0x200000u32
		const Validate             = 0x200000u32;
	}
}

bitflags::bitflags! {
	/// REDRAW_WINDOW_FLAGS
	#[repr(transparent)]
	pub struct RedrawWindowFlags: u32 {
		/// RDW_INVALIDATE = 0x1u32
		const Invalidate           = 0x1u32;
		/// RDW_INTERNALPAINT = 0x2u32
		const InternalPaint        = 0x2u32;
		/// RDW_ERASE = 0x4u32
		const Erase                = 0x4u32;
		/// RDW_VALIDATE = 0x8u32
		const Validate             = 0x8u32;
		/// RDW_NOINTERNALPAINT = 0x10u32
		const NoInternalPaint      = 0x10u32;
		/// RDW_NOERASE = 0x20u32
		const NoErase              = 0x20u32;
		/// RDW_NOCHILDREN = 0x40u32
		const NoChildren           = 0x40u32;
		/// RDW_ALLCHILDREN = 0x80u32
		const AllChildren          = 0x80u32;
		/// RDW_UPDATENOW = 0x100u32
		const UpdateNow            = 0x100u32;
		/// RDW_ERASENOW = 0x200u32
		const EraseNow             = 0x200u32;
		/// RDW_FRAME = 0x400u32
		const Frame                = 0x400u32;
		/// RDW_NOFRAME = 0x800u32
		const NoFrame              = 0x800u32;
	}
}

/// DISP_CHANGE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DispChange
{
	/// DISP_CHANGE_SUCCESSFUL = 0x0i32
	Successful           = 0x0i32,
	/// DISP_CHANGE_RESTART = 0x1i32
	Restart              = 0x1i32,
	/// DISP_CHANGE_FAILED = -0x1i32
	Failed               = -0x1i32,
	/// DISP_CHANGE_BADMODE = -0x2i32
	BadMode              = -0x2i32,
	/// DISP_CHANGE_NOTUPDATED = -0x3i32
	NotUpdated           = -0x3i32,
	/// DISP_CHANGE_BADFLAGS = -0x4i32
	BadFlags             = -0x4i32,
	/// DISP_CHANGE_BADPARAM = -0x5i32
	BadParam             = -0x5i32,
	/// DISP_CHANGE_BADDUALVIEW = -0x6i32
	BadDualView          = -0x6i32,
}

bitflags::bitflags! {
	/// CDS_TYPE
	#[repr(transparent)]
	pub struct CdsType: u32 {
		/// CDS_FULLSCREEN = 0x4u32
		const FullScreen           = 0x4u32;
		/// CDS_GLOBAL = 0x8u32
		const Global               = 0x8u32;
		/// CDS_NORESET = 0x10000000u32
		const NoReset              = 0x10000000u32;
		/// CDS_RESET = 0x40000000u32
		const Reset                = 0x40000000u32;
		/// CDS_SET_PRIMARY = 0x10u32
		const SetPrimary           = 0x10u32;
		/// CDS_TEST = 0x2u32
		const Test                 = 0x2u32;
		/// CDS_UPDATEREGISTRY = 0x1u32
		const UpdateRegistry       = 0x1u32;
		/// CDS_VIDEOPARAMETERS = 0x20u32
		const VideoParameters      = 0x20u32;
		/// CDS_ENABLE_UNSAFE_MODES = 0x100u32
		const EnableUnsafeModes    = 0x100u32;
		/// CDS_DISABLE_UNSAFE_MODES = 0x200u32
		const DisableUnsafeModes   = 0x200u32;
		/// CDS_RESET_EX = 0x20000000u32
		const ResetEx              = 0x20000000u32;
	}
}

/// ENUM_DISPLAY_SETTINGS_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EnumDisplaySettingsMode
{
	/// ENUM_CURRENT_SETTINGS = 0xFFFFFFFFu32
	CurrentSettings      = 0xFFFFFFFFu32,
	/// ENUM_REGISTRY_SETTINGS = 0xFFFFFFFEu32
	RegistrySettings     = 0xFFFFFFFEu32,
}

/// MONITOR_FROM_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MonitorFromFlags
{
	/// MONITOR_DEFAULTTONEAREST = 0x2u32
	DefaultToNearest     = 0x2u32,
	/// MONITOR_DEFAULTTONULL = 0x0u32
	DefaultToNull        = 0x0u32,
	/// MONITOR_DEFAULTTOPRIMARY = 0x1u32
	DefaultToPrimary     = 0x1u32,
}

