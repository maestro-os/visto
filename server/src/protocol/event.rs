//! This module implements events.

// TODO

/// Enumeration of events.
pub enum Event {
	KeyPress,
	KeyRelease,
	ButtonPress,
	ButtonRelease,
	MotionNotify,

	EnterNotify,
	LeaveNotify,

	FocusIn,
	FocusOut,

	KeymapNotify,
	Expose,
	GraphicsExposure,
	NoExposure,
	VisibilityNotify,
	CreateNotify,
	DestroyNotify,
	UnmapNotify,
	MapNotify,
	MapRequest,
	ReparentNotify,
	ConfigureNotify,
	GravityNotify,
	ResizeRequest,
	ConfigureRequest,
	CirculateNotify,
	CirculateRequest,
	PropertyNotify,
	SelectionClear,
	SelectionRequest,
	SelectionNotify,
	ColormapNotify,
	MappingNotify,
	ClientMessage,
}
