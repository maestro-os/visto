//! This module implements Graphics Contexts (GC).

use crate::protocol::error::Error;

/// TODO doc
#[derive(Clone)]
pub enum Function {
	/// TODO doc
	Clear,
	/// TODO doc
	And,
	/// TODO doc
	AndReverse,
	/// TODO doc
	Copy,
	/// TODO doc
	AndInverted,
	/// TODO doc
	NoOp,
	/// TODO doc
	Xor,
	/// TODO doc
	Or,
	/// TODO doc
	Nor,
	/// TODO doc
	Equiv,
	/// TODO doc
	Invert,
	/// TODO doc
	OrReverse,
	/// TODO doc
	CopyInverted,
	/// TODO doc
	OrInverted,
	/// TODO doc
	Nand,
	/// TODO doc
	Set,
}

impl TryFrom<u8> for Function {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Clear),
			1 => Ok(Self::And),
			2 => Ok(Self::AndReverse),
			3 => Ok(Self::Copy),
			4 => Ok(Self::AndInverted),
			5 => Ok(Self::NoOp),
			6 => Ok(Self::Xor),
			7 => Ok(Self::Or),
			8 => Ok(Self::Nor),
			9 => Ok(Self::Equiv),
			10 => Ok(Self::Invert),
			11 => Ok(Self::OrReverse),
			12 => Ok(Self::CopyInverted),
			13 => Ok(Self::OrInverted),
			14 => Ok(Self::Nand),
			15 => Ok(Self::Set),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum LineStyle {
	/// TODO doc
	Solid,
	/// TODO doc
	OnOffDash,
	/// TODO doc
	DoubleDash,
}

impl TryFrom<u8> for LineStyle {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Solid),
			1 => Ok(Self::OnOffDash),
			2 => Ok(Self::DoubleDash),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum CapStyle {
	/// TODO doc
	NotLast,
	/// TODO doc
	Butt,
	/// TODO doc
	Round,
	/// TODO doc
	Projecting,
}

impl TryFrom<u8> for CapStyle {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::NotLast),
			1 => Ok(Self::Butt),
			2 => Ok(Self::Round),
			3 => Ok(Self::Projecting),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum JoinStyle {
	/// TODO doc
	Solid,
	/// TODO doc
	OnOffDash,
	/// TODO doc
	DoubleDash,
}

impl TryFrom<u8> for JoinStyle {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Solid),
			1 => Ok(Self::OnOffDash),
			2 => Ok(Self::DoubleDash),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum FillStyle {
	/// TODO doc
	Solid,
	/// TODO doc
	Tiled,
	/// TODO doc
	Stippled,
	/// TODO doc
	OpaqueStippled,
}

impl TryFrom<u8> for FillStyle {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Solid),
			1 => Ok(Self::Tiled),
			2 => Ok(Self::Stippled),
			3 => Ok(Self::OpaqueStippled),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum FillRule {
	/// TODO doc
	EvenOdd,
	/// TODO doc
	Winding,
}

impl TryFrom<u8> for FillRule {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::EvenOdd),
			1 => Ok(Self::Winding),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum SubWindowMode {
	/// TODO doc
	ClipByChildren,
	/// TODO doc
	IncludeInferiors,
}

impl TryFrom<u8> for SubWindowMode {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::ClipByChildren),
			1 => Ok(Self::IncludeInferiors),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum ArcMode {
	/// TODO doc
	Chord,
	/// TODO doc
	PieSlice,
}

impl TryFrom<u8> for ArcMode {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Chord),
			1 => Ok(Self::PieSlice),

			_ => Err(Error::Value(v as _)),
		}
	}
}

/// Value of a graphics context.
#[derive(Clone)]
pub enum Value {
	/// TODO doc
	Function(Function),
	/// TODO doc
	PlaneMask(u32),
	/// TODO doc
	Foreground(u32),
	/// TODO doc
	Background(u32),
	/// TODO doc
	LineWidth(u16),
	/// TODO doc
	LineStyle(LineStyle),
	/// TODO doc
	CapStyle(CapStyle),
	/// TODO doc
	JoinStyle(JoinStyle),
	/// TODO doc
	FillStyle(FillStyle),
	/// TODO doc
	FillRule(FillRule),
	/// TODO doc
	Tile(u32),
	/// TODO doc
	Stipple(u32),
	/// TODO doc
	TileStippleXOrigin(i16),
	/// TODO doc
	TileStippleYOrigin(i16),
	/// TODO doc
	Font(u32),
	/// TODO doc
	SubwindowMode(SubWindowMode),
	/// TODO doc
	GraphicsExposures(u8),
	/// TODO doc
	ClipXOrigin(i16),
	/// TODO doc
	ClipYOrigin(i16),
	/// TODO doc
	ClipMask(u32),
	/// TODO doc
	DashOffset(u16),
	/// TODO doc
	Dashes(u8),
	/// TODO doc
	ArcMode(ArcMode),
}

/// Structure representing a graphics context.
#[derive(Clone)]
pub struct GC {
	/// The ID of the drawable.
	pub drawable: u32,
	/// The list of values.
	pub values: Vec<Value>,
}
