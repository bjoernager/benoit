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

use crate::benoit::render::PaletteFunction;
use crate::benoit::render::paint;

use std::mem::transmute;
use std::ops::Add;

#[derive(Clone, Copy)]
pub enum Palette {
	Ancient,
	Fire,
	Greyscale,
	Hsv,
	Lch,
	Sapphire,
}

impl Palette {
	const MAX: u8 = Palette::Sapphire as u8;

	pub fn get_name(self) -> &'static str {
		return match self {
			Palette::Ancient   => "ancient",
			Palette::Fire      => "fire",
			Palette::Greyscale => "greyscale",
			Palette::Hsv       => "hsv",
			Palette::Lch       => "lch",
			Palette::Sapphire  => "sapphire",
		};
	}

	pub fn get_function(self) -> PaletteFunction {
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

impl Add<i8> for Palette {
	type Output = Palette;

	fn add(self, direction: i8) -> Self {
		assert!(direction != 0x0);

		let raw = self as i8 + direction;
		let raw: u8 = if raw < 0x0 {
			Palette::MAX
		} else if raw > Palette::MAX as i8 {
			0x0
		} else {
			raw as u8
		};

		let new: Self = unsafe { transmute(raw) };

		return new;
	}
}
