#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// DWRITE_MEASURING_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteMeasuringMode
{
	/// DWRITE_MEASURING_MODE_NATURAL = 0x0i32
	Natural              = 0x0i32,
	/// DWRITE_MEASURING_MODE_GDI_CLASSIC = 0x1i32
	GdiClassic           = 0x1i32,
	/// DWRITE_MEASURING_MODE_GDI_NATURAL = 0x2i32
	GdiNatural           = 0x2i32,
}

bitflags::bitflags! {
	/// DWRITE_GLYPH_IMAGE_FORMATS
	#[repr(transparent)]
	pub struct DWriteGlyphImageFormats: u32 {
		/// DWRITE_GLYPH_IMAGE_FORMATS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE = 0x1u32
		const TrueType             = 0x1u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_CFF = 0x2u32
		const Cff                  = 0x2u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_COLR = 0x4u32
		const ColR                 = 0x4u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_SVG = 0x8u32
		const Svg                  = 0x8u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_PNG = 0x10u32
		const Png                  = 0x10u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_JPEG = 0x20u32
		const Jpeg                 = 0x20u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_TIFF = 0x40u32
		const Tiff                 = 0x40u32;
		/// DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8 = 0x80u32
		const PremultipliedB8G8R8A8 = 0x80u32;
	}
}

/// DWRITE_PIXEL_GEOMETRY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWritePixelGeometry
{
	/// DWRITE_PIXEL_GEOMETRY_FLAT = 0x0i32
	Flat                 = 0x0i32,
	/// DWRITE_PIXEL_GEOMETRY_RGB = 0x1i32
	Rgb                  = 0x1i32,
	/// DWRITE_PIXEL_GEOMETRY_BGR = 0x2i32
	Bgr                  = 0x2i32,
}

/// DWRITE_RENDERING_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteRenderingMode
{
	/// DWRITE_RENDERING_MODE_DEFAULT = 0x0i32
	Default              = 0x0i32,
	/// DWRITE_RENDERING_MODE_ALIASED = 0x1i32
	Aliased              = 0x1i32,
	/// DWRITE_RENDERING_MODE_GDI_CLASSIC = 0x2i32
	GdiClassic           = 0x2i32,
	/// DWRITE_RENDERING_MODE_GDI_NATURAL = 0x3i32
	GdiNatural           = 0x3i32,
	/// DWRITE_RENDERING_MODE_NATURAL = 0x4i32
	Natural              = 0x4i32,
	/// DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC = 0x5i32
	NaturalSymmetric     = 0x5i32,
	/// DWRITE_RENDERING_MODE_OUTLINE = 0x6i32
	Outline              = 0x6i32,
}

impl DWriteRenderingMode {
	/// DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC = 0x2i32
	pub const ClearTypeGdiClassic : Self = unsafe { transmute(0x2i32) };
	/// DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL = 0x3i32
	pub const ClearTypeGdiNatural : Self = unsafe { transmute(0x3i32) };
	/// DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL = 0x4i32
	pub const ClearTypeNatural    : Self = unsafe { transmute(0x4i32) };
	/// DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC = 0x5i32
	pub const ClearTypeNaturalSymmetric: Self = unsafe { transmute(0x5i32) };
}

/// DWRITE_TEXT_ALIGNMENT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteTextAlignment
{
	/// DWRITE_TEXT_ALIGNMENT_LEADING = 0x0i32
	Leading              = 0x0i32,
	/// DWRITE_TEXT_ALIGNMENT_TRAILING = 0x1i32
	Trailing             = 0x1i32,
	/// DWRITE_TEXT_ALIGNMENT_CENTER = 0x2i32
	Center               = 0x2i32,
	/// DWRITE_TEXT_ALIGNMENT_JUSTIFIED = 0x3i32
	Justified            = 0x3i32,
}

/// DWRITE_PARAGRAPH_ALIGNMENT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteParagraphAlignment
{
	/// DWRITE_PARAGRAPH_ALIGNMENT_NEAR = 0x0i32
	Near                 = 0x0i32,
	/// DWRITE_PARAGRAPH_ALIGNMENT_FAR = 0x1i32
	Far                  = 0x1i32,
	/// DWRITE_PARAGRAPH_ALIGNMENT_CENTER = 0x2i32
	Center               = 0x2i32,
}

