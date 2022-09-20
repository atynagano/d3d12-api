#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

bitflags::bitflags! {
	/// WNDCLASS_STYLES
	#[repr(transparent)]
	pub struct WndClassStyles: u32 {
		/// CS_VREDRAW = 0x1u32
		const VRedraw              = 0x1u32;
		/// CS_HREDRAW = 0x2u32
		const HRedraw              = 0x2u32;
		/// CS_DBLCLKS = 0x8u32
		const DblClks              = 0x8u32;
		/// CS_OWNDC = 0x20u32
		const OwnDc                = 0x20u32;
		/// CS_CLASSDC = 0x40u32
		const ClassDc              = 0x40u32;
		/// CS_PARENTDC = 0x80u32
		const ParentDc             = 0x80u32;
		/// CS_NOCLOSE = 0x200u32
		const NoClose              = 0x200u32;
		/// CS_SAVEBITS = 0x800u32
		const SaveBits             = 0x800u32;
		/// CS_BYTEALIGNCLIENT = 0x1000u32
		const ByteAlignClient      = 0x1000u32;
		/// CS_BYTEALIGNWINDOW = 0x2000u32
		const ByteAlignWindow      = 0x2000u32;
		/// CS_GLOBALCLASS = 0x4000u32
		const GlobalClass          = 0x4000u32;
		/// CS_IME = 0x10000u32
		const Ime                  = 0x10000u32;
		/// CS_DROPSHADOW = 0x20000u32
		const DropShadow           = 0x20000u32;
	}
}

bitflags::bitflags! {
	/// CWP_FLAGS
	#[repr(transparent)]
	pub struct CwpFlags: u32 {
		/// CWP_ALL = 0x0u32
		const All                  = 0x0u32;
		/// CWP_SKIPINVISIBLE = 0x1u32
		const SkipInvisible        = 0x1u32;
		/// CWP_SKIPDISABLED = 0x2u32
		const SkipDisabled         = 0x2u32;
		/// CWP_SKIPTRANSPARENT = 0x4u32
		const SkipTransparent      = 0x4u32;
	}
}

bitflags::bitflags! {
	/// MESSAGEBOX_STYLE
	#[repr(transparent)]
	pub struct MessageBoxStyle: u32 {
		/// MB_ABORTRETRYIGNORE = 0x2u32
		const AbortRetryIgnore     = 0x2u32;
		/// MB_CANCELTRYCONTINUE = 0x6u32
		const CancelTryContinue    = 0x6u32;
		/// MB_HELP = 0x4000u32
		const Help                 = 0x4000u32;
		/// MB_OK = 0x0u32
		const Ok                   = 0x0u32;
		/// MB_OKCANCEL = 0x1u32
		const OkCancel             = 0x1u32;
		/// MB_RETRYCANCEL = 0x5u32
		const RetryCancel          = 0x5u32;
		/// MB_YESNO = 0x4u32
		const YesNo                = 0x4u32;
		/// MB_YESNOCANCEL = 0x3u32
		const YesNoCancel          = 0x3u32;
		/// MB_ICONHAND = 0x10u32
		const IconHand             = 0x10u32;
		/// MB_ICONQUESTION = 0x20u32
		const IconQuestion         = 0x20u32;
		/// MB_ICONEXCLAMATION = 0x30u32
		const IconExclamation      = 0x30u32;
		/// MB_ICONASTERISK = 0x40u32
		const IconAsterisk         = 0x40u32;
		/// MB_USERICON = 0x80u32
		const UserIcon             = 0x80u32;
		/// MB_ICONWARNING = 0x30u32
		const IconWarning          = 0x30u32;
		/// MB_ICONERROR = 0x10u32
		const IconError            = 0x10u32;
		/// MB_ICONINFORMATION = 0x40u32
		const IconInformation      = 0x40u32;
		/// MB_ICONSTOP = 0x10u32
		const IconStop             = 0x10u32;
		/// MB_DEFBUTTON1 = 0x0u32
		const DefButton1           = 0x0u32;
		/// MB_DEFBUTTON2 = 0x100u32
		const DefButton2           = 0x100u32;
		/// MB_DEFBUTTON3 = 0x200u32
		const DefButton3           = 0x200u32;
		/// MB_DEFBUTTON4 = 0x300u32
		const DefButton4           = 0x300u32;
		/// MB_APPLMODAL = 0x0u32
		const ApplModal            = 0x0u32;
		/// MB_SYSTEMMODAL = 0x1000u32
		const SystemModal          = 0x1000u32;
		/// MB_TASKMODAL = 0x2000u32
		const TaskModal            = 0x2000u32;
		/// MB_NOFOCUS = 0x8000u32
		const NoFocus              = 0x8000u32;
		/// MB_SETFOREGROUND = 0x10000u32
		const SetForeground        = 0x10000u32;
		/// MB_DEFAULT_DESKTOP_ONLY = 0x20000u32
		const DefaultDesktopOnly   = 0x20000u32;
		/// MB_TOPMOST = 0x40000u32
		const Topmost              = 0x40000u32;
		/// MB_RIGHT = 0x80000u32
		const Right                = 0x80000u32;
		/// MB_RTLREADING = 0x100000u32
		const RtlReading           = 0x100000u32;
		/// MB_SERVICE_NOTIFICATION = 0x200000u32
		const ServiceNotification  = 0x200000u32;
		/// MB_SERVICE_NOTIFICATION_NT3X = 0x40000u32
		const ServiceNotificationNT3X = 0x40000u32;
		/// MB_TYPEMASK = 0xFu32
		const TypeMask             = 0xFu32;
		/// MB_ICONMASK = 0xF0u32
		const IconMask             = 0xF0u32;
		/// MB_DEFMASK = 0xF00u32
		const DefMask              = 0xF00u32;
		/// MB_MODEMASK = 0x3000u32
		const ModeMask             = 0x3000u32;
		/// MB_MISCMASK = 0xC000u32
		const MiscMask             = 0xC000u32;
	}
}

bitflags::bitflags! {
	/// MENU_ITEM_FLAGS
	#[repr(transparent)]
	pub struct MenuItemFlags: u32 {
		/// MF_BYCOMMAND = 0x0u32
		const ByCommand            = 0x0u32;
		/// MF_BYPOSITION = 0x400u32
		const ByPosition           = 0x400u32;
		/// MF_BITMAP = 0x4u32
		const Bitmap               = 0x4u32;
		/// MF_CHECKED = 0x8u32
		const Checked              = 0x8u32;
		/// MF_DISABLED = 0x2u32
		const Disabled             = 0x2u32;
		/// MF_ENABLED = 0x0u32
		const Enabled              = 0x0u32;
		/// MF_GRAYED = 0x1u32
		const Grayed               = 0x1u32;
		/// MF_MENUBARBREAK = 0x20u32
		const MenuBarBreak         = 0x20u32;
		/// MF_MENUBREAK = 0x40u32
		const MenuBreak            = 0x40u32;
		/// MF_OWNERDRAW = 0x100u32
		const OwnerDraw            = 0x100u32;
		/// MF_POPUP = 0x10u32
		const Popup                = 0x10u32;
		/// MF_SEPARATOR = 0x800u32
		const Separator            = 0x800u32;
		/// MF_STRING = 0x0u32
		const String               = 0x0u32;
		/// MF_UNCHECKED = 0x0u32
		const Unchecked            = 0x0u32;
		/// MF_INSERT = 0x0u32
		const Insert               = 0x0u32;
		/// MF_CHANGE = 0x80u32
		const Change               = 0x80u32;
		/// MF_APPEND = 0x100u32
		const Append               = 0x100u32;
		/// MF_DELETE = 0x200u32
		const Delete               = 0x200u32;
		/// MF_REMOVE = 0x1000u32
		const Remove               = 0x1000u32;
		/// MF_USECHECKBITMAPS = 0x200u32
		const UseCheckbitMaps      = 0x200u32;
		/// MF_UNHILITE = 0x0u32
		const UnHilite             = 0x0u32;
		/// MF_HILITE = 0x80u32
		const Hilite               = 0x80u32;
		/// MF_DEFAULT = 0x1000u32
		const Default              = 0x1000u32;
		/// MF_SYSMENU = 0x2000u32
		const SysMenu              = 0x2000u32;
		/// MF_HELP = 0x4000u32
		const Help                 = 0x4000u32;
		/// MF_RIGHTJUSTIFY = 0x4000u32
		const RightJustify         = 0x4000u32;
		/// MF_MOUSESELECT = 0x8000u32
		const MouseSelect          = 0x8000u32;
		/// MF_END = 0x80u32
		const End                  = 0x80u32;
	}
}

bitflags::bitflags! {
	/// SHOW_WINDOW_CMD
	#[repr(transparent)]
	pub struct ShowWindowCmd: u32 {
		/// SW_FORCEMINIMIZE = 0xBu32
		const ForceMinimize        = 0xBu32;
		/// SW_HIDE = 0x0u32
		const Hide                 = 0x0u32;
		/// SW_MAXIMIZE = 0x3u32
		const Maximize             = 0x3u32;
		/// SW_MINIMIZE = 0x6u32
		const Minimize             = 0x6u32;
		/// SW_RESTORE = 0x9u32
		const Restore              = 0x9u32;
		/// SW_SHOW = 0x5u32
		const Show                 = 0x5u32;
		/// SW_SHOWDEFAULT = 0xAu32
		const ShowDefault          = 0xAu32;
		/// SW_SHOWMAXIMIZED = 0x3u32
		const ShowMaximized        = 0x3u32;
		/// SW_SHOWMINIMIZED = 0x2u32
		const ShowMinimized        = 0x2u32;
		/// SW_SHOWMINNOACTIVE = 0x7u32
		const ShowMinNoActive      = 0x7u32;
		/// SW_SHOWNA = 0x8u32
		const ShownA               = 0x8u32;
		/// SW_SHOWNOACTIVATE = 0x4u32
		const ShowNoActivate       = 0x4u32;
		/// SW_SHOWNORMAL = 0x1u32
		const ShowNormal           = 0x1u32;
		/// SW_NORMAL = 0x1u32
		const Normal               = 0x1u32;
		/// SW_MAX = 0xBu32
		const Max                  = 0xBu32;
		/// SW_PARENTCLOSING = 0x1u32
		const ParentClosing        = 0x1u32;
		/// SW_OTHERZOOM = 0x2u32
		const OtherZoom            = 0x2u32;
		/// SW_PARENTOPENING = 0x3u32
		const ParentOpening        = 0x3u32;
		/// SW_OTHERUNZOOM = 0x4u32
		const OtherUnzoom          = 0x4u32;
		/// SW_SCROLLCHILDREN = 0x1u32
		const ScrollChildren       = 0x1u32;
		/// SW_INVALIDATE = 0x2u32
		const Invalidate           = 0x2u32;
		/// SW_ERASE = 0x4u32
		const Erase                = 0x4u32;
		/// SW_SMOOTHSCROLL = 0x10u32
		const SmoothScroll         = 0x10u32;
	}
}

