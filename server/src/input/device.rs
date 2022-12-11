//! TODO doc

use crate::util;
use std::ffi::c_int;
use std::ffi::c_short;
use std::fs::File;
use std::io::Read;
use std::io;
use std::mem::size_of;
use std::os::unix::prelude::AsRawFd;
use std::path::PathBuf;
use super::Input;

/*
 * Device properties and quirks
 */

/// needs a pointer
pub const INPUT_PROP_POINTER: u16 = 0x00;
/// direct input devices
pub const INPUT_PROP_DIRECT: u16 = 0x01;
/// has button(s) under pad
pub const INPUT_PROP_BUTTONPAD: u16 = 0x02;
/// touch rectangle only
pub const INPUT_PROP_SEMI_MT: u16 = 0x03;
/// softbuttons at top of pad
pub const INPUT_PROP_TOPBUTTONPAD: u16 = 0x04;
/// is a pointing stick
pub const INPUT_PROP_POINTING_STICK: u16 = 0x05;
/// has accelerometer
pub const INPUT_PROP_ACCELEROMETER: u16 = 0x06;

pub const INPUT_PROP_MAX: u16 = 0x1f;
pub const INPUT_PROP_CNT: u16 = INPUT_PROP_MAX + 1;

/*
 * Event types
 */

pub const EV_SYN: c_short = 0x00;
pub const EV_KEY: c_short = 0x01;
pub const EV_REL: c_short = 0x02;
pub const EV_ABS: c_short = 0x03;
pub const EV_MSC: c_short = 0x04;
pub const EV_SW: c_short = 0x05;
pub const EV_LED: c_short = 0x11;
pub const EV_SND: c_short = 0x12;
pub const EV_REP: c_short = 0x14;
pub const EV_FF: c_short = 0x15;
pub const EV_PWR: c_short = 0x16;
pub const EV_FF_STATUS: c_short = 0x17;
pub const EV_MAX: c_short = 0x1f;
pub const EV_CNT: c_short = EV_MAX + 1;

/*
 * Synchronization events.
 */

pub const SYN_REPORT: u16 = 0;
pub const SYN_CONFIG: u16 = 1;
pub const SYN_MT_REPORT: u16 = 2;
pub const SYN_DROPPED: u16 = 3;
pub const SYN_MAX: u16 = 0xf;
pub const SYN_CNT: u16 = SYN_MAX + 1;

/*
 * Keys and buttons
 */

pub const KEY_RESERVED: u16 = 0;
pub const KEY_ESC: u16 = 1;
pub const KEY_1: u16 = 2;
pub const KEY_2: u16 = 3;
pub const KEY_3: u16 = 4;
pub const KEY_4: u16 = 5;
pub const KEY_5: u16 = 6;
pub const KEY_6: u16 = 7;
pub const KEY_7: u16 = 8;
pub const KEY_8: u16 = 9;
pub const KEY_9: u16 = 10;
pub const KEY_0: u16 = 11;
pub const KEY_MINUS: u16 = 12;
pub const KEY_EQUAL: u16 = 13;
pub const KEY_BACKSPACE: u16 = 14;
pub const KEY_TAB: u16 = 15;
pub const KEY_Q: u16 = 16;
pub const KEY_W: u16 = 17;
pub const KEY_E: u16 = 18;
pub const KEY_R: u16 = 19;
pub const KEY_T: u16 = 20;
pub const KEY_Y: u16 = 21;
pub const KEY_U: u16 = 22;
pub const KEY_I: u16 = 23;
pub const KEY_O: u16 = 24;
pub const KEY_P: u16 = 25;
pub const KEY_LEFTBRACE: u16 = 26;
pub const KEY_RIGHTBRACE: u16 = 27;
pub const KEY_ENTER: u16 = 28;
pub const KEY_LEFTCTRL: u16 = 29;
pub const KEY_A: u16 = 30;
pub const KEY_S: u16 = 31;
pub const KEY_D: u16 = 32;
pub const KEY_F: u16 = 33;
pub const KEY_G: u16 = 34;
pub const KEY_H: u16 = 35;
pub const KEY_J: u16 = 36;
pub const KEY_K: u16 = 37;
pub const KEY_L: u16 = 38;
pub const KEY_SEMICOLON: u16 = 39;
pub const KEY_APOSTROPHE: u16 = 40;
pub const KEY_GRAVE: u16 = 41;
pub const KEY_LEFTSHIFT: u16 = 42;
pub const KEY_BACKSLASH: u16 = 43;
pub const KEY_Z: u16 = 44;
pub const KEY_X: u16 = 45;
pub const KEY_C: u16 = 46;
pub const KEY_V: u16 = 47;
pub const KEY_B: u16 = 48;
pub const KEY_N: u16 = 49;
pub const KEY_M: u16 = 50;
pub const KEY_COMMA: u16 = 51;
pub const KEY_DOT: u16 = 52;
pub const KEY_SLASH: u16 = 53;
pub const KEY_RIGHTSHIFT: u16 = 54;
pub const KEY_KPASTERISK: u16 = 55;
pub const KEY_LEFTALT: u16 = 56;
pub const KEY_SPACE: u16 = 57;
pub const KEY_CAPSLOCK: u16 = 58;
pub const KEY_F1: u16 = 59;
pub const KEY_F2: u16 = 60;
pub const KEY_F3: u16 = 61;
pub const KEY_F4: u16 = 62;
pub const KEY_F5: u16 = 63;
pub const KEY_F6: u16 = 64;
pub const KEY_F7: u16 = 65;
pub const KEY_F8: u16 = 66;
pub const KEY_F9: u16 = 67;
pub const KEY_F10: u16 = 68;
pub const KEY_NUMLOCK: u16 = 69;
pub const KEY_SCROLLLOCK: u16 = 70;
pub const KEY_KP7: u16 = 71;
pub const KEY_KP8: u16 = 72;
pub const KEY_KP9: u16 = 73;
pub const KEY_KPMINUS: u16 = 74;
pub const KEY_KP4: u16 = 75;
pub const KEY_KP5: u16 = 76;
pub const KEY_KP6: u16 = 77;
pub const KEY_KPPLUS: u16 = 78;
pub const KEY_KP1: u16 = 79;
pub const KEY_KP2: u16 = 80;
pub const KEY_KP3: u16 = 81;
pub const KEY_KP0: u16 = 82;
pub const KEY_KPDOT: u16 = 83;

