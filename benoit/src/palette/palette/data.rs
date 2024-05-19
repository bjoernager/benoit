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
use crate::palette::{Palette, PaletteData};

use enum_iterator::{all, cardinality};
use lazy_static::lazy_static;
use rayon::prelude::*;
use rug::Float;

impl Palette {
	#[must_use]
	pub fn data(self) -> &'static PaletteData {
		&DATA[self as usize]
	}
}

lazy_static! {
	static ref DATA: Box<[PaletteData]> = {
		const BUFFER_LENGTH: usize = 0x10000;

		let mut data = vec![PaletteData::new(BUFFER_LENGTH).unwrap(); cardinality::<Palette>()].into_boxed_slice();

		for palette in all::<Palette>() {
			#[allow(clippy::enum_glob_use)]
			use Palette::*;

			let colour_mapper = match palette {
				Emerald   => map_emerald,
				Fire      => map_fire,
				Glacier   => map_glacier,
				Greyscale => map_greyscale,
				Hsv       => map_hsv,
				Ink       => map_ink,
				Lch       => map_lch,
				Mask      => map_mask,
				Ruby      => map_ruby,
				Sapphire  => map_sapphire,
				Simple    => map_simple,
				Thunder   => map_thunder,
				Twilight  => map_twilight,
			};

			data
				.get_mut(palette as usize)
				.unwrap()
				.par_iter_mut()
				.enumerate()
				.for_each(move |(i, &mut (ref mut body_buffer, ref mut complement_buffer))| {
					const PRECISION: u32 = usize::BITS * 0x8;

					// We need to guarantee that
					let factor = Float::with_val(PRECISION, i) / BUFFER_LENGTH;

					let (body, complement) = colour_mapper(factor.to_f64());

					(*body_buffer, *complement_buffer) = (body.into(), complement.into());
				});
		}

		data
	};
}

/// Normalises to 0..1.
///
/// The given factor is clamped to the range 0..1 and
fn normalise_factor(mut factor: f64) -> f64 {
	factor = factor.clamp(0.0, 1.0);

	factor = if factor >= 1.0 / 2.0 {
		1.0 - factor
	} else {
		factor
	};

	factor * 2.0
}

#[must_use]
fn map_emerald(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		let factor = normalise_factor(factor);

		Colour::from_srgba(
			factor * factor,
			factor,
			factor * factor * factor,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_fire(mut factor: f64) -> (Colour, Colour) {
	// Classic colour palette, fixed (and smoothed)
	// from version ↋.

	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		let mut factor = factor;

		let (red, green, blue) = if factor <= 0.25 {
			factor *= 4.0;

			(factor, 0.0, 0.0)
		} else if factor <= 0.5 {
			factor -= 0.25;
			factor *= 4.0;

			(1.0, factor, 0.0)
		} else if factor <= 0.75 {
			factor -= 0.5;
			factor *= 4.0;

			(1.0, 1.0, factor)
		} else {
			factor -= 0.75;
			factor *= 4.0;
			factor  = 1.0 - factor;

			(factor, factor, factor)
		};

		Colour::from_srgba(
			red,
			green,
			blue,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_glacier(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		let mut factor = 1.0 - normalise_factor(factor);

		factor *= factor;

		Colour::from_rgba(
			(-factor).mul_add(1.0,  1.0),
			(-factor).mul_add(0.25, 1.0),
			1.0,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_greyscale(mut factor: f64) -> (Colour, Colour) {
	// Original palette function after the Rust-
	// rewrite (version 10).

	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		let factor = normalise_factor(factor);

		Colour::from_srgba(
			factor,
			factor,
			factor,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_hsv(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		//let mut factor = factor;

		Colour::from_hsva(
			factor,
			0.875,
			0.875,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_ink(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.5,
			0.5,
			0.625,
			1.0,
		).unwrap()
	};

	let complement = {
		let mut factor = normalise_factor(factor);

		factor *= factor;

		Colour::from_srgba(
			(factor).mul_add(0.5,   1.0),
			(factor).mul_add(0.5,   1.0),
			(factor).mul_add(0.375, 1.0),
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_lch(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		//let mut factor = factor;

		//// sRGB:
		//Colour::from_lcha(
		//	66.667,
		//	40.080,
		//	factor,
		//	1.0,
		//).unwrap()

		// DCI P3:
		Colour::from_lcha(
			66.667,
			52.650,
			factor,
			1.0,
		).unwrap()

		//// Rec2020:
		//Colour::from_lcha(
		//	66.667,
		//	55.630,
		//	factor,
		//	1.0,
		//).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_mask(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			1.0,
			0.0,
			factor,
			1.0,
		).unwrap()
	};

	let complement = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			factor,
			0.0,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_ruby(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		let factor = normalise_factor(factor);

		Colour::from_srgba(
			factor,
			factor * factor * factor,
			factor * factor,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_sapphire(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		let factor = normalise_factor(factor);

		Colour::from_srgba(
			factor * factor * factor,
			factor * factor,
			factor,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_simple(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		//let mut factor = factor;

		let red   = factor * 3.0 % 1.0;
		let green = factor * 5.0 % 1.0;
		let blue  = factor * 7.0 % 1.0;

		Colour::from_srgba(
			red,
			green,
			blue,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_thunder(mut factor: f64) -> (Colour, Colour) {
	// This palette is horrible and I hope to reimple-
	// ment it differently in the future.

	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		let mut factor = factor;

		let (red, green, blue) = if factor <= 0.5 {
			factor *= 2.0;

			(factor, factor, 1.0 - factor)
		} else {
			factor -= 0.5;
			factor *= 2.0;

			(1.0 - factor, 1.0 - factor, factor)
		};

		Colour::from_rgba(
			red,
			green,
			blue,
			1.0,
		).unwrap()
	};

	(body, complement)
}

#[must_use]
fn map_twilight(mut factor: f64) -> (Colour, Colour) {
	factor = factor.clamp(0.0, 1.0);

	let body = {
		//let mut factor = factor;

		Colour::from_srgba(
			0.0,
			0.0,
			0.0,
			1.0,
		).unwrap()
	};

	let complement = {
		//let mut factor = factor;

		let red   = 9.0  * (1.0 - factor) * factor         * factor         * factor;
		let green = 15.0 * (1.0 - factor) * (1.0 - factor) * factor         * factor;
		let blue  = 8.5  * (1.0 - factor) * (1.0 - factor) * (1.0 - factor) * factor;

		Colour::from_srgba(
			red,
			green,
			blue,
			1.0,
		).unwrap()
	};

	(body, complement)
}
