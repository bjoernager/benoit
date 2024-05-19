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

use crate::PRECISION;
use crate::complex::Complex;
use crate::fractal::Fractal;
use crate::render::{RawElement, Render, RenderConfig};

use rayon::prelude::*;
use rug::{Assign, Float};
use rug::float::{free_cache, FreeCache, Special as FloatSpecial};

// The GenerationCache structure saves some values
// which are needed at every point. As these va-
// lues are constant during renders, we share a si-
// ngle cache for efficiency's sake.

struct GenerationCache<'a> {
	pub fractal: &'a Fractal,
	pub inverse: bool,
	pub julia:   bool,

	pub max_iter_count: u64,

	pub centre: &'a Complex,
	pub seed:   &'a Complex,
	pub zoom:   &'a Float,
}

impl Render {
	/// Generates the render as defined by the provided configuration.
	///
	/// This populates (and allocates) the internal "raw" buffer, allowing for [`plot`](Self::plot) to be called.
	pub fn generate(&mut self, config: &RenderConfig) {
		let mut data = self.raw_data.take().map_or_else(|| self.allocate_buffer(), |d| d);

		// Account for non-square proportions. Each ratio
		// is the factor between that side's width and that
		// of the other side.
		let (width_ratio, height_ratio): (f64, f64) = {
			let width  = f64::from(self.size.0);
			let height = f64::from(self.size.1);

			if width > height {
				(1.0, height / width)
			} else {
				(width / height, 1.0)
			}
		};

		let cache = GenerationCache {
			fractal: &config.fractal,
			inverse: config.inverse,
			julia:   config.julia,

			max_iter_count: config.max_iter_count,
			centre:         &config.centre,
			seed:           &config.seed,
			zoom:           &config.zoom,
		};

		// This factor maps cavas coordinates from pixels
		// to natural units, keeping the canvas ratio in
		// mind.
		//
		// At default zoom (i.e. a factor of one), the
		// width of the entire set is (4) in natural units.
		// In pixels, this corresponds to render width (e.
		// g. 256 pixel).
		let x_factor = 4.0 / f64::from(self.size.0) * width_ratio;
		let y_factor = 4.0 / f64::from(self.size.1) * height_ratio;

		data
			.par_iter_mut()
			.enumerate()
			.for_each(|(index, point)| {
				// These conversions are infallible:
				let x = u32::try_from(index % self.size.0 as usize).unwrap();
				let y = u32::try_from(index / self.size.0 as usize).unwrap();

				// Convert the pixel coordinates to natural units.
				let x = f64::from(x).mul_add(x_factor, -2.0);
				let y = f64::from(y).mul_add(y_factor, -2.0);

				*point = generate_at_pixel(x, y, &cache);

				// This is recommended.
				free_cache(FreeCache::Local);
			});

		self.raw_data    = Some(data);
		self.last_config = Some(config.clone());
	}
}

fn generate_at_pixel(x: f64, y: f64, cache: &GenerationCache) -> RawElement {
	// Resize the viewfinder according to the zoom
	// level.

	let mut c = Complex::with_val(PRECISION, x / cache.zoom, y / cache.zoom);

	// Remember that pixel coordinates grow "down-
	// wards," whilst euclidian coordinates grop up-
	// wards.

	c.real += &cache.centre.real;
	c.imag -= &cache.centre.imag;

	if cache.inverse {
		let factor = Float::with_val(PRECISION, &c.real * &c.real + &c.imag * &c.imag).recip();

		c.real *= &factor;
		c.imag *= &factor;
	}

	// The seed is treated as the Julia set's equiva-
	// lent point on the complex plane. If not in "Ju-
	// lia mode," the seed is instead used for the
	// starting value of `z`.

	let (mut z, c) = if cache.julia {
		(c, cache.seed)
	} else {
		(cache.seed.clone(), &c)
	};

	// Used for periodicity checking.
	let mut prev_z = Complex::with_val(PRECISION, FloatSpecial::Nan, FloatSpecial::Nan);

	let mut iter_count  = 0x0u64;
	let mut square_dist = Float::with_val(PRECISION, FloatSpecial::Nan);

	while {
		square_dist.assign(&z.real * &z.real + &z.imag * &z.imag);

		// Check if the value is periodic, i.e. its
		// sequence repeats.

		if z == prev_z {
			iter_count = cache.max_iter_count;
		}

		// Having a larger escape radius gives better
		// results with regard to smoothing. Source: tests.

		square_dist <= 0x100 && iter_count < cache.max_iter_count
	} {
		prev_z.real.assign(&z.real);
		prev_z.imag.assign(&z.imag);

		(cache.fractal.iter)(&mut z, c);

		iter_count += 0x1;
	}

	let distance = square_dist.sqrt().to_f64();
	RawElement { iter_count, distance }
}
