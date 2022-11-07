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

/*
 * Device properties and quirks
 */

/// needs a pointer
const INPUT_PROP_POINTER: u16 = 0x00;
/// direct input devices
const INPUT_PROP_DIRECT: u16 = 0x01;
/// has button(s) under pad
const INPUT_PROP_BUTTONPAD: u16 = 0x02;
/// touch rectangle only
const INPUT_PROP_SEMI_MT: u16 = 0x03;
/// softbuttons at top of pad
const INPUT_PROP_TOPBUTTONPAD: u16 = 0x04;
/// is a pointing stick
const INPUT_PROP_POINTING_STICK: u16 = 0x05;
/// has accelerometer
const INPUT_PROP_ACCELEROMETER: u16 = 0x06;

const INPUT_PROP_MAX: u16 = 0x1f;
const INPUT_PROP_CNT: u16 = INPUT_PROP_MAX + 1;

/*
 * Event types
 */

const EV_SYN: u16 = 0x00;
const EV_KEY: u16 = 0x01;
const EV_REL: u16 = 0x02;
const EV_ABS: u16 = 0x03;
const EV_MSC: u16 = 0x04;
const EV_SW: u16 = 0x05;
const EV_LED: u16 = 0x11;
const EV_SND: u16 = 0x12;
const EV_REP: u16 = 0x14;
const EV_FF: u16 = 0x15;
const EV_PWR: u16 = 0x16;
const EV_FF_STATUS: u16 = 0x17;
const EV_MAX: u16 = 0x1f;
const EV_CNT: u16 = EV_MAX + 1;

/*
 * Synchronization events.
 */

const SYN_REPORT: u16 = 0;
const SYN_CONFIG: u16 = 1;
const SYN_MT_REPORT: u16 = 2;
const SYN_DROPPED: u16 = 3;
const SYN_MAX: u16 = 0xf;
const SYN_CNT: u16 = SYN_MAX+1;

/*
 * Keys and buttons
 */

const KEY_RESERVED: u16 = 0;
const KEY_ESC: u16 = 1;
const KEY_1: u16 = 2;
const KEY_2: u16 = 3;
const KEY_3: u16 = 4;
const KEY_4: u16 = 5;
const KEY_5: u16 = 6;
const KEY_6: u16 = 7;
const KEY_7: u16 = 8;
const KEY_8: u16 = 9;
const KEY_9: u16 = 10;
const KEY_0: u16 = 11;
const KEY_MINUS: u16 = 12;
const KEY_EQUAL: u16 = 13;
const KEY_BACKSPACE: u16 = 14;
const KEY_TAB: u16 = 15;
const KEY_Q: u16 = 16;
const KEY_W: u16 = 17;
const KEY_E: u16 = 18;
const KEY_R: u16 = 19;
const KEY_T: u16 = 20;
const KEY_Y: u16 = 21;
const KEY_U: u16 = 22;
const KEY_I: u16 = 23;
const KEY_O: u16 = 24;
const KEY_P: u16 = 25;
const KEY_LEFTBRACE: u16 = 26;
const KEY_RIGHTBRACE: u16 = 27;
const KEY_ENTER: u16 = 28;
const KEY_LEFTCTRL: u16 = 29;
const KEY_A: u16 = 30;
const KEY_S: u16 = 31;
const KEY_D: u16 = 32;
const KEY_F: u16 = 33;
const KEY_G: u16 = 34;
const KEY_H: u16 = 35;
const KEY_J: u16 = 36;
const KEY_K: u16 = 37;
const KEY_L: u16 = 38;
const KEY_SEMICOLON: u16 = 39;
const KEY_APOSTROPHE: u16 = 40;
const KEY_GRAVE: u16 = 41;
const KEY_LEFTSHIFT: u16 = 42;
const KEY_BACKSLASH: u16 = 43;
const KEY_Z: u16 = 44;
const KEY_X: u16 = 45;
const KEY_C: u16 = 46;
const KEY_V: u16 = 47;
const KEY_B: u16 = 48;
const KEY_N: u16 = 49;
const KEY_M: u16 = 50;
const KEY_COMMA: u16 = 51;
const KEY_DOT: u16 = 52;
const KEY_SLASH: u16 = 53;
const KEY_RIGHTSHIFT: u16 = 54;
const KEY_KPASTERISK: u16 = 55;
const KEY_LEFTALT: u16 = 56;
const KEY_SPACE: u16 = 57;
const KEY_CAPSLOCK: u16 = 58;
const KEY_F1: u16 = 59;
const KEY_F2: u16 = 60;
const KEY_F3: u16 = 61;
const KEY_F4: u16 = 62;
const KEY_F5: u16 = 63;
const KEY_F6: u16 = 64;
const KEY_F7: u16 = 65;
const KEY_F8: u16 = 66;
const KEY_F9: u16 = 67;
const KEY_F10: u16 = 68;
const KEY_NUMLOCK: u16 = 69;
const KEY_SCROLLLOCK: u16 = 70;
const KEY_KP7: u16 = 71;
const KEY_KP8: u16 = 72;
const KEY_KP9: u16 = 73;
const KEY_KPMINUS: u16 = 74;
const KEY_KP4: u16 = 75;
const KEY_KP5: u16 = 76;
const KEY_KP6: u16 = 77;
const KEY_KPPLUS: u16 = 78;
const KEY_KP1: u16 = 79;
const KEY_KP2: u16 = 80;
const KEY_KP3: u16 = 81;
const KEY_KP0: u16 = 82;
const KEY_KPDOT: u16 = 83;