bitflags::bitflags! {
	/// SYSTEM_PARAMETERS_INFO_ACTION
	#[repr(transparent)]
	pub struct SystemParametersInfoAction: u32 {
		/// SPI_GETBEEP = 0x1u32
		const GetBeep              = 0x1u32;
		/// SPI_SETBEEP = 0x2u32
		const SetBeep              = 0x2u32;
		/// SPI_GETMOUSE = 0x3u32
		const GetMouse             = 0x3u32;
		/// SPI_SETMOUSE = 0x4u32
		const SetMouse             = 0x4u32;
		/// SPI_GETBORDER = 0x5u32
		const GetBorder            = 0x5u32;
		/// SPI_SETBORDER = 0x6u32
		const SetBorder            = 0x6u32;
		/// SPI_GETKEYBOARDSPEED = 0xAu32
		const GetKeyboardSpeed     = 0xAu32;
		/// SPI_SETKEYBOARDSPEED = 0xBu32
		const SetKeyboardSpeed     = 0xBu32;
		/// SPI_LANGDRIVER = 0xCu32
		const LangDriver           = 0xCu32;
		/// SPI_ICONHORIZONTALSPACING = 0xDu32
		const IconHorizontalSpacing = 0xDu32;
		/// SPI_GETSCREENSAVETIMEOUT = 0xEu32
		const GetScreenSaveTimeout = 0xEu32;
		/// SPI_SETSCREENSAVETIMEOUT = 0xFu32
		const SetScreenSaveTimeout = 0xFu32;
		/// SPI_GETSCREENSAVEACTIVE = 0x10u32
		const GetScreenSaveActive  = 0x10u32;
		/// SPI_SETSCREENSAVEACTIVE = 0x11u32
		const SetScreenSaveActive  = 0x11u32;
		/// SPI_GETGRIDGRANULARITY = 0x12u32
		const GetGridGranularity   = 0x12u32;
		/// SPI_SETGRIDGRANULARITY = 0x13u32
		const SetGridGranularity   = 0x13u32;
		/// SPI_SETDESKWALLPAPER = 0x14u32
		const SetDeskWallpaper     = 0x14u32;
		/// SPI_SETDESKPATTERN = 0x15u32
		const SetDeskPattern       = 0x15u32;
		/// SPI_GETKEYBOARDDELAY = 0x16u32
		const GetKeyboardDelay     = 0x16u32;
		/// SPI_SETKEYBOARDDELAY = 0x17u32
		const SetKeyboardDelay     = 0x17u32;
		/// SPI_ICONVERTICALSPACING = 0x18u32
		const IconVerticalSpacing  = 0x18u32;
		/// SPI_GETICONTITLEWRAP = 0x19u32
		const GetIconTitleWrap     = 0x19u32;
		/// SPI_SETICONTITLEWRAP = 0x1Au32
		const SetIconTitleWrap     = 0x1Au32;
		/// SPI_GETMENUDROPALIGNMENT = 0x1Bu32
		const GetMenuDropAlignment = 0x1Bu32;
		/// SPI_SETMENUDROPALIGNMENT = 0x1Cu32
		const SetMenuDropAlignment = 0x1Cu32;
		/// SPI_SETDOUBLECLKWIDTH = 0x1Du32
		const SetDoubleClkWidth    = 0x1Du32;
		/// SPI_SETDOUBLECLKHEIGHT = 0x1Eu32
		const SetDoubleClkHeight   = 0x1Eu32;
		/// SPI_GETICONTITLELOGFONT = 0x1Fu32
		const GetIconTitleLogFont  = 0x1Fu32;
		/// SPI_SETDOUBLECLICKTIME = 0x20u32
		const SetDoubleClickTime   = 0x20u32;
		/// SPI_SETMOUSEBUTTONSWAP = 0x21u32
		const SetMouseButtonSwap   = 0x21u32;
		/// SPI_SETICONTITLELOGFONT = 0x22u32
		const SetIconTitleLogFont  = 0x22u32;
		/// SPI_GETFASTTASKSWITCH = 0x23u32
		const GetFastTaskSwitch    = 0x23u32;
		/// SPI_SETFASTTASKSWITCH = 0x24u32
		const SetFastTaskSwitch    = 0x24u32;
		/// SPI_SETDRAGFULLWINDOWS = 0x25u32
		const SetDragFullWindows   = 0x25u32;
		/// SPI_GETDRAGFULLWINDOWS = 0x26u32
		const GetDragFullWindows   = 0x26u32;
		/// SPI_GETNONCLIENTMETRICS = 0x29u32
		const GetNonClientMetrics  = 0x29u32;
		/// SPI_SETNONCLIENTMETRICS = 0x2Au32
		const SetNonClientMetrics  = 0x2Au32;
		/// SPI_GETMINIMIZEDMETRICS = 0x2Bu32
		const GetMinimizedMetrics  = 0x2Bu32;
		/// SPI_SETMINIMIZEDMETRICS = 0x2Cu32
		const SetMinimizedMetrics  = 0x2Cu32;
		/// SPI_GETICONMETRICS = 0x2Du32
		const GetIconMetrics       = 0x2Du32;
		/// SPI_SETICONMETRICS = 0x2Eu32
		const SetIconMetrics       = 0x2Eu32;
		/// SPI_SETWORKAREA = 0x2Fu32
		const SetWorkArea          = 0x2Fu32;
		/// SPI_GETWORKAREA = 0x30u32
		const GetWorkArea          = 0x30u32;
		/// SPI_SETPENWINDOWS = 0x31u32
		const SetPenWindows        = 0x31u32;
		/// SPI_GETHIGHCONTRAST = 0x42u32
		const GetHighContrast      = 0x42u32;
		/// SPI_SETHIGHCONTRAST = 0x43u32
		const SetHighContrast      = 0x43u32;
		/// SPI_GETKEYBOARDPREF = 0x44u32
		const GetKeyboardPref      = 0x44u32;
		/// SPI_SETKEYBOARDPREF = 0x45u32
		const SetKeyboardPref      = 0x45u32;
		/// SPI_GETSCREENREADER = 0x46u32
		const GetScreenReader      = 0x46u32;
		/// SPI_SETSCREENREADER = 0x47u32
		const SetScreenReader      = 0x47u32;
		/// SPI_GETANIMATION = 0x48u32
		const GetAnimation         = 0x48u32;
		/// SPI_SETANIMATION = 0x49u32
		const SetAnimation         = 0x49u32;
		/// SPI_GETFONTSMOOTHING = 0x4Au32
		const GetFontSmoothing     = 0x4Au32;
		/// SPI_SETFONTSMOOTHING = 0x4Bu32
		const SetFontSmoothing     = 0x4Bu32;
		/// SPI_SETDRAGWIDTH = 0x4Cu32
		const SetDragWidth         = 0x4Cu32;
		/// SPI_SETDRAGHEIGHT = 0x4Du32
		const SetDragHeight        = 0x4Du32;
		/// SPI_SETHANDHELD = 0x4Eu32
		const SetHandHeld          = 0x4Eu32;
		/// SPI_GETLOWPOWERTIMEOUT = 0x4Fu32
		const GetLowPowerTimeout   = 0x4Fu32;
		/// SPI_GETPOWEROFFTIMEOUT = 0x50u32
		const GetPowerOffTimeout   = 0x50u32;
		/// SPI_SETLOWPOWERTIMEOUT = 0x51u32
		const SetLowPowerTimeout   = 0x51u32;
		/// SPI_SETPOWEROFFTIMEOUT = 0x52u32
		const SetPowerOffTimeout   = 0x52u32;
		/// SPI_GETLOWPOWERACTIVE = 0x53u32
		const GetLowPowerActive    = 0x53u32;
		/// SPI_GETPOWEROFFACTIVE = 0x54u32
		const GetPowerOfFactive    = 0x54u32;
		/// SPI_SETLOWPOWERACTIVE = 0x55u32
		const SetLowPowerActive    = 0x55u32;
		/// SPI_SETPOWEROFFACTIVE = 0x56u32
		const SetPowerOfFactive    = 0x56u32;
		/// SPI_SETCURSORS = 0x57u32
		const SetCursors           = 0x57u32;
		/// SPI_SETICONS = 0x58u32
		const SetIcons             = 0x58u32;
		/// SPI_GETDEFAULTINPUTLANG = 0x59u32
		const GetDefaultInputLang  = 0x59u32;
		/// SPI_SETDEFAULTINPUTLANG = 0x5Au32
		const SetDefaultInputLang  = 0x5Au32;
		/// SPI_SETLANGTOGGLE = 0x5Bu32
		const SetLangToggle        = 0x5Bu32;
		/// SPI_GETWINDOWSEXTENSION = 0x5Cu32
		const GetWindowsExtension  = 0x5Cu32;
		/// SPI_SETMOUSETRAILS = 0x5Du32
		const SetMouseTrails       = 0x5Du32;
		/// SPI_GETMOUSETRAILS = 0x5Eu32
		const GetMouseTrails       = 0x5Eu32;
		/// SPI_SETSCREENSAVERRUNNING = 0x61u32
		const SetScreenSaverRunning = 0x61u32;
		/// SPI_SCREENSAVERRUNNING = 0x61u32
		const ScreenSaverRunning   = 0x61u32;
		/// SPI_GETFILTERKEYS = 0x32u32
		const GetFilterKeys        = 0x32u32;
		/// SPI_SETFILTERKEYS = 0x33u32
		const SetFilterKeys        = 0x33u32;
		/// SPI_GETTOGGLEKEYS = 0x34u32
		const GetToggleKeys        = 0x34u32;
		/// SPI_SETTOGGLEKEYS = 0x35u32
		const SetToggleKeys        = 0x35u32;
		/// SPI_GETMOUSEKEYS = 0x36u32
		const GetMouseKeys         = 0x36u32;
		/// SPI_SETMOUSEKEYS = 0x37u32
		const SetMouseKeys         = 0x37u32;
		/// SPI_GETSHOWSOUNDS = 0x38u32
		const GetShowSounds        = 0x38u32;
		/// SPI_SETSHOWSOUNDS = 0x39u32
		const SetShowSounds        = 0x39u32;
		/// SPI_GETSTICKYKEYS = 0x3Au32
		const GetStickyKeys        = 0x3Au32;
		/// SPI_SETSTICKYKEYS = 0x3Bu32
		const SetStickyKeys        = 0x3Bu32;
		/// SPI_GETACCESSTIMEOUT = 0x3Cu32
		const GetAccessTimeout     = 0x3Cu32;
		/// SPI_SETACCESSTIMEOUT = 0x3Du32
		const SetAccessTimeout     = 0x3Du32;
		/// SPI_GETSERIALKEYS = 0x3Eu32
		const GetSerialKeys        = 0x3Eu32;
		/// SPI_SETSERIALKEYS = 0x3Fu32
		const SetSerialKeys        = 0x3Fu32;
		/// SPI_GETSOUNDSENTRY = 0x40u32
		const GetSoundSentry       = 0x40u32;
		/// SPI_SETSOUNDSENTRY = 0x41u32
		const SetSoundSentry       = 0x41u32;
		/// SPI_GETSNAPTODEFBUTTON = 0x5Fu32
		const GetSnapToDefButton   = 0x5Fu32;
		/// SPI_SETSNAPTODEFBUTTON = 0x60u32
		const SetSnapToDefButton   = 0x60u32;
		/// SPI_GETMOUSEHOVERWIDTH = 0x62u32
		const GetMouseHoverWidth   = 0x62u32;
		/// SPI_SETMOUSEHOVERWIDTH = 0x63u32
		const SetMouseHoverWidth   = 0x63u32;
		/// SPI_GETMOUSEHOVERHEIGHT = 0x64u32
		const GetMouseHOverheight  = 0x64u32;
		/// SPI_SETMOUSEHOVERHEIGHT = 0x65u32
		const SetMouseHOverheight  = 0x65u32;
		/// SPI_GETMOUSEHOVERTIME = 0x66u32
		const GetMouseHOvertime    = 0x66u32;
		/// SPI_SETMOUSEHOVERTIME = 0x67u32
		const SetMouseHOvertime    = 0x67u32;
		/// SPI_GETWHEELSCROLLLINES = 0x68u32
		const GetWheelScrollLines  = 0x68u32;
		/// SPI_SETWHEELSCROLLLINES = 0x69u32
		const SetWheelScrollLines  = 0x69u32;
		/// SPI_GETMENUSHOWDELAY = 0x6Au32
		const GetMenuShowDelay     = 0x6Au32;
		/// SPI_SETMENUSHOWDELAY = 0x6Bu32
		const SetMenuShowDelay     = 0x6Bu32;
		/// SPI_GETWHEELSCROLLCHARS = 0x6Cu32
		const GetWheelScrollChars  = 0x6Cu32;
		/// SPI_SETWHEELSCROLLCHARS = 0x6Du32
		const SetWheelScrollChars  = 0x6Du32;
		/// SPI_GETSHOWIMEUI = 0x6Eu32
		const GetShowImeUi         = 0x6Eu32;
		/// SPI_SETSHOWIMEUI = 0x6Fu32
		const SetShowImeUi         = 0x6Fu32;
		/// SPI_GETMOUSESPEED = 0x70u32
		const GetMouseSpeed        = 0x70u32;
		/// SPI_SETMOUSESPEED = 0x71u32
		const SetMouseSpeed        = 0x71u32;
		/// SPI_GETSCREENSAVERRUNNING = 0x72u32
		const GetScreenSaverRunning = 0x72u32;
		/// SPI_GETDESKWALLPAPER = 0x73u32
		const GetDeskWallpaper     = 0x73u32;
		/// SPI_GETAUDIODESCRIPTION = 0x74u32
		const GetAudioDescription  = 0x74u32;
		/// SPI_SETAUDIODESCRIPTION = 0x75u32
		const SetAudioDescription  = 0x75u32;
		/// SPI_GETSCREENSAVESECURE = 0x76u32
		const GetScreenSaveSecure  = 0x76u32;
		/// SPI_SETSCREENSAVESECURE = 0x77u32
		const SetScreenSaveSecure  = 0x77u32;
		/// SPI_GETHUNGAPPTIMEOUT = 0x78u32
		const GetHungAppTimeout    = 0x78u32;
		/// SPI_SETHUNGAPPTIMEOUT = 0x79u32
		const SetHungAppTimeout    = 0x79u32;
		/// SPI_GETWAITTOKILLTIMEOUT = 0x7Au32
		const GetWaitToKillTimeout = 0x7Au32;
		/// SPI_SETWAITTOKILLTIMEOUT = 0x7Bu32
		const SetWaitToKillTimeout = 0x7Bu32;
		/// SPI_GETWAITTOKILLSERVICETIMEOUT = 0x7Cu32
		const GetWaitToKillServiceTimeout = 0x7Cu32;
		/// SPI_SETWAITTOKILLSERVICETIMEOUT = 0x7Du32
		const SetWaitToKillServiceTimeout = 0x7Du32;
		/// SPI_GETMOUSEDOCKTHRESHOLD = 0x7Eu32
		const GetMouseDockThreshold = 0x7Eu32;
		/// SPI_SETMOUSEDOCKTHRESHOLD = 0x7Fu32
		const SetMouseDockThreshold = 0x7Fu32;
		/// SPI_GETPENDOCKTHRESHOLD = 0x80u32
		const GetPenDockThreshold  = 0x80u32;
		/// SPI_SETPENDOCKTHRESHOLD = 0x81u32
		const SetPenDockThreshold  = 0x81u32;
		/// SPI_GETWINARRANGING = 0x82u32
		const GetWinArranging      = 0x82u32;
		/// SPI_SETWINARRANGING = 0x83u32
		const SetWinArranging      = 0x83u32;
		/// SPI_GETMOUSEDRAGOUTTHRESHOLD = 0x84u32
		const GetMousedRagoutThreshold = 0x84u32;
		/// SPI_SETMOUSEDRAGOUTTHRESHOLD = 0x85u32
		const SetMousedRagoutThreshold = 0x85u32;
		/// SPI_GETPENDRAGOUTTHRESHOLD = 0x86u32
		const GetPendRagoutThreshold = 0x86u32;
		/// SPI_SETPENDRAGOUTTHRESHOLD = 0x87u32
		const SetPendRagoutThreshold = 0x87u32;
		/// SPI_GETMOUSESIDEMOVETHRESHOLD = 0x88u32
		const GetMouseSideMoveThreshold = 0x88u32;
		/// SPI_SETMOUSESIDEMOVETHRESHOLD = 0x89u32
		const SetMouseSideMoveThreshold = 0x89u32;
		/// SPI_GETPENSIDEMOVETHRESHOLD = 0x8Au32
		const GetPenSideMoveThreshold = 0x8Au32;
		/// SPI_SETPENSIDEMOVETHRESHOLD = 0x8Bu32
		const SetPenSideMoveThreshold = 0x8Bu32;
		/// SPI_GETDRAGFROMMAXIMIZE = 0x8Cu32
		const GetDragFromMaximize  = 0x8Cu32;
		/// SPI_SETDRAGFROMMAXIMIZE = 0x8Du32
		const SetDragFromMaximize  = 0x8Du32;
		/// SPI_GETSNAPSIZING = 0x8Eu32
		const GetSnapSizing        = 0x8Eu32;
		/// SPI_SETSNAPSIZING = 0x8Fu32
		const SetSnapSizing        = 0x8Fu32;
		/// SPI_GETDOCKMOVING = 0x90u32
		const GetDockMoving        = 0x90u32;
		/// SPI_SETDOCKMOVING = 0x91u32
		const SetDockMoving        = 0x91u32;
		/// SPI_GETTOUCHPREDICTIONPARAMETERS = 0x9Cu32
		const GetTouchPredictionParameters = 0x9Cu32;
		/// SPI_SETTOUCHPREDICTIONPARAMETERS = 0x9Du32
		const SetTouchPredictionParameters = 0x9Du32;
		/// SPI_GETLOGICALDPIOVERRIDE = 0x9Eu32
		const GetLogicalDpiOverride = 0x9Eu32;
		/// SPI_SETLOGICALDPIOVERRIDE = 0x9Fu32
		const SetLogicalDpiOverride = 0x9Fu32;
		/// SPI_GETMENURECT = 0xA2u32
		const GetMenuRect          = 0xA2u32;
		/// SPI_SETMENURECT = 0xA3u32
		const SetMenuRect          = 0xA3u32;
		/// SPI_GETACTIVEWINDOWTRACKING = 0x1000u32
		const GetActiveWindowTracking = 0x1000u32;
		/// SPI_SETACTIVEWINDOWTRACKING = 0x1001u32
		const SetActiveWindowTracking = 0x1001u32;
		/// SPI_GETMENUANIMATION = 0x1002u32
		const GetMenuAnimation     = 0x1002u32;
		/// SPI_SETMENUANIMATION = 0x1003u32
		const SetMenuAnimation     = 0x1003u32;
		/// SPI_GETCOMBOBOXANIMATION = 0x1004u32
		const GetComboBoxAnimation = 0x1004u32;
		/// SPI_SETCOMBOBOXANIMATION = 0x1005u32
		const SetComboBoxAnimation = 0x1005u32;
		/// SPI_GETLISTBOXSMOOTHSCROLLING = 0x1006u32
		const GetListBoxSmoothScrolling = 0x1006u32;
		/// SPI_SETLISTBOXSMOOTHSCROLLING = 0x1007u32
		const SetListBoxSmoothScrolling = 0x1007u32;
		/// SPI_GETGRADIENTCAPTIONS = 0x1008u32
		const GetGradientCaptions  = 0x1008u32;
		/// SPI_SETGRADIENTCAPTIONS = 0x1009u32
		const SetGradientCaptions  = 0x1009u32;
		/// SPI_GETKEYBOARDCUES = 0x100Au32
		const GetKeyboardCues      = 0x100Au32;
		/// SPI_SETKEYBOARDCUES = 0x100Bu32
		const SetKeyboardCues      = 0x100Bu32;
		/// SPI_GETMENUUNDERLINES = 0x100Au32
		const GetMenuUnderlines    = 0x100Au32;
		/// SPI_SETMENUUNDERLINES = 0x100Bu32
		const SetMenuUnderlines    = 0x100Bu32;
		/// SPI_GETACTIVEWNDTRKZORDER = 0x100Cu32
		const GetActiveWndTRKZOrder = 0x100Cu32;
		/// SPI_SETACTIVEWNDTRKZORDER = 0x100Du32
		const SetActiveWndTRKZOrder = 0x100Du32;
		/// SPI_GETHOTTRACKING = 0x100Eu32
		const GetHotTracking       = 0x100Eu32;
		/// SPI_SETHOTTRACKING = 0x100Fu32
		const SetHotTracking       = 0x100Fu32;
		/// SPI_GETMENUFADE = 0x1012u32
		const GetMenuFade          = 0x1012u32;
		/// SPI_SETMENUFADE = 0x1013u32
		const SetMenuFade          = 0x1013u32;
		/// SPI_GETSELECTIONFADE = 0x1014u32
		const GetSelectionFade     = 0x1014u32;
		/// SPI_SETSELECTIONFADE = 0x1015u32
		const SetSelectionFade     = 0x1015u32;
		/// SPI_GETTOOLTIPANIMATION = 0x1016u32
		const GetToolTipAnimation  = 0x1016u32;
		/// SPI_SETTOOLTIPANIMATION = 0x1017u32
		const SetToolTipAnimation  = 0x1017u32;
		/// SPI_GETTOOLTIPFADE = 0x1018u32
		const GetToolTipFade       = 0x1018u32;
		/// SPI_SETTOOLTIPFADE = 0x1019u32
		const SetToolTipFade       = 0x1019u32;
		/// SPI_GETCURSORSHADOW = 0x101Au32
		const GetCursorShadow      = 0x101Au32;
		/// SPI_SETCURSORSHADOW = 0x101Bu32
		const SetCursorShadow      = 0x101Bu32;
		/// SPI_GETMOUSESONAR = 0x101Cu32
		const GetMouseSonar        = 0x101Cu32;
		/// SPI_SETMOUSESONAR = 0x101Du32
		const SetMouseSonar        = 0x101Du32;
		/// SPI_GETMOUSECLICKLOCK = 0x101Eu32
		const GetMouseClickLock    = 0x101Eu32;
		/// SPI_SETMOUSECLICKLOCK = 0x101Fu32
		const SetMouseClickLock    = 0x101Fu32;
		/// SPI_GETMOUSEVANISH = 0x1020u32
		const GetMouSEvanish       = 0x1020u32;
		/// SPI_SETMOUSEVANISH = 0x1021u32
		const SetMouSEvanish       = 0x1021u32;
		/// SPI_GETFLATMENU = 0x1022u32
		const GetFlatMenu          = 0x1022u32;
		/// SPI_SETFLATMENU = 0x1023u32
		const SetFlatMenu          = 0x1023u32;
		/// SPI_GETDROPSHADOW = 0x1024u32
		const GetDropShadow        = 0x1024u32;
		/// SPI_SETDROPSHADOW = 0x1025u32
		const SetDropShadow        = 0x1025u32;
		/// SPI_GETBLOCKSENDINPUTRESETS = 0x1026u32
		const GetBlockSendInputResets = 0x1026u32;
		/// SPI_SETBLOCKSENDINPUTRESETS = 0x1027u32
		const SetBlockSendInputResets = 0x1027u32;
		/// SPI_GETUIEFFECTS = 0x103Eu32
		const GEtuiEffects         = 0x103Eu32;
		/// SPI_SETUIEFFECTS = 0x103Fu32
		const SEtuiEffects         = 0x103Fu32;
		/// SPI_GETDISABLEOVERLAPPEDCONTENT = 0x1040u32
		const GetDisableOverlappedContent = 0x1040u32;
		/// SPI_SETDISABLEOVERLAPPEDCONTENT = 0x1041u32
		const SetDisableOverlappedContent = 0x1041u32;
		/// SPI_GETCLIENTAREAANIMATION = 0x1042u32
		const GEtcLienTareaAnimation = 0x1042u32;
		/// SPI_SETCLIENTAREAANIMATION = 0x1043u32
		const SEtcLienTareaAnimation = 0x1043u32;
		/// SPI_GETCLEARTYPE = 0x1048u32
		const GetClearType         = 0x1048u32;
		/// SPI_SETCLEARTYPE = 0x1049u32
		const SetClearType         = 0x1049u32;
		/// SPI_GETSPEECHRECOGNITION = 0x104Au32
		const GetSpeechRecognition = 0x104Au32;
		/// SPI_SETSPEECHRECOGNITION = 0x104Bu32
		const SetSpeechRecognition = 0x104Bu32;
		/// SPI_GETCARETBROWSING = 0x104Cu32
		const GetCaretBrowsing     = 0x104Cu32;
		/// SPI_SETCARETBROWSING = 0x104Du32
		const SetCaretBrowsing     = 0x104Du32;
		/// SPI_GETTHREADLOCALINPUTSETTINGS = 0x104Eu32
		const GetThreadLocalInputSettings = 0x104Eu32;
		/// SPI_SETTHREADLOCALINPUTSETTINGS = 0x104Fu32
		const SetThreadLocalInputSettings = 0x104Fu32;
		/// SPI_GETSYSTEMLANGUAGEBAR = 0x1050u32
		const GetSystemLanguageBar = 0x1050u32;
		/// SPI_SETSYSTEMLANGUAGEBAR = 0x1051u32
		const SetSystemLanguageBar = 0x1051u32;
		/// SPI_GETFOREGROUNDLOCKTIMEOUT = 0x2000u32
		const GetForegroundLockTimeout = 0x2000u32;
		/// SPI_SETFOREGROUNDLOCKTIMEOUT = 0x2001u32
		const SetForegroundLockTimeout = 0x2001u32;
		/// SPI_GETACTIVEWNDTRKTIMEOUT = 0x2002u32
		const GetActiveWndTRKTimeout = 0x2002u32;
		/// SPI_SETACTIVEWNDTRKTIMEOUT = 0x2003u32
		const SetActiveWndTRKTimeout = 0x2003u32;
		/// SPI_GETFOREGROUNDFLASHCOUNT = 0x2004u32
		const GetForegroundFlashCount = 0x2004u32;
		/// SPI_SETFOREGROUNDFLASHCOUNT = 0x2005u32
		const SetForegroundFlashCount = 0x2005u32;
		/// SPI_GETCARETWIDTH = 0x2006u32
		const GetCaretWidth        = 0x2006u32;
		/// SPI_SETCARETWIDTH = 0x2007u32
		const SetCaretWidth        = 0x2007u32;
		/// SPI_GETMOUSECLICKLOCKTIME = 0x2008u32
		const GetMouseClickLockTime = 0x2008u32;
		/// SPI_SETMOUSECLICKLOCKTIME = 0x2009u32
		const SetMouseClickLockTime = 0x2009u32;
		/// SPI_GETFONTSMOOTHINGTYPE = 0x200Au32
		const GetFontSmoothingType = 0x200Au32;
		/// SPI_SETFONTSMOOTHINGTYPE = 0x200Bu32
		const SetFontSmoothingType = 0x200Bu32;
		/// SPI_GETFONTSMOOTHINGCONTRAST = 0x200Cu32
		const GetFontSmoothingContrast = 0x200Cu32;
		/// SPI_SETFONTSMOOTHINGCONTRAST = 0x200Du32
		const SetFontSmoothingContrast = 0x200Du32;
		/// SPI_GETFOCUSBORDERWIDTH = 0x200Eu32
		const GetFocusBorderWidth  = 0x200Eu32;
		/// SPI_SETFOCUSBORDERWIDTH = 0x200Fu32
		const SetFocusBorderWidth  = 0x200Fu32;
		/// SPI_GETFOCUSBORDERHEIGHT = 0x2010u32
		const GetFocusBorderHeight = 0x2010u32;
		/// SPI_SETFOCUSBORDERHEIGHT = 0x2011u32
		const SetFocusBorderHeight = 0x2011u32;
		/// SPI_GETFONTSMOOTHINGORIENTATION = 0x2012u32
		const GetFontSmoothingOrientation = 0x2012u32;
		/// SPI_SETFONTSMOOTHINGORIENTATION = 0x2013u32
		const SetFontSmoothingOrientation = 0x2013u32;
		/// SPI_GETMINIMUMHITRADIUS = 0x2014u32
		const GetMinimumHitRadius  = 0x2014u32;
		/// SPI_SETMINIMUMHITRADIUS = 0x2015u32
		const SetMinimumHitRadius  = 0x2015u32;
		/// SPI_GETMESSAGEDURATION = 0x2016u32
		const GetMessageDuration   = 0x2016u32;
		/// SPI_SETMESSAGEDURATION = 0x2017u32
		const SetMessageDuration   = 0x2017u32;
		/// SPI_GETCONTACTVISUALIZATION = 0x2018u32
		const GetContactVisualization = 0x2018u32;
		/// SPI_SETCONTACTVISUALIZATION = 0x2019u32
		const SetContactVisualization = 0x2019u32;
		/// SPI_GETGESTUREVISUALIZATION = 0x201Au32
		const GetGEStuRevisualization = 0x201Au32;
		/// SPI_SETGESTUREVISUALIZATION = 0x201Bu32
		const SetGEStuRevisualization = 0x201Bu32;
		/// SPI_GETMOUSEWHEELROUTING = 0x201Cu32
		const GetMouseWheelRouting = 0x201Cu32;
		/// SPI_SETMOUSEWHEELROUTING = 0x201Du32
		const SetMouseWheelRouting = 0x201Du32;
		/// SPI_GETPENVISUALIZATION = 0x201Eu32
		const GetPenVisualization  = 0x201Eu32;
		/// SPI_SETPENVISUALIZATION = 0x201Fu32
		const SetPenVisualization  = 0x201Fu32;
		/// SPI_GETPENARBITRATIONTYPE = 0x2020u32
		const GetPenArbitrationType = 0x2020u32;
		/// SPI_SETPENARBITRATIONTYPE = 0x2021u32
		const SetPenArbitrationType = 0x2021u32;
		/// SPI_GETCARETTIMEOUT = 0x2022u32
		const GetCaretTimeout      = 0x2022u32;
		/// SPI_SETCARETTIMEOUT = 0x2023u32
		const SetCaretTimeout      = 0x2023u32;
		/// SPI_GETHANDEDNESS = 0x2024u32
		const GetHandedness        = 0x2024u32;
		/// SPI_SETHANDEDNESS = 0x2025u32
		const SetHandedness        = 0x2025u32;
	}
}