pub const KEY_ZENKAKUHANKAKU: u16 = 85;
pub const KEY_102ND: u16 = 86;
pub const KEY_F11: u16 = 87;
pub const KEY_F12: u16 = 88;
pub const KEY_RO: u16 = 89;
pub const KEY_KATAKANA: u16 = 90;
pub const KEY_HIRAGANA: u16 = 91;
pub const KEY_HENKAN: u16 = 92;
pub const KEY_KATAKANAHIRAGANA: u16 = 93;
pub const KEY_MUHENKAN: u16 = 94;
pub const KEY_KPJPCOMMA: u16 = 95;
pub const KEY_KPENTER: u16 = 96;
pub const KEY_RIGHTCTRL: u16 = 97;
pub const KEY_KPSLASH: u16 = 98;
pub const KEY_SYSRQ: u16 = 99;
pub const KEY_RIGHTALT: u16 = 100;
pub const KEY_LINEFEED: u16 = 101;
pub const KEY_HOME: u16 = 102;
pub const KEY_UP: u16 = 103;
pub const KEY_PAGEUP: u16 = 104;
pub const KEY_LEFT: u16 = 105;
pub const KEY_RIGHT: u16 = 106;
pub const KEY_END: u16 = 107;
pub const KEY_DOWN: u16 = 108;
pub const KEY_PAGEDOWN: u16 = 109;
pub const KEY_INSERT: u16 = 110;
pub const KEY_DELETE: u16 = 111;
pub const KEY_MACRO: u16 = 112;
pub const KEY_MUTE: u16 = 113;
pub const KEY_VOLUMEDOWN: u16 = 114;
pub const KEY_VOLUMEUP: u16 = 115;
/// System Power Down
pub const KEY_POWER: u16 = 116;
pub const KEY_KPEQUAL: u16 = 117;
pub const KEY_KPPLUSMINUS: u16 = 118;
pub const KEY_PAUSE: u16 = 119;
/// Compiz Scale (Expose)
pub const KEY_SCALE: u16 = 120;

pub const KEY_KPCOMMA: u16 = 121;
pub const KEY_HANGEUL: u16 = 122;
pub const KEY_HANJA: u16 = 123;
pub const KEY_YEN: u16 = 124;
pub const KEY_LEFTMETA: u16 = 125;
pub const KEY_RIGHTMETA: u16 = 126;
pub const KEY_COMPOSE: u16 = 127;