const KEY_ZENKAKUHANKAKU: u16 = 85;
const KEY_102ND: u16 = 86;
const KEY_F11: u16 = 87;
const KEY_F12: u16 = 88;
const KEY_RO: u16 = 89;
const KEY_KATAKANA: u16 = 90;
const KEY_HIRAGANA: u16 = 91;
const KEY_HENKAN: u16 = 92;
const KEY_KATAKANAHIRAGANA: u16 = 93;
const KEY_MUHENKAN: u16 = 94;
const KEY_KPJPCOMMA: u16 = 95;
const KEY_KPENTER: u16 = 96;
const KEY_RIGHTCTRL: u16 = 97;
const KEY_KPSLASH: u16 = 98;
const KEY_SYSRQ: u16 = 99;
const KEY_RIGHTALT: u16 = 100;
const KEY_LINEFEED: u16 = 101;
const KEY_HOME: u16 = 102;
const KEY_UP: u16 = 103;
const KEY_PAGEUP: u16 = 104;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;
const KEY_END: u16 = 107;
const KEY_DOWN: u16 = 108;
const KEY_PAGEDOWN: u16 = 109;
const KEY_INSERT: u16 = 110;
const KEY_DELETE: u16 = 111;
const KEY_MACRO: u16 = 112;
const KEY_MUTE: u16 = 113;
const KEY_VOLUMEDOWN: u16 = 114;
const KEY_VOLUMEUP: u16 = 115;
/// System Power Down
const KEY_POWER: u16 = 116;
const KEY_KPEQUAL: u16 = 117;
const KEY_KPPLUSMINUS: u16 = 118;
const KEY_PAUSE: u16 = 119;
/// Compiz Scale (Expose)
const KEY_SCALE: u16 = 120	;

const KEY_KPCOMMA: u16 = 121;
const KEY_HANGEUL: u16 = 122;
const KEY_HANJA: u16 = 123;
const KEY_YEN: u16 = 124;
const KEY_LEFTMETA: u16 = 125;
const KEY_RIGHTMETA: u16 = 126;
const KEY_COMPOSE: u16 = 127;

