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

extern crate sdl2;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;

pub mod initialise;
pub mod poll_events;
pub mod render;
pub mod render_row;
pub mod run;

pub struct Application {
	sdl:       Sdl,
	sdl_video: VideoSubsystem,
	canvas:    WindowCanvas,

	thread_count: u32,

	canvas_width:  u32,
	canvas_height: u32,
	scale:         u32,

	position_x:              f64,
	position_y:              f64,
	zoom:                    f64,
	maximum_iteration_count: u32,

	do_render: bool,
}
