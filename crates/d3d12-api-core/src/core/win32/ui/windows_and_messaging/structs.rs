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
use crate::core::win32::ui::windows_and_messaging::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::gdi::*;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HHook(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HIcon(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HMenu(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HCursor(pub NonZeroUsize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HAccel(pub NonZeroUsize);

/// MESSAGE_RESOURCE_ENTRY
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MessageResourceEntry {
	pub length: u16,
	pub flags: u16,
	pub text: [u8; 1],
}

/// MESSAGE_RESOURCE_BLOCK
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MessageResourceBlock {
	pub low_id: u32,
	pub high_id: u32,
	pub offset_to_entries: u32,
}

/// MESSAGE_RESOURCE_DATA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MessageResourceData {
	pub number_of_blocks: u32,
	pub blocks: [MessageResourceBlock; 1],
}

/// CBT_CREATEWNDA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CbtCreateWndA<'a> {
	pub lpcs: &'a CreateStructA<'a>,
	pub hwnd_insert_after: HWnd,
}

/// CBT_CREATEWNDW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CbtCreateWndW<'a> {
	pub lpcs: &'a CreateStructW<'a>,
	pub hwnd_insert_after: HWnd,
}

/// CBTACTIVATESTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CbTActivateStruct {
	pub f_mouse: Bool,
	pub wnd_active: HWnd,
}

/// SHELLHOOKINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShellHookInfo {
	pub hwnd: HWnd,
	pub rc: Rect,
}

/// EVENTMSG
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EventMsg {
	pub message: u32,
	pub param_l: u32,
	pub param_h: u32,
	pub time: u32,
	pub hwnd: HWnd,
}

/// CWPSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CWPStruct {
	pub l_param: LParam,
	pub w_param: WParam,
	pub message: u32,
	pub hwnd: HWnd,
}

/// CWPRETSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CWPretStruct {
	pub result: LResult,
	pub l_param: LParam,
	pub w_param: WParam,
	pub message: u32,
	pub hwnd: HWnd,
}

/// KBDLLHOOKSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct KBDllHookStruct {
	pub vk_code: u32,
	pub scan_code: u32,
	pub flags: KBDllHookStructFlags,
	pub time: u32,
	pub extra_info: usize,
}

/// MSLLHOOKSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MslLHookStruct {
	pub pt: Point,
	pub mouse_data: MouseHookStrUCTexMouseData,
	pub flags: u32,
	pub time: u32,
	pub extra_info: usize,
}

/// DEBUGHOOKINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DebugHookInfo {
	pub id_thread: u32,
	pub id_thread_installer: u32,
	pub l_param: LParam,
	pub w_param: WParam,
	pub code: i32,
}

/// MOUSEHOOKSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MouseHookStruct {
	pub pt: Point,
	pub hwnd: HWnd,
	pub hit_test_code: u32,
	pub extra_info: usize,
}

/// MOUSEHOOKSTRUCTEX
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MouseHookStrUCTex {
	pub _anonymous_base_winuser_l1166_c46: MouseHookStruct,
	pub mouse_data: MouseHookStrUCTexMouseData,
}

/// HARDWAREHOOKSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HardWARehookStruct {
	pub hwnd: HWnd,
	pub message: u32,
	pub w_param: WParam,
	pub l_param: LParam,
}

/// WNDCLASSEXA
#[repr(C)]
pub struct WndClassExA<'a> {
	pub size: u32,
	pub style: WndClassStyles,
	pub wnd_proc: WndProc,
	pub cls_extra: i32,
	pub wnd_extra: i32,
	pub instance: HInstance,
	pub icon: Option<HIcon>,
	pub cursor: Option<HCursor>,
	pub hbr_background: Option<HBrush>,
	pub menu_name: Option<PStr<'a>>,
	pub class_name: PStr<'a>,
	pub icon_sm: Option<HIcon>,
}

/// WNDCLASSEXW
#[repr(C)]
pub struct WndClassExW<'a> {
	pub size: u32,
	pub style: WndClassStyles,
	pub wnd_proc: WndProc,
	pub cls_extra: i32,
	pub wnd_extra: i32,
	pub instance: HInstance,
	pub icon: HIcon,
	pub cursor: HCursor,
	pub hbr_background: HBrush,
	pub menu_name: PWStr<'a>,
	pub class_name: PWStr<'a>,
	pub icon_sm: HIcon,
}

/// WNDCLASSA
#[repr(C)]
pub struct WndClassA<'a> {
	pub style: WndClassStyles,
	pub wnd_proc: WndProc,
	pub cls_extra: i32,
	pub wnd_extra: i32,
	pub instance: HInstance,
	pub icon: HIcon,
	pub cursor: HCursor,
	pub hbr_background: HBrush,
	pub menu_name: PStr<'a>,
	pub class_name: PStr<'a>,
}