/// Stop
const KEY_STOP: u16 = 128;
const KEY_AGAIN: u16 = 129;
/// Properties */
const KEY_PROPS: u16 = 130;
/// Undo
const KEY_UNDO: u16 = 131;
const KEY_FRONT: u16 = 132;
/// Copy
const KEY_COPY: u16 = 133;
/// Open
const KEY_OPEN: u16 = 134;
/// Paste
const KEY_PASTE: u16 = 135;
/// Search
const KEY_FIND: u16 = 136;
/// Cut
const KEY_CUT: u16 = 137;
/// Integrated Help Center
const KEY_HELP: u16 = 138;
/// Menu (show menu)
const KEY_MENU: u16 = 139;
/// Calculator
const KEY_CALC: u16 = 140;
const KEY_SETUP: u16 = 141;
/// System Sleep
const KEY_SLEEP: u16 = 142;
/// System Wake Up
const KEY_WAKEUP: u16 = 143;
/// Local Machine Browser
const KEY_FILE: u16 = 144;
const KEY_SENDFILE: u16 = 145;
const KEY_DELETEFILE: u16 = 146;
const KEY_XFER: u16 = 147;
const KEY_PROG1: u16 = 148;
const KEY_PROG2: u16 = 149;
/// Internet Browser
const KEY_WWW: u16 = 150;
const KEY_MSDOS: u16 = 151;
/// Terminal Lock/Screensaver
const KEY_COFFEE: u16 = 152;
/// Display orientation for e.g. tablets
const KEY_ROTATE_DISPLAY: u16 = 153;
const KEY_CYCLEWINDOWS: u16 = 154;
const KEY_MAIL: u16 = 155;
/// Bookmarks
const KEY_BOOKMARKS: u16 = 156;
const KEY_COMPUTER: u16 = 157;
/// Back
const KEY_BACK: u16 = 158;
/// Forward */
const KEY_FORWARD: u16 = 159;
const KEY_CLOSECD: u16 = 160;
const KEY_EJECTCD: u16 = 161;
const KEY_EJECTCLOSECD: u16 = 162;
const KEY_NEXTSONG: u16 = 163;
const KEY_PLAYPAUSE: u16 = 164;
const KEY_PREVIOUSSONG: u16 = 165;
const KEY_STOPCD: u16 = 166;
const KEY_RECORD: u16 = 167;
const KEY_REWIND: u16 = 168;
/// Media Select Telephone
const KEY_PHONE: u16 = 169;
const KEY_ISO: u16 = 170;
/// Consumer Control Configuration
const KEY_CONFIG: u16 = 171;
/// Home
const KEY_HOMEPAGE: u16 = 172;
/// Refresh
const KEY_REFRESH: u16 = 173;
/// Exit
const KEY_EXIT: u16 = 174;
const KEY_MOVE: u16 = 175;
const KEY_EDIT: u16 = 176;
const KEY_SCROLLUP: u16 = 177;
const KEY_SCROLLDOWN: u16 = 178;
const KEY_KPLEFTPAREN: u16 = 179;
const KEY_KPRIGHTPAREN: u16 = 180;
/// New
const KEY_NEW: u16 = 181;
/// Redo/Repeat
const KEY_REDO: u16 = 182;

const KEY_F13: u16 = 183;
const KEY_F14: u16 = 184;
const KEY_F15: u16 = 185;
const KEY_F16: u16 = 186;
const KEY_F17: u16 = 187;
const KEY_F18: u16 = 188;
const KEY_F19: u16 = 189;
const KEY_F20: u16 = 190;
const KEY_F21: u16 = 191;
const KEY_F22: u16 = 192;
const KEY_F23: u16 = 193;
const KEY_F24: u16 = 194;

/// Cycle between available video outputs (Monitor/LCD/TV-out/etc)
const KEY_SWITCHVIDEOMODE: u16 = 227;
const KEY_KBDILLUMTOGGLE: u16 = 228;
const KEY_KBDILLUMDOWN: u16 = 229;
const KEY_KBDILLUMUP: u16 = 230;

const KEY_BLUETOOTH: u16 = 237;
const KEY_WLAN: u16 = 238;
const KEY_UWB: u16 = 239;

