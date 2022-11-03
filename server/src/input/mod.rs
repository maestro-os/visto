//! TODO doc

pub mod device;

// TODO Input device manager, using inotify to detect plug/unplug
// The manager must **poll** on inotify to avoid blocking the server. Thus, using the same poll
// handler as client sockets
