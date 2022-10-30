//! This module implements each requests of the X protocol.

pub mod query_extension;

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::XRequest;
use crate::util;
use std::error::Error;
use std::mem::size_of;

/// Request opcode: CreateWindow
pub const CREATE_WINDOW: u8 = 1;
/// Request opcode: ChangeWindowAttributes
pub const CHANGE_WINDOW_ATTRIBUTES: u8 = 2;
/// Request opcode: GetWindowAttributes
pub const GET_WINDOW_ATTRIBUTES: u8 = 3;
/// Request opcode: DestroyWindow
pub const DESTROY_WINDOW: u8 = 4;
/// Request opcode: DestroySubwindows
pub const DESTROY_SUBWINDOWS: u8 = 5;
/// Request opcode: ChangeSaveSet
pub const CHANGE_SAVE_SET: u8 = 6;
/// Request opcode: ReparentWindow
pub const REPARENT_WINDOW: u8 = 7;
/// Request opcode: MapWindow
pub const MAP_WINDOW: u8 = 8;
/// Request opcode: MapSubwindows
pub const MAP_SUBWINDOWS: u8 = 9;
/// Request opcode: UnmapWindow
pub const UNMAP_WINDOW: u8 = 10;
/// Request opcode: UnmapSubwindows
pub const UNMAP_SUBWINDOWS: u8 = 11;
/// Request opcode: ConfigureWindow
pub const CONFIGURE_WINDOW: u8 = 12;
/// Request opcode: CirculateWindow
pub const CIRCULATE_WINDOW: u8 = 13;
/// Request opcode: GetGeometry
pub const GET_GEOMETRY: u8 = 14;
/// Request opcode: QueryTree
pub const QUERY_TREE: u8 = 15;
/// Request opcode: InternAtom
pub const INTERN_ATOM: u8 = 16;
/// Request opcode: GetAtomName
pub const GET_ATOM_NAME: u8 = 17;
/// Request opcode: ChangeProperty
pub const CHANGE_PROPERTY: u8 = 18;
/// Request opcode: DeleteProperty
pub const DELETE_PROPERTY: u8 = 19;
/// Request opcode: GetProperty
pub const GET_PROPERTY: u8 = 20;
/// Request opcode: ListProperties
pub const LIST_PROPERTIES: u8 = 21;
/// Request opcode: SetSelectionOwner
pub const SET_SELECTION_OWNER: u8 = 22;
/// Request opcode: GetSelectionOwner
pub const GET_SELECTION_OWNER: u8 = 23;
/// Request opcode: ConvertSelection
pub const CONVERT_SELECTION: u8 = 24;
/// Request opcode: SendEvent
pub const SEND_EVENT: u8 = 25;
/// Request opcode: GrabPointer
pub const GRAB_POINTER: u8 = 26;
/// Request opcode: UngrabPointer
pub const UNGRAB_POINTER: u8 = 27;
/// Request opcode: GrabButton
pub const GRAB_BUTTON: u8 = 28;
/// Request opcode: UngrabButton
pub const UNGRAB_BUTTON: u8 = 29;
/// Request opcode: ChangeActivePointerGrab
pub const CHANGE_ACTIVE_POINTER_GRAB: u8 = 30;
/// Request opcode: GrabKeyboard
pub const GRAB_KEYBOARD: u8 = 31;
/// Request opcode: UngrabKeyboard
pub const UNGRAB_KEYBOARD: u8 = 32;
/// Request opcode: GrabKey
pub const GRAB_KEY: u8 = 33;
/// Request opcode: UngrabKey
pub const UNGRAB_KEY: u8 = 34;
/// Request opcode: AllowEvents
pub const ALLOW_EVENTS: u8 = 35;
/// Request opcode: GrabServer
pub const GRAB_SERVER: u8 = 36;
/// Request opcode: UngrabServer
pub const UNGRAB_SERVER: u8 = 37;
/// Request opcode: QueryPointer
pub const QUERY_POINTER: u8 = 38;
/// Request opcode: GetMotionEvents
pub const GET_MOTION_EVENTS: u8 = 39;
/// Request opcode: TranslateCoordinates
pub const TRANSLATE_COORDINATES: u8 = 40;
/// Request opcode: WarpPointer
pub const WARP_POINTER: u8 = 41;
/// Request opcode: SetInputFocus
pub const SET_INPUT_FOCUS: u8 = 42;
/// Request opcode: GetInputFocus
pub const GET_INPUT_FOCUS: u8 = 43;
/// Request opcode: QueryKeymap
pub const QUERY_KEYMAP: u8 = 44;
/// Request opcode: OpenFont
pub const OPEN_FONT: u8 = 45;
/// Request opcode: CloseFont
pub const CLOSE_FONT: u8 = 46;
/// Request opcode: QueryFont
pub const QUERY_FONT: u8 = 47;
/// Request opcode: QueryTextExtents
pub const QUERY_TEXT_EXTENTS: u8 = 48;
/// Request opcode: ListFonts
pub const LIST_FONTS: u8 = 49;
/// Request opcode: ListFontsWithInfo
pub const LIST_FONTS_WITH_INFO: u8 = 50;
/// Request opcode: SetFontPath
pub const SET_FONT_PATH: u8 = 51;
/// Request opcode: GetFontPath
pub const GET_FONT_PATH: u8 = 52;
/// Request opcode: CreatePixmap
pub const CREATE_PIXMAP: u8 = 53;
/// Request opcode: FreePixmap
pub const FREE_PIXMAP: u8 = 54;
/// Request opcode: CreateGC
pub const CREATE_GC: u8 = 55;
/// Request opcode: ChangeGC
pub const CHANGE_GC: u8 = 56;
/// Request opcode: CopyGC
pub const COPY_GC: u8 = 57;
/// Request opcode: SetDashes
pub const SET_DASHES: u8 = 58;
/// Request opcode: SetClipRectangles
pub const SET_CLIP_RECTANGLES: u8 = 59;
/// Request opcode: FreeGC
pub const FREE_GC: u8 = 60;
/// Request opcode: ClearArea
pub const CLEAR_AREA: u8 = 61;
/// Request opcode: CopyArea
pub const COPY_AREA: u8 = 62;
/// Request opcode: CopyPlane
pub const COPY_PLANE: u8 = 63;
/// Request opcode: PolyPoint
pub const POLY_POINT: u8 = 64;
/// Request opcode: PolyLine
pub const POLY_LINE: u8 = 65;
/// Request opcode: PolySegment
pub const POLY_SEGMENT: u8 = 66;
/// Request opcode: PolyRectangle
pub const POLY_RECTANGLE: u8 = 67;
/// Request opcode: PolyArc
pub const POLY_ARC: u8 = 68;
/// Request opcode: FillPoly
pub const FILL_POLY: u8 = 69;
/// Request opcode: PolyFillRectangle
pub const POLY_FILL_RECTANGLE: u8 = 70;
/// Request opcode: PolyFillArc
pub const POLY_FILL_ARC: u8 = 71;
/// Request opcode: PutImage
pub const PUT_IMAGE: u8 = 72;
/// Request opcode: GetImage
pub const GET_IMAGE: u8 = 73;
/// Request opcode: PolyText8
pub const POLY_TEXT8: u8 = 74;
/// Request opcode: PolyText16
pub const POLY_TEXT16: u8 = 75;
/// Request opcode: ImageText8
pub const IMAGE_TEXT8: u8 = 76;
/// Request opcode: ImageText16
pub const IMAGE_TEXT16: u8 = 77;
/// Request opcode: CreateColormap
pub const CREATE_COLORMAP: u8 = 78;
/// Request opcode: FreeColormap
pub const FREE_COLORMAP: u8 = 79;
/// Request opcode: CopyColormapAndFree
pub const COPY_COLORMAP_AND_FREE: u8 = 80;
/// Request opcode: InstallColormap
pub const INSTALL_COLORMAP: u8 = 81;
/// Request opcode: UninstallColormap
pub const UNINSTALL_COLORMAP: u8 = 82;
/// Request opcode: ListInstalledColormaps
pub const LIST_INSTALLED_COLORMAPS: u8 = 83;
/// Request opcode: AllocColor
pub const ALLOC_COLOR: u8 = 84;
/// Request opcode: AllocNamedColor
pub const ALLOC_NAMED_COLOR: u8 = 85;
/// Request opcode: AllocColorCells
pub const ALLOC_COLOR_CELLS: u8 = 86;
/// Request opcode: AllocColorPlanes
pub const ALLOC_COLOR_PLANES: u8 = 87;
/// Request opcode: FreeColors
pub const FREE_COLORS: u8 = 88;
/// Request opcode: StoreColors
pub const STORE_COLORS: u8 = 89;
/// Request opcode: StoreNamedColor
pub const STORE_NAMED_COLOR: u8 = 90;
/// Request opcode: QueryColors
pub const QUERY_COLORS: u8 = 91;
/// Request opcode: LookupColor
pub const LOOKUP_COLOR: u8 = 92;
/// Request opcode: CreateCursor
pub const CREATE_CURSOR: u8 = 93;
/// Request opcode: CreateGlyphCursor
pub const CREATE_GLYPH_CURSOR: u8 = 94;
/// Request opcode: FreeCursor
pub const FREE_CURSOR: u8 = 95;
/// Request opcode: RecolorCursor
pub const RECOLOR_CURSOR: u8 = 96;
/// Request opcode: QueryBestSize
pub const QUERY_BEST_SIZE: u8 = 97;
/// Request opcode: QueryExtension
pub const QUERY_EXTENSION: u8 = 98;
/// Request opcode: ListExtensions
pub const LIST_EXTENSIONS: u8 = 99;
/// Request opcode: ChangeKeyboardMapping
pub const CHANGE_KEYBOARD_MAPPING: u8 = 100;
/// Request opcode: GetKeyboardMapping
pub const GET_KEYBOARD_MAPPING: u8 = 101;
/// Request opcode: ChangeKeyboardControl
pub const CHANGE_KEYBOARD_CONTROL: u8 = 102;
/// Request opcode: GetKeyboardControl
pub const GET_KEYBOARD_CONTROL: u8 = 103;
/// Request opcode: Bell
pub const BELL: u8 = 104;
/// Request opcode: ChangePointerControl
pub const CHANGE_POINTER_CONTROL: u8 = 105;
/// Request opcode: GetPointerControl
pub const GET_POINTER_CONTROL: u8 = 106;
/// Request opcode: SetScreenSaver
pub const SET_SCREEN_SAVER: u8 = 107;
/// Request opcode: GetScreenSaver
pub const GET_SCREEN_SAVER: u8 = 108;
/// Request opcode: ChangeHosts
pub const CHANGE_HOSTS: u8 = 109;
/// Request opcode: ListHosts
pub const LIST_HOSTS: u8 = 110;
/// Request opcode: SetAccessControl
pub const SET_ACCESS_CONTROL: u8 = 111;
/// Request opcode: SetCloseDownMode
pub const SET_CLOSE_DOWNMODE: u8 = 112;
/// Request opcode: KillClient
pub const KILL_CLIENT: u8 = 113;
/// Request opcode: RotateProperties
pub const ROTATE_PROPERTIES: u8 = 114;
/// Request opcode: ForceScreenSaver
pub const FORCE_SCREEN_SAVER: u8 = 115;
/// Request opcode: SetPointerMapping
pub const SET_POINTER_MAPPING: u8 = 116;
/// Request opcode: GetPointerMapping
pub const GET_POINTER_MAPPING: u8 = 117;
/// Request opcode: SetModifierMapping
pub const SET_MODIFIER_MAPPING: u8 = 118;
/// Request opcode: GetModifierMapping
pub const GET_MODIFIER_MAPPING: u8 = 119;
/// Request opcode: NoOperation
pub const NO_OPERATION: u8 = 127;