const KEY_UNKNOWN: u16 = 240;

/// drive next video source
const KEY_VIDEO_NEXT: u16 = 241;
/// drive previous video source
const KEY_VIDEO_PREV: u16 = 242;
/// brightness up, after max is min
const KEY_BRIGHTNESS_CYCLE: u16 = 243;
/// Set Auto Brightness: manual brightness control is off, rely on ambient
const KEY_BRIGHTNESS_AUTO: u16 = 244;
/// display device to off state
const KEY_DISPLAY_OFF: u16 = 245;

/// Wireless WAN (LTE, UMTS, GSM, etc.)
const KEY_WWAN: u16 = 246;
/// Key that controls all radios
const KEY_RFKILL: u16 = 247;

/// Mute / unmute the microphone
const KEY_MICMUTE: u16 = 248;

const BTN_MISC: u16 = 0x100;
const BTN_0: u16 = 0x100;
const BTN_1: u16 = 0x101;
const BTN_2: u16 = 0x102;
const BTN_3: u16 = 0x103;
const BTN_4: u16 = 0x104;
const BTN_5: u16 = 0x105;
const BTN_6: u16 = 0x106;
const BTN_7: u16 = 0x107;
const BTN_8: u16 = 0x108;
const BTN_9: u16 = 0x109;

const BTN_MOUSE: u16 = 0x110;
const BTN_LEFT: u16 = 0x110;
const BTN_RIGHT: u16 = 0x111;
const BTN_MIDDLE: u16 = 0x112;
const BTN_SIDE: u16 = 0x113;
const BTN_EXTRA: u16 = 0x114;
const BTN_FORWARD: u16 = 0x115;
const BTN_BACK: u16 = 0x116;
const BTN_TASK: u16 = 0x117;

const BTN_JOYSTICK: u16 = 0x120;
const BTN_TRIGGER: u16 = 0x120;
const BTN_THUMB: u16 = 0x121;
const BTN_THUMB2: u16 = 0x122;
const BTN_TOP: u16 = 0x123;
const BTN_TOP2: u16 = 0x124;
const BTN_PINKIE: u16 = 0x125;
const BTN_BASE: u16 = 0x126;
const BTN_BASE2: u16 = 0x127;
const BTN_BASE3: u16 = 0x128;
const BTN_BASE4: u16 = 0x129;
const BTN_BASE5: u16 = 0x12a;
const BTN_BASE6: u16 = 0x12b;
const BTN_DEAD: u16 = 0x12f;

const BTN_GAMEPAD: u16 = 0x130;
const BTN_SOUTH: u16 = 0x130;
const BTN_EAST: u16 = 0x131;
const BTN_C: u16 = 0x132;
const BTN_NORTH: u16 = 0x133;
const BTN_WEST: u16 = 0x134;
const BTN_Z: u16 = 0x135;
const BTN_TL: u16 = 0x136;
const BTN_TR: u16 = 0x137;
const BTN_TL2: u16 = 0x138;
const BTN_TR2: u16 = 0x139;
const BTN_SELECT: u16 = 0x13a;
const BTN_START: u16 = 0x13b;
const BTN_MODE: u16 = 0x13c;
const BTN_THUMBL: u16 = 0x13d;
const BTN_THUMBR: u16 = 0x13e;

const BTN_DIGI: u16 = 0x140;
const BTN_TOOL_PEN: u16 = 0x140;
const BTN_TOOL_RUBBER: u16 = 0x141;
const BTN_TOOL_BRUSH: u16 = 0x142;
const BTN_TOOL_PENCIL: u16 = 0x143;
const BTN_TOOL_AIRBRUSH: u16 = 0x144;
const BTN_TOOL_FINGER: u16 = 0x145;
const BTN_TOOL_MOUSE: u16 = 0x146;
const BTN_TOOL_LENS: u16 = 0x147;
/// Five fingers on trackpad
const BTN_TOOL_QUINTTAP: u16 = 0x148;
const BTN_STYLUS3: u16 = 0x149;
const BTN_TOUCH: u16 = 0x14a;
const BTN_STYLUS: u16 = 0x14b;
const BTN_STYLUS2: u16 = 0x14c;
const BTN_TOOL_DOUBLETAP: u16 = 0x14d;
const BTN_TOOL_TRIPLETAP: u16 = 0x14e;
/// Four fingers on trackpad
const BTN_TOOL_QUADTAP: u16 = 0x14f;

