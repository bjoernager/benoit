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

use crate::benoit::application::Application;

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (f32, f32, f32) {
	return if saturation <= 0.0 {
		let value = value.min(1.0);

		(value, value, value)
	} else {
		let h = hue % 1.0 * 6.0;
		let s = saturation.min(1.0);
		let v = value.min(1.0);

		let f = h % 1.0;
		let p = v * (1.0 - s);
		let q = v * (1.0 - s * f);
		let t = v * (1.0 - s * (1.0 - f));

		match h.trunc() as u8 {
			0x0   => (v, t, p),
			0x1   => (q, v, p),
			0x2   => (p, v, t),
			0x3   => (p, q, v),
			0x4   => (t, p, v),
			0x5   => (v, p, q),
			value => unreachable!(),
		}
	};
}

impl Application {
	pub fn colour(&self, buffer: &mut [u8], iter_count_buffer: &[u32], square_dist_buffer: &[f32]) {
		let canvas_size = self.canvas_width * self.canvas_width;

		for pixel in 0x0..canvas_size {
			let iter_count = iter_count_buffer[pixel as usize];

			let distance = square_dist_buffer[pixel as usize].sqrt();

			let factor_range = 16.0_f32.min(self.max_iter_count as f32);
			let factor = (iter_count as f32 + 1.0 - distance.log2().log2()) / factor_range % 1.0;

			let (red, green, blue) = if iter_count != self.max_iter_count {
				hsv_to_rgb(factor, 7.0 / 8.0, 7.0 / 8.0)
			} else {
				(0.0, 0.0, 0.0)
			};

			let red   = (red   * 255.0).round() as u8;
			let green = (green * 255.0).round() as u8;
			let blue  = (blue  * 255.0).round() as u8;

			buffer[pixel as usize * 0x3]       = red;
			buffer[pixel as usize * 0x3 + 0x1] = green;
			buffer[pixel as usize * 0x3 + 0x2] = blue;
		}
	}
}
