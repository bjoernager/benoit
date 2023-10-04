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

use crate::benoit::ImageFormat;
use crate::benoit::factorisation::Factorisation;
use crate::benoit::fractal::Fractal;
use crate::benoit::palette::Palette;
use crate::benoit::rendering::Rendering;
use crate::benoit::render::{FactoriserFunction, IteratorFunction, PaletteFunction, RowRenderer};
use crate::benoit::video::Video;

extern crate rug;

use rug::Float;

pub mod allocate_buffers;
pub mod animate;
pub mod colour;
pub mod colour_row;
pub mod drop;
pub mod dump;
pub mod handle_keys;
pub mod image_filename;
pub mod initialise;
pub mod interactive;
pub mod poll_events;
pub mod render;
pub mod run;
pub mod still;

pub struct App {
	#[allow(dead_code)]
	thread_count: u32,

	fractal:   Fractal,
	rendering: Rendering,

	canvas_width:  u32,
	canvas_height: u32,
	scale:         u32,
	frame_count:   u32,

	centre_real: Float,
	centre_imag: Float,
	zoom:        Float,

	multibrot_exponent: f32,

	max_iter_count: u32,

	factorisation: Factorisation,
	palette:       Palette,
	colour_range:  f32,

	dump_path:    String,
	image_format: ImageFormat,

	video: Option<Video>,

	interactive:         bool,
	do_render:           bool,
	do_textual_feedback: bool,

	row_renderer:      RowRenderer,
	iterator_function: IteratorFunction,

	factoriser:       FactoriserFunction,
	palette_function: PaletteFunction,
}