/// DWRITE_WORD_WRAPPING
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteWordWrapping
{
	/// DWRITE_WORD_WRAPPING_WRAP = 0x0i32
	Wrap                 = 0x0i32,
	/// DWRITE_WORD_WRAPPING_NO_WRAP = 0x1i32
	NoWrap               = 0x1i32,
	/// DWRITE_WORD_WRAPPING_EMERGENCY_BREAK = 0x2i32
	EmergencyBreak       = 0x2i32,
	/// DWRITE_WORD_WRAPPING_WHOLE_WORD = 0x3i32
	WholeWord            = 0x3i32,
	/// DWRITE_WORD_WRAPPING_CHARACTER = 0x4i32
	Character            = 0x4i32,
}

/// DWRITE_READING_DIRECTION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteReadingDirection
{
	/// DWRITE_READING_DIRECTION_LEFT_TO_RIGHT = 0x0i32
	LeftToRight          = 0x0i32,
	/// DWRITE_READING_DIRECTION_RIGHT_TO_LEFT = 0x1i32
	RightToLeft          = 0x1i32,
	/// DWRITE_READING_DIRECTION_TOP_TO_BOTTOM = 0x2i32
	TopToBottom          = 0x2i32,
	/// DWRITE_READING_DIRECTION_BOTTOM_TO_TOP = 0x3i32
	BottomToTop          = 0x3i32,
}

/// DWRITE_FLOW_DIRECTION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteFlowDirection
{
	/// DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM = 0x0i32
	TopToBottom          = 0x0i32,
	/// DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP = 0x1i32
	BottomToTop          = 0x1i32,
	/// DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT = 0x2i32
	LeftToRight          = 0x2i32,
	/// DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT = 0x3i32
	RightToLeft          = 0x3i32,
}

/// DWRITE_TRIMMING_GRANULARITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteTrimmingGranularity
{
	/// DWRITE_TRIMMING_GRANULARITY_NONE = 0x0i32
	None                 = 0x0i32,
	/// DWRITE_TRIMMING_GRANULARITY_CHARACTER = 0x1i32
	Character            = 0x1i32,
	/// DWRITE_TRIMMING_GRANULARITY_WORD = 0x2i32
	Word                 = 0x2i32,
}

/// DWRITE_LINE_SPACING_METHOD
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteLineSpacingMethod
{
	/// DWRITE_LINE_SPACING_METHOD_DEFAULT = 0x0i32
	Default              = 0x0i32,
	/// DWRITE_LINE_SPACING_METHOD_UNIFORM = 0x1i32
	Uniform              = 0x1i32,
	/// DWRITE_LINE_SPACING_METHOD_PROPORTIONAL = 0x2i32
	Proportional         = 0x2i32,
}

/// DWRITE_FONT_WEIGHT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteFontWeight
{
	/// DWRITE_FONT_WEIGHT_THIN = 0x64i32
	Thin                 = 0x64i32,
	/// DWRITE_FONT_WEIGHT_EXTRA_LIGHT = 0xC8i32
	ExtraLight           = 0xC8i32,
	/// DWRITE_FONT_WEIGHT_LIGHT = 0x12Ci32
	Light                = 0x12Ci32,
	/// DWRITE_FONT_WEIGHT_SEMI_LIGHT = 0x15Ei32
	SemiLight            = 0x15Ei32,
	/// DWRITE_FONT_WEIGHT_NORMAL = 0x190i32
	Normal               = 0x190i32,
	/// DWRITE_FONT_WEIGHT_MEDIUM = 0x1F4i32
	Medium               = 0x1F4i32,
	/// DWRITE_FONT_WEIGHT_DEMI_BOLD = 0x258i32
	DemiBold             = 0x258i32,
	/// DWRITE_FONT_WEIGHT_BOLD = 0x2BCi32
	Bold                 = 0x2BCi32,
	/// DWRITE_FONT_WEIGHT_EXTRA_BOLD = 0x320i32
	ExtraBold            = 0x320i32,
	/// DWRITE_FONT_WEIGHT_BLACK = 0x384i32
	Black                = 0x384i32,
	/// DWRITE_FONT_WEIGHT_EXTRA_BLACK = 0x3B6i32
	ExtraBlack           = 0x3B6i32,
}

