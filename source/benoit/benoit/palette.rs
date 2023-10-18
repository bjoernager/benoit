/*
	Copyright 2021, 2023 Gabriel Bjørnager Jensen.

	This file is part of Benoit.

	Benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero General Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU
	Affero General Public License along with Benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

extern crate enum_iterator;

use enum_iterator::Sequence;
use std::mem::transmute;

mod data;
mod paint;

pub use data::fill_palettes;

pub const PALETTE_DATA_LENGTH: usize = 0x1000;
pub type PaletteData = [(f32, f32, f32); PALETTE_DATA_LENGTH];

#[derive(Clone, Copy, Sequence)]
#[repr(u8)]
pub enum Palette {
	// Sorted according to date of addition.
	Ancient,
	Fire,
	Greyscale,
	Sapphire,
	Hsv,
	Lch,
}

impl Palette {
	const MIN: Self = Palette::Ancient;
	const MAX: Self = Palette::Lch;

	#[must_use]
	pub fn name(self) -> &'static str {
		return match self {
			Palette::Ancient   => "ancient",
			Palette::Fire      => "fire",
			Palette::Greyscale => "greyscale",
			Palette::Hsv       => "hsv",
			Palette::Lch       => "lch",
			Palette::Sapphire  => "sapphire",
		};
	}

	#[must_use]
	pub fn data(self) -> &'static PaletteData {
		// This is safe as we return it as immutable.
		return unsafe { &*self.mut_data() };
	}

	#[must_use]
	pub fn cycle(&self, direction: i8) -> Self {
		let raw = *self as i8 + direction;

		let new: Palette = if raw < 0x0 {
			Palette::MAX
		} else if raw > Palette::MAX as i8 {
			Palette::MIN
		} else {
			unsafe { transmute(raw) }
		};

		return new;
	}

	#[must_use]
	fn function(self) -> fn(f32) -> (f32, f32, f32) {
		return match self {
			Palette::Ancient   => paint::ancient,
			Palette::Fire      => paint::fire,
			Palette::Greyscale => paint::greyscale,
			Palette::Hsv       => paint::hsv,
			Palette::Lch       => paint::lch,
			Palette::Sapphire  => paint::sapphire,
		};
	}
}