/// WNDCLASSW
#[repr(C)]
pub struct WndClassW<'a> {
	pub style: WndClassStyles,
	pub wnd_proc: WndProc,
	pub cls_extra: i32,
	pub wnd_extra: i32,
	pub instance: HInstance,
	pub icon: HIcon,
	pub cursor: HCursor,
	pub hbr_background: HBrush,
	pub menu_name: PWStr<'a>,
	pub class_name: PWStr<'a>,
}

/// MSG
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Msg {
	pub hwnd: Option<HWnd>,
	pub message: WindowMessage,
	pub w_param: WParam,
	pub l_param: LParam,
	pub time: u32,
	pub pt: Point,
}

/// MINMAXINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MinMaxInfo {
	pub pt_reserved: Point,
	pub pt_max_size: Point,
	pub pt_max_position: Point,
	pub pt_min_track_size: Point,
	pub pt_max_track_size: Point,
}

/// MDINEXTMENU
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MdiNextMenu {
	pub hmenu_in: HMenu,
	pub hmenu_next: HMenu,
	pub hwnd_next: HWnd,
}

/// WINDOWPOS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WindowPos {
	pub hwnd: HWnd,
	pub hwnd_insert_after: HWnd,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub flags: SetWindowPosFlags,
}

/// NCCALCSIZE_PARAMS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NcCalcSizeParams<'a> {
	pub rgrc: [Rect; 3],
	pub lppos: &'a WindowPos,
}

/// ACCEL
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Accel {
	pub f_virt: u8,
	pub key: u16,
	pub cmd: u16,
}

/// CREATESTRUCTA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CreateStructA<'a> {
	pub create_params: &'a c_void,
	pub instance: HInstance,
	pub menu: HMenu,
	pub hwnd_parent: HWnd,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: i32,
	pub name: PStr<'a>,
	pub class: PStr<'a>,
	pub ex_style: u32,
}

/// CREATESTRUCTW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CreateStructW<'a> {
	pub create_params: &'a c_void,
	pub instance: HInstance,
	pub menu: HMenu,
	pub hwnd_parent: HWnd,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: i32,
	pub name: PWStr<'a>,
	pub class: PWStr<'a>,
	pub ex_style: u32,
}

/// WINDOWPLACEMENT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WindowPlacement {
	pub length: u32,
	pub flags: WindowPlacementFlags,
	pub show_cmd: ShowWindowCmd,
	pub pt_min_position: Point,
	pub pt_max_position: Point,
	pub rc_normal_position: Rect,
}

/// STYLESTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StyleStruct {
	pub style_old: u32,
	pub style_new: u32,
}

/// UPDATELAYEREDWINDOWINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UpdateLayeredWindowInfo<'a> {
	pub size: u32,
	pub hdc_dst: HDc,
	pub ppt_dst: &'a Point,
	pub psize: &'a Size,
	pub hdc_src: HDc,
	pub ppt_src: &'a Point,
	pub cr_key: u32,
	pub pblend: &'a BlendFunction,
	pub flags: UpdateLayeredWindowFlags,
	pub prc_dirty: &'a Rect,
}

/// FLASHWINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FlashWInfo {
	pub size: u32,
	pub hwnd: HWnd,
	pub flags: FlashWInfoFlags,
	pub u_count: u32,
	pub timeout: u32,
}

/// DLGTEMPLATE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DlgTemplate {
	pub style: u32,
	pub extended_style: u32,
	pub cdit: u16,
	pub x: i16,
	pub y: i16,
	pub cx: i16,
	pub cy: i16,
}

/// DLGITEMTEMPLATE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DlgItemTemplate {
	pub style: u32,
	pub extended_style: u32,
	pub x: i16,
	pub y: i16,
	pub cx: i16,
	pub cy: i16,
	pub id: u16,
}

/// TPMPARAMS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TpmParams {
	pub size: u32,
	pub rc_exclude: Rect,
}

/// MENUINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MenuInfo {
	pub size: u32,
	pub f_mask: MenuInfoMask,
	pub style: MenuInfoStyle,
	pub cy_max: u32,
	pub hbr_back: HBrush,
	pub context_help_id: u32,
	pub menu_data: usize,
}

/// MENUGETOBJECTINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MenuGetObjectInfo<'a> {
	pub flags: MenuGetObjectInfoFlags,
	pub u_pos: u32,
	pub hmenu: HMenu,
	pub riid: &'a c_void,
	pub pv_obj: &'a c_void,
}

