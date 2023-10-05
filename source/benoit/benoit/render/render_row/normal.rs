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

pub fn normal(data: Arc<RenderData>, y: u32, iterator: IteratorFunction) {
	let (iter_count_buffer, square_dist_buffer) = data.output_buffers(y);

	let (canvas_width, centre_real, centre_imag, zoom, max_iter_count) = data.input();

	let (x_offset, y_offset, x_factor, y_factor) = data.consts();

	for x in 0x0..canvas_width {
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

		let mut za = ca.clone();
		let mut zb = cb.clone();

		let mut za_prev = Float::with_val(PRECISION, Special::Nan);
		let mut zb_prev = Float::with_val(PRECISION, Special::Nan);

		let mut iter_count: u32 = 0x1;
		let mut square_dist;
		while {
			square_dist = Float::with_val(PRECISION, &za * &za + &zb * &zb).to_f32();
			// Having a larger escape radius gives better
			// results with regard to smoothing.

			// Check if the value is periodic, i.e. its
			// sequence repeats.
			let periodic = za == za_prev && zb == zb_prev;
			if periodic { iter_count = max_iter_count }

			square_dist <= 256.0 && iter_count < max_iter_count
		} {
			za_prev.assign(&za);
			zb_prev.assign(&zb);

			iterator(&mut za, &mut zb, &ca, &cb);

			iter_count += 0x1;
		}

		// Sacrifice safety for speed by removing bounds-
		// checking.
		unsafe {
			*iter_count_buffer.get_unchecked_mut( x as usize) = iter_count;
			*square_dist_buffer.get_unchecked_mut(x as usize) = square_dist;
		}
	}
}