bitflags::bitflags! {
	/// TRACK_POPUP_MENU_FLAGS
	#[repr(transparent)]
	pub struct TrackPopupMenuFlags: u32 {
		/// TPM_LEFTBUTTON = 0x0u32
		const LeftButton           = 0x0u32;
		/// TPM_RIGHTBUTTON = 0x2u32
		const RightButton          = 0x2u32;
		/// TPM_LEFTALIGN = 0x0u32
		const LeftAlign            = 0x0u32;
		/// TPM_CENTERALIGN = 0x4u32
		const CenterAlign          = 0x4u32;
		/// TPM_RIGHTALIGN = 0x8u32
		const RightAlign           = 0x8u32;
		/// TPM_TOPALIGN = 0x0u32
		const TopAlign             = 0x0u32;
		/// TPM_VCENTERALIGN = 0x10u32
		const VCenterAlign         = 0x10u32;
		/// TPM_BOTTOMALIGN = 0x20u32
		const BottomAlign          = 0x20u32;
		/// TPM_HORIZONTAL = 0x0u32
		const Horizontal           = 0x0u32;
		/// TPM_VERTICAL = 0x40u32
		const Vertical             = 0x40u32;
		/// TPM_NONOTIFY = 0x80u32
		const NoNotify             = 0x80u32;
		/// TPM_RETURNCMD = 0x100u32
		const ReturnCmd            = 0x100u32;
		/// TPM_RECURSE = 0x1u32
		const Recurse              = 0x1u32;
		/// TPM_HORPOSANIMATION = 0x400u32
		const HorPosAnimation      = 0x400u32;
		/// TPM_HORNEGANIMATION = 0x800u32
		const HorNegAnimation      = 0x800u32;
		/// TPM_VERPOSANIMATION = 0x1000u32
		const VerPosAnimation      = 0x1000u32;
		/// TPM_VERNEGANIMATION = 0x2000u32
		const VerNegAnimation      = 0x2000u32;
		/// TPM_NOANIMATION = 0x4000u32
		const NoAnimation          = 0x4000u32;
		/// TPM_LAYOUTRTL = 0x8000u32
		const LayoutRtl            = 0x8000u32;
		/// TPM_WORKAREA = 0x10000u32
		const WorkArea             = 0x10000u32;
	}
}

bitflags::bitflags! {
	/// WINDOW_EX_STYLE
	#[repr(transparent)]
	pub struct WindowExStyle: u32 {
		/// WS_EX_DLGMODALFRAME = 0x1u32
		const DlgModalFrame        = 0x1u32;
		/// WS_EX_NOPARENTNOTIFY = 0x4u32
		const NoParentNotify       = 0x4u32;
		/// WS_EX_TOPMOST = 0x8u32
		const Topmost              = 0x8u32;
		/// WS_EX_ACCEPTFILES = 0x10u32
		const AcceptFiles          = 0x10u32;
		/// WS_EX_TRANSPARENT = 0x20u32
		const Transparent          = 0x20u32;
		/// WS_EX_MDICHILD = 0x40u32
		const MdiChild             = 0x40u32;
		/// WS_EX_TOOLWINDOW = 0x80u32
		const ToolWindow           = 0x80u32;
		/// WS_EX_WINDOWEDGE = 0x100u32
		const WindowEdge           = 0x100u32;
		/// WS_EX_CLIENTEDGE = 0x200u32
		const ClientEdge           = 0x200u32;
		/// WS_EX_CONTEXTHELP = 0x400u32
		const ContextHelp          = 0x400u32;
		/// WS_EX_RIGHT = 0x1000u32
		const Right                = 0x1000u32;
		/// WS_EX_LEFT = 0x0u32
		const Left                 = 0x0u32;
		/// WS_EX_RTLREADING = 0x2000u32
		const RtlReading           = 0x2000u32;
		/// WS_EX_LTRREADING = 0x0u32
		const LtrReading           = 0x0u32;
		/// WS_EX_LEFTSCROLLBAR = 0x4000u32
		const LeftScrollbar        = 0x4000u32;
		/// WS_EX_RIGHTSCROLLBAR = 0x0u32
		const RightScrollbar       = 0x0u32;
		/// WS_EX_CONTROLPARENT = 0x10000u32
		const ControlParent        = 0x10000u32;
		/// WS_EX_STATICEDGE = 0x20000u32
		const StaticEdge           = 0x20000u32;
		/// WS_EX_APPWINDOW = 0x40000u32
		const AppWindow            = 0x40000u32;
		/// WS_EX_OVERLAPPEDWINDOW = 0x300u32
		const OverlappedWindow     = 0x300u32;
		/// WS_EX_PALETTEWINDOW = 0x188u32
		const PaletteWindow        = 0x188u32;
		/// WS_EX_LAYERED = 0x80000u32
		const Layered              = 0x80000u32;
		/// WS_EX_NOINHERITLAYOUT = 0x100000u32
		const NoInheritLayout      = 0x100000u32;
		/// WS_EX_NOREDIRECTIONBITMAP = 0x200000u32
		const NoRedirectionBitmap  = 0x200000u32;
		/// WS_EX_LAYOUTRTL = 0x400000u32
		const LayoutRtl            = 0x400000u32;
		/// WS_EX_COMPOSITED = 0x2000000u32
		const Composited           = 0x2000000u32;
		/// WS_EX_NOACTIVATE = 0x8000000u32
		const NoActivate           = 0x8000000u32;
	}
}

bitflags::bitflags! {
	/// WINDOW_STYLE
	#[repr(transparent)]
	pub struct WindowStyle: u32 {
		/// WS_OVERLAPPED = 0x0u32
		const Overlapped           = 0x0u32;
		/// WS_POPUP = 0x80000000u32
		const Popup                = 0x80000000u32;
		/// WS_CHILD = 0x40000000u32
		const Child                = 0x40000000u32;
		/// WS_MINIMIZE = 0x20000000u32
		const Minimize             = 0x20000000u32;
		/// WS_VISIBLE = 0x10000000u32
		const Visible              = 0x10000000u32;
		/// WS_DISABLED = 0x8000000u32
		const Disabled             = 0x8000000u32;
		/// WS_CLIPSIBLINGS = 0x4000000u32
		const ClipSiblings         = 0x4000000u32;
		/// WS_CLIPCHILDREN = 0x2000000u32
		const ClipChildren         = 0x2000000u32;
		/// WS_MAXIMIZE = 0x1000000u32
		const Maximize             = 0x1000000u32;
		/// WS_CAPTION = 0xC00000u32
		const Caption              = 0xC00000u32;
		/// WS_BORDER = 0x800000u32
		const Border               = 0x800000u32;
		/// WS_DLGFRAME = 0x400000u32
		const DlgFrame             = 0x400000u32;
		/// WS_VSCROLL = 0x200000u32
		const VScroll              = 0x200000u32;
		/// WS_HSCROLL = 0x100000u32
		const HScroll              = 0x100000u32;
		/// WS_SYSMENU = 0x80000u32
		const SysMenu              = 0x80000u32;
		/// WS_THICKFRAME = 0x40000u32
		const ThickFrame           = 0x40000u32;
		/// WS_GROUP = 0x20000u32
		const Group                = 0x20000u32;
		/// WS_TABSTOP = 0x10000u32
		const TabStop              = 0x10000u32;
		/// WS_MINIMIZEBOX = 0x20000u32
		const MinimizeBox          = 0x20000u32;
		/// WS_MAXIMIZEBOX = 0x10000u32
		const MaximizeBox          = 0x10000u32;
		/// WS_TILED = 0x0u32
		const Tiled                = 0x0u32;
		/// WS_ICONIC = 0x20000000u32
		const Iconic               = 0x20000000u32;
		/// WS_SIZEBOX = 0x40000u32
		const SizeBox              = 0x40000u32;
		/// WS_TILEDWINDOW = 0xCF0000u32
		const TiledWindow          = 0xCF0000u32;
		/// WS_OVERLAPPEDWINDOW = 0xCF0000u32
		const OverlappedWindow     = 0xCF0000u32;
		/// WS_POPUPWINDOW = 0x80880000u32
		const PopupWindow          = 0x80880000u32;
		/// WS_CHILDWINDOW = 0x40000000u32
		const ChildWindow          = 0x40000000u32;
		/// WS_ACTIVECAPTION = 0x1u32
		const ActiveCaption        = 0x1u32;
	}
}

/// OBJECT_IDENTIFIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObjectIdentifier
{
	/// OBJID_WINDOW = 0x0i32
	Window               = 0x0i32,
	/// OBJID_SYSMENU = -0x1i32
	SysMenu              = -0x1i32,
	/// OBJID_TITLEBAR = -0x2i32
	TitleBar             = -0x2i32,
	/// OBJID_MENU = -0x3i32
	Menu                 = -0x3i32,
	/// OBJID_CLIENT = -0x4i32
	Client               = -0x4i32,
	/// OBJID_VSCROLL = -0x5i32
	VScroll              = -0x5i32,
	/// OBJID_HSCROLL = -0x6i32
	HScroll              = -0x6i32,
	/// OBJID_SIZEGRIP = -0x7i32
	SizeGrip             = -0x7i32,
	/// OBJID_CARET = -0x8i32
	Caret                = -0x8i32,
	/// OBJID_CURSOR = -0x9i32
	Cursor               = -0x9i32,
	/// OBJID_ALERT = -0xAi32
	Alert                = -0xAi32,
	/// OBJID_SOUND = -0xBi32
	Sound                = -0xBi32,
	/// OBJID_QUERYCLASSNAMEIDX = -0xCi32
	QueryClassNameIdX    = -0xCi32,
	/// OBJID_NATIVEOM = -0x10i32
	NAtiVEom             = -0x10i32,
}