/// MENUITEMINFOA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MenuItemInfoA<'a> {
	pub size: u32,
	pub f_mask: MenuItemMask,
	pub f_type: MenuItemType,
	pub f_state: MenuItemState,
	pub id: u32,
	pub sub_menu: HMenu,
	pub hbmp_checked: HBitmap,
	pub hbmp_unchecked: HBitmap,
	pub item_data: usize,
	pub type_data: PStr<'a>,
	pub cch: u32,
	pub hbmp_item: HBitmap,
}

/// MENUITEMINFOW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MenuItemInfoW<'a> {
	pub size: u32,
	pub f_mask: MenuItemMask,
	pub f_type: MenuItemType,
	pub f_state: MenuItemState,
	pub id: u32,
	pub sub_menu: HMenu,
	pub hbmp_checked: HBitmap,
	pub hbmp_unchecked: HBitmap,
	pub item_data: usize,
	pub type_data: PWStr<'a>,
	pub cch: u32,
	pub hbmp_item: HBitmap,
}

/// DROPSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DropStruct {
	pub hwnd_source: HWnd,
	pub hwnd_sink: HWnd,
	pub fmt: u32,
	pub data: usize,
	pub pt_drop: Point,
	pub control_data: u32,
}

/// MSGBOXPARAMSA
#[repr(C)]
pub struct MsgBoxParamsA<'a> {
	pub size: u32,
	pub hwnd_owner: HWnd,
	pub instance: HInstance,
	pub text: PStr<'a>,
	pub caption: PStr<'a>,
	pub style: MessageBoxStyle,
	pub icon: PStr<'a>,
	pub context_help_id: usize,
	pub msg_box_callback: MsgBoxCallback,
	pub language_id: u32,
}

/// MSGBOXPARAMSW
#[repr(C)]
pub struct MsgBoxParamsW<'a> {
	pub size: u32,
	pub hwnd_owner: HWnd,
	pub instance: HInstance,
	pub text: PWStr<'a>,
	pub caption: PWStr<'a>,
	pub style: MessageBoxStyle,
	pub icon: PWStr<'a>,
	pub context_help_id: usize,
	pub msg_box_callback: MsgBoxCallback,
	pub language_id: u32,
}

/// MENUITEMTEMPLATEHEADER
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MenuItemTemplateHeader {
	pub version_number: u16,
	pub offset: u16,
}

/// MENUITEMTEMPLATE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MenuItemTemplate {
	pub mt_option: u16,
	pub mt_id: u16,
	pub mt_string: [u16; 1],
}

/// ICONINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IconInfo {
	pub f_icon: Bool,
	pub x_hotspot: u32,
	pub y_hotspot: u32,
	pub hbm_mask: HBitmap,
	pub hbm_color: HBitmap,
}

/// CURSORSHAPE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CursorShape {
	pub x_hot_spot: i32,
	pub y_hot_spot: i32,
	pub cx: i32,
	pub cy: i32,
	pub width: i32,
	pub planes: u8,
	pub bits_pixel: u8,
}

/// ICONINFOEXA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IconInfoExA {
	pub size: u32,
	pub f_icon: Bool,
	pub x_hotspot: u32,
	pub y_hotspot: u32,
	pub hbm_mask: HBitmap,
	pub hbm_color: HBitmap,
	pub res_id: u16,
	pub sz_mod_name: [Char; 260],
	pub sz_res_name: [Char; 260],
}

/// ICONINFOEXW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IconInfoExW {
	pub size: u32,
	pub f_icon: Bool,
	pub x_hotspot: u32,
	pub y_hotspot: u32,
	pub hbm_mask: HBitmap,
	pub hbm_color: HBitmap,
	pub res_id: u16,
	pub sz_mod_name: [u16; 260],
	pub sz_res_name: [u16; 260],
}

/// SCROLLINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ScrollInfo {
	pub size: u32,
	pub f_mask: ScrollInfoMask,
	pub min: i32,
	pub max: i32,
	pub page: u32,
	pub pos: i32,
	pub track_pos: i32,
}

/// MDICREATESTRUCTA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MdiCreateStructA<'a> {
	pub sz_class: PStr<'a>,
	pub sz_title: PStr<'a>,
	pub owner: Handle,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub style: WindowStyle,
	pub l_param: LParam,
}

/// MDICREATESTRUCTW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MdiCreateStructW<'a> {
	pub sz_class: PWStr<'a>,
	pub sz_title: PWStr<'a>,
	pub owner: Handle,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub style: WindowStyle,
	pub l_param: LParam,
}

/// CLIENTCREATESTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClientCreateStruct {
	pub window_menu: Handle,
	pub id_first_child: u32,
}

/// TouchPredictionParameters
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TouchPredictionParameters {
	pub size: u32,
	pub latency: u32,
	pub sample_time: u32,
	pub use_hw_time_stamp: u32,
}

