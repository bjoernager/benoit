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

use crate::benoit::VERSION;
use crate::benoit::video::Video;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::BlendMode;

impl Video {
	#[must_use]
	pub fn initialise(canvas_width: u32, canvas_height: u32, scale: u32) -> Result<Video, String> {
		let sdl = match sdl2::init() {
			Ok( sdl) => sdl,
			Err(_)   => return Err("unable to initialise sdl2".to_string()),
		};

		let sdl_video = match sdl.video() {
			Ok( video) => video,
			Err(_)     => return Err("unable to initialise video".to_string()),
		};

		let window_title = format!("BENO\u{CE}T {:X}.{:X}.{:X}", VERSION.0, VERSION.1, VERSION.2);

		let mut window_builder = sdl_video.window(window_title.as_str(), canvas_width * scale, canvas_height * scale);
		window_builder.position_centered();

		let window = match window_builder.build() {
			Ok( window) => window,
			Err(_)      => return Err("unable to open window".to_string()),
		};

		let mut canvas = match window.into_canvas().build() {
			Ok( canvas) => canvas,
			Err(_)      => return Err("unable to build canvas".to_string()),
		};

		canvas.set_blend_mode(BlendMode::Blend);

		// We only want to scale the render, not the
		// feedback, so we can't use SDL's scaling feature.

		let clear_colour = Color::RGB(0x00, 0x00, 0x00);
		canvas.set_draw_color(clear_colour);
		canvas.clear();
		canvas.present();

		return Ok(Video {
			sdl:       sdl,
			sdl_video: sdl_video,
			canvas:    canvas,
		});
	}
}
