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

use crate::benoit::palette::{Palette, PALETTE_DATA_LENGTH, PaletteData};

extern crate enum_iterator;

use enum_iterator::all;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

impl Palette {
	#[must_use]
	pub(super) unsafe fn mut_data(self) -> &'static mut PaletteData {
		return match self {
			Palette::Emerald   => &mut DATA_EMERALD,
			Palette::Fire      => &mut DATA_FIRE,
			Palette::Greyscale => &mut DATA_GREYSCALE,
			Palette::Hsv       => &mut DATA_HSV,
			Palette::Lch       => &mut DATA_LCH,
			Palette::Ruby      => &mut DATA_RUBY,
			Palette::Sapphire  => &mut DATA_SAPPHIRE,
			Palette::Twilight  => &mut DATA_TWILIGHT,
		};
	}
}

#[must_use]
const fn default_palette_data() -> PaletteData {
	return [(0.0, 0.0, 0.0); PALETTE_DATA_LENGTH];
}

static mut DATA_EMERALD:   PaletteData = default_palette_data();
static mut DATA_FIRE:      PaletteData = default_palette_data();
static mut DATA_GREYSCALE: PaletteData = default_palette_data();
static mut DATA_HSV:       PaletteData = default_palette_data();
static mut DATA_LCH:       PaletteData = default_palette_data();
static mut DATA_RUBY:      PaletteData = default_palette_data();
static mut DATA_SAPPHIRE:  PaletteData = default_palette_data();
static mut DATA_TWILIGHT:  PaletteData = default_palette_data();

static PALETTES_FILLED: AtomicBool = AtomicBool::new(false);

pub fn fill_palettes() {
	match PALETTES_FILLED.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
		Err(_) => panic!("palettes already filled"),
		_      => {},
	};

	// We would like to precalculate the palettes at
	// compile-time, but Rust does not allow
	// const floating-point arithmetic.

	eprint!("filling palettes...");
	let time = Instant::now();

	for palette in all::<Palette>() {
		let data     = unsafe { palette.mut_data() };
		let function = palette.function();

		for index in 0x0..PALETTE_DATA_LENGTH {
			let factor = index as f32 / PALETTE_DATA_LENGTH as f32;

			let (red, green, blue) = function(factor);

			data[index as usize] = (red, green, blue);
		}
	}

	eprintln!(" {:.3}ms", time.elapsed().as_micros() as f32 / 1000.0);
}
