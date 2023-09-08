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

use crate::benoit::fractal::Fractal;
use crate::benoit::iteration::IteratorFunction;
use crate::benoit::render_data::RenderData;
use crate::benoit::video::Video;

extern crate rug;

use rug::Float;
use std::sync::Arc;

pub mod animate;
pub mod colour;
pub mod draw;
pub mod dump;
pub mod get_iterator_function;
pub mod get_row_renderer;
pub mod handle_keys;
pub mod initialise;
pub mod r#loop;
pub mod poll_events;
pub mod render_row_julia;
pub mod render_row_normal;
pub mod render;
pub mod run;

pub type RowRenderer = fn(Arc<RenderData>, u32, IteratorFunction);

// We pass this struct to draw the offset feedback.
// Currently unused, however.
#[allow(dead_code)]
pub struct PreviousPosition {
	centre_real: Float,
	centre_imag: Float,
	zoom:        Float,
}

pub struct Application {
	thread_count: u32,

	fractal: Fractal,
	julia:   bool,

	canvas_width:  u32,
	canvas_height: u32,
	scale:         u32,
	frame_count:   u32,

	centre_real:    Float,
	centre_imag:    Float,
	zoom:           Float,
	max_iter_count: u32,

	dump_path: String,

	video: Option<Video>,

	interactive: bool,
	do_render:   bool,
	do_dump:     bool,

	row_renderer:      RowRenderer,
	iterator_function: IteratorFunction,
}
