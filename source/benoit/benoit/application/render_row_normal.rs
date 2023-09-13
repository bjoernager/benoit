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
use crate::benoit::application::Application;
use crate::benoit::iteration::IteratorFunction;
use crate::benoit::render_data::RenderData;

extern crate rug;

use rug::Float;
use std::sync::Arc;

impl Application {
	pub fn render_row_normal(data: Arc<RenderData>, y: u32, iterator: IteratorFunction) {
		let (iter_count_buffer, square_dist_buffer) = unsafe { data.slice(y) };

		for x in 0x0..data.canvas_width {
			let canvas_width = Float::with_val(PRECISION, data.canvas_width);

			let x_float = Float::with_val(PRECISION, x);
			let y_float = Float::with_val(PRECISION, y);

			let ca = {
				let tmp0 = Float::with_val(PRECISION, &canvas_width / 2.0);

				let mut ca = Float::with_val(PRECISION, &x_float - &tmp0);
				ca *= 4.0;
				ca /= &canvas_width;
				ca /= &data.zoom;
				ca += &data.centre_real;

				ca
			};

			let cb = {
				let tmp0 = Float::with_val(PRECISION, &canvas_width / 2.0);

				let mut cb = Float::with_val(PRECISION, &y_float - &tmp0);
				cb *= 4.0;
				cb /= &canvas_width;
				cb /= &data.zoom;
				cb += &data.centre_imag;

				cb
			};

			let mut za = Float::with_val(PRECISION, &ca);
			let mut zb = Float::with_val(PRECISION, &cb);

			let mut iter_count: u32 = 0x0;
			let mut square_dist;
			while {
				square_dist = Float::with_val(PRECISION, &za * &za + &zb * &zb).to_f32();
				// Having a larger escape radius gives better
				// results with regard to smoothing.
				square_dist <= 256.0 && iter_count < data.max_iter_count
			} {
				iterator(&mut za, &mut zb, &ca, &cb);

				iter_count += 0x1;
			}

			unsafe {
				*iter_count_buffer.get_unchecked_mut( x as usize) = iter_count;
				*square_dist_buffer.get_unchecked_mut(x as usize) = square_dist;
			}
		}
	}
}