bitflags::bitflags! {
	/// MENU_ITEM_TYPE
	#[repr(transparent)]
	pub struct MenuItemType: u32 {
		/// MFT_BITMAP = 0x4u32
		const Bitmap               = 0x4u32;
		/// MFT_MENUBARBREAK = 0x20u32
		const MenuBarBreak         = 0x20u32;
		/// MFT_MENUBREAK = 0x40u32
		const MenuBreak            = 0x40u32;
		/// MFT_OWNERDRAW = 0x100u32
		const OwnerDraw            = 0x100u32;
		/// MFT_RADIOCHECK = 0x200u32
		const RadioCheck           = 0x200u32;
		/// MFT_RIGHTJUSTIFY = 0x4000u32
		const RightJustify         = 0x4000u32;
		/// MFT_RIGHTORDER = 0x2000u32
		const RightOrder           = 0x2000u32;
		/// MFT_SEPARATOR = 0x800u32
		const Separator            = 0x800u32;
		/// MFT_STRING = 0x0u32
		const String               = 0x0u32;
	}
}

/// MESSAGEBOX_RESULT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MessageBoxResult
{
	/// IDOK = 0x1i32
	Ok                   = 0x1i32,
	/// IDCANCEL = 0x2i32
	Cancel               = 0x2i32,
	/// IDABORT = 0x3i32
	Abort                = 0x3i32,
	/// IDRETRY = 0x4i32
	Retry                = 0x4i32,
	/// IDIGNORE = 0x5i32
	Ignore               = 0x5i32,
	/// IDYES = 0x6i32
	Yes                  = 0x6i32,
	/// IDNO = 0x7i32
	No                   = 0x7i32,
	/// IDCLOSE = 0x8i32
	Close                = 0x8i32,
	/// IDHELP = 0x9i32
	Help                 = 0x9i32,
	/// IDTRYAGAIN = 0xAi32
	TryAgain             = 0xAi32,
	/// IDCONTINUE = 0xBi32
	Continue             = 0xBi32,
	/// IDASYNC = 0x7D01i32
	Async                = 0x7D01i32,
	/// IDTIMEOUT = 0x7D00i32
	Timeout              = 0x7D00i32,
}

bitflags::bitflags! {
	/// MENU_ITEM_STATE
	#[repr(transparent)]
	pub struct MenuItemState: u32 {
		/// MFS_GRAYED = 0x3u32
		const Grayed               = 0x3u32;
		/// MFS_DISABLED = 0x3u32
		const Disabled             = 0x3u32;
		/// MFS_CHECKED = 0x8u32
		const Checked              = 0x8u32;
		/// MFS_HILITE = 0x80u32
		const Hilite               = 0x80u32;
		/// MFS_ENABLED = 0x0u32
		const Enabled              = 0x0u32;
		/// MFS_UNCHECKED = 0x0u32
		const Unchecked            = 0x0u32;
		/// MFS_UNHILITE = 0x0u32
		const UnHilite             = 0x0u32;
		/// MFS_DEFAULT = 0x1000u32
		const Default              = 0x1000u32;
	}
}

bitflags::bitflags! {
	/// SCROLLBAR_CONSTANTS
	#[repr(transparent)]
	pub struct ScrollbarConstants: u32 {
		/// SB_CTL = 0x2u32
		const Ctl                  = 0x2u32;
		/// SB_HORZ = 0x0u32
		const HorZ                 = 0x0u32;
		/// SB_VERT = 0x1u32
		const Vert                 = 0x1u32;
		/// SB_BOTH = 0x3u32
		const Both                 = 0x3u32;
	}
}

/// GET_CLASS_LONG_INDEX
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetClassLongIndex
{
	/// GCW_ATOM = -0x20i32
	Atom                 = -0x20i32,
	/// GCL_CBCLSEXTRA = -0x14i32
	CbCLSExtra           = -0x14i32,
	/// GCL_CBWNDEXTRA = -0x12i32
	CbWndExtra           = -0x12i32,
	/// GCL_HBRBACKGROUND = -0xAi32
	HBRBackground        = -0xAi32,
	/// GCL_HCURSOR = -0xCi32
	HCursor              = -0xCi32,
	/// GCL_HICON = -0xEi32
	HIcon                = -0xEi32,
	/// GCL_HICONSM = -0x22i32
	HIconSm              = -0x22i32,
	/// GCL_HMODULE = -0x10i32
	HModule              = -0x10i32,
	/// GCL_MENUNAME = -0x8i32
	MenuName             = -0x8i32,
	/// GCL_STYLE = -0x1Ai32
	Style                = -0x1Ai32,
	/// GCL_WNDPROC = -0x18i32
	WndProc              = -0x18i32,
}

/// UPDATE_LAYERED_WINDOW_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UpdateLayeredWindowFlags
{
	/// ULW_ALPHA = 0x2u32
	Alpha                = 0x2u32,
	/// ULW_COLORKEY = 0x1u32
	ColorKey             = 0x1u32,
	/// ULW_OPAQUE = 0x4u32
	Opaque               = 0x4u32,
	/// ULW_EX_NORESIZE = 0x8u32
	ExNoResize           = 0x8u32,
}

/// WINDOW_LONG_PTR_INDEX
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindowLongPtrIndex
{
	/// GWL_EXSTYLE = -0x14i32
	ExStyle              = -0x14i32,
	/// GWLP_HINSTANCE = -0x6i32
	HInstance            = -0x6i32,
	/// GWLP_HWNDPARENT = -0x8i32
	HWndParent           = -0x8i32,
	/// GWLP_ID = -0xCi32
	Id                   = -0xCi32,
	/// GWL_STYLE = -0x10i32
	Style                = -0x10i32,
	/// GWLP_USERDATA = -0x15i32
	UserData             = -0x15i32,
	/// GWLP_WNDPROC = -0x4i32
	WndProc              = -0x4i32,
}

bitflags::bitflags! {
	/// ANIMATE_WINDOW_FLAGS
	#[repr(transparent)]
	pub struct AnimateWindowFlags: u32 {
		/// AW_ACTIVATE = 0x20000u32
		const Activate             = 0x20000u32;
		/// AW_BLEND = 0x80000u32
		const Blend                = 0x80000u32;
		/// AW_CENTER = 0x10u32
		const Center               = 0x10u32;
		/// AW_HIDE = 0x10000u32
		const Hide                 = 0x10000u32;
		/// AW_HOR_POSITIVE = 0x1u32
		const HorPositive          = 0x1u32;
		/// AW_HOR_NEGATIVE = 0x2u32
		const HorNegative          = 0x2u32;
		/// AW_SLIDE = 0x40000u32
		const Slide                = 0x40000u32;
		/// AW_VER_POSITIVE = 0x4u32
		const VerPositive          = 0x4u32;
		/// AW_VER_NEGATIVE = 0x8u32
		const VerNegative          = 0x8u32;
	}
}

/// CHANGE_WINDOW_MESSAGE_FILTER_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChangeWindowMessageFilterFlags
{
	/// MSGFLT_ADD = 0x1u32
	Add                  = 0x1u32,
	/// MSGFLT_REMOVE = 0x2u32
	Remove               = 0x2u32,
}

/// GDI_IMAGE_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GdiImageType
{
	/// IMAGE_BITMAP = 0x0u32
	Bitmap               = 0x0u32,
	/// IMAGE_CURSOR = 0x2u32
	Cursor               = 0x2u32,
	/// IMAGE_ICON = 0x1u32
	Icon                 = 0x1u32,
}

/// WINDOWS_HOOK_ID
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindowsHookId
{
	/// WH_CALLWNDPROC = 0x4i32
	CallWndProc          = 0x4i32,
	/// WH_CALLWNDPROCRET = 0xCi32
	CallWndProcRet       = 0xCi32,
	/// WH_CBT = 0x5i32
	Cbt                  = 0x5i32,
	/// WH_DEBUG = 0x9i32
	Debug                = 0x9i32,
	/// WH_FOREGROUNDIDLE = 0xBi32
	FORegRounDidle       = 0xBi32,
	/// WH_GETMESSAGE = 0x3i32
	GetMessage           = 0x3i32,
	/// WH_JOURNALPLAYBACK = 0x1i32
	JournalPlayback      = 0x1i32,
	/// WH_JOURNALRECORD = 0x0i32
	JournalRecord        = 0x0i32,
	/// WH_KEYBOARD = 0x2i32
	Keyboard             = 0x2i32,
	/// WH_KEYBOARD_LL = 0xDi32
	KeyboardLl           = 0xDi32,
	/// WH_MOUSE = 0x7i32
	Mouse                = 0x7i32,
	/// WH_MOUSE_LL = 0xEi32
	MouseLl              = 0xEi32,
	/// WH_MSGFILTER = -0x1i32
	MsgFilter            = -0x1i32,
	/// WH_SHELL = 0xAi32
	Shell                = 0xAi32,
	/// WH_SYSMSGFILTER = 0x6i32
	SysMsgFilter         = 0x6i32,
}

bitflags::bitflags! {
	/// IMAGE_FLAGS
	#[repr(transparent)]
	pub struct ImageFlags: u32 {
		/// LR_CREATEDIBSECTION = 0x2000u32
		const CreateDibSection     = 0x2000u32;
		/// LR_DEFAULTCOLOR = 0x0u32
		const DefaultColor         = 0x0u32;
		/// LR_DEFAULTSIZE = 0x40u32
		const DefaultSize          = 0x40u32;
		/// LR_LOADFROMFILE = 0x10u32
		const LoadFromfile         = 0x10u32;
		/// LR_LOADMAP3DCOLORS = 0x1000u32
		const LoadMap3dColors      = 0x1000u32;
		/// LR_LOADTRANSPARENT = 0x20u32
		const LoadTransparent      = 0x20u32;
		/// LR_MONOCHROME = 0x1u32
		const Monochrome           = 0x1u32;
		/// LR_SHARED = 0x8000u32
		const Shared               = 0x8000u32;
		/// LR_VGACOLOR = 0x80u32
		const VgaColor             = 0x80u32;
		/// LR_COPYDELETEORG = 0x8u32
		const CopyDeleteOrg        = 0x8u32;
		/// LR_COPYFROMRESOURCE = 0x4000u32
		const CopyFromResource     = 0x4000u32;
		/// LR_COPYRETURNORG = 0x4u32
		const CopyReturnOrg        = 0x4u32;
	}
}

bitflags::bitflags! {
	/// SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS
	#[repr(transparent)]
	pub struct SystemParametersInfoUpdateFlags: u32 {
		/// SPIF_UPDATEINIFILE = 0x1u32
		const UpdateIniFile        = 0x1u32;
		/// SPIF_SENDCHANGE = 0x2u32
		const SendChange           = 0x2u32;
		/// SPIF_SENDWININICHANGE = 0x2u32
		const SendWinIniChange     = 0x2u32;
	}
}

bitflags::bitflags! {
	/// SET_WINDOW_POS_FLAGS
	#[repr(transparent)]
	pub struct SetWindowPosFlags: u32 {
		/// SWP_ASYNCWINDOWPOS = 0x4000u32
		const AsyncWindowPos       = 0x4000u32;
		/// SWP_DEFERERASE = 0x2000u32
		const DeferErase           = 0x2000u32;
		/// SWP_DRAWFRAME = 0x20u32
		const DrawFrame            = 0x20u32;
		/// SWP_FRAMECHANGED = 0x20u32
		const FrameChanged         = 0x20u32;
		/// SWP_HIDEWINDOW = 0x80u32
		const HideWindow           = 0x80u32;
		/// SWP_NOACTIVATE = 0x10u32
		const NoActivate           = 0x10u32;
		/// SWP_NOCOPYBITS = 0x100u32
		const NoCopyBits           = 0x100u32;
		/// SWP_NOMOVE = 0x2u32
		const NoMove               = 0x2u32;
		/// SWP_NOOWNERZORDER = 0x200u32
		const NoOwnerZOrder        = 0x200u32;
		/// SWP_NOREDRAW = 0x8u32
		const NoRedraw             = 0x8u32;
		/// SWP_NOREPOSITION = 0x200u32
		const NoReposition         = 0x200u32;
		/// SWP_NOSENDCHANGING = 0x400u32
		const NoSendChanging       = 0x400u32;
		/// SWP_NOSIZE = 0x1u32
		const NoSize               = 0x1u32;
		/// SWP_NOZORDER = 0x4u32
		const NoZOrder             = 0x4u32;
		/// SWP_SHOWWINDOW = 0x40u32
		const ShowWindow           = 0x40u32;
	}
}

bitflags::bitflags! {
	/// MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS
	#[repr(transparent)]
	pub struct MsgWaitForMultipleObjectsExFlags: u32 {
		/// MWMO_NONE = 0x0u32
		const None                 = 0x0u32;
		/// MWMO_ALERTABLE = 0x2u32
		const Alertable            = 0x2u32;
		/// MWMO_INPUTAVAILABLE = 0x4u32
		const InputAvailable       = 0x4u32;
		/// MWMO_WAITALL = 0x1u32
		const WaitAll              = 0x1u32;
	}
}

bitflags::bitflags! {
	/// QUEUE_STATUS_FLAGS
	#[repr(transparent)]
	pub struct QueueStatusFlags: u32 {
		/// QS_ALLEVENTS = 0x4BFu32
		const AllEvents            = 0x4BFu32;
		/// QS_ALLINPUT = 0x4FFu32
		const AllInput             = 0x4FFu32;
		/// QS_ALLPOSTMESSAGE = 0x100u32
		const AllPostMessage       = 0x100u32;
		/// QS_HOTKEY = 0x80u32
		const Hotkey               = 0x80u32;
		/// QS_INPUT = 0x407u32
		const Input                = 0x407u32;
		/// QS_KEY = 0x1u32
		const Key                  = 0x1u32;
		/// QS_MOUSE = 0x6u32
		const Mouse                = 0x6u32;
		/// QS_MOUSEBUTTON = 0x4u32
		const MouseButton          = 0x4u32;
		/// QS_MOUSEMOVE = 0x2u32
		const MouseMove            = 0x2u32;
		/// QS_PAINT = 0x20u32
		const Paint                = 0x20u32;
		/// QS_POSTMESSAGE = 0x8u32
		const PostMessage          = 0x8u32;
		/// QS_RAWINPUT = 0x400u32
		const RawInput             = 0x400u32;
		/// QS_SENDMESSAGE = 0x40u32
		const SendMessage          = 0x40u32;
		/// QS_TIMER = 0x10u32
		const Timer                = 0x10u32;
	}
}

/// SYSTEM_CURSOR_ID
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SystemCursorId
{
	/// OCR_APPSTARTING = 0x7F8Au32
	AppStarting          = 0x7F8Au32,
	/// OCR_NORMAL = 0x7F00u32
	Normal               = 0x7F00u32,
	/// OCR_CROSS = 0x7F03u32
	Cross                = 0x7F03u32,
	/// OCR_HAND = 0x7F89u32
	Hand                 = 0x7F89u32,
	/// OCR_HELP = 0x7F8Bu32
	Help                 = 0x7F8Bu32,
	/// OCR_IBEAM = 0x7F01u32
	IBeam                = 0x7F01u32,
	/// OCR_NO = 0x7F88u32
	No                   = 0x7F88u32,
	/// OCR_SIZEALL = 0x7F86u32
	SizeAll              = 0x7F86u32,
	/// OCR_SIZENESW = 0x7F83u32
	SizeNesw             = 0x7F83u32,
	/// OCR_SIZENS = 0x7F85u32
	SIZEns               = 0x7F85u32,
	/// OCR_SIZENWSE = 0x7F82u32
	SIZenWSE             = 0x7F82u32,
	/// OCR_SIZEWE = 0x7F84u32
	SizeWe               = 0x7F84u32,
	/// OCR_UP = 0x7F04u32
	Up                   = 0x7F04u32,
	/// OCR_WAIT = 0x7F02u32
	Wait                 = 0x7F02u32,
}

bitflags::bitflags! {
	/// LAYERED_WINDOW_ATTRIBUTES_FLAGS
	#[repr(transparent)]
	pub struct LayeredWindowAttributesFlags: u32 {
		/// LWA_ALPHA = 0x2u32
		const Alpha                = 0x2u32;
		/// LWA_COLORKEY = 0x1u32
		const ColorKey             = 0x1u32;
	}
}

bitflags::bitflags! {
	/// SEND_MESSAGE_TIMEOUT_FLAGS
	#[repr(transparent)]
	pub struct SendMessageTimeoutFlags: u32 {
		/// SMTO_ABORTIFHUNG = 0x2u32
		const AbortIfHung          = 0x2u32;
		/// SMTO_BLOCK = 0x1u32
		const Block                = 0x1u32;
		/// SMTO_NORMAL = 0x0u32
		const Normal               = 0x0u32;
		/// SMTO_NOTIMEOUTIFNOTHUNG = 0x8u32
		const NoTimeoutIfNotHung   = 0x8u32;
		/// SMTO_ERRORONEXIT = 0x20u32
		const ErrorOnExit          = 0x20u32;
	}
}