const BTN_WHEEL: u16 = 0x150;
const BTN_GEAR_DOWN: u16 = 0x150;
const BTN_GEAR_UP: u16 = 0x151;

const KEY_FN: u16 = 0x1d0;
const KEY_FN_ESC: u16 = 0x1d1;
const KEY_FN_F1: u16 = 0x1d2;
const KEY_FN_F2: u16 = 0x1d3;
const KEY_FN_F3: u16 = 0x1d4;
const KEY_FN_F4: u16 = 0x1d5;
const KEY_FN_F5: u16 = 0x1d6;
const KEY_FN_F6: u16 = 0x1d7;
const KEY_FN_F7: u16 = 0x1d8;
const KEY_FN_F8: u16 = 0x1d9;
const KEY_FN_F9: u16 = 0x1da;
const KEY_FN_F10: u16 = 0x1db;
const KEY_FN_F11: u16 = 0x1dc;
const KEY_FN_F12: u16 = 0x1dd;
const KEY_FN_1: u16 = 0x1de;
const KEY_FN_2: u16 = 0x1df;
const KEY_FN_D: u16 = 0x1e0;
const KEY_FN_E: u16 = 0x1e1;
const KEY_FN_F: u16 = 0x1e2;
const KEY_FN_S: u16 = 0x1e3;
const KEY_FN_B: u16 = 0x1e4;
const KEY_FN_RIGHT_SHIFT: u16 = 0x1e5;

/*
 * Relative axes
 */

const REL_X: u16 = 0x00;
const REL_Y: u16 = 0x01;
const REL_Z: u16 = 0x02;
const REL_RX: u16 = 0x03;
const REL_RY: u16 = 0x04;
const REL_RZ: u16 = 0x05;
const REL_HWHEEL: u16 = 0x06;
const REL_DIAL: u16 = 0x07;
const REL_WHEEL: u16 = 0x08;
const REL_MISC: u16 = 0x09;
/*
 * 0x0a is reserved and should not be used in input drivers.
 * It was used by HID as REL_MISC+1 and userspace needs to detect if
 * the next REL_* event is correct or is just REL_MISC + n.
 * We define here REL_RESERVED so userspace can rely on it and detect
 * the situation described above.
 */
const REL_RESERVED: u16 = 0x0a;
const REL_WHEEL_HI_RES: u16 = 0x0b;
const REL_HWHEEL_HI_RES: u16 = 0x0c;
const REL_MAX: u16 = 0x0f;
const REL_CNT: u16 = REL_MAX + 1;

/*
 * Absolute axes
 */

const ABS_X: u16 = 0x00;
const ABS_Y: u16 = 0x01;
const ABS_Z: u16 = 0x02;
const ABS_RX: u16 = 0x03;
const ABS_RY: u16 = 0x04;
const ABS_RZ: u16 = 0x05;
const ABS_THROTTLE: u16 = 0x06;
const ABS_RUDDER: u16 = 0x07;
const ABS_WHEEL: u16 = 0x08;
const ABS_GAS: u16 = 0x09;
const ABS_BRAKE: u16 = 0x0a;
const ABS_HAT0X: u16 = 0x10;
const ABS_HAT0Y: u16 = 0x11;
const ABS_HAT1X: u16 = 0x12;
const ABS_HAT1Y: u16 = 0x13;
const ABS_HAT2X: u16 = 0x14;
const ABS_HAT2Y: u16 = 0x15;
const ABS_HAT3X: u16 = 0x16;
const ABS_HAT3Y: u16 = 0x17;
const ABS_PRESSURE: u16 = 0x18;
const ABS_DISTANCE: u16 = 0x19;
const ABS_TILT_X: u16 = 0x1a;
const ABS_TILT_Y: u16 = 0x1b;
const ABS_TOOL_WIDTH: u16 = 0x1c;