/// Stop
pub const KEY_STOP: u16 = 128;
pub const KEY_AGAIN: u16 = 129;
/// Properties */
pub const KEY_PROPS: u16 = 130;
/// Undo
pub const KEY_UNDO: u16 = 131;
pub const KEY_FRONT: u16 = 132;
/// Copy
pub const KEY_COPY: u16 = 133;
/// Open
pub const KEY_OPEN: u16 = 134;
/// Paste
pub const KEY_PASTE: u16 = 135;
/// Search
pub const KEY_FIND: u16 = 136;
/// Cut
pub const KEY_CUT: u16 = 137;
/// Integrated Help Center
pub const KEY_HELP: u16 = 138;
/// Menu (show menu)
pub const KEY_MENU: u16 = 139;
/// Calculator
pub const KEY_CALC: u16 = 140;
pub const KEY_SETUP: u16 = 141;
/// System Sleep
pub const KEY_SLEEP: u16 = 142;
/// System Wake Up
pub const KEY_WAKEUP: u16 = 143;
/// Local Machine Browser
pub const KEY_FILE: u16 = 144;
pub const KEY_SENDFILE: u16 = 145;
pub const KEY_DELETEFILE: u16 = 146;
pub const KEY_XFER: u16 = 147;
pub const KEY_PROG1: u16 = 148;
pub const KEY_PROG2: u16 = 149;
/// Internet Browser
pub const KEY_WWW: u16 = 150;
pub const KEY_MSDOS: u16 = 151;
/// Terminal Lock/Screensaver
pub const KEY_COFFEE: u16 = 152;
/// Display orientation for e.g. tablets
pub const KEY_ROTATE_DISPLAY: u16 = 153;
pub const KEY_CYCLEWINDOWS: u16 = 154;
pub const KEY_MAIL: u16 = 155;
/// Bookmarks
pub const KEY_BOOKMARKS: u16 = 156;
pub const KEY_COMPUTER: u16 = 157;
/// Back
pub const KEY_BACK: u16 = 158;
/// Forward */
pub const KEY_FORWARD: u16 = 159;
pub const KEY_CLOSECD: u16 = 160;
pub const KEY_EJECTCD: u16 = 161;
pub const KEY_EJECTCLOSECD: u16 = 162;
pub const KEY_NEXTSONG: u16 = 163;
pub const KEY_PLAYPAUSE: u16 = 164;
pub const KEY_PREVIOUSSONG: u16 = 165;
pub const KEY_STOPCD: u16 = 166;
pub const KEY_RECORD: u16 = 167;
pub const KEY_REWIND: u16 = 168;
/// Media Select Telephone
pub const KEY_PHONE: u16 = 169;
pub const KEY_ISO: u16 = 170;
/// Consumer Control Configuration
pub const KEY_CONFIG: u16 = 171;
/// Home
pub const KEY_HOMEPAGE: u16 = 172;
/// Refresh
pub const KEY_REFRESH: u16 = 173;
/// Exit
pub const KEY_EXIT: u16 = 174;
pub const KEY_MOVE: u16 = 175;
pub const KEY_EDIT: u16 = 176;
pub const KEY_SCROLLUP: u16 = 177;
pub const KEY_SCROLLDOWN: u16 = 178;
pub const KEY_KPLEFTPAREN: u16 = 179;
pub const KEY_KPRIGHTPAREN: u16 = 180;
/// New
pub const KEY_NEW: u16 = 181;
/// Redo/Repeat
pub const KEY_REDO: u16 = 182;

pub const KEY_F13: u16 = 183;
pub const KEY_F14: u16 = 184;
pub const KEY_F15: u16 = 185;
pub const KEY_F16: u16 = 186;
pub const KEY_F17: u16 = 187;
pub const KEY_F18: u16 = 188;
pub const KEY_F19: u16 = 189;
pub const KEY_F20: u16 = 190;
pub const KEY_F21: u16 = 191;
pub const KEY_F22: u16 = 192;
pub const KEY_F23: u16 = 193;
pub const KEY_F24: u16 = 194;

/// Cycle between available video outputs (Monitor/LCD/TV-out/etc)
pub const KEY_SWITCHVIDEOMODE: u16 = 227;
pub const KEY_KBDILLUMTOGGLE: u16 = 228;
pub const KEY_KBDILLUMDOWN: u16 = 229;
pub const KEY_KBDILLUMUP: u16 = 230;

pub const KEY_BLUETOOTH: u16 = 237;
pub const KEY_WLAN: u16 = 238;
pub const KEY_UWB: u16 = 239;

