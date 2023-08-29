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

extern crate rug;

use rug::Float;

impl Application {
	pub fn render_row_julia(data: &mut [u32], y: u32, canvas_width: u32, canvas_height: u32, center_real: Float, center_imaginary: Float, zoom: Float, maximum_iteration_count: u32, julia_real: Float, julia_imaginary: Float) {
		for x in 0x0..canvas_width {
			let canvas_width  = Float::with_val(PRECISION, canvas_width);
			let canvas_height = Float::with_val(PRECISION, canvas_height);

			let x_float = Float::with_val(PRECISION, x);
			let y_float = Float::with_val(PRECISION, y);

			// The Julia set of the Mandelbrot set is similar
			// but not quite identical: The start value of (z)
			// is now relative to the canvas and (c) is
			// constant corresponds to a point in the
			// Mandelbrot Set.

			let ca = &julia_real;
			let cb = &julia_imaginary;

			// Re(z) = (x-canvas_width/2)*4/canvas_width/zoom+Re(z)
			let mut za = {
				let tmp0 = Float::with_val(PRECISION, &canvas_width / 2.0);

				let mut za = Float::with_val(PRECISION, &x_float - &tmp0);
				za *= 4.0;
				za /= &canvas_width;
				za /= &zoom;
				za += &center_real;

				za
			};

			// Im(Z) = (x-canvas_height/2)*4/canvas_height/zoom+Im(z)
			let mut zb = {
				let tmp0 = Float::with_val(PRECISION, &canvas_height / 2.0);

				let mut zb = Float::with_val(PRECISION, &y_float - &tmp0);
				zb *= 4.0;
				zb /= &canvas_height;
				zb /= &zoom;
				zb += &center_imaginary;

				zb
			};

			let mut iteration_count: u32 = 0x0;
			while {
				let square_distance = Float::with_val(PRECISION, &za * &za + &zb * &zb);
				square_distance <= 4.0 && iteration_count < maximum_iteration_count
			} {
				{
					// The overall iterations of the Julia of M are
					// identical to those of M:
					//
					// z = z^2+c
					//
					// with only the initial value of (z) and (c)
					// differing.

					let za_temporary = Float::with_val(PRECISION, &za);

					za = za.square();
					za -= &zb * &zb;
					za += ca;

					zb *= &za_temporary;
					zb *= 2.0;
					zb += cb;
				}

				iteration_count += 0x1;
			}

			unsafe { *data.get_unchecked_mut(x as usize) = iteration_count }
		}
	}
}
