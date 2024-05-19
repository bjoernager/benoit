/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::colour::Colour;

/// Type for points on the image canvas.
///
/// This type is used in the image buffer of the [`Render`](crate::render::Render) type, as well as in the [`PaletteData`](crate::palette::PaletteData) type.
///
/// As PNG specifies network byte order, the fields of this structure must also be big-endian.
/// It is up to the user of the type to ensure that this is done, perhaps using the [`from_ne`](crate::render::ImageElement::from_ne) method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct ImageElement {
	/// The red channel.
	pub red: u16,

	/// The green channel.
	pub green: u16,

	/// The blue channel.
	pub blue: u16,

	/// The alpha channel.
	pub alpha: u16,
}

impl ImageElement {
	/// Zips the provided parameters.
	///
	/// All four values are converted to big-endian before zipping.
	#[must_use]
	pub const fn from_ne(red: u16, green: u16, blue: u16, alpha: u16) -> Self {
		// Remember network byte order.
		Self {
			red:   red.to_be(),
			green: green.to_be(),
			blue:  blue.to_be(),
			alpha: alpha.to_be(),
		}
	}

	/// Unzips the structure into its parameters.
	///
	/// All values are converted to the native endian, if needed.
	#[must_use]
	pub const fn to_ne(self) -> (u16, u16, u16, u16) {
		(
			u16::from_be(self.red),
			u16::from_be(self.green),
			u16::from_be(self.blue),
			u16::from_be(self.alpha),
		)
	}
}

impl Default for ImageElement {
	#[inline(always)]
	fn default() -> Self { Self::from_ne(0x0, 0x0, 0x0, 0x0) }
}

impl From<Colour> for ImageElement {
	fn from(value: Colour) -> Self {
		let (red, green, blue, alpha) = value.to_srgba16();
		Self::from_ne(red, green, blue, alpha)
	}
}

#[allow(clippy::fallible_impl_from)]
impl From<ImageElement> for Colour {
	fn from(value: ImageElement) -> Self {
		let (red, green, blue, alpha) = value.to_ne();

		Self::from_rgba(
			f32::from(red)   / f32::from(u16::MAX),
			f32::from(green) / f32::from(u16::MAX),
			f32::from(blue)  / f32::from(u16::MAX),
			f32::from(alpha) / f32::from(u16::MAX),
		).unwrap() // This is infallible.
	}
}
