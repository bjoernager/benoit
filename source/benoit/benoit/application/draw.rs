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

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl Application {
	pub fn draw(&mut self, data: &mut [u32], image: &mut [u8]) {
		let canvas_size = self.canvas_height * self.canvas_width;

		self.render(&mut data[..], &self.center_real, &self.center_imaginary, &self.zoom, self.maximum_iteration_count, &self.julia_real, &self.julia_imaginary);
		self.colour(&mut image[..], &data[..]);

		for pixel in 0x0..canvas_size {
			let y = pixel as u32 / self.canvas_width;
			let x = pixel as u32 - y * self.canvas_width;

			let colour = {
				let red   = image[pixel as usize * 0x3];
				let green = image[pixel as usize * 0x3 + 0x1];
				let blue  = image[pixel as usize * 0x3 + 0x2];

				Color::RGB(red, green, blue)
			};

			self.video.as_mut().unwrap().canvas.set_draw_color(colour);

			let rectangle = Rect::new(
				(x * self.scale) as i32,
				(y * self.scale) as i32,
				self.scale,
				self.scale
			);
			self.video.as_mut().unwrap().canvas.fill_rects(&[rectangle]).unwrap();
		}

		self.video.as_mut().unwrap().canvas.present();
	}
}