/// NONCLIENTMETRICSA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NonClientMetricsA {
	pub size: u32,
	pub i_border_width: i32,
	pub i_scroll_width: i32,
	pub i_scroll_height: i32,
	pub i_caption_width: i32,
	pub i_caption_height: i32,
	pub lf_caption_font: LogFontA,
	pub i_sm_caption_width: i32,
	pub i_sm_caption_height: i32,
	pub lf_sm_caption_font: LogFontA,
	pub i_menu_width: i32,
	pub i_menu_height: i32,
	pub lf_menu_font: LogFontA,
	pub lf_status_font: LogFontA,
	pub lf_message_font: LogFontA,
	pub i_padded_border_width: i32,
}

/// NONCLIENTMETRICSW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NonClientMetricsW {
	pub size: u32,
	pub i_border_width: i32,
	pub i_scroll_width: i32,
	pub i_scroll_height: i32,
	pub i_caption_width: i32,
	pub i_caption_height: i32,
	pub lf_caption_font: LogFontW,
	pub i_sm_caption_width: i32,
	pub i_sm_caption_height: i32,
	pub lf_sm_caption_font: LogFontW,
	pub i_menu_width: i32,
	pub i_menu_height: i32,
	pub lf_menu_font: LogFontW,
	pub lf_status_font: LogFontW,
	pub lf_message_font: LogFontW,
	pub i_padded_border_width: i32,
}

/// MINIMIZEDMETRICS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MinimizedMetrics {
	pub size: u32,
	pub i_width: i32,
	pub i_horz_gap: i32,
	pub i_vert_gap: i32,
	pub i_arrange: MinimizedMetricsArrange,
}

/// ICONMETRICSA
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IconMetricsA {
	pub size: u32,
	pub horz_spacing: i32,
	pub vert_spacing: i32,
	pub title_wrap: i32,
	pub lf_font: LogFontA,
}

/// ICONMETRICSW
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IconMetricsW {
	pub size: u32,
	pub horz_spacing: i32,
	pub vert_spacing: i32,
	pub title_wrap: i32,
	pub lf_font: LogFontW,
}

/// ANIMATIONINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AnimationInfo {
	pub size: u32,
	pub i_min_animate: i32,
}

/// AUDIODESCRIPTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AudioDescription {
	pub size: u32,
	pub enabled: Bool,
	pub locale: u32,
}

/// GUITHREADINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GuiThreadInfo {
	pub size: u32,
	pub flags: GuiThreadInfoFlags,
	pub hwnd_active: HWnd,
	pub hwnd_focus: HWnd,
	pub hwnd_capture: HWnd,
	pub hwnd_menu_owner: HWnd,
	pub hwnd_move_size: HWnd,
	pub hwnd_caret: HWnd,
	pub rc_caret: Rect,
}

/// CURSORINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CursorInfo {
	pub size: u32,
	pub flags: CursorInfoFlags,
	pub cursor: HCursor,
	pub pt_screen_pos: Point,
}

/// WINDOWINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WindowInfo {
	pub size: u32,
	pub rc_window: Rect,
	pub rc_client: Rect,
	pub style: u32,
	pub ex_style: u32,
	pub window_status: u32,
	pub cx_window_borders: u32,
	pub cy_window_borders: u32,
	pub atom_window_type: u16,
	pub creator_version: u16,
}

/// TITLEBARINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TitleBarInfo {
	pub size: u32,
	pub rc_title_bar: Rect,
	pub rgstate: [u32; 6],
}

/// TITLEBARINFOEX
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TitleBarInfoEx {
	pub size: u32,
	pub rc_title_bar: Rect,
	pub rgstate: [u32; 6],
	pub rgrect: [Rect; 6],
}

/// MENUBARINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MenuBarInfo {
	pub size: u32,
	pub rc_bar: Rect,
	pub menu: HMenu,
	pub hwnd_menu: HWnd,
	pub bitfield: i32,
}

/// SCROLLBARINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ScrollbarInfo {
	pub size: u32,
	pub rc_scroll_bar: Rect,
	pub dxy_line_button: i32,
	pub xy_thumb_top: i32,
	pub xy_thumb_bottom: i32,
	pub reserved: i32,
	pub rgstate: [u32; 6],
}

/// ALTTABINFO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AltTabInfo {
	pub size: u32,
	pub items: i32,
	pub columns: i32,
	pub rows: i32,
	pub i_col_focus: i32,
	pub i_row_focus: i32,
	pub cx_item: i32,
	pub cy_item: i32,
	pub pt_start: Point,
}

/// CHANGEFILTERSTRUCT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ChangeFilterStruct {
	pub size: u32,
	pub ext_status: MsgFltInfoStatus,
}

/// IndexedResourceQualifier
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IndexedResourceQualifier<'a> {
	pub name: PWStr<'a>,
	pub value: PWStr<'a>,
}