/// The maximum length of a request in bytes.
pub const MAX_REQUEST_LEN: usize = 4194304;

/// A request with the given opcode and buffer.
///
/// Arguments:
/// - `ctx` is the current context.
/// - `opcode` is the request's opcode.
/// - `buff` is the body of the request.
///
/// If the opcode is not assigned, the function returns None.
pub fn build_request(
	ctx: &Context,
	opcode: u8,
	buff: &[u8],
) -> Result<Option<Box<dyn Request>>, Box<dyn Error>> {
	// TODO rm
	println!("=> {}", opcode);

	match ctx.get_custom_requests().get(&opcode) {
		Some(f) => return f(buff),
		None => {},
	}

	let request = match opcode {
		// TODO

		QUERY_EXTENSION => query_extension::read(buff)?
			.map(|r| Box::new(r) as Box<dyn Request>),

		_ => None // TODO Error instead?
	};

	Ok(request)
}

/// A function to call to read a function of a specific type.
/// Each request type has its own function.
pub type RequestReadFn = dyn Fn(&[u8]) -> Result<Option<Box<dyn Request>>, Box<dyn Error>>;

/// Trait representing a request.
pub trait Request {
	/// Handles the request for the given client.
	///
	/// Arguments:
	/// - `ctx` is the current context.
	/// - `seq_nbr` is the sequence number for the request.
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), Box<dyn Error>>;
}