pub const KEY_UNKNOWN: u16 = 240;

/// drive next video source
pub const KEY_VIDEO_NEXT: u16 = 241;
/// drive previous video source
pub const KEY_VIDEO_PREV: u16 = 242;
/// brightness up, after max is min
pub const KEY_BRIGHTNESS_CYCLE: u16 = 243;
/// Set Auto Brightness: manual brightness control is off, rely on ambient
pub const KEY_BRIGHTNESS_AUTO: u16 = 244;
/// display device to off state
pub const KEY_DISPLAY_OFF: u16 = 245;

/// Wireless WAN (LTE, UMTS, GSM, etc.)
pub const KEY_WWAN: u16 = 246;
/// Key that controls all radios
pub const KEY_RFKILL: u16 = 247;

/// Mute / unmute the microphone
pub const KEY_MICMUTE: u16 = 248;

pub const BTN_MISC: u16 = 0x100;
pub const BTN_0: u16 = 0x100;
pub const BTN_1: u16 = 0x101;
pub const BTN_2: u16 = 0x102;
pub const BTN_3: u16 = 0x103;
pub const BTN_4: u16 = 0x104;
pub const BTN_5: u16 = 0x105;
pub const BTN_6: u16 = 0x106;
pub const BTN_7: u16 = 0x107;
pub const BTN_8: u16 = 0x108;
pub const BTN_9: u16 = 0x109;

pub const BTN_MOUSE: u16 = 0x110;
pub const BTN_LEFT: u16 = 0x110;
pub const BTN_RIGHT: u16 = 0x111;
pub const BTN_MIDDLE: u16 = 0x112;
pub const BTN_SIDE: u16 = 0x113;
pub const BTN_EXTRA: u16 = 0x114;
pub const BTN_FORWARD: u16 = 0x115;
pub const BTN_BACK: u16 = 0x116;
pub const BTN_TASK: u16 = 0x117;

pub const BTN_JOYSTICK: u16 = 0x120;
pub const BTN_TRIGGER: u16 = 0x120;
pub const BTN_THUMB: u16 = 0x121;
pub const BTN_THUMB2: u16 = 0x122;
pub const BTN_TOP: u16 = 0x123;
pub const BTN_TOP2: u16 = 0x124;
pub const BTN_PINKIE: u16 = 0x125;
pub const BTN_BASE: u16 = 0x126;
pub const BTN_BASE2: u16 = 0x127;
pub const BTN_BASE3: u16 = 0x128;
pub const BTN_BASE4: u16 = 0x129;
pub const BTN_BASE5: u16 = 0x12a;
pub const BTN_BASE6: u16 = 0x12b;
pub const BTN_DEAD: u16 = 0x12f;

pub const BTN_GAMEPAD: u16 = 0x130;
pub const BTN_SOUTH: u16 = 0x130;
pub const BTN_EAST: u16 = 0x131;
pub const BTN_C: u16 = 0x132;
pub const BTN_NORTH: u16 = 0x133;
pub const BTN_WEST: u16 = 0x134;
pub const BTN_Z: u16 = 0x135;
pub const BTN_TL: u16 = 0x136;
pub const BTN_TR: u16 = 0x137;
pub const BTN_TL2: u16 = 0x138;
pub const BTN_TR2: u16 = 0x139;
pub const BTN_SELECT: u16 = 0x13a;
pub const BTN_START: u16 = 0x13b;
pub const BTN_MODE: u16 = 0x13c;
pub const BTN_THUMBL: u16 = 0x13d;
pub const BTN_THUMBR: u16 = 0x13e;

pub const BTN_DIGI: u16 = 0x140;
pub const BTN_TOOL_PEN: u16 = 0x140;
pub const BTN_TOOL_RUBBER: u16 = 0x141;
pub const BTN_TOOL_BRUSH: u16 = 0x142;
pub const BTN_TOOL_PENCIL: u16 = 0x143;
pub const BTN_TOOL_AIRBRUSH: u16 = 0x144;
pub const BTN_TOOL_FINGER: u16 = 0x145;
pub const BTN_TOOL_MOUSE: u16 = 0x146;
pub const BTN_TOOL_LENS: u16 = 0x147;
/// Five fingers on trackpad
pub const BTN_TOOL_QUINTTAP: u16 = 0x148;
pub const BTN_STYLUS3: u16 = 0x149;
pub const BTN_TOUCH: u16 = 0x14a;
pub const BTN_STYLUS: u16 = 0x14b;
pub const BTN_STYLUS2: u16 = 0x14c;
pub const BTN_TOOL_DOUBLETAP: u16 = 0x14d;
pub const BTN_TOOL_TRIPLETAP: u16 = 0x14e;
/// Four fingers on trackpad
pub const BTN_TOOL_QUADTAP: u16 = 0x14f;