bitflags::bitflags! {
	/// PEEK_MESSAGE_REMOVE_TYPE
	#[repr(transparent)]
	pub struct PeekMessageRemoveType: u32 {
		/// PM_NOREMOVE = 0x0u32
		const NoRemove             = 0x0u32;
		/// PM_REMOVE = 0x1u32
		const Remove               = 0x1u32;
		/// PM_NOYIELD = 0x2u32
		const NoYield              = 0x2u32;
		/// PM_QS_INPUT = 0x4070000u32
		const QsInput              = 0x4070000u32;
		/// PM_QS_POSTMESSAGE = 0x980000u32
		const QsPostMessage        = 0x980000u32;
		/// PM_QS_PAINT = 0x200000u32
		const QsPaint              = 0x200000u32;
		/// PM_QS_SENDMESSAGE = 0x400000u32
		const QsSendMessage        = 0x400000u32;
	}
}

/// SYS_COLOR_INDEX
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SysColorIndex
{
	/// COLOR_3DDKSHADOW = 0x15u32
	_3dDKShadow          = 0x15u32,
	/// COLOR_3DFACE = 0xFu32
	_3dFace              = 0xFu32,
	/// COLOR_3DHIGHLIGHT = 0x14u32
	_3dHighlight         = 0x14u32,
	/// COLOR_3DLIGHT = 0x16u32
	_3dLight             = 0x16u32,
	/// COLOR_3DSHADOW = 0x10u32
	_3dShadow            = 0x10u32,
	/// COLOR_ACTIVEBORDER = 0xAu32
	ActiveBorder         = 0xAu32,
	/// COLOR_ACTIVECAPTION = 0x2u32
	ActiveCaption        = 0x2u32,
	/// COLOR_APPWORKSPACE = 0xCu32
	AppWorkspace         = 0xCu32,
	/// COLOR_BACKGROUND = 0x1u32
	Background           = 0x1u32,
	/// COLOR_BTNTEXT = 0x12u32
	BtnText              = 0x12u32,
	/// COLOR_CAPTIONTEXT = 0x9u32
	CaptionText          = 0x9u32,
	/// COLOR_GRADIENTACTIVECAPTION = 0x1Bu32
	GradientActiveCaption = 0x1Bu32,
	/// COLOR_GRADIENTINACTIVECAPTION = 0x1Cu32
	GradientInactiveCaption = 0x1Cu32,
	/// COLOR_GRAYTEXT = 0x11u32
	GrayText             = 0x11u32,
	/// COLOR_HIGHLIGHT = 0xDu32
	Highlight            = 0xDu32,
	/// COLOR_HIGHLIGHTTEXT = 0xEu32
	HighlightText        = 0xEu32,
	/// COLOR_HOTLIGHT = 0x1Au32
	HotLight             = 0x1Au32,
	/// COLOR_INACTIVEBORDER = 0xBu32
	InactiveBorder       = 0xBu32,
	/// COLOR_INACTIVECAPTION = 0x3u32
	InactiveCaption      = 0x3u32,
	/// COLOR_INACTIVECAPTIONTEXT = 0x13u32
	InactiveCaptionText  = 0x13u32,
	/// COLOR_INFOBK = 0x18u32
	InFobK               = 0x18u32,
	/// COLOR_INFOTEXT = 0x17u32
	InfoText             = 0x17u32,
	/// COLOR_MENU = 0x4u32
	Menu                 = 0x4u32,
	/// COLOR_MENUHILIGHT = 0x1Du32
	MenuHilight          = 0x1Du32,
	/// COLOR_MENUBAR = 0x1Eu32
	MenuBar              = 0x1Eu32,
	/// COLOR_MENUTEXT = 0x7u32
	MenuText             = 0x7u32,
	/// COLOR_SCROLLBAR = 0x0u32
	Scrollbar            = 0x0u32,
	/// COLOR_WINDOW = 0x5u32
	Window               = 0x5u32,
	/// COLOR_WINDOWFRAME = 0x6u32
	WindowFrame          = 0x6u32,
	/// COLOR_WINDOWTEXT = 0x8u32
	WindowText           = 0x8u32,
}

impl SysColorIndex {
	/// COLOR_3DHILIGHT = 0x14u32
	pub const _3dHilight          : Self = unsafe { transmute(0x14u32) };
	/// COLOR_BTNFACE = 0xFu32
	pub const BtnFace             : Self = unsafe { transmute(0xFu32) };
	/// COLOR_BTNHIGHLIGHT = 0x14u32
	pub const BtnHighlight        : Self = unsafe { transmute(0x14u32) };
	/// COLOR_BTNHILIGHT = 0x14u32
	pub const BtnHilight          : Self = unsafe { transmute(0x14u32) };
	/// COLOR_BTNSHADOW = 0x10u32
	pub const BtnShadow           : Self = unsafe { transmute(0x10u32) };
	/// COLOR_DESKTOP = 0x1u32
	pub const Desktop             : Self = unsafe { transmute(0x1u32) };
}

/// GET_WINDOW_CMD
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetWindowCmd
{
	/// GW_CHILD = 0x5u32
	Child                = 0x5u32,
	/// GW_ENABLEDPOPUP = 0x6u32
	EnabledPopup         = 0x6u32,
	/// GW_HWNDFIRST = 0x0u32
	HWndFirst            = 0x0u32,
	/// GW_HWNDLAST = 0x1u32
	HWndLast             = 0x1u32,
	/// GW_HWNDNEXT = 0x2u32
	HWndNext             = 0x2u32,
	/// GW_HWNDPREV = 0x3u32
	HWndPrev             = 0x3u32,
	/// GW_OWNER = 0x4u32
	Owner                = 0x4u32,
}

/// SYSTEM_METRICS_INDEX
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SystemMetricsIndex
{
	/// SM_ARRANGE = 0x38u32
	Arrange              = 0x38u32,
	/// SM_CLEANBOOT = 0x43u32
	CleanBoot            = 0x43u32,
	/// SM_CMONITORS = 0x50u32
	CMonitors            = 0x50u32,
	/// SM_CMOUSEBUTTONS = 0x2Bu32
	CMouseButtons        = 0x2Bu32,
	/// SM_CONVERTIBLESLATEMODE = 0x2003u32
	ConvertibleSlateMode = 0x2003u32,
	/// SM_CXBORDER = 0x5u32
	CXBorder             = 0x5u32,
	/// SM_CXCURSOR = 0xDu32
	CXCursor             = 0xDu32,
	/// SM_CXDLGFRAME = 0x7u32
	CXDlgFrame           = 0x7u32,
	/// SM_CXDOUBLECLK = 0x24u32
	CXDoubleClk          = 0x24u32,
	/// SM_CXDRAG = 0x44u32
	CXDrag               = 0x44u32,
	/// SM_CXEDGE = 0x2Du32
	CXEdge               = 0x2Du32,
	/// SM_CXFOCUSBORDER = 0x53u32
	CXFocusBorder        = 0x53u32,
	/// SM_CXFRAME = 0x20u32
	CXFrame              = 0x20u32,
	/// SM_CXFULLSCREEN = 0x10u32
	CXFullScreen         = 0x10u32,
	/// SM_CXHSCROLL = 0x15u32
	CXHScroll            = 0x15u32,
	/// SM_CXHTHUMB = 0xAu32
	CXHThumb             = 0xAu32,
	/// SM_CXICON = 0xBu32
	CXIcon               = 0xBu32,
	/// SM_CXICONSPACING = 0x26u32
	CXIconSpacing        = 0x26u32,
	/// SM_CXMAXIMIZED = 0x3Du32
	CXMaximized          = 0x3Du32,
	/// SM_CXMAXTRACK = 0x3Bu32
	CXMaxTrack           = 0x3Bu32,
	/// SM_CXMENUCHECK = 0x47u32
	CXMenuCheck          = 0x47u32,
	/// SM_CXMENUSIZE = 0x36u32
	CXMenuSize           = 0x36u32,
	/// SM_CXMIN = 0x1Cu32
	CXMin                = 0x1Cu32,
	/// SM_CXMINIMIZED = 0x39u32
	CXMinimized          = 0x39u32,
	/// SM_CXMINSPACING = 0x2Fu32
	CXMinSpacing         = 0x2Fu32,
	/// SM_CXMINTRACK = 0x22u32
	CXMinTrack           = 0x22u32,
	/// SM_CXPADDEDBORDER = 0x5Cu32
	CXPaddedBorder       = 0x5Cu32,
	/// SM_CXSCREEN = 0x0u32
	CXScreen             = 0x0u32,
	/// SM_CXSIZE = 0x1Eu32
	CXSize               = 0x1Eu32,
	/// SM_CXSMICON = 0x31u32
	CXSmIcon             = 0x31u32,
	/// SM_CXSMSIZE = 0x34u32
	CXSmSize             = 0x34u32,
	/// SM_CXVIRTUALSCREEN = 0x4Eu32
	CXVirtualScreen      = 0x4Eu32,
	/// SM_CXVSCROLL = 0x2u32
	CXVScroll            = 0x2u32,
	/// SM_CYBORDER = 0x6u32
	CYBorder             = 0x6u32,
	/// SM_CYCAPTION = 0x4u32
	CYCaption            = 0x4u32,
	/// SM_CYCURSOR = 0xEu32
	CYCursor             = 0xEu32,
	/// SM_CYDLGFRAME = 0x8u32
	CYDlgFrame           = 0x8u32,
	/// SM_CYDOUBLECLK = 0x25u32
	CYDoubleClk          = 0x25u32,
	/// SM_CYDRAG = 0x45u32
	CYDrag               = 0x45u32,
	/// SM_CYEDGE = 0x2Eu32
	CYEdge               = 0x2Eu32,
	/// SM_CYFOCUSBORDER = 0x54u32
	CYFocusBorder        = 0x54u32,
	/// SM_CYFRAME = 0x21u32
	CYFrame              = 0x21u32,
	/// SM_CYFULLSCREEN = 0x11u32
	CYFullScreen         = 0x11u32,
	/// SM_CYHSCROLL = 0x3u32
	CYHScroll            = 0x3u32,
	/// SM_CYICON = 0xCu32
	CYIcon               = 0xCu32,
	/// SM_CYICONSPACING = 0x27u32
	CYIconSpacing        = 0x27u32,
	/// SM_CYKANJIWINDOW = 0x12u32
	CYKanjiWindow        = 0x12u32,
	/// SM_CYMAXIMIZED = 0x3Eu32
	CYMaximized          = 0x3Eu32,
	/// SM_CYMAXTRACK = 0x3Cu32
	CYMaxTrack           = 0x3Cu32,
	/// SM_CYMENU = 0xFu32
	CYMenu               = 0xFu32,
	/// SM_CYMENUCHECK = 0x48u32
	CYMenuCheck          = 0x48u32,
	/// SM_CYMENUSIZE = 0x37u32
	CYMenuSize           = 0x37u32,
	/// SM_CYMIN = 0x1Du32
	CYMin                = 0x1Du32,
	/// SM_CYMINIMIZED = 0x3Au32
	CYMinimized          = 0x3Au32,
	/// SM_CYMINSPACING = 0x30u32
	CYMinSpacing         = 0x30u32,
	/// SM_CYMINTRACK = 0x23u32
	CYMinTrack           = 0x23u32,
	/// SM_CYSCREEN = 0x1u32
	CYScreen             = 0x1u32,
	/// SM_CYSIZE = 0x1Fu32
	CYSize               = 0x1Fu32,
	/// SM_CYSMCAPTION = 0x33u32
	CYSmCaption          = 0x33u32,
	/// SM_CYSMICON = 0x32u32
	CYSmIcon             = 0x32u32,
	/// SM_CYSMSIZE = 0x35u32
	CYSmSize             = 0x35u32,
	/// SM_CYVIRTUALSCREEN = 0x4Fu32
	CYVirtualScreen      = 0x4Fu32,
	/// SM_CYVSCROLL = 0x14u32
	CYVScroll            = 0x14u32,
	/// SM_CYVTHUMB = 0x9u32
	CYVThumb             = 0x9u32,
	/// SM_DBCSENABLED = 0x2Au32
	DBCsEnabled          = 0x2Au32,
	/// SM_DEBUG = 0x16u32
	Debug                = 0x16u32,
	/// SM_DIGITIZER = 0x5Eu32
	Digitizer            = 0x5Eu32,
	/// SM_IMMENABLED = 0x52u32
	IMmEnabled           = 0x52u32,
	/// SM_MAXIMUMTOUCHES = 0x5Fu32
	MaximumTouches       = 0x5Fu32,
	/// SM_MEDIACENTER = 0x57u32
	MediaCenter          = 0x57u32,
	/// SM_MENUDROPALIGNMENT = 0x28u32
	MenuDropAlignment    = 0x28u32,
	/// SM_MIDEASTENABLED = 0x4Au32
	MideastEnabled       = 0x4Au32,
	/// SM_MOUSEPRESENT = 0x13u32
	MousePresent         = 0x13u32,
	/// SM_MOUSEHORIZONTALWHEELPRESENT = 0x5Bu32
	MouseHorizontalWheelPresent = 0x5Bu32,
	/// SM_MOUSEWHEELPRESENT = 0x4Bu32
	MouseWheelPresent    = 0x4Bu32,
	/// SM_NETWORK = 0x3Fu32
	Network              = 0x3Fu32,
	/// SM_PENWINDOWS = 0x29u32
	PenWindows           = 0x29u32,
	/// SM_REMOTECONTROL = 0x2001u32
	RemoteControl        = 0x2001u32,
	/// SM_REMOTESESSION = 0x1000u32
	RemoteSession        = 0x1000u32,
	/// SM_SAMEDISPLAYFORMAT = 0x51u32
	SameDisplayFormat    = 0x51u32,
	/// SM_SECURE = 0x2Cu32
	Secure               = 0x2Cu32,
	/// SM_SERVERR2 = 0x59u32
	ServErr2             = 0x59u32,
	/// SM_SHOWSOUNDS = 0x46u32
	ShowSounds           = 0x46u32,
	/// SM_SHUTTINGDOWN = 0x2000u32
	ShuttingDown         = 0x2000u32,
	/// SM_SLOWMACHINE = 0x49u32
	SlowMachine          = 0x49u32,
	/// SM_STARTER = 0x58u32
	Starter              = 0x58u32,
	/// SM_SWAPBUTTON = 0x17u32
	SwapButton           = 0x17u32,
	/// SM_SYSTEMDOCKED = 0x2004u32
	SystemDocked         = 0x2004u32,
	/// SM_TABLETPC = 0x56u32
	TabletPc             = 0x56u32,
	/// SM_XVIRTUALSCREEN = 0x4Cu32
	XVirtualScreen       = 0x4Cu32,
	/// SM_YVIRTUALSCREEN = 0x4Du32
	YVirtualScreen       = 0x4Du32,
}

impl SystemMetricsIndex {
	/// SM_CXFIXEDFRAME = 0x7u32
	pub const CXFixedFrame        : Self = unsafe { transmute(0x7u32) };
	/// SM_CXSIZEFRAME = 0x20u32
	pub const CXSizeFrame         : Self = unsafe { transmute(0x20u32) };
	/// SM_CYFIXEDFRAME = 0x8u32
	pub const CYFixedFrame        : Self = unsafe { transmute(0x8u32) };
	/// SM_CYSIZEFRAME = 0x21u32
	pub const CYSizeFrame         : Self = unsafe { transmute(0x21u32) };
}

/// GET_ANCESTOR_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetAncestorFlags
{
	/// GA_PARENT = 0x1u32
	Parent               = 0x1u32,
	/// GA_ROOT = 0x2u32
	Root                 = 0x2u32,
	/// GA_ROOTOWNER = 0x3u32
	RooTowner            = 0x3u32,
}

/// TILE_WINDOWS_HOW
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TileWindowsHow
{
	/// MDITILE_HORIZONTAL = 0x1u32
	Horizontal           = 0x1u32,
	/// MDITILE_VERTICAL = 0x0u32
	Vertical             = 0x0u32,
}

/// WINDOW_DISPLAY_AFFINITY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindowDisplayAffinity
{
	/// WDA_NONE = 0x0u32
	None                 = 0x0u32,
	/// WDA_MONITOR = 0x1u32
	Monitor              = 0x1u32,
	/// WDA_EXCLUDEFROMCAPTURE = 0x11u32
	ExcludeFromCapture   = 0x11u32,
}

/// FOREGROUND_WINDOW_LOCK_CODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ForegroundWindowLockCode
{
	/// LSFW_LOCK = 0x1u32
	Lock                 = 0x1u32,
	/// LSFW_UNLOCK = 0x2u32
	Unlock               = 0x2u32,
}

bitflags::bitflags! {
	/// CASCADE_WINDOWS_HOW
	#[repr(transparent)]
	pub struct CascadeWindowsHow: u32 {
		/// MDITILE_SKIPDISABLED = 0x2u32
		const SkipDisabled         = 0x2u32;
		/// MDITILE_ZORDER = 0x4u32
		const ZOrder               = 0x4u32;
	}
}

/// WINDOW_MESSAGE_FILTER_ACTION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindowMessageFilterAction
{
	/// MSGFLT_ALLOW = 0x1u32
	Allow                = 0x1u32,
	/// MSGFLT_DISALLOW = 0x2u32
	Disallow             = 0x2u32,
	/// MSGFLT_RESET = 0x0u32
	Reset                = 0x0u32,
}