/// Trait representing an object used to read a request.
pub trait RequestReader {
	/// Reads a request from the given buffer.
	/// If not enough data is present in the buffer, the function returns None.
	///
	/// `ctx` is the current context.
	fn read(
		&self,
		ctx: &Context,
		buff: &[u8],
	) -> Result<Option<(Box<dyn Request>, usize)>, Box<dyn Error>>;
}

/// The default request reader.
pub struct DefaultRequestReader {}

impl RequestReader for DefaultRequestReader {
	fn read(
		&self,
		ctx: &Context,
		buff: &[u8],
	) -> Result<Option<(Box<dyn Request>, usize)>, Box<dyn Error>> {
		// If not enough bytes are available, return
		let hdr_len = size_of::<XRequest>();
		if buff.len() < hdr_len {
			return Ok(None);
		}

		let hdr: &XRequest = unsafe {
			util::reinterpret(&buff[0])
		};
		// Required number of bytes
		let req = hdr.length as usize * 4;

		// If the request is too long, ignore it
		if req > MAX_REQUEST_LEN {
			// TODO
			todo!();
		}
		// If not enough bytes are available, return
		if buff.len() < req {
			return Ok(None);
		}

		let opcode = hdr.major_opcode;
		let buff = &buff[hdr_len..];

		let request = build_request(ctx, opcode, buff)?;
		Ok(request.map(|r| (r, req)))
	}
}
