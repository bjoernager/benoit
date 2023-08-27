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
	pub fn render_row(data: &mut [u32], y: u32, canvas_width: u32, canvas_height: u32, center_real: Float, center_imaginary: Float, zoom: Float, maximum_iteration_count: u32) {
		for x in 0x0..canvas_width {
			let canvas_width  = Float::with_val(PRECISION, canvas_width);
			let canvas_height = Float::with_val(PRECISION, canvas_height);

			let x_float = Float::with_val(PRECISION, x);
			let y_float = Float::with_val(PRECISION, y);

			// Re(c) = (x-canvas_width/2)/canvas_width*4/zoom+Re(z)
			let ca = {
				let tmp0 = Float::with_val(PRECISION, &canvas_width / 2.0);

				let mut ca = Float::with_val(PRECISION, &x_float - &tmp0);
				ca /= &canvas_width;
				ca *= 4.0;
				ca /= &zoom;
				ca += &center_real;

				ca
			};

			// Im(c) = (x-canvas_height/2)/canvas_height*4/zoom+Im(z)
			let cb = {
				let tmp0 = Float::with_val(PRECISION, &canvas_height / 2.0);

				let mut cb = Float::with_val(PRECISION, &y_float - &tmp0);
				cb /= &canvas_height;
				cb *= 4.0;
				cb /= &zoom;
				cb += &center_imaginary;

				cb
			};

			// Formaly, the initial condition that
			//
			// z = 0
			//
			// may be skipped as the first two iterations will
			// always be
			//
			// z0 = 0
			// z1 = z0^2+c = 0^2+c = 0+c = c
			//
			// That is, the result of the second iteration will
			// always be the value of (c).
			let mut za = Float::with_val(PRECISION, &ca);
			let mut zb = Float::with_val(PRECISION, &cb);

			let mut iteration_count: u32 = 0x0;
			while {
				let square_distance = Float::with_val(PRECISION, &za * &za + &zb * &zb);
				square_distance <= 4.0 && iteration_count < maximum_iteration_count
			} {
				{
					// The Mandelbrot Set (M) is defined as the set of values in the complex plane where the iterating function
					//
					// z = z^2+c
					//
					// stays bounded: I.e. the absolute value of (z) stays bounded:
					//
					// abs(z) = sqrt(Re(z)^2+Im(z)^2) <= 2^2 = 4

					let za_temporary = Float::with_val(PRECISION, &za);

					// We can calculate the square of a complex number (z) as:
					//
					// z^2 = (a+ib)^2 = (a+ib)(a+ib) = a^2+iab+iab-b^2 = a^2-b^2+2iab

					za = za.square();
					za -= &zb * &zb;
					za += &ca;

					zb *= &za_temporary;
					zb *= 2.0;
					zb += &cb;
				}

				iteration_count += 0x1;
			}

			unsafe { *data.get_unchecked_mut(x as usize) = iteration_count }
		}
	}
}
