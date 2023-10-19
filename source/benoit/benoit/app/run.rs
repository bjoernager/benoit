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

use crate::benoit::app::App;
use crate::benoit::image::Image;
use crate::benoit::render::Render;
use crate::benoit::video::Video;

extern crate rug;

use rug::Assign;
use std::time::Instant;

impl App {
	#[must_use]
	pub fn run(mut self) -> i32 {
		let mut video = Video::initialise(self.canvas_width, self.canvas_height, self.scale);

		App::print_controls();

		let mut event_pump = video.sdl.event_pump().expect("unable to get event pump");

		let mut render = Render::allocate(self.canvas_width, self.canvas_height);
		let mut image  = Image::allocate( self.canvas_width, self.canvas_height);

		// Used for translation feedback:
		let mut prev_centre = self.centre.clone();
		let mut prev_zoom   = self.zoom.clone();

		loop {
			let frame_start = Instant::now();

			if self.poll_events(&mut event_pump) { break }

			if self.do_render {
				self.render(&mut render);

				prev_centre.assign(&self.centre);
				prev_zoom.assign(  &self.zoom);

				self.do_render = false;
			};

			image.colour(&render, self.palette, self.max_iter_count, self.colour_range);

			video.draw_image(&image, self.scale);
			self.draw_feedback(&mut video, &prev_centre, &prev_zoom);
			video.update();

			video.sync(&frame_start);
		}

		return 0x0;
	}
}