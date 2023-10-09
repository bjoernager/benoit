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

use crate::benoit::{BAILOUT, PRECISION};
use crate::benoit::complex::Complex;
use crate::benoit::render::IteratorFunction;
use crate::benoit::render::render_data::RenderData;

extern crate rug;

use rug::{Assign, Float};
use rug::float::Special;

pub fn julia(data: &RenderData, x: u32, y: u32, iterator: IteratorFunction) -> (u32, f32) {
	// For more information, see render_point::normal.

	let (centre, extra, zoom, max_iter_count) = data.input();

	let (x_offset, y_offset, x_factor, y_factor) = data.consts();

	let x_temporary = (x as f32 + x_offset) * x_factor;
	let y_temporary = (y as f32 + y_offset) * y_factor;

	let c = extra;

	let mut z = {
		let mut a = Float::with_val(PRECISION, x_temporary / zoom);
		a += &centre.real;

		let mut b = Float::with_val(PRECISION, y_temporary / zoom);
		b -= &centre.imag;

		Complex {real: a, imag: b}
	};

	let inverse_factor = data.inverse_factor(&z);

	z.real *= &inverse_factor;
	z.imag *= &inverse_factor;

	let mut z_prev = Complex {
		real: Float::with_val(PRECISION, Special::Nan),
		imag: Float::with_val(PRECISION, Special::Nan),
	};

	let mut iter_count:  u32 = 0x1;
	let mut square_dist      = Float::with_val(PRECISION, Special::Nan);
	while {
		square_dist.assign(&z.real * &z.real + &z.imag * &z.imag);

		let periodic = z.real == z_prev.real && z.imag == z_prev.imag;
		if periodic { iter_count = max_iter_count }

		square_dist <= BAILOUT && iter_count < max_iter_count
	} {
		z_prev.assign(&z);

		iterator(&mut z, c);

		iter_count += 0x1;
	}

	return (iter_count, square_dist.to_f32());
}