bitflags::bitflags! {
	/// GET_MENU_DEFAULT_ITEM_FLAGS
	#[repr(transparent)]
	pub struct GetMenuDefaultItemFlags: u32 {
		/// GMDI_GOINTOPOPUPS = 0x2u32
		const GoIntoPopups         = 0x2u32;
		/// GMDI_USEDISABLED = 0x1u32
		const UseDisabled          = 0x1u32;
	}
}

/// MSGFLTINFO_STATUS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MsgFltInfoStatus
{
	/// MSGFLTINFO_NONE = 0x0u32
	None                 = 0x0u32,
	/// MSGFLTINFO_ALLOWED_HIGHER = 0x3u32
	AllowedHigher        = 0x3u32,
	/// MSGFLTINFO_ALREADYALLOWED_FORWND = 0x1u32
	AlreadyAllowedForWnd = 0x1u32,
	/// MSGFLTINFO_ALREADYDISALLOWED_FORWND = 0x2u32
	AlreadyDisallowedForWnd = 0x2u32,
}

bitflags::bitflags! {
	/// MOUSEHOOKSTRUCTEX_MOUSE_DATA
	#[repr(transparent)]
	pub struct MouseHookStrUCTexMouseData: u32 {
		/// XBUTTON1 = 0x1u32
		const XButton1             = 0x1u32;
		/// XBUTTON2 = 0x2u32
		const XButton2             = 0x2u32;
	}
}

bitflags::bitflags! {
	/// MENU_ITEM_MASK
	#[repr(transparent)]
	pub struct MenuItemMask: u32 {
		/// MIIM_BITMAP = 0x80u32
		const Bitmap               = 0x80u32;
		/// MIIM_CHECKMARKS = 0x8u32
		const CheckMarks           = 0x8u32;
		/// MIIM_DATA = 0x20u32
		const Data                 = 0x20u32;
		/// MIIM_FTYPE = 0x100u32
		const FType                = 0x100u32;
		/// MIIM_ID = 0x2u32
		const Id                   = 0x2u32;
		/// MIIM_STATE = 0x1u32
		const State                = 0x1u32;
		/// MIIM_STRING = 0x40u32
		const String               = 0x40u32;
		/// MIIM_SUBMENU = 0x4u32
		const SubMenu              = 0x4u32;
		/// MIIM_TYPE = 0x10u32
		const Type                 = 0x10u32;
	}
}

bitflags::bitflags! {
	/// FLASHWINFO_FLAGS
	#[repr(transparent)]
	pub struct FlashWInfoFlags: u32 {
		/// FLASHW_ALL = 0x3u32
		const All                  = 0x3u32;
		/// FLASHW_CAPTION = 0x1u32
		const Caption              = 0x1u32;
		/// FLASHW_STOP = 0x0u32
		const Stop                 = 0x0u32;
		/// FLASHW_TIMER = 0x4u32
		const Timer                = 0x4u32;
		/// FLASHW_TIMERNOFG = 0xCu32
		const TimErnOfG            = 0xCu32;
		/// FLASHW_TRAY = 0x2u32
		const Tray                 = 0x2u32;
	}
}

/// CURSORINFO_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CursorInfoFlags
{
	/// CURSOR_SHOWING = 0x1u32
	Showing              = 0x1u32,
	/// CURSOR_SUPPRESSED = 0x2u32
	Suppressed           = 0x2u32,
}

bitflags::bitflags! {
	/// MENUINFO_STYLE
	#[repr(transparent)]
	pub struct MenuInfoStyle: u32 {
		/// MNS_AUTODISMISS = 0x10000000u32
		const AutoDismiss          = 0x10000000u32;
		/// MNS_CHECKORBMP = 0x4000000u32
		const CheCKorBmp           = 0x4000000u32;
		/// MNS_DRAGDROP = 0x20000000u32
		const DragDrop             = 0x20000000u32;
		/// MNS_MODELESS = 0x40000000u32
		const Modeless             = 0x40000000u32;
		/// MNS_NOCHECK = 0x80000000u32
		const NoCheck              = 0x80000000u32;
		/// MNS_NOTIFYBYPOS = 0x8000000u32
		const NotifyByPos          = 0x8000000u32;
	}
}

bitflags::bitflags! {
	/// WINDOWPLACEMENT_FLAGS
	#[repr(transparent)]
	pub struct WindowPlacementFlags: u32 {
		/// WPF_ASYNCWINDOWPLACEMENT = 0x4u32
		const AsyncWindowPlacement = 0x4u32;
		/// WPF_RESTORETOMAXIMIZED = 0x2u32
		const RestoreToMaximized   = 0x2u32;
		/// WPF_SETMINPOSITION = 0x1u32
		const SetMinPosition       = 0x1u32;
	}
}

bitflags::bitflags! {
	/// MENUINFO_MASK
	#[repr(transparent)]
	pub struct MenuInfoMask: u32 {
		/// MIM_APPLYTOSUBMENUS = 0x80000000u32
		const ApplyToSubMenus      = 0x80000000u32;
		/// MIM_BACKGROUND = 0x2u32
		const Background           = 0x2u32;
		/// MIM_HELPID = 0x4u32
		const HelpId               = 0x4u32;
		/// MIM_MAXHEIGHT = 0x1u32
		const MaxHeight            = 0x1u32;
		/// MIM_MENUDATA = 0x8u32
		const MenuData             = 0x8u32;
		/// MIM_STYLE = 0x10u32
		const Style                = 0x10u32;
	}
}

/// MINIMIZEDMETRICS_ARRANGE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MinimizedMetricsArrange
{
	/// ARW_BOTTOMLEFT = 0x0i32
	BottomLeft           = 0x0i32,
	/// ARW_BOTTOMRIGHT = 0x1i32
	BottomRight          = 0x1i32,
	/// ARW_TOPLEFT = 0x2i32
	TopLeft              = 0x2i32,
	/// ARW_TOPRIGHT = 0x3i32
	TopRight             = 0x3i32,
}

bitflags::bitflags! {
	/// SCROLLINFO_MASK
	#[repr(transparent)]
	pub struct ScrollInfoMask: u32 {
		/// SIF_ALL = 0x17u32
		const All                  = 0x17u32;
		/// SIF_DISABLENOSCROLL = 0x8u32
		const DisableNoScroll      = 0x8u32;
		/// SIF_PAGE = 0x2u32
		const Page                 = 0x2u32;
		/// SIF_POS = 0x4u32
		const Pos                  = 0x4u32;
		/// SIF_RANGE = 0x1u32
		const Range                = 0x1u32;
		/// SIF_TRACKPOS = 0x10u32
		const TrackPos             = 0x10u32;
	}
}

/// MENUGETOBJECTINFO_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MenuGetObjectInfoFlags
{
	/// MNGOF_BOTTOMGAP = 0x2u32
	BottomGap            = 0x2u32,
	/// MNGOF_TOPGAP = 0x1u32
	TopGap               = 0x1u32,
}

bitflags::bitflags! {
	/// GUITHREADINFO_FLAGS
	#[repr(transparent)]
	pub struct GuiThreadInfoFlags: u32 {
		/// GUI_CARETBLINKING = 0x1u32
		const CaretBlinking        = 0x1u32;
		/// GUI_INMENUMODE = 0x4u32
		const InMenuMode           = 0x4u32;
		/// GUI_INMOVESIZE = 0x2u32
		const InMoveSize           = 0x2u32;
		/// GUI_POPUPMENUMODE = 0x10u32
		const PopupMenuMode        = 0x10u32;
		/// GUI_SYSTEMMENUMODE = 0x8u32
		const SystemMenuMode       = 0x8u32;
	}
}

bitflags::bitflags! {
	/// KBDLLHOOKSTRUCT_FLAGS
	#[repr(transparent)]
	pub struct KBDllHookStructFlags: u32 {
		/// LLKHF_EXTENDED = 0x1u32
		const Extended             = 0x1u32;
		/// LLKHF_ALTDOWN = 0x20u32
		const AltDown              = 0x20u32;
		/// LLKHF_UP = 0x80u32
		const Up                   = 0x80u32;
		/// LLKHF_INJECTED = 0x10u32
		const Injected             = 0x10u32;
		/// LLKHF_LOWER_IL_INJECTED = 0x2u32
		const LowerIlInjected      = 0x2u32;
	}
}