pub const BTN_WHEEL: u16 = 0x150;
pub const BTN_GEAR_DOWN: u16 = 0x150;
pub const BTN_GEAR_UP: u16 = 0x151;

pub const KEY_FN: u16 = 0x1d0;
pub const KEY_FN_ESC: u16 = 0x1d1;
pub const KEY_FN_F1: u16 = 0x1d2;
pub const KEY_FN_F2: u16 = 0x1d3;
pub const KEY_FN_F3: u16 = 0x1d4;
pub const KEY_FN_F4: u16 = 0x1d5;
pub const KEY_FN_F5: u16 = 0x1d6;
pub const KEY_FN_F6: u16 = 0x1d7;
pub const KEY_FN_F7: u16 = 0x1d8;
pub const KEY_FN_F8: u16 = 0x1d9;
pub const KEY_FN_F9: u16 = 0x1da;
pub const KEY_FN_F10: u16 = 0x1db;
pub const KEY_FN_F11: u16 = 0x1dc;
pub const KEY_FN_F12: u16 = 0x1dd;
pub const KEY_FN_1: u16 = 0x1de;
pub const KEY_FN_2: u16 = 0x1df;
pub const KEY_FN_D: u16 = 0x1e0;
pub const KEY_FN_E: u16 = 0x1e1;
pub const KEY_FN_F: u16 = 0x1e2;
pub const KEY_FN_S: u16 = 0x1e3;
pub const KEY_FN_B: u16 = 0x1e4;
pub const KEY_FN_RIGHT_SHIFT: u16 = 0x1e5;

/*
 * Relative axes
 */

pub const REL_X: c_short = 0x00;
pub const REL_Y: c_short = 0x01;
pub const REL_Z: c_short = 0x02;
pub const REL_RX: c_short = 0x03;
pub const REL_RY: c_short = 0x04;
pub const REL_RZ: c_short = 0x05;
pub const REL_HWHEEL: c_short = 0x06;
pub const REL_DIAL: c_short = 0x07;
pub const REL_WHEEL: c_short = 0x08;
pub const REL_MISC: c_short = 0x09;
pub const REL_RESERVED: c_short = 0x0a;
pub const REL_WHEEL_HI_RES: c_short = 0x0b;
pub const REL_HWHEEL_HI_RES: c_short = 0x0c;
pub const REL_MAX: c_short = 0x0f;
pub const REL_CNT: c_short = REL_MAX + 1;

/*
 * Absolute axes
 */

pub const ABS_X: c_short = 0x00;
pub const ABS_Y: c_short = 0x01;
pub const ABS_Z: c_short = 0x02;
pub const ABS_RX: c_short = 0x03;
pub const ABS_RY: c_short = 0x04;
pub const ABS_RZ: c_short = 0x05;
pub const ABS_THROTTLE: c_short = 0x06;
pub const ABS_RUDDER: c_short = 0x07;
pub const ABS_WHEEL: c_short = 0x08;
pub const ABS_GAS: c_short = 0x09;
pub const ABS_BRAKE: c_short = 0x0a;
pub const ABS_HAT0X: c_short = 0x10;
pub const ABS_HAT0Y: c_short = 0x11;
pub const ABS_HAT1X: c_short = 0x12;
pub const ABS_HAT1Y: c_short = 0x13;
pub const ABS_HAT2X: c_short = 0x14;
pub const ABS_HAT2Y: c_short = 0x15;
pub const ABS_HAT3X: c_short = 0x16;
pub const ABS_HAT3Y: c_short = 0x17;
pub const ABS_PRESSURE: c_short = 0x18;
pub const ABS_DISTANCE: c_short = 0x19;
pub const ABS_TILT_X: c_short = 0x1a;
pub const ABS_TILT_Y: c_short = 0x1b;
pub const ABS_TOOL_WIDTH: c_short = 0x1c;

pub const ABS_VOLUME: c_short = 0x20;

pub const ABS_MISC: c_short = 0x28;

pub const ABS_RESERVED: c_short = 0x2e;

