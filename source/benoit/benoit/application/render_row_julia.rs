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
	pub fn render_row_julia(data: Arc<RenderData>, y: u32, iterator: IteratorFunction) {
		let (iteration_count_buffer, square_distance_buffer) = unsafe { data.slice(y) };

		for x in 0x0..data.canvas_width {
			let canvas_width  = Float::with_val(PRECISION, data.canvas_width);
			let canvas_height = Float::with_val(PRECISION, data.canvas_height);

			let x_float = Float::with_val(PRECISION, x);
			let y_float = Float::with_val(PRECISION, y);

			// For more information, see render_row_normal.

			let ca = &data.real;
			let cb = &data.imaginary;

			// When rendering the Julia fractals, the value of
			// (c) remains constant throughout the entire
			// canvas. The value of (z) - however - takes the
			// position-determined value that (c) would've had.

			let mut za = {
				let tmp0 = Float::with_val(PRECISION, &canvas_width / 2.0);

				let mut za = Float::with_val(PRECISION, &x_float - &tmp0);
				za *= 4.0;
				za /= &canvas_width;

				za
			};

			let mut zb = {
				let tmp0 = Float::with_val(PRECISION, &canvas_height / 2.0);

				let mut zb = Float::with_val(PRECISION, &y_float - &tmp0);
				zb *= 4.0;
				zb /= &canvas_height;

				zb
			};

			let mut iteration_count: u32 = 0x0;
			let mut square_distance;
			while {
				square_distance = Float::with_val(PRECISION, &za * &za + &zb * &zb).to_f32();
				square_distance <= 256.0 && iteration_count < data.maximum_iteration_count
			} {
				iterator(&mut za, &mut zb, ca, cb);

				iteration_count += 0x1;
			}

			unsafe {
				*iteration_count_buffer.get_unchecked_mut(x as usize) = iteration_count;
				*square_distance_buffer.get_unchecked_mut(x as usize) = square_distance;
			}
		}
	}
}