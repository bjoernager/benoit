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

use crate::error::Error;
use crate::palette::{Palette, PaletteData};
use crate::render::{ImageElement, RawElement, Render};

use rayon::prelude::*;

struct PlottingCache<'a> {
	pub max_iter_count: u64,
	pub colour_range:   f64,

	pub exponent: f64,

	pub palette_data: &'a PaletteData,
}

impl Render {
	/// Plots the image according to the raw render data.
	///
	/// This populates (and allocates) the internal image buffer, allowing for [`dump_image`](Self::dump_image) to be called.
	///
	/// # Errors
	///
	/// Yields an error if no raw render data has been generated (as by using [`generate`](Self::generate)).
	pub fn plot(&mut self, palette: Palette, colour_range: f64) -> Result<(), Error> {
		let mut data = self.image_data.take().map_or_else(|| self.allocate_buffer(), |d| d);

		let config = self.last_config()
			.ok_or_else(|| Error::MissingRenderGeneration)?;

		let cache = PlottingCache {
			max_iter_count: config.max_iter_count,
			colour_range,

			exponent: config.fractal.exponent,

			palette_data: palette.data(),
		};

		// We have checked if there is a generation.
		let raw_data = self.raw_data.as_mut().unwrap();

		let get_colour = |data: RawElement| -> ImageElement {
			if data.iter_count == cache.max_iter_count {
				let factor = data.distance;

				cache.palette_data[factor].0
			} else {
				let mut factor = data.iter_count as f64;
				factor += 1.0 - data.distance.ln().log(cache.exponent);
				factor /= cache.colour_range;
				factor %= 1.0;

				cache.palette_data[factor].1
			}
		};

		data
			.par_iter_mut()
			.enumerate()
			.for_each(move |(index, point)| {
				let data = raw_data[index];

				*point = get_colour(data);
			});

		self.image_data = Some(data);

		Ok(())
	}
}
