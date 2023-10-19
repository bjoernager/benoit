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

use crate::benoit::{BAILOUT_DISTANCE, PRECISION};
use crate::benoit::complex::Complex;
use crate::benoit::fractal::{Fractal, IteratorFunction};
use crate::benoit::render::Render;
use crate::benoit::render_data::RenderData;

extern crate rayon;
extern crate rug;

use rayon::prelude::*;
use rug::{Assign, Float};
use rug::float::Special;

impl Render {
	pub fn render(
		&mut self,
		fractal:        Fractal,
		centre:         &Complex,
		zoom:           &Float,
		extra:          &Complex,
		max_iter_count: u32,
	) {
		assert!(max_iter_count > 0x0);

		let data = RenderData::new(
			&mut self.data[..],
			self.canvas_width,
			self.canvas_height,
			centre.clone(),
			extra.clone(),
			zoom.clone(),
			max_iter_count,
			fractal.inverse,
			fractal.julia,
		);

		let iterator = fractal.iterator();

		self.data.par_iter_mut().for_each(|point| {
			let (x, y) = data.coordinate(point);
			*point = render_point(&data, x, y, iterator);
		});

		self.info = Some((fractal, max_iter_count));
	}
}

fn render_point(data: &RenderData, x: u32, y: u32, iterator: IteratorFunction) -> (u32, f32) {
	let (centre, extra, zoom, max_iter_count, inverse, julia) = data.input();

	let (x_offset, y_offset, x_factor, y_factor) = data.consts();

	let x_temporary = (x as f32 + x_offset) * x_factor;
	let y_temporary = (y as f32 + y_offset) * y_factor;

	let mut z = {
		let mut a = Float::with_val(PRECISION, x_temporary / zoom);
		a += &centre.real;

		let mut b = Float::with_val(PRECISION, y_temporary / zoom);
		b -= &centre.imag;

		Complex {real: a, imag: b}
	};

	if inverse {
		let mut factor_inverse = Float::with_val(PRECISION, &z.real * &z.real);
		factor_inverse += &z.imag * &z.imag;
		factor_inverse.recip_mut();

		z.real *= &factor_inverse;
		z.imag *= factor_inverse;
	}

	// We can optimise pertubation by adding w (extra)
	// to c.
	let c = match julia {
		false => Complex { real: Float::with_val(PRECISION, &z.real + &extra.real), imag: Float::with_val(PRECISION, &z.imag + &extra.imag) },
		true  => extra.clone(),
	};

	let mut z_prev = Complex {
		real: Float::with_val(PRECISION, Special::Nan),
		imag: Float::with_val(PRECISION, Special::Nan),
	};

	let mut iter_count:  u32 = 0x1;
	let mut square_dist      = Float::with_val(PRECISION, Special::Nan);
	while {
		square_dist.assign(&z.real * &z.real + &z.imag * &z.imag);
		// Having a larger escape radius gives better
		// results with regard to smoothing.

		// Check if the value is periodic, i.e. its
		// sequence repeats.
		let periodic = z.real == z_prev.real && z.imag == z_prev.imag;
		if periodic { iter_count = max_iter_count }

		square_dist <= BAILOUT_DISTANCE && iter_count < max_iter_count
	} {
		z_prev.assign(&z);

		iterator(&mut z, &c);

		iter_count += 0x1;
	}

	return (iter_count, square_dist.to_f32());
}