/// MT slot being modified
pub const ABS_MT_SLOT: c_short = 0x2f;
/// Major axis of touching ellipse
pub const ABS_MT_TOUCH_MAJOR: c_short = 0x30;
/// Minor axis (omit if circular)
pub const ABS_MT_TOUCH_MINOR: c_short = 0x31;
/// Major axis of approaching ellipse
pub const ABS_MT_WIDTH_MAJOR: c_short = 0x32;
/// Minor axis (omit if circular)
pub const ABS_MT_WIDTH_MINOR: c_short = 0x33;
/// Ellipse orientation
pub const ABS_MT_ORIENTATION: c_short = 0x34;
/// Center X touch position
pub const ABS_MT_POSITION_X: c_short = 0x35;
/// Center Y touch position
pub const ABS_MT_POSITION_Y: c_short = 0x36;
/// Type of touching device
pub const ABS_MT_TOOL_TYPE: c_short = 0x37;
/// Group a set of packets as a blob
pub const ABS_MT_BLOB_ID: c_short = 0x38;
/// Unique ID of initiated contact
pub const ABS_MT_TRACKING_ID: c_short = 0x39;
/// Pressure on contact area
pub const ABS_MT_PRESSURE: c_short = 0x3a;
/// Contact hover distance
pub const ABS_MT_DISTANCE: c_short = 0x3b;
/// Center X tool position
pub const ABS_MT_TOOL_X: c_short = 0x3c;
/// Center Y tool position
pub const ABS_MT_TOOL_Y: c_short = 0x3d;

pub const ABS_MAX: c_short = 0x3f;
pub const ABS_CNT: c_short = ABS_MAX + 1;

// TODO Allow buffering of several events at once

/// EvDev notifies events in the format represented by this structure.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EvDevInputEvent {
	pub time: libc::timeval,
	pub r#type: c_short,
	pub code: c_short,
	pub value: c_int,
}

/// Structure representing an input device.
pub struct InputDevice {
	/// The device's file.
	file: File,

	/// A buffer storing a partial event structure.
	buff: [u8; size_of::<EvDevInputEvent>()],
	/// The cursor on the buffer.
	cursor: usize,

	/// The current slot being used.
	/// For touchpads, this represents the finger number.
	current_slot: u32,
}

impl InputDevice {
	/// Returns a device from the given device file's path.
	pub fn from_path(path: &PathBuf) -> io::Result<Self> {
		Ok(Self {
			file: File::open(path)?,

			buff: [0; size_of::<EvDevInputEvent>()],
			cursor: 0,

			current_slot: 0,
		})
	}

	/// Reads an event from the device file.
	///
	/// The function may return None if further data needs to be read.
	fn read_input(&mut self) -> io::Result<Option<EvDevInputEvent>> {
		loop {
			let len = self.file.read(&mut self.buff[self.cursor..])?;
			if len == 0 {
				break;
			}

			self.cursor += len;
		}

		if self.cursor >= size_of::<EvDevInputEvent>() {
			let ev = *unsafe { util::reinterpret(&self.buff) };
			self.cursor = 0;

			Ok(Some(ev))
		} else {
			Ok(None)
		}
	}

	/// Returns the next event.
	/// The function blocks until at least one event is available.
	///
	/// The function may return None if EOF has been reached or if further events are required.
	pub fn next(&mut self) -> io::Result<Option<Input>> {
		let input = match self.read_input()? {
			Some(input) => input,
			None => return Ok(None),
		};
		//println!("-> {} {} {}", input.r#type, input.code, input.value);

		match (input.r#type, input.code) {
			(EV_KEY, _) => match input.value {
				0 => return Ok(Some(Input::KeyPress(input.code as _))),
				1 => return Ok(Some(Input::KeyRelease(input.code as _))),
				2 => return Ok(Some(Input::KeyRepeat(input.code as _))),

				_ => {},
			},

			(EV_REL, REL_X) => {
				println!("rel x: {}", input.value);
				// TODO
			},

			(EV_REL, REL_Y) => {
				println!("rel y: {}", input.value);
				// TODO
			},

			(EV_ABS, ABS_MT_SLOT) => self.current_slot = input.value as _,

			(EV_ABS, ABS_X) => {
				println!("x: {}", input.value);
				// TODO
			},

			(EV_ABS, ABS_Y) => {
				println!("y: {}", input.value);
				// TODO
			},

			_ => {},
		}

		Ok(None)
	}
}

impl AsRawFd for InputDevice {
	fn as_raw_fd(&self) -> i32 {
		self.file.as_raw_fd()
	}
}