/// 
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindowMessage
{
	///  = 0xAEu32
	NcUahDrawCaption     = 0xAEu32,
	///  = 0xAFu32
	NcUahDrawFrame       = 0xAFu32,
	/// WM_DEVICECHANGE = 0x219u32
	DeviceChange         = 0x219u32,
	/// WM_CONTEXTMENU = 0x7Bu32
	ContextMenu          = 0x7Bu32,
	/// WM_UNICHAR = 0x109u32
	UniChar              = 0x109u32,
	/// WM_PRINTCLIENT = 0x318u32
	PrintClient          = 0x318u32,
	/// WM_NOTIFY = 0x4Eu32
	Notify               = 0x4Eu32,
	/// WM_NULL = 0x0u32
	Null                 = 0x0u32,
	/// WM_CREATE = 0x1u32
	Create               = 0x1u32,
	/// WM_DESTROY = 0x2u32
	Destroy              = 0x2u32,
	/// WM_MOVE = 0x3u32
	Move                 = 0x3u32,
	/// WM_SIZE = 0x5u32
	Size                 = 0x5u32,
	/// WM_ACTIVATE = 0x6u32
	Activate             = 0x6u32,
	/// WM_SETFOCUS = 0x7u32
	SetFocus             = 0x7u32,
	/// WM_KILLFOCUS = 0x8u32
	KillFocus            = 0x8u32,
	/// WM_ENABLE = 0xAu32
	Enable               = 0xAu32,
	/// WM_SETREDRAW = 0xBu32
	SetRedraw            = 0xBu32,
	/// WM_SETTEXT = 0xCu32
	SetText              = 0xCu32,
	/// WM_GETTEXT = 0xDu32
	GetText              = 0xDu32,
	/// WM_GETTEXTLENGTH = 0xEu32
	GetTextLength        = 0xEu32,
	/// WM_PAINT = 0xFu32
	Paint                = 0xFu32,
	/// WM_CLOSE = 0x10u32
	Close                = 0x10u32,
	/// WM_QUERYENDSESSION = 0x11u32
	QueryEndSession      = 0x11u32,
	/// WM_QUERYOPEN = 0x13u32
	QueryOpen            = 0x13u32,
	/// WM_ENDSESSION = 0x16u32
	EndSession           = 0x16u32,
	/// WM_QUIT = 0x12u32
	Quit                 = 0x12u32,
	/// WM_ERASEBKGND = 0x14u32
	EraseBkgnd           = 0x14u32,
	/// WM_SYSCOLORCHANGE = 0x15u32
	SysColorChange       = 0x15u32,
	/// WM_SHOWWINDOW = 0x18u32
	ShowWindow           = 0x18u32,
	/// WM_WININICHANGE = 0x1Au32
	WinIniChange         = 0x1Au32,
	/// WM_DEVMODECHANGE = 0x1Bu32
	DevModeChange        = 0x1Bu32,
	/// WM_ACTIVATEAPP = 0x1Cu32
	ActivateApp          = 0x1Cu32,
	/// WM_FONTCHANGE = 0x1Du32
	FontChange           = 0x1Du32,
	/// WM_TIMECHANGE = 0x1Eu32
	TimeChange           = 0x1Eu32,
	/// WM_CANCELMODE = 0x1Fu32
	CancelMode           = 0x1Fu32,
	/// WM_SETCURSOR = 0x20u32
	SetCursor            = 0x20u32,
	/// WM_MOUSEACTIVATE = 0x21u32
	MouseActivate        = 0x21u32,
	/// WM_CHILDACTIVATE = 0x22u32
	ChildActivate        = 0x22u32,
	/// WM_QUEUESYNC = 0x23u32
	QueueSync            = 0x23u32,
	/// WM_GETMINMAXINFO = 0x24u32
	GetMinMaxInfo        = 0x24u32,
	/// WM_PAINTICON = 0x26u32
	PaintIcon            = 0x26u32,
	/// WM_ICONERASEBKGND = 0x27u32
	IconEraseBkgnd       = 0x27u32,
	/// WM_NEXTDLGCTL = 0x28u32
	NextDlgCtl           = 0x28u32,
	/// WM_SPOOLERSTATUS = 0x2Au32
	SpoolerStatus        = 0x2Au32,
	/// WM_DRAWITEM = 0x2Bu32
	DrawItem             = 0x2Bu32,
	/// WM_MEASUREITEM = 0x2Cu32
	MeasureItem          = 0x2Cu32,
	/// WM_DELETEITEM = 0x2Du32
	DeleteItem           = 0x2Du32,
	/// WM_VKEYTOITEM = 0x2Eu32
	VKeyToItem           = 0x2Eu32,
	/// WM_CHARTOITEM = 0x2Fu32
	CharToItem           = 0x2Fu32,
	/// WM_SETFONT = 0x30u32
	SetFont              = 0x30u32,
	/// WM_GETFONT = 0x31u32
	GetFont              = 0x31u32,
	/// WM_SETHOTKEY = 0x32u32
	SetHotkey            = 0x32u32,
	/// WM_GETHOTKEY = 0x33u32
	GetHotkey            = 0x33u32,
	/// WM_QUERYDRAGICON = 0x37u32
	QueryDragIcon        = 0x37u32,
	/// WM_COMPAREITEM = 0x39u32
	CompareItem          = 0x39u32,
	/// WM_GETOBJECT = 0x3Du32
	GetObject            = 0x3Du32,
	/// WM_COMPACTING = 0x41u32
	Compacting           = 0x41u32,
	/// WM_COMMNOTIFY = 0x44u32
	CommNotify           = 0x44u32,
	/// WM_WINDOWPOSCHANGING = 0x46u32
	WindowPosChanging    = 0x46u32,
	/// WM_WINDOWPOSCHANGED = 0x47u32
	WindowPosChanged     = 0x47u32,
	/// WM_POWER = 0x48u32
	Power                = 0x48u32,
	/// WM_COPYDATA = 0x4Au32
	CopyData             = 0x4Au32,
	/// WM_CANCELJOURNAL = 0x4Bu32
	CancelJournal        = 0x4Bu32,
	/// WM_INPUTLANGCHANGEREQUEST = 0x50u32
	InputLangChangeRequest = 0x50u32,
	/// WM_INPUTLANGCHANGE = 0x51u32
	InputLangChange      = 0x51u32,
	/// WM_TCARD = 0x52u32
	TCard                = 0x52u32,
	/// WM_HELP = 0x53u32
	Help                 = 0x53u32,
	/// WM_USERCHANGED = 0x54u32
	UserChanged          = 0x54u32,
	/// WM_NOTIFYFORMAT = 0x55u32
	NotifyFormat         = 0x55u32,
	/// WM_STYLECHANGING = 0x7Cu32
	StyleChanging        = 0x7Cu32,
	/// WM_STYLECHANGED = 0x7Du32
	StyleChanged         = 0x7Du32,
	/// WM_DISPLAYCHANGE = 0x7Eu32
	DisplayChange        = 0x7Eu32,
	/// WM_GETICON = 0x7Fu32
	GetIcon              = 0x7Fu32,
	/// WM_SETICON = 0x80u32
	SetIcon              = 0x80u32,
	/// WM_NCCREATE = 0x81u32
	NcCreate             = 0x81u32,
	/// WM_NCDESTROY = 0x82u32
	NcDestroy            = 0x82u32,
	/// WM_NCCALCSIZE = 0x83u32
	NcCalcSize           = 0x83u32,
	/// WM_NCHITTEST = 0x84u32
	NcHitTest            = 0x84u32,
	/// WM_NCPAINT = 0x85u32
	NcPaint              = 0x85u32,
	/// WM_NCACTIVATE = 0x86u32
	NcActivate           = 0x86u32,
	/// WM_GETDLGCODE = 0x87u32
	GetDlgCode           = 0x87u32,
	/// WM_SYNCPAINT = 0x88u32
	SyncPaint            = 0x88u32,
	/// WM_NCMOUSEMOVE = 0xA0u32
	NcMouseMove          = 0xA0u32,
	/// WM_NCLBUTTONDOWN = 0xA1u32
	NclButtonDown        = 0xA1u32,
	/// WM_NCLBUTTONUP = 0xA2u32
	NclButtonUp          = 0xA2u32,
	/// WM_NCLBUTTONDBLCLK = 0xA3u32
	NclButtonDblClk      = 0xA3u32,
	/// WM_NCRBUTTONDOWN = 0xA4u32
	NcRButtonDown        = 0xA4u32,
	/// WM_NCRBUTTONUP = 0xA5u32
	NcRButtonUp          = 0xA5u32,
	/// WM_NCRBUTTONDBLCLK = 0xA6u32
	NcRButtonDblClk      = 0xA6u32,
	/// WM_NCMBUTTONDOWN = 0xA7u32
	NcMButtonDown        = 0xA7u32,
	/// WM_NCMBUTTONUP = 0xA8u32
	NcMButtonUp          = 0xA8u32,
	/// WM_NCMBUTTONDBLCLK = 0xA9u32
	NcMButtonDblClk      = 0xA9u32,
	/// WM_NCXBUTTONDOWN = 0xABu32
	NcXButtonDown        = 0xABu32,
	/// WM_NCXBUTTONUP = 0xACu32
	NcXButtonUp          = 0xACu32,
	/// WM_NCXBUTTONDBLCLK = 0xADu32
	NcXButtonDblClk      = 0xADu32,
	/// WM_INPUT_DEVICE_CHANGE = 0xFEu32
	InputDeviceChange    = 0xFEu32,
	/// WM_INPUT = 0xFFu32
	Input                = 0xFFu32,
	/// WM_KEYFIRST = 0x100u32
	KeyFirst             = 0x100u32,
	/// WM_KEYUP = 0x101u32
	KeyUp                = 0x101u32,
	/// WM_CHAR = 0x102u32
	Char                 = 0x102u32,
	/// WM_DEADCHAR = 0x103u32
	DeadChar             = 0x103u32,
	/// WM_SYSKEYDOWN = 0x104u32
	SysKeyDown           = 0x104u32,
	/// WM_SYSKEYUP = 0x105u32
	SysKeyUp             = 0x105u32,
	/// WM_SYSCHAR = 0x106u32
	SysChar              = 0x106u32,
	/// WM_SYSDEADCHAR = 0x107u32
	SysDeadChar          = 0x107u32,
	/// WM_IME_STARTCOMPOSITION = 0x10Du32
	ImeStartComposition  = 0x10Du32,
	/// WM_IME_ENDCOMPOSITION = 0x10Eu32
	ImeEndComposition    = 0x10Eu32,
	/// WM_IME_COMPOSITION = 0x10Fu32
	ImeComposition       = 0x10Fu32,
	/// WM_INITDIALOG = 0x110u32
	InitDialog           = 0x110u32,
	/// WM_COMMAND = 0x111u32
	Command              = 0x111u32,
	/// WM_SYSCOMMAND = 0x112u32
	SysCommand           = 0x112u32,
	/// WM_TIMER = 0x113u32
	Timer                = 0x113u32,
	/// WM_HSCROLL = 0x114u32
	HScroll              = 0x114u32,
	/// WM_VSCROLL = 0x115u32
	VScroll              = 0x115u32,
	/// WM_INITMENU = 0x116u32
	InitMenu             = 0x116u32,
	/// WM_INITMENUPOPUP = 0x117u32
	InitMenuPopup        = 0x117u32,
	/// WM_GESTURE = 0x119u32
	Gesture              = 0x119u32,
	/// WM_GESTURENOTIFY = 0x11Au32
	GestureNotify        = 0x11Au32,
	/// WM_MENUSELECT = 0x11Fu32
	MenuSelect           = 0x11Fu32,
	/// WM_MENUCHAR = 0x120u32
	MenuChar             = 0x120u32,
	/// WM_ENTERIDLE = 0x121u32
	EnterIdle            = 0x121u32,
	/// WM_MENURBUTTONUP = 0x122u32
	MenuRButtonUp        = 0x122u32,
	/// WM_MENUDRAG = 0x123u32
	MenuDrag             = 0x123u32,
	/// WM_MENUGETOBJECT = 0x124u32
	MenuGetObject        = 0x124u32,
	/// WM_UNINITMENUPOPUP = 0x125u32
	UninitMenuPopup      = 0x125u32,
	/// WM_MENUCOMMAND = 0x126u32
	MenuCommand          = 0x126u32,
	/// WM_CHANGEUISTATE = 0x127u32
	ChangeUiState        = 0x127u32,
	/// WM_UPDATEUISTATE = 0x128u32
	UpdateUiState        = 0x128u32,
	/// WM_QUERYUISTATE = 0x129u32
	QueryUiState         = 0x129u32,
	/// WM_CTLCOLORMSGBOX = 0x132u32
	CtlColorMsgBox       = 0x132u32,
	/// WM_CTLCOLOREDIT = 0x133u32
	CtlColorEdit         = 0x133u32,
	/// WM_CTLCOLORLISTBOX = 0x134u32
	CtlColorListBox      = 0x134u32,
	/// WM_CTLCOLORBTN = 0x135u32
	CtlColorBtn          = 0x135u32,
	/// WM_CTLCOLORDLG = 0x136u32
	CtlColorDlg          = 0x136u32,
	/// WM_CTLCOLORSCROLLBAR = 0x137u32
	CtlColorScrollbar    = 0x137u32,
	/// WM_CTLCOLORSTATIC = 0x138u32
	CtlColorStatic       = 0x138u32,
	/// WM_MOUSEFIRST = 0x200u32
	MouseFirst           = 0x200u32,
	/// WM_LBUTTONDOWN = 0x201u32
	LButtonDown          = 0x201u32,
	/// WM_LBUTTONUP = 0x202u32
	LButtonUp            = 0x202u32,
	/// WM_LBUTTONDBLCLK = 0x203u32
	LButtonDblClk        = 0x203u32,
	/// WM_RBUTTONDOWN = 0x204u32
	RButtonDown          = 0x204u32,
	/// WM_RBUTTONUP = 0x205u32
	RButtonUp            = 0x205u32,
	/// WM_RBUTTONDBLCLK = 0x206u32
	RButtonDblClk        = 0x206u32,
	/// WM_MBUTTONDOWN = 0x207u32
	MButtonDown          = 0x207u32,
	/// WM_MBUTTONUP = 0x208u32
	MButtonUp            = 0x208u32,
	/// WM_MBUTTONDBLCLK = 0x209u32
	MButtonDblClk        = 0x209u32,
	/// WM_MOUSEWHEEL = 0x20Au32
	MouseWheel           = 0x20Au32,
	/// WM_XBUTTONDOWN = 0x20Bu32
	XButtonDown          = 0x20Bu32,
	/// WM_XBUTTONUP = 0x20Cu32
	XButtonUp            = 0x20Cu32,
	/// WM_XBUTTONDBLCLK = 0x20Du32
	XButtonDblClk        = 0x20Du32,
	/// WM_MOUSEHWHEEL = 0x20Eu32
	MouseHWheel          = 0x20Eu32,
	/// WM_PARENTNOTIFY = 0x210u32
	ParentNotify         = 0x210u32,
	/// WM_ENTERMENULOOP = 0x211u32
	EnterMenuLoop        = 0x211u32,
	/// WM_EXITMENULOOP = 0x212u32
	ExitMenuLoop         = 0x212u32,
	/// WM_NEXTMENU = 0x213u32
	NextMenu             = 0x213u32,
	/// WM_SIZING = 0x214u32
	Sizing               = 0x214u32,
	/// WM_CAPTURECHANGED = 0x215u32
	CaptureChanged       = 0x215u32,
	/// WM_MOVING = 0x216u32
	Moving               = 0x216u32,
	/// WM_POWERBROADCAST = 0x218u32
	PowerBroadcast       = 0x218u32,
	/// WM_MDICREATE = 0x220u32
	MdiCreate            = 0x220u32,
	/// WM_MDIDESTROY = 0x221u32
	MdiDestroy           = 0x221u32,
	/// WM_MDIACTIVATE = 0x222u32
	MdiActivate          = 0x222u32,
	/// WM_MDIRESTORE = 0x223u32
	MdiRestore           = 0x223u32,
	/// WM_MDINEXT = 0x224u32
	MdiNext              = 0x224u32,
	/// WM_MDIMAXIMIZE = 0x225u32
	MdiMaximize          = 0x225u32,
	/// WM_MDITILE = 0x226u32
	MdiTile              = 0x226u32,
	/// WM_MDICASCADE = 0x227u32
	MdiCascade           = 0x227u32,
	/// WM_MDIICONARRANGE = 0x228u32
	MdiIconArrange       = 0x228u32,
	/// WM_MDIGETACTIVE = 0x229u32
	MdiGetActive         = 0x229u32,
	/// WM_MDISETMENU = 0x230u32
	MdiSetMenu           = 0x230u32,
	/// WM_ENTERSIZEMOVE = 0x231u32
	EnterSizeMove        = 0x231u32,
	/// WM_EXITSIZEMOVE = 0x232u32
	ExitSizeMove         = 0x232u32,
	/// WM_DROPFILES = 0x233u32
	DropFiles            = 0x233u32,
	/// WM_MDIREFRESHMENU = 0x234u32
	MdiRefreshMenu       = 0x234u32,
	/// WM_POINTERDEVICECHANGE = 0x238u32
	PointerDeviceChange  = 0x238u32,
	/// WM_POINTERDEVICEINRANGE = 0x239u32
	PointerDeviceInRange = 0x239u32,
	/// WM_POINTERDEVICEOUTOFRANGE = 0x23Au32
	PointerDeviceOutOfRange = 0x23Au32,
	/// WM_TOUCH = 0x240u32
	Touch                = 0x240u32,
	/// WM_NCPOINTERUPDATE = 0x241u32
	NcPointerUpdate      = 0x241u32,
	/// WM_NCPOINTERDOWN = 0x242u32
	NcPointerDown        = 0x242u32,
	/// WM_NCPOINTERUP = 0x243u32
	NcPointerUp          = 0x243u32,
	/// WM_POINTERUPDATE = 0x245u32
	PointerUpdate        = 0x245u32,
	/// WM_POINTERDOWN = 0x246u32
	PointerDown          = 0x246u32,
	/// WM_POINTERUP = 0x247u32
	PointerUp            = 0x247u32,
	/// WM_POINTERENTER = 0x249u32
	PointerEnter         = 0x249u32,
	/// WM_POINTERLEAVE = 0x24Au32
	POInterleave         = 0x24Au32,
	/// WM_POINTERACTIVATE = 0x24Bu32
	PointerActivate      = 0x24Bu32,
	/// WM_POINTERCAPTURECHANGED = 0x24Cu32
	PointerCaptureChanged = 0x24Cu32,
	/// WM_TOUCHHITTESTING = 0x24Du32
	TouchHitTesting      = 0x24Du32,
	/// WM_POINTERWHEEL = 0x24Eu32
	PointerWheel         = 0x24Eu32,
	/// WM_POINTERHWHEEL = 0x24Fu32
	PointerHWheel        = 0x24Fu32,
	/// WM_POINTERROUTEDTO = 0x251u32
	PointerRoutedTo      = 0x251u32,
	/// WM_POINTERROUTEDAWAY = 0x252u32
	PointerRoutedAway    = 0x252u32,
	/// WM_POINTERROUTEDRELEASED = 0x253u32
	PointerRoutedReleased = 0x253u32,
	/// WM_IME_SETCONTEXT = 0x281u32
	ImeSetContext        = 0x281u32,
	/// WM_IME_NOTIFY = 0x282u32
	ImeNotify            = 0x282u32,
	/// WM_IME_CONTROL = 0x283u32
	ImeControl           = 0x283u32,
	/// WM_IME_COMPOSITIONFULL = 0x284u32
	ImeCompositionFull   = 0x284u32,
	/// WM_IME_SELECT = 0x285u32
	ImeSelect            = 0x285u32,
	/// WM_IME_CHAR = 0x286u32
	ImeChar              = 0x286u32,
	/// WM_IME_REQUEST = 0x288u32
	ImeRequest           = 0x288u32,
	/// WM_IME_KEYDOWN = 0x290u32
	ImeKeyDown           = 0x290u32,
	/// WM_IME_KEYUP = 0x291u32
	ImeKeyUp             = 0x291u32,
	/// WM_NCMOUSEHOVER = 0x2A0u32
	NcMouseHover         = 0x2A0u32,
	/// WM_NCMOUSELEAVE = 0x2A2u32
	NcMouseLeave         = 0x2A2u32,
	/// WM_WTSSESSION_CHANGE = 0x2B1u32
	WTSSessionChange     = 0x2B1u32,
	/// WM_TABLET_FIRST = 0x2C0u32
	TabletFirst          = 0x2C0u32,
	/// WM_TABLET_LAST = 0x2DFu32
	TabletLast           = 0x2DFu32,
	/// WM_DPICHANGED = 0x2E0u32
	DpiChanged           = 0x2E0u32,
	/// WM_DPICHANGED_BEFOREPARENT = 0x2E2u32
	DpiChangedBeforeParent = 0x2E2u32,
	/// WM_DPICHANGED_AFTERPARENT = 0x2E3u32
	DpiChangedAfterParent = 0x2E3u32,
	/// WM_GETDPISCALEDSIZE = 0x2E4u32
	GetDpiScaledSize     = 0x2E4u32,
	/// WM_CUT = 0x300u32
	Cut                  = 0x300u32,
	/// WM_COPY = 0x301u32
	Copy                 = 0x301u32,
	/// WM_PASTE = 0x302u32
	Paste                = 0x302u32,
	/// WM_CLEAR = 0x303u32
	Clear                = 0x303u32,
	/// WM_UNDO = 0x304u32
	Undo                 = 0x304u32,
	/// WM_RENDERFORMAT = 0x305u32
	RenderFormat         = 0x305u32,
	/// WM_RENDERALLFORMATS = 0x306u32
	RenderAllFormats     = 0x306u32,
	/// WM_DESTROYCLIPBOARD = 0x307u32
	DestroyClipboard     = 0x307u32,
	/// WM_DRAWCLIPBOARD = 0x308u32
	DrawClipboard        = 0x308u32,
	/// WM_PAINTCLIPBOARD = 0x309u32
	PaintClipboard       = 0x309u32,
	/// WM_VSCROLLCLIPBOARD = 0x30Au32
	VScrollClipboard     = 0x30Au32,
	/// WM_SIZECLIPBOARD = 0x30Bu32
	SizeClipboard        = 0x30Bu32,
	/// WM_ASKCBFORMATNAME = 0x30Cu32
	AskCbFormatName      = 0x30Cu32,
	/// WM_CHANGECBCHAIN = 0x30Du32
	ChangEcbChain        = 0x30Du32,
	/// WM_HSCROLLCLIPBOARD = 0x30Eu32
	HScrollClipboard     = 0x30Eu32,
	/// WM_QUERYNEWPALETTE = 0x30Fu32
	QueryNewPalette      = 0x30Fu32,
	/// WM_PALETTEISCHANGING = 0x310u32
	PaletteIsChanging    = 0x310u32,
	/// WM_PALETTECHANGED = 0x311u32
	PaletteChanged       = 0x311u32,
	/// WM_HOTKEY = 0x312u32
	Hotkey               = 0x312u32,
	/// WM_PRINT = 0x317u32
	Print                = 0x317u32,
	/// WM_APPCOMMAND = 0x319u32
	AppCommand           = 0x319u32,
	/// WM_THEMECHANGED = 0x31Au32
	ThemeChanged         = 0x31Au32,
	/// WM_CLIPBOARDUPDATE = 0x31Du32
	ClipboardUpdate      = 0x31Du32,
	/// WM_DWMCOMPOSITIONCHANGED = 0x31Eu32
	DwmCompositionChanged = 0x31Eu32,
	/// WM_DWMNCRENDERINGCHANGED = 0x31Fu32
	DwmNcRenderingChanged = 0x31Fu32,
	/// WM_DWMCOLORIZATIONCOLORCHANGED = 0x320u32
	DwmColorizationColorChanged = 0x320u32,
	/// WM_DWMWINDOWMAXIMIZEDCHANGE = 0x321u32
	DwmWindowMaximizedChange = 0x321u32,
	/// WM_DWMSENDICONICTHUMBNAIL = 0x323u32
	DwmSendIconicThumbnail = 0x323u32,
	/// WM_DWMSENDICONICLIVEPREVIEWBITMAP = 0x326u32
	DwmSendIconicLivePreviewBitmap = 0x326u32,
	/// WM_GETTITLEBARINFOEX = 0x33Fu32
	GetTitleBarInfoEx    = 0x33Fu32,
	/// WM_HANDHELDFIRST = 0x358u32
	HandHeldFirst        = 0x358u32,
	/// WM_HANDHELDLAST = 0x35Fu32
	HandHeldLast         = 0x35Fu32,
	/// WM_AFXFIRST = 0x360u32
	AfxFirst             = 0x360u32,
	/// WM_AFXLAST = 0x37Fu32
	AfxLast              = 0x37Fu32,
	/// WM_PENWINFIRST = 0x380u32
	PenWinFirst          = 0x380u32,
	/// WM_PENWINLAST = 0x38Fu32
	PenWinLast           = 0x38Fu32,
	/// WM_APP = 0x8000u32
	App                  = 0x8000u32,
	/// WM_USER = 0x400u32
	User                 = 0x400u32,
}