impl DWriteFontWeight {
	/// DWRITE_FONT_WEIGHT_ULTRA_LIGHT = 0xC8i32
	pub const UltraLight          : Self = unsafe { transmute(0xC8i32) };
	/// DWRITE_FONT_WEIGHT_REGULAR = 0x190i32
	pub const Regular             : Self = unsafe { transmute(0x190i32) };
	/// DWRITE_FONT_WEIGHT_SEMI_BOLD = 0x258i32
	pub const SemiBold            : Self = unsafe { transmute(0x258i32) };
	/// DWRITE_FONT_WEIGHT_ULTRA_BOLD = 0x320i32
	pub const UltraBold           : Self = unsafe { transmute(0x320i32) };
	/// DWRITE_FONT_WEIGHT_HEAVY = 0x384i32
	pub const Heavy               : Self = unsafe { transmute(0x384i32) };
	/// DWRITE_FONT_WEIGHT_ULTRA_BLACK = 0x3B6i32
	pub const UltraBlack          : Self = unsafe { transmute(0x3B6i32) };
}

/// DWRITE_FONT_STYLE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteFontStyle
{
	/// DWRITE_FONT_STYLE_NORMAL = 0x0i32
	Normal               = 0x0i32,
	/// DWRITE_FONT_STYLE_OBLIQUE = 0x1i32
	Oblique              = 0x1i32,
	/// DWRITE_FONT_STYLE_ITALIC = 0x2i32
	Italic               = 0x2i32,
}

/// DWRITE_FONT_STRETCH
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteFontStretch
{
	/// DWRITE_FONT_STRETCH_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// DWRITE_FONT_STRETCH_ULTRA_CONDENSED = 0x1i32
	UltraCondensed       = 0x1i32,
	/// DWRITE_FONT_STRETCH_EXTRA_CONDENSED = 0x2i32
	ExtraCondensed       = 0x2i32,
	/// DWRITE_FONT_STRETCH_CONDENSED = 0x3i32
	Condensed            = 0x3i32,
	/// DWRITE_FONT_STRETCH_SEMI_CONDENSED = 0x4i32
	SemiCondensed        = 0x4i32,
	/// DWRITE_FONT_STRETCH_NORMAL = 0x5i32
	Normal               = 0x5i32,
	/// DWRITE_FONT_STRETCH_SEMI_EXPANDED = 0x6i32
	SemiExpanded         = 0x6i32,
	/// DWRITE_FONT_STRETCH_EXPANDED = 0x7i32
	Expanded             = 0x7i32,
	/// DWRITE_FONT_STRETCH_EXTRA_EXPANDED = 0x8i32
	ExtraExpanded        = 0x8i32,
	/// DWRITE_FONT_STRETCH_ULTRA_EXPANDED = 0x9i32
	UltraExpanded        = 0x9i32,
}

impl DWriteFontStretch {
	/// DWRITE_FONT_STRETCH_MEDIUM = 0x5i32
	pub const Medium              : Self = unsafe { transmute(0x5i32) };
}

/// DWRITE_FONT_FACE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteFontFaceType
{
	/// DWRITE_FONT_FACE_TYPE_CFF = 0x0i32
	Cff                  = 0x0i32,
	/// DWRITE_FONT_FACE_TYPE_TRUETYPE = 0x1i32
	TrueType             = 0x1i32,
	/// DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION = 0x2i32
	OpenTypeCollection   = 0x2i32,
	/// DWRITE_FONT_FACE_TYPE_TYPE1 = 0x3i32
	Type1                = 0x3i32,
	/// DWRITE_FONT_FACE_TYPE_VECTOR = 0x4i32
	Vector               = 0x4i32,
	/// DWRITE_FONT_FACE_TYPE_BITMAP = 0x5i32
	Bitmap               = 0x5i32,
	/// DWRITE_FONT_FACE_TYPE_UNKNOWN = 0x6i32
	Unknown              = 0x6i32,
	/// DWRITE_FONT_FACE_TYPE_RAW_CFF = 0x7i32
	RawCff               = 0x7i32,
}

impl DWriteFontFaceType {
	/// DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION = 0x2i32
	pub const TrueTypeCollection  : Self = unsafe { transmute(0x2i32) };
}

bitflags::bitflags! {
	/// DWRITE_FONT_SIMULATIONS
	#[repr(transparent)]
	pub struct DWriteFontSimulations: u32 {
		/// DWRITE_FONT_SIMULATIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// DWRITE_FONT_SIMULATIONS_BOLD = 0x1u32
		const Bold                 = 0x1u32;
		/// DWRITE_FONT_SIMULATIONS_OBLIQUE = 0x2u32
		const Oblique              = 0x2u32;
	}
}