const ABS_VOLUME: u16 = 0x20;

const ABS_MISC: u16 = 0x28;

/*
 * 0x2e is reserved and should not be used in input drivers.
 * It was used by HID as ABS_MISC+6 and userspace needs to detect if
 * the next ABS_* event is correct or is just ABS_MISC + n.
 * We define here ABS_RESERVED so userspace can rely on it and detect
 * the situation described above.
 */
const ABS_RESERVED: u16 = 0x2e;

/// MT slot being modified
const ABS_MT_SLOT: u16 = 0x2f;
/// Major axis of touching ellipse
const ABS_MT_TOUCH_MAJOR: u16 = 0x30;
/// Minor axis (omit if circular)
const ABS_MT_TOUCH_MINOR: u16 = 0x31;
/// Major axis of approaching ellipse
const ABS_MT_WIDTH_MAJOR: u16 = 0x32;
/// Minor axis (omit if circular)
const ABS_MT_WIDTH_MINOR: u16 = 0x33;
/// Ellipse orientation
const ABS_MT_ORIENTATION: u16 = 0x34;
/// Center X touch position
const ABS_MT_POSITION_X: u16 = 0x35;
/// Center Y touch position
const ABS_MT_POSITION_Y: u16 = 0x36;
/// Type of touching device
const ABS_MT_TOOL_TYPE: u16 = 0x37;
/// Group a set of packets as a blob
const ABS_MT_BLOB_ID: u16 = 0x38;
/// Unique ID of initiated contact
const ABS_MT_TRACKING_ID: u16 = 0x39;
/// Pressure on contact area
const ABS_MT_PRESSURE: u16 = 0x3a;
/// Contact hover distance
const ABS_MT_DISTANCE: u16 = 0x3b;
/// Center X tool position
const ABS_MT_TOOL_X: u16 = 0x3c;
/// Center Y tool position
const ABS_MT_TOOL_Y: u16 = 0x3d;

const ABS_MAX: u16 = 0x3f;
const ABS_CNT: u16 = ABS_MAX + 1;

// TODO Allow buffering of several events at once

/// EvDev notifies events in the format represented by this structure.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EvDevInputEvent {
	time: libc::timeval,
	r#type: c_short,
	code: c_short,
	value: c_int,
}

/// Structure representing an input device.
pub struct InputDevice {
	/// The device's file.
	file: File,

	/// A buffer storing a partial event structure.
	buff: [u8; size_of::<EvDevInputEvent>()],
	/// The cursor on the buffer.
	cursor: usize,
}

impl InputDevice {
	/// Returns a device from the given device file's path.
	pub fn from_path(path: &PathBuf) -> io::Result<Self> {
		Ok(Self {
			file: File::open(path)?,

			buff: [0; size_of::<EvDevInputEvent>()],
			cursor: 0,
		})
	}

	/// Returns the next event. The function blocks until at least one event is available.
	///
	/// If EOF has been reached, the function returns None.
	pub fn next(&mut self) -> io::Result<Option<EvDevInputEvent>> {
		loop {
			let len = self.file.read(&mut self.buff[self.cursor..])?;
			if len == 0 {
				break;
			}

			self.cursor += len;
		}

		if self.cursor >= size_of::<EvDevInputEvent>() {
			let ev = *unsafe {
				util::reinterpret(&self.buff)
			};
			self.cursor = 0;

			Ok(Some(ev))
		} else {
			Ok(None)
		}
	}
}

impl AsRawFd for InputDevice {
	fn as_raw_fd(&self) -> i32 {
		self.file.as_raw_fd()
	}
}
