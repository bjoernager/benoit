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

pub mod from_str;

mod data;
mod paint;

pub use data::fill_palettes;

pub const PALETTE_DATA_LENGTH: usize = 0x1000;
pub type PaletteData = [(f32, f32, f32); PALETTE_DATA_LENGTH];

#[derive(Clone, Copy, Sequence)]
#[repr(u8)]
pub enum Palette {
	Simple,
	Twilight,
	Fire,
	Greyscale,
	Ruby,
	Emerald,
	Sapphire,
	Hsv,
	Lch,
}

impl Palette {
	pub const NUM: usize = Self::Lch as usize + 0x1;

	#[must_use]
	pub fn name(self) -> &'static str {
		return match self {
			Palette::Emerald   => "emerald",
			Palette::Fire      => "fire",
			Palette::Greyscale => "greyscale",
			Palette::Hsv       => "hsv",
			Palette::Lch       => "lch",
			Palette::Ruby      => "ruby",
			Palette::Sapphire  => "sapphire",
			Palette::Simple    => "simple",
			Palette::Twilight  => "twilight",
		};
	}

	#[must_use]
	pub fn data(self) -> &'static PaletteData {
		// This is safe as we return it as immutable.
		return unsafe { &*self.mut_data() };
	}

	pub fn cycle(&mut self, direction: i8) {
		let raw = *self as i16 + direction as i16;

		const NUM: isize = Palette::NUM as isize;
		let new: u8 = match raw as isize {
			-0x1 => (Self::NUM - 0x1) as u8,
			NUM  => 0x0,
			_    => raw as u8,
		};

		*self = unsafe { transmute(new) };
	}

	#[must_use]
	fn function(self) -> fn(f32) -> (f32, f32, f32) {
		return match self {
			Palette::Emerald   => paint::emerald,
			Palette::Fire      => paint::fire,
			Palette::Greyscale => paint::greyscale,
			Palette::Hsv       => paint::hsv,
			Palette::Lch       => paint::lch,
			Palette::Ruby      => paint::ruby,
			Palette::Sapphire  => paint::sapphire,
			Palette::Simple    => paint::simple,
			Palette::Twilight  => paint::twilight,
		};
	}
}
