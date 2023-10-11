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

extern crate ctor;
extern crate enum_iterator;

use ctor::ctor;
use enum_iterator::{all, Sequence};
use std::mem::transmute;

mod paint;

#[derive(Clone, Copy, Sequence)]
#[repr(u8)]
pub enum Palette {
	Ancient,
	Fire,
	Greyscale,
	Hsv,
	Lch,
	Sapphire,
}

impl Palette {
	const MIN: Self = Palette::Ancient;
	const MAX: Self = Palette::Sapphire;

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
		return &*self.mut_data();
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

	#[must_use]
	fn mut_data(self) -> &'static mut PaletteData {
		return unsafe { match self {
			Palette::Ancient   => &mut DATA_ANCIENT,
			Palette::Fire      => &mut DATA_FIRE,
			Palette::Greyscale => &mut DATA_GREYSCALE,
			Palette::Hsv       => &mut DATA_HSV,
			Palette::Lch       => &mut DATA_LCH,
			Palette::Sapphire  => &mut DATA_SAPPHIRE,
		} };
	}
}

// We would like to precalculate the palettes at
// compile-time, but Rust does not support
// floating-point arithmetic there.

pub const PALETTE_DATA_LENGTH: usize = 0x1000;
pub type PaletteData = [(f32, f32, f32); PALETTE_DATA_LENGTH];

const fn default_palette_data() -> PaletteData {
	return [(0.0, 0.0, 0.0); PALETTE_DATA_LENGTH];
}

static mut DATA_ANCIENT:   PaletteData = default_palette_data();
static mut DATA_FIRE:      PaletteData = default_palette_data();
static mut DATA_GREYSCALE: PaletteData = default_palette_data();
static mut DATA_HSV:       PaletteData = default_palette_data();
static mut DATA_LCH:       PaletteData = default_palette_data();
static mut DATA_SAPPHIRE:  PaletteData = default_palette_data();

#[ctor]
fn calculate_palettes() {
	for palette in all::<Palette>() {
		let data     = palette.mut_data();
		let function = palette.function();

		for index in 0x0..PALETTE_DATA_LENGTH {
			let factor = index as f32 / PALETTE_DATA_LENGTH as f32;

			let (red, green, blue) = function(factor);

			data[index as usize] = (red, green, blue);
		}
	}
}