/// DWRITE_BREAK_CONDITION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteBreakCondition
{
	/// DWRITE_BREAK_CONDITION_NEUTRAL = 0x0i32
	Neutral              = 0x0i32,
	/// DWRITE_BREAK_CONDITION_CAN_BREAK = 0x1i32
	CanBreak             = 0x1i32,
	/// DWRITE_BREAK_CONDITION_MAY_NOT_BREAK = 0x2i32
	MayNotBreak          = 0x2i32,
	/// DWRITE_BREAK_CONDITION_MUST_BREAK = 0x3i32
	MustBreak            = 0x3i32,
}

/// DWRITE_FONT_FEATURE_TAG
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteFontFeatureTag
{
	/// DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS = 0x63726661u32
	AlternativeFractions = 0x63726661u32,
	/// DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS = 0x63703263u32
	PetiteCapitalsFromCapitals = 0x63703263u32,
	/// DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS = 0x63733263u32
	SmallCapitalsFromCapitals = 0x63733263u32,
	/// DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES = 0x746C6163u32
	ContextualAlternates = 0x746C6163u32,
	/// DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS = 0x65736163u32
	CaseSensitiveForms   = 0x65736163u32,
	/// DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION = 0x706D6363u32
	GlyphCompositionDecomposition = 0x706D6363u32,
	/// DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES = 0x67696C63u32
	ContextualLigatures  = 0x67696C63u32,
	/// DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING = 0x70737063u32
	CapitalSpacing       = 0x70737063u32,
	/// DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH = 0x68777363u32
	ContextualSwash      = 0x68777363u32,
	/// DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING = 0x73727563u32
	CursivePositioning   = 0x73727563u32,
	/// DWRITE_FONT_FEATURE_TAG_DEFAULT = 0x746C6664u32
	Default              = 0x746C6664u32,
	/// DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES = 0x67696C64u32
	DiscretionaryLigatures = 0x67696C64u32,
	/// DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS = 0x74707865u32
	ExpertForms          = 0x74707865u32,
	/// DWRITE_FONT_FEATURE_TAG_FRACTIONS = 0x63617266u32
	Fractions            = 0x63617266u32,
	/// DWRITE_FONT_FEATURE_TAG_FULL_WIDTH = 0x64697766u32
	FullWidth            = 0x64697766u32,
	/// DWRITE_FONT_FEATURE_TAG_HALF_FORMS = 0x666C6168u32
	HalfForms            = 0x666C6168u32,
	/// DWRITE_FONT_FEATURE_TAG_HALANT_FORMS = 0x6E6C6168u32
	HAlantForms          = 0x6E6C6168u32,
	/// DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH = 0x746C6168u32
	AlternateHalfWidth   = 0x746C6168u32,
	/// DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS = 0x74736968u32
	HistoricalForms      = 0x74736968u32,
	/// DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES = 0x616E6B68u32
	HorizontalKanaAlternates = 0x616E6B68u32,
	/// DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES = 0x67696C68u32
	HistoricalLigatures  = 0x67696C68u32,
	/// DWRITE_FONT_FEATURE_TAG_HALF_WIDTH = 0x64697768u32
	HalfWidth            = 0x64697768u32,
	/// DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS = 0x6F6A6F68u32
	HOJOKanjiForms       = 0x6F6A6F68u32,
	/// DWRITE_FONT_FEATURE_TAG_JIS04_FORMS = 0x3430706Au32
	JIs04Forms           = 0x3430706Au32,
	/// DWRITE_FONT_FEATURE_TAG_JIS78_FORMS = 0x3837706Au32
	JIs78Forms           = 0x3837706Au32,
	/// DWRITE_FONT_FEATURE_TAG_JIS83_FORMS = 0x3338706Au32
	JIs83Forms           = 0x3338706Au32,
	/// DWRITE_FONT_FEATURE_TAG_JIS90_FORMS = 0x3039706Au32
	JIs90Forms           = 0x3039706Au32,
	/// DWRITE_FONT_FEATURE_TAG_KERNING = 0x6E72656Bu32
	Kerning              = 0x6E72656Bu32,
	/// DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES = 0x6167696Cu32
	StandardLigatures    = 0x6167696Cu32,
	/// DWRITE_FONT_FEATURE_TAG_LINING_FIGURES = 0x6D756E6Cu32
	LiningFigures        = 0x6D756E6Cu32,
	/// DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS = 0x6C636F6Cu32
	LocalizedForms       = 0x6C636F6Cu32,
	/// DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING = 0x6B72616Du32
	MarkPositioning      = 0x6B72616Du32,
	/// DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK = 0x6B72676Du32
	MathematicalGreek    = 0x6B72676Du32,
	/// DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING = 0x6B6D6B6Du32
	MarkToMarkPositioning = 0x6B6D6B6Du32,
	/// DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS = 0x746C616Eu32
	AlternateAnnotationForms = 0x746C616Eu32,
	/// DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS = 0x6B636C6Eu32
	NlcKanjiForms        = 0x6B636C6Eu32,
	/// DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES = 0x6D756E6Fu32
	OldStyleFigures      = 0x6D756E6Fu32,
	/// DWRITE_FONT_FEATURE_TAG_ORDINALS = 0x6E64726Fu32
	Ordinals             = 0x6E64726Fu32,
	/// DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH = 0x746C6170u32
	ProportionalAlternateWidth = 0x746C6170u32,
	/// DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS = 0x70616370u32
	PetiteCapitals       = 0x70616370u32,
	/// DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES = 0x6D756E70u32
	ProportionalFigures  = 0x6D756E70u32,
	/// DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS = 0x64697770u32
	ProportionalWidths   = 0x64697770u32,
	/// DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS = 0x64697771u32
	QuarterWidths        = 0x64697771u32,
	/// DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES = 0x67696C72u32
	RequiredLigatures    = 0x67696C72u32,
	/// DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS = 0x79627572u32
	RubyNotationForms    = 0x79627572u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES = 0x746C6173u32
	StylisticAlternates  = 0x746C6173u32,
	/// DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS = 0x666E6973u32
	ScientificInferiors  = 0x666E6973u32,
	/// DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS = 0x70636D73u32
	SmallCapitals        = 0x70636D73u32,
	/// DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS = 0x6C706D73u32
	SimplifiedForms      = 0x6C706D73u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1 = 0x31307373u32
	StylisticSet1        = 0x31307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2 = 0x32307373u32
	StylisticSet2        = 0x32307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3 = 0x33307373u32
	StylisticSet3        = 0x33307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4 = 0x34307373u32
	StylisticSet4        = 0x34307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5 = 0x35307373u32
	StylisticSet5        = 0x35307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6 = 0x36307373u32
	StylisticSet6        = 0x36307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7 = 0x37307373u32
	StylisticSet7        = 0x37307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8 = 0x38307373u32
	StylisticSet8        = 0x38307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9 = 0x39307373u32
	StylisticSet9        = 0x39307373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10 = 0x30317373u32
	StylisticSet10       = 0x30317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11 = 0x31317373u32
	StylisticSet11       = 0x31317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12 = 0x32317373u32
	StylisticSet12       = 0x32317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13 = 0x33317373u32
	StylisticSet13       = 0x33317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14 = 0x34317373u32
	StylisticSet14       = 0x34317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15 = 0x35317373u32
	StylisticSet15       = 0x35317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16 = 0x36317373u32
	StylisticSet16       = 0x36317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17 = 0x37317373u32
	StylisticSet17       = 0x37317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18 = 0x38317373u32
	StylisticSet18       = 0x38317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19 = 0x39317373u32
	StylisticSet19       = 0x39317373u32,
	/// DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20 = 0x30327373u32
	StylisticSet20       = 0x30327373u32,
	/// DWRITE_FONT_FEATURE_TAG_SUBSCRIPT = 0x73627573u32
	Subscript            = 0x73627573u32,
	/// DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT = 0x73707573u32
	Superscript          = 0x73707573u32,
	/// DWRITE_FONT_FEATURE_TAG_SWASH = 0x68737773u32
	Swash                = 0x68737773u32,
	/// DWRITE_FONT_FEATURE_TAG_TITLING = 0x6C746974u32
	Titling              = 0x6C746974u32,
	/// DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS = 0x6D616E74u32
	TraditionalNameForms = 0x6D616E74u32,
	/// DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES = 0x6D756E74u32
	TabularFigures       = 0x6D756E74u32,
	/// DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS = 0x64617274u32
	TraditionalForms     = 0x64617274u32,
	/// DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS = 0x64697774u32
	ThirdWidths          = 0x64697774u32,
	/// DWRITE_FONT_FEATURE_TAG_UNICASE = 0x63696E75u32
	UniCase              = 0x63696E75u32,
	/// DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING = 0x74726576u32
	VerticalWriting      = 0x74726576u32,
	/// DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION = 0x32747276u32
	VerticalAlternatesAndRotation = 0x32747276u32,
	/// DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO = 0x6F72657Au32
	SlashedZero          = 0x6F72657Au32,
}