impl WindowMessage {
	/// WM_SETTINGCHANGE = 0x1Au32
	pub const SettingChange       : Self = unsafe { transmute(0x1Au32) };
	/// WM_KEYDOWN = 0x100u32
	pub const KeyDown             : Self = unsafe { transmute(0x100u32) };
	/// WM_KEYLAST = 0x109u32
	pub const KeyLast             : Self = unsafe { transmute(0x109u32) };
	/// WM_IME_KEYLAST = 0x10Fu32
	pub const ImeKeyLast          : Self = unsafe { transmute(0x10Fu32) };
	/// WM_MOUSEMOVE = 0x200u32
	pub const MouseMove           : Self = unsafe { transmute(0x200u32) };
	/// WM_MOUSELAST = 0x20Eu32
	pub const MouseLast           : Self = unsafe { transmute(0x20Eu32) };
}

/// 
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SizeFlag
{
	/// SIZE_RESTORED = 0x0u32
	Restored             = 0x0u32,
	/// SIZE_MINIMIZED = 0x1u32
	Minimized            = 0x1u32,
	/// SIZE_MAXIMIZED = 0x2u32
	Maximized            = 0x2u32,
	/// SIZE_MAXSHOW = 0x3u32
	MaxShow              = 0x3u32,
	/// SIZE_MAXHIDE = 0x4u32
	MaxHide              = 0x4u32,
}

/// 
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SizingFlag
{
	/// WMSZ_LEFT = 0x1u32
	Left                 = 0x1u32,
	/// WMSZ_RIGHT = 0x2u32
	Right                = 0x2u32,
	/// WMSZ_TOP = 0x3u32
	Top                  = 0x3u32,
	/// WMSZ_TOPLEFT = 0x4u32
	TopLeft              = 0x4u32,
	/// WMSZ_TOPRIGHT = 0x5u32
	TopRight             = 0x5u32,
	/// WMSZ_BOTTOM = 0x6u32
	Bottom               = 0x6u32,
	/// WMSZ_BOTTOMLEFT = 0x7u32
	BottomLeft           = 0x7u32,
	/// WMSZ_BOTTOMRIGHT = 0x8u32
	BottomRight          = 0x8u32,
}

/// 
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SysCommandFlag
{
	///  = 0xF140u32
	ScreenSave           = 0xF140u32,
	/// SC_SIZE = 0xF000u32
	Size                 = 0xF000u32,
	/// SC_MOVE = 0xF010u32
	Move                 = 0xF010u32,
	/// SC_MINIMIZE = 0xF020u32
	Minimize             = 0xF020u32,
	/// SC_MAXIMIZE = 0xF030u32
	Maximize             = 0xF030u32,
	/// SC_NEXTWINDOW = 0xF040u32
	NextWindow           = 0xF040u32,
	/// SC_PREVWINDOW = 0xF050u32
	PrevWindow           = 0xF050u32,
	/// SC_CLOSE = 0xF060u32
	Close                = 0xF060u32,
	/// SC_VSCROLL = 0xF070u32
	VScroll              = 0xF070u32,
	/// SC_HSCROLL = 0xF080u32
	HScroll              = 0xF080u32,
	/// SC_MOUSEMENU = 0xF090u32
	MouseMenu            = 0xF090u32,
	/// SC_KEYMENU = 0xF100u32
	KeyMenu              = 0xF100u32,
	/// SC_ARRANGE = 0xF110u32
	Arrange              = 0xF110u32,
	/// SC_RESTORE = 0xF120u32
	Restore              = 0xF120u32,
	/// SC_TASKLIST = 0xF130u32
	TaskList             = 0xF130u32,
	/// SC_HOTKEY = 0xF150u32
	Hotkey               = 0xF150u32,
	/// SC_DEFAULT = 0xF160u32
	Default              = 0xF160u32,
	/// SC_MONITORPOWER = 0xF170u32
	MonitorPower         = 0xF170u32,
	/// SC_CONTEXTHELP = 0xF180u32
	ContextHelp          = 0xF180u32,
	/// SC_SEPARATOR = 0xF00Fu32
	Separator            = 0xF00Fu32,
	/// SCF_ISSECURE = 0x1u32
	IsSecure             = 0x1u32,
}

impl SysCommandFlag {
	/// SC_ICON = 0xF020u32
	pub const Icon                : Self = unsafe { transmute(0xF020u32) };
	/// SC_ZOOM = 0xF030u32
	pub const Zoom                : Self = unsafe { transmute(0xF030u32) };
}

bitflags::bitflags! {
	/// 
	#[repr(transparent)]
	pub struct WindowValidRectFlags: u32 {
		///  = 0x0u32
		const Default              = 0x0u32;
		/// WVR_ALIGNTOP = 0x10u32
		const AlignTop             = 0x10u32;
		/// WVR_ALIGNLEFT = 0x20u32
		const AlignLeft            = 0x20u32;
		/// WVR_ALIGNBOTTOM = 0x40u32
		const AlignBottom          = 0x40u32;
		/// WVR_ALIGNRIGHT = 0x80u32
		const AlignRight           = 0x80u32;
		/// WVR_HREDRAW = 0x100u32
		const HRedraw              = 0x100u32;
		/// WVR_VREDRAW = 0x200u32
		const VRedraw              = 0x200u32;
		/// WVR_VALIDRECTS = 0x400u32
		const ValidRects           = 0x400u32;
	}
}

/// 
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HitTestFlag
{
	/// HTERROR = -0x2i32
	Error                = -0x2i32,
	/// HTTRANSPARENT = -0x1i32
	Transparent          = -0x1i32,
	/// HTNOWHERE = 0x0i32
	Nowhere              = 0x0i32,
	/// HTCLIENT = 0x1i32
	Client               = 0x1i32,
	/// HTCAPTION = 0x2i32
	Caption              = 0x2i32,
	/// HTSYSMENU = 0x3i32
	SysMenu              = 0x3i32,
	/// HTGROWBOX = 0x4i32
	GrowBox              = 0x4i32,
	/// HTMENU = 0x5i32
	Menu                 = 0x5i32,
	/// HTHSCROLL = 0x6i32
	HScroll              = 0x6i32,
	/// HTVSCROLL = 0x7i32
	VScroll              = 0x7i32,
	/// HTMINBUTTON = 0x8i32
	MinButton            = 0x8i32,
	/// HTMAXBUTTON = 0x9i32
	MaxButton            = 0x9i32,
	/// HTLEFT = 0xAi32
	Left                 = 0xAi32,
	/// HTRIGHT = 0xBi32
	Right                = 0xBi32,
	/// HTTOP = 0xCi32
	Top                  = 0xCi32,
	/// HTTOPLEFT = 0xDi32
	TopLeft              = 0xDi32,
	/// HTTOPRIGHT = 0xEi32
	TopRight             = 0xEi32,
	/// HTBOTTOM = 0xFi32
	Bottom               = 0xFi32,
	/// HTBOTTOMLEFT = 0x10i32
	BottomLeft           = 0x10i32,
	/// HTBOTTOMRIGHT = 0x11i32
	BottomRight          = 0x11i32,
	/// HTBORDER = 0x12i32
	Border               = 0x12i32,
	/// HTOBJECT = 0x13i32
	Object               = 0x13i32,
	/// HTCLOSE = 0x14i32
	Close                = 0x14i32,
	/// HTHELP = 0x15i32
	Help                 = 0x15i32,
}

impl HitTestFlag {
	/// HTSIZE = 0x4i32
	pub const Size                : Self = unsafe { transmute(0x4i32) };
	/// HTREDUCE = 0x8i32
	pub const Reduce              : Self = unsafe { transmute(0x8i32) };
	/// HTZOOM = 0x9i32
	pub const Zoom                : Self = unsafe { transmute(0x9i32) };
	/// HTSIZEFIRST = 0xAi32
	pub const SizeFirst           : Self = unsafe { transmute(0xAi32) };
	/// HTSIZELAST = 0x11i32
	pub const SizeLast            : Self = unsafe { transmute(0x11i32) };
}

/// 
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindowActivateFlag
{
	/// WA_INACTIVE = 0x0u32
	Inactive             = 0x0u32,
	/// WA_ACTIVE = 0x1u32
	Active               = 0x1u32,
	/// WA_CLICKACTIVE = 0x2u32
	ClickActive          = 0x2u32,
}

/// 
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MouseActivateFlag
{
	/// MA_ACTIVATE = 0x1u32
	Activate             = 0x1u32,
	/// MA_ACTIVATEANDEAT = 0x2u32
	ActivateAndEat       = 0x2u32,
	/// MA_NOACTIVATE = 0x3u32
	NoActivate           = 0x3u32,
	/// MA_NOACTIVATEANDEAT = 0x4u32
	NoActivateAndEat     = 0x4u32,
}

/// 
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceType
{
	/// RT_CURSOR = 0x1i32
	Cursor               = 0x1i32,
	/// RT_BITMAP = 0x2i32
	Bitmap               = 0x2i32,
	/// RT_ICON = 0x3i32
	Icon                 = 0x3i32,
	/// RT_MENU = 0x4i32
	Menu                 = 0x4i32,
	/// RT_DIALOG = 0x5i32
	Dialog               = 0x5i32,
	/// RT_FONTDIR = 0x7i32
	FontDir              = 0x7i32,
	/// RT_FONT = 0x8i32
	Font                 = 0x8i32,
	/// RT_ACCELERATOR = 0x9i32
	Accelerator          = 0x9i32,
	/// RT_MESSAGETABLE = 0xBi32
	MessAGetable         = 0xBi32,
	/// RT_VERSION = 0x10i32
	Version              = 0x10i32,
	/// RT_DLGINCLUDE = 0x11i32
	DlgInclude           = 0x11i32,
	/// RT_PLUGPLAY = 0x13i32
	PlugPlay             = 0x13i32,
	/// RT_VXD = 0x14i32
	Vxd                  = 0x14i32,
	/// RT_ANICURSOR = 0x15i32
	AniCursor            = 0x15i32,
	/// RT_ANIICON = 0x16i32
	AniIcon              = 0x16i32,
	/// RT_HTML = 0x17i32
	Html                 = 0x17i32,
	/// RT_MANIFEST = 0x18i32
	Manifest             = 0x18i32,
}

/// 
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ScrollbarFlag
{
	/// SB_LINEUP = 0x0u32
	Lineup               = 0x0u32,
	/// SB_LINEDOWN = 0x1u32
	LineDown             = 0x1u32,
	/// SB_PAGEUP = 0x2u32
	PageUp               = 0x2u32,
	/// SB_PAGEDOWN = 0x3u32
	PageDown             = 0x3u32,
	/// SB_THUMBPOSITION = 0x4u32
	ThumbPosition        = 0x4u32,
	/// SB_THUMBTRACK = 0x5u32
	ThumbTrack           = 0x5u32,
	/// SB_TOP = 0x6u32
	Top                  = 0x6u32,
	/// SB_BOTTOM = 0x7u32
	Bottom               = 0x7u32,
	/// SB_ENDSCROLL = 0x8u32
	EndScroll            = 0x8u32,
}

impl ScrollbarFlag {
	/// SB_LINELEFT = 0x0u32
	pub const LineLeft            : Self = unsafe { transmute(0x0u32) };
	/// SB_LINERIGHT = 0x1u32
	pub const LineRight           : Self = unsafe { transmute(0x1u32) };
	/// SB_PAGELEFT = 0x2u32
	pub const PageLeft            : Self = unsafe { transmute(0x2u32) };
	/// SB_PAGERIGHT = 0x3u32
	pub const PageRight           : Self = unsafe { transmute(0x3u32) };
	/// SB_LEFT = 0x6u32
	pub const Left                : Self = unsafe { transmute(0x6u32) };
	/// SB_RIGHT = 0x7u32
	pub const Right               : Self = unsafe { transmute(0x7u32) };
}

/// 
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComboBoxMessage
{
	/// CB_OKAY = 0x0i32
	Okay                 = 0x0i32,
	/// CB_ERR = -0x1i32
	Err                  = -0x1i32,
	/// CB_ERRSPACE = -0x2i32
	ErrSpace             = -0x2i32,
	/// CB_GETEDITSEL = 0x140i32
	GetEditSel           = 0x140i32,
	/// CB_LIMITTEXT = 0x141i32
	LimitText            = 0x141i32,
	/// CB_SETEDITSEL = 0x142i32
	SetEditSel           = 0x142i32,
	/// CB_ADDSTRING = 0x143i32
	AddString            = 0x143i32,
	/// CB_DELETESTRING = 0x144i32
	DeleteString         = 0x144i32,
	/// CB_DIR = 0x145i32
	Dir                  = 0x145i32,
	/// CB_GETCOUNT = 0x146i32
	GetCount             = 0x146i32,
	/// CB_GETCURSEL = 0x147i32
	GetCurSel            = 0x147i32,
	/// CB_GETLBTEXT = 0x148i32
	GetLBText            = 0x148i32,
	/// CB_GETLBTEXTLEN = 0x149i32
	GetLBTextLen         = 0x149i32,
	/// CB_INSERTSTRING = 0x14Ai32
	InsertString         = 0x14Ai32,
	/// CB_RESETCONTENT = 0x14Bi32
	ResetContent         = 0x14Bi32,
	/// CB_FINDSTRING = 0x14Ci32
	FindString           = 0x14Ci32,
	/// CB_SELECTSTRING = 0x14Di32
	SelectString         = 0x14Di32,
	/// CB_SETCURSEL = 0x14Ei32
	SetCurSel            = 0x14Ei32,
	/// CB_SHOWDROPDOWN = 0x14Fi32
	ShowDropDown         = 0x14Fi32,
	/// CB_GETITEMDATA = 0x150i32
	GetItemData          = 0x150i32,
	/// CB_SETITEMDATA = 0x151i32
	SetItemData          = 0x151i32,
	/// CB_GETDROPPEDCONTROLRECT = 0x152i32
	GetDroppedControlRect = 0x152i32,
	/// CB_SETITEMHEIGHT = 0x153i32
	SetItemHeight        = 0x153i32,
	/// CB_GETITEMHEIGHT = 0x154i32
	GetItemHeight        = 0x154i32,
	/// CB_SETEXTENDEDUI = 0x155i32
	SetExtendedUi        = 0x155i32,
	/// CB_GETEXTENDEDUI = 0x156i32
	GetExtendedUi        = 0x156i32,
	/// CB_GETDROPPEDSTATE = 0x157i32
	GetDroppedState      = 0x157i32,
	/// CB_FINDSTRINGEXACT = 0x158i32
	FindStringExact      = 0x158i32,
	/// CB_SETLOCALE = 0x159i32
	SetLocale            = 0x159i32,
	/// CB_GETLOCALE = 0x15Ai32
	GetLocale            = 0x15Ai32,
	/// CB_GETTOPINDEX = 0x15Bi32
	GetTopIndex          = 0x15Bi32,
	/// CB_SETTOPINDEX = 0x15Ci32
	SetTopIndex          = 0x15Ci32,
	/// CB_GETHORIZONTALEXTENT = 0x15Di32
	GetHorizontalExtent  = 0x15Di32,
	/// CB_SETHORIZONTALEXTENT = 0x15Ei32
	SetHorizontalExtent  = 0x15Ei32,
	/// CB_GETDROPPEDWIDTH = 0x15Fi32
	GetDroppedWidth      = 0x15Fi32,
	/// CB_SETDROPPEDWIDTH = 0x160i32
	SetDroppedWidth      = 0x160i32,
	/// CB_INITSTORAGE = 0x161i32
	InitStorage          = 0x161i32,
	/// CB_MULTIPLEADDSTRING = 0x163i32
	MultipleAddString    = 0x163i32,
	/// CB_GETCOMBOBOXINFO = 0x164i32
	GetComboBoxInfo      = 0x164i32,
	/// CB_MSGMAX = 0x165i32
	MsgMax               = 0x165i32,
}

bitflags::bitflags! {
	/// DI_FLAGS
	#[repr(transparent)]
	pub struct DiFlags: u32 {
		/// DI_MASK = 0x1u32
		const Mask                 = 0x1u32;
		/// DI_IMAGE = 0x2u32
		const Image                = 0x2u32;
		/// DI_NORMAL = 0x3u32
		const Normal               = 0x3u32;
		/// DI_COMPAT = 0x4u32
		const ComPat               = 0x4u32;
		/// DI_DEFAULTSIZE = 0x8u32
		const DefaultSize          = 0x8u32;
		/// DI_NOMIRROR = 0x10u32
		const NoMirror             = 0x10u32;
	}
}

/// POINTER_INPUT_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerInputType
{
	/// PT_POINTER = 0x1i32
	Pointer              = 0x1i32,
	/// PT_TOUCH = 0x2i32
	Touch                = 0x2i32,
	/// PT_PEN = 0x3i32
	Pen                  = 0x3i32,
	/// PT_MOUSE = 0x4i32
	Mouse                = 0x4i32,
	/// PT_TOUCHPAD = 0x5i32
	TouchPad             = 0x5i32,
}

/// EDIT_CONTROL_FEATURE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EditControlFeature
{
	/// EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT = 0x0i32
	EnterpriseDataProtectionPasteSupport = 0x0i32,
	/// EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS = 0x1i32
	PasteNotifications   = 0x1i32,
}

/// HANDEDNESS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Handedness
{
	/// HANDEDNESS_LEFT = 0x0i32
	Left                 = 0x0i32,
	/// HANDEDNESS_RIGHT = 0x1i32
	Right                = 0x1i32,
}

