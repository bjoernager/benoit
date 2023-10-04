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

use crate::benoit::{FeedbackInfo, PRECISION, width_height_ratio};
use crate::benoit::video::Video;

extern crate rug;
extern crate sdl2;

use rug::Float;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl Video {
	pub fn draw_translation_feedback(&mut self, canvas_width: u32, canvas_height: u32, scale: u32, feedback_info: &FeedbackInfo) {
		let (width_ratio, height_ratio) = width_height_ratio(canvas_width, canvas_height);

		let canvas_width  = Float::with_val(PRECISION, canvas_width  * scale);
		let canvas_height = Float::with_val(PRECISION, canvas_height * scale);

		let viewport = {
			let (offset_x, offset_y, width, height) = {
				let zoom_ratio = Float::with_val(PRECISION, feedback_info.next_zoom / feedback_info.prev_zoom);

				let mut width  = Float::with_val(PRECISION, 1.0 / &zoom_ratio);
				let mut height = Float::with_val(PRECISION, 1.0 / &zoom_ratio);

				// Remember that cartesian coordinates have an
				// inverted vertical axis compared to those of
				// SDL's coordinate system.

				let mut offset_x = feedback_info.next_centre_real.clone();
				let mut offset_y = Float::with_val(PRECISION, -feedback_info.next_centre_imag);

				offset_x -= feedback_info.prev_centre_real;
				offset_y += feedback_info.prev_centre_imag;

				offset_x /= 4.0;
				offset_y /= 4.0;

				offset_x *= feedback_info.prev_zoom;
				offset_y *= feedback_info.prev_zoom;

				offset_x /= width_ratio;
				offset_y /= height_ratio;

				let mut zoom_offset = Float::with_val(PRECISION, 1.0 - &width);
				zoom_offset /= 2.0;

				offset_x += &zoom_offset;
				offset_y += &zoom_offset;

				offset_x *= &canvas_width;
				offset_y *= &canvas_height;
				width    *= &canvas_width;
				height   *= &canvas_height;

				(offset_x.to_f32().round() as i32, offset_y.to_f32().round() as i32, width.to_f32().round() as u32, height.to_f32().round() as u32)
			};

			Rect::new(
				offset_x,
				offset_y,
				width,
				height,
			)
		};

		self.canvas.set_draw_color(Color::RGBA(0x0, 0x0, 0x0, 0x7F));
		self.canvas.fill_rect(viewport).unwrap();

		self.canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
		self.canvas.draw_rect(viewport).unwrap();
	}
}
