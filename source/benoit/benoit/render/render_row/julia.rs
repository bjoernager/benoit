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

use crate::benoit::PRECISION;
use crate::benoit::render::IteratorFunction;
use crate::benoit::render::render_data::RenderData;

extern crate rug;

use rug::{Assign, Float};
use rug::float::Special;
use std::sync::Arc;

pub fn julia(data: Arc<RenderData>, y: u32, iterator: IteratorFunction) {
	let (iter_count_buffer, square_dist_buffer) = data.output_buffers(y);

	let (canvas_width, centre_real, centre_imag, _zoom, max_iter_count) = data.input();

	let (x_offset, y_offset, x_factor, y_factor) = data.consts();

	for x in 0x0..canvas_width {
		// For more information, see render_row::normal.

		let x_temporary = (x as f32 + x_offset) * x_factor;
		let y_temporary = (y as f32 + y_offset) * y_factor;

		let ca = centre_real;
		let cb = centre_imag;

		// When rendering the Julia fractals, the value of
		// (c) remains constant throughout the entire
		// canvas. The value of (z) - however - takes the
		// position-determined value that (c) would've had.

		let mut za = Float::with_val(PRECISION, x_temporary);
		let mut zb = Float::with_val(PRECISION, y_temporary);

		let mut za_prev = Float::with_val(PRECISION, Special::Nan);
		let mut zb_prev = Float::with_val(PRECISION, Special::Nan);

		let mut iter_count: u32 = 0x1;
		let mut square_dist;
		while {
			square_dist = Float::with_val(PRECISION, &za * &za + &zb * &zb).to_f32();

			let periodic = za == za_prev && zb == zb_prev;
			if periodic { iter_count = max_iter_count }

			square_dist <= 256.0 && iter_count < max_iter_count
		} {
			za_prev.assign(&za);
			zb_prev.assign(&zb);

			iterator(&mut za, &mut zb, ca, cb);

			iter_count += 0x1;
		}

		unsafe {
			*iter_count_buffer.get_unchecked_mut( x as usize) = iter_count;
			*square_dist_buffer.get_unchecked_mut(x as usize) = square_dist;
		}
	}
}
