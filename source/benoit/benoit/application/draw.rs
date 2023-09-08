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
use crate::benoit::application::{Application, PreviousPosition};

extern crate rug;
extern crate sdl2;

use rug::Float;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl Application {
	pub fn draw(&mut self, image: &[u8], previous_position: &PreviousPosition) {
		let canvas_size = self.canvas_height * self.canvas_width;

		for pixel in 0x0..canvas_size {
			let y = pixel as u32 / self.canvas_width;
			let x = pixel as u32 - y * self.canvas_width;

			let colour = {
				let red   = image[pixel as usize * 0x3];
				let green = image[pixel as usize * 0x3 + 0x1];
				let blue  = image[pixel as usize * 0x3 + 0x2];

				Color::RGB(red, green, blue)
			};

			let rectangle = Rect::new(
				(x * self.scale) as i32,
				(y * self.scale) as i32,
				self.scale,
				self.scale
			);

			self.video.as_mut().unwrap().canvas.set_draw_color(colour);
			self.video.as_mut().unwrap().canvas.fill_rects(&[rectangle]).unwrap();
		}

		if !self.julia {
			let viewport_colour = Color::RGB(0xFF, 0x0, 0x0);

			let canvas_width = {
				let mut canvas_width = Float::with_val(PRECISION, self.canvas_width);
				canvas_width *= self.scale;

				canvas_width
			};

			let viewport = {
				let ((offset_x, offset_y), width) = {
					let zoom_ratio = Float::with_val(PRECISION, &self.zoom / &previous_position.zoom);

					let mut width = Float::with_val(PRECISION, 1.0 / &zoom_ratio);

					let mut offset_x = Float::with_val(PRECISION, 1.0 - &width);
					let mut offset_y = Float::with_val(PRECISION, 1.0 - &width);

					offset_x /= 2.0;
					offset_y /= 2.0;

					offset_x *= &canvas_width;
					offset_y *= &canvas_width;
					width    *= &canvas_width;

					((offset_x.to_f32().round() as i32, offset_y.to_f32().round() as i32), width.to_f32().round() as u32)
				};

				Rect::new(
					offset_x,
					offset_y,
					width,
					width,
				)
			};

			self.video.as_mut().unwrap().canvas.set_draw_color(viewport_colour);
			self.video.as_mut().unwrap().canvas.draw_rects(&[viewport]).unwrap();
		}

		self.video.as_mut().unwrap().canvas.present();
	}
}
