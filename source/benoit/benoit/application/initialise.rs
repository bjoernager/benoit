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
use crate::benoit::configuration::Configuration;

use std::env::args;

impl Application {
	pub fn initialise() -> Application {
		let mut arguments = args();

		let configuration = match arguments.nth(0x1) {
			Some(path) => Configuration::load(path.as_str()),
			None       => Configuration::default(),
		};

		let sdl       = sdl2::init().expect("unable to initialise sdl2");
		let sdl_video = sdl.video().expect("unable to initialise video");

		let window = sdl_video.window("Benoit", configuration.canvas_width * configuration.scale, configuration.canvas_height * configuration.scale).position_centered().build().expect("unable to open window");

		let canvas = window.into_canvas().build().expect("unable to create canvas");

		return Application {
			thread_count: configuration.thread_count,

			canvas_width:  configuration.canvas_width,
			canvas_height: configuration.canvas_height,
			scale:         configuration.scale,

			position_x:              configuration.position_x,
			position_y:              configuration.position_y,
			zoom:                    configuration.zoom,
			maximum_iteration_count: configuration.maximum_iteration_count,

			dump_path: configuration.dump_path,

			sdl:       sdl,
			sdl_video: sdl_video,
			canvas:    canvas,

			do_draw: true,
		};
	}
}