/// DWRITE_FONT_FILE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteFontFileType
{
	/// DWRITE_FONT_FILE_TYPE_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// DWRITE_FONT_FILE_TYPE_CFF = 0x1i32
	Cff                  = 0x1i32,
	/// DWRITE_FONT_FILE_TYPE_TRUETYPE = 0x2i32
	TrueType             = 0x2i32,
	/// DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION = 0x3i32
	OpenTypeCollection   = 0x3i32,
	/// DWRITE_FONT_FILE_TYPE_TYPE1_PFM = 0x4i32
	Type1Pfm             = 0x4i32,
	/// DWRITE_FONT_FILE_TYPE_TYPE1_PFB = 0x5i32
	Type1Pfb             = 0x5i32,
	/// DWRITE_FONT_FILE_TYPE_VECTOR = 0x6i32
	Vector               = 0x6i32,
	/// DWRITE_FONT_FILE_TYPE_BITMAP = 0x7i32
	Bitmap               = 0x7i32,
}

impl DWriteFontFileType {
	/// DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION = 0x3i32
	pub const TrueTypeCollection  : Self = unsafe { transmute(0x3i32) };
}

/// DWRITE_INFORMATIONAL_STRING_ID
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DWriteInformationalStringId
{
	/// DWRITE_INFORMATIONAL_STRING_NONE = 0x0i32
	None                 = 0x0i32,
	/// DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE = 0x1i32
	CopyrightNotice      = 0x1i32,
	/// DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS = 0x2i32
	VersionStrings       = 0x2i32,
	/// DWRITE_INFORMATIONAL_STRING_TRADEMARK = 0x3i32
	Trademark            = 0x3i32,
	/// DWRITE_INFORMATIONAL_STRING_MANUFACTURER = 0x4i32
	Manufacturer         = 0x4i32,
	/// DWRITE_INFORMATIONAL_STRING_DESIGNER = 0x5i32
	Designer             = 0x5i32,
	/// DWRITE_INFORMATIONAL_STRING_DESIGNER_URL = 0x6i32
	DesignerUrl          = 0x6i32,
	/// DWRITE_INFORMATIONAL_STRING_DESCRIPTION = 0x7i32
	Description          = 0x7i32,
	/// DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL = 0x8i32
	FontVendorUrl        = 0x8i32,
	/// DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION = 0x9i32
	LicenseDescription   = 0x9i32,
	/// DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL = 0xAi32
	LicenseInfoUrl       = 0xAi32,
	/// DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES = 0xBi32
	Win32FamilyNames     = 0xBi32,
	/// DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES = 0xCi32
	Win32SubfamilyNames  = 0xCi32,
	/// DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES = 0xDi32
	TypographicFamilyNames = 0xDi32,
	/// DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES = 0xEi32
	TypographicSubfamilyNames = 0xEi32,
	/// DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT = 0xFi32
	SampleText           = 0xFi32,
	/// DWRITE_INFORMATIONAL_STRING_FULL_NAME = 0x10i32
	FullName             = 0x10i32,
	/// DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME = 0x11i32
	PostscriptName       = 0x11i32,
	/// DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME = 0x12i32
	PostscriptCidName    = 0x12i32,
	/// DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME = 0x13i32
	WeightStretchStyleFamilyName = 0x13i32,
	/// DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG = 0x14i32
	DesignScriptLanguageTag = 0x14i32,
	/// DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG = 0x15i32
	SupportedScriptLanguageTag = 0x15i32,
}

impl DWriteInformationalStringId {
	/// DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES = 0xDi32
	pub const PreferredFamilyNames: Self = unsafe { transmute(0xDi32) };
	/// DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES = 0xEi32
	pub const PreferredSubfamilyNames: Self = unsafe { transmute(0xEi32) };
	/// DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME = 0x13i32
	pub const WwsFamilyName       : Self = unsafe { transmute(0x13i32) };
}

