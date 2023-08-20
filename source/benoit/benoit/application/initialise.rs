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

impl Application {
	pub fn initialise() -> Application {
		let canvas_width  = 0x400;
		let canvas_height = 0x400;

		let sdl       = sdl2::init().expect("unable to initialise sdl2");
		let sdl_video = sdl.video().expect("unable to initialise video");

		let window = sdl_video.window("Benoit", canvas_width, canvas_height).position_centered().build().expect("unable to open window");

		let canvas = window.into_canvas().build().expect("unable to create canvas");

		return Application {
			sdl:       sdl,
			sdl_video: sdl_video,
			canvas:    canvas,

			canvas_width:  canvas_width,
			canvas_height: canvas_height,

			position_x:          0.0,
			position_y:          0.0,
			zoom:                1.0,
			max_iteration_count: 0xFF,

			do_render: true,
		};
	}
}
