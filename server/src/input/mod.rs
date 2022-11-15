//! Input devices are the set of devices that allow users to interact with the system.
//!
//! An input device is either:
//! - A keyboard
//! - A mouse
//! - A touchpad
//! - A touchscreen

pub mod device;

use crate::poll::PollHandler;
use device::EvDevInputEvent;
use device::InputDevice;
use std::fs;
use std::io;
use std::os::unix::io::AsRawFd;

/// The path to the directory containing evdev device files.
const EV_DEV_DIR: &str = "/dev/input";

/// A keycode.
pub type Keycode = u8;

/// Enumeration of mouse button.
#[derive(Debug)]
pub enum MouseButton {
	/// Left click.
	Button1,
	/// Right click.
	Button2,
	/// Middle click.
	Button3,
	/// Scroll up.
	Button4,
	/// Scroll down.
	Button5,
}

// TODO Specify units in doc
/// An enumeration of input actions.
#[derive(Debug)]
pub enum Input {
	// TODO Specify keycodes
	/// Keyboard key press.
	KeyPress(Keycode),
	/// Keyboard key release.
	KeyRelease(Keycode),

	/// Moving the cursor relative to the previous position.
	RelativeMove {
		/// The X delta relative to the previous position.
		delta_x: i32,
		/// The Y delta relative to the previous position.
		delta_y: i32,
	},

	/// Moving the cursor to an absolute position.
	AbsoluteMove {
		/// The X position.
		x: u32,
		/// The Y position.
		y: u32,
	},

	/// Mouse button press.
	ButtonPress(MouseButton),
	/// Mouse button release.
	ButtonRelease(MouseButton),
	// TODO touchpad
}

impl TryFrom<EvDevInputEvent> for Input {
	type Error = ();

	fn try_from(ev: EvDevInputEvent) -> Result<Self, Self::Error> {
		// TODO
		println!("input: {} {} {}", ev.r#type, ev.code, ev.value);
		Err(())
	}
}

/// Structure managing input devices.
pub struct InputManager {
	/// The list of devices.
	devs: Vec<InputDevice>,
}

impl InputManager {
	/// Creates a new instance.
	///
	/// The function registers devices to the given poll handler in order to wake it up when a
	/// device is ready for reading.
	pub fn new(poll: &mut PollHandler) -> io::Result<Self> {
		let mut devs = vec![];
		for ent in fs::read_dir(EV_DEV_DIR)? {
			let ent = ent?;
			let ent_type = ent.file_type()?;
			if ent_type.is_dir() {
				continue;
			}

			let path = ent.path();
			let result = InputDevice::from_path(&path);

			let dev = match result {
				Ok(dev) => {
					println!("Acquired input: {}", path.display());
					dev
				}

				Err(e) => {
					eprintln!("Cannot acquire input `{}`: {}", path.display(), e);
					continue;
				}
			};

			poll.add_fd(&dev);
			devs.push(dev);
		}

		// TODO Init inotify (hotplug)

		Ok(Self {
			devs,
		})
	}

	/// Consumes and returns the next input. If no input is available, the function returns None.
	pub fn next(&mut self) -> io::Result<Option<Input>> {
		// TODO Clean and Optimize

		let mut poll_handler = PollHandler::new();
		for d in &mut self.devs {
			println!("-> {}", d.as_raw_fd());
			poll_handler.add_fd(d);
		}
		let fds = poll_handler.poll();
		println!("=> {:?}", fds);

		for d in &mut self.devs {
			if !fds.iter().filter(|f| **f == d.as_raw_fd()).next().is_some() {
				continue;
			}

			if let Some(i) = d.next()? {
				if let Ok(i) = i.try_into() {
					return Ok(Some(i));
				}
			}
		}

		Ok(None)
	}
}
