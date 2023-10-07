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
use crate::benoit::render::IteratorFunction;
use crate::benoit::render::render_data::RenderData;

extern crate rug;

use rug::{Assign, Float};
use rug::float::Special;

pub fn normal(data: &RenderData, x: u32, y: u32, iterator: IteratorFunction) -> (u32, f32) {
	let (centre_real, centre_imag, zoom, max_iter_count) = data.input();

	let (x_offset, y_offset, x_factor, y_factor) = data.consts();

	let x_temporary = (x as f32 + x_offset) * x_factor;
	let y_temporary = (y as f32 + y_offset) * y_factor;

	let ca = {
		let mut ca = Float::with_val(PRECISION, x_temporary / zoom);
		ca += centre_real;

		ca
	};

	let cb = {
		let mut cb = Float::with_val(PRECISION, y_temporary / zoom);
		cb -= centre_imag;

		cb
	};

	let inverse_factor = data.inverse_factor(&ca, &cb);

	let ca = ca * &inverse_factor;
	let cb = cb * &inverse_factor;

	let mut za = ca.clone();
	let mut zb = cb.clone();

	let mut za_prev = Float::with_val(PRECISION, Special::Nan);
	let mut zb_prev = Float::with_val(PRECISION, Special::Nan);

	let mut iter_count:  u32 = 0x1;
	let mut square_dist       = Float::with_val(PRECISION, Special::Nan);
	while {
		square_dist.assign(&za * &za + &zb * &zb);
		// Having a larger escape radius gives better
		// results with regard to smoothing.

		// Check if the value is periodic, i.e. its
		// sequence repeats.
		let periodic = za == za_prev && zb == zb_prev;
		if periodic { iter_count = max_iter_count }

		square_dist <= BAILOUT && iter_count < max_iter_count
	} {
		za_prev.assign(&za);
		zb_prev.assign(&zb);

		iterator(&mut za, &mut zb, &ca, &cb);

		iter_count += 0x1;
	}

	return (iter_count, square_dist.to_f32());
}
