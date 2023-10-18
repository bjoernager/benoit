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

use crate::benoit::complex::Complex;
use crate::benoit::fractal::Fractal;
use crate::benoit::palette::Palette;

extern crate rug;

use rug::Float;

pub mod configure;
pub mod draw_feedback;
pub mod drop;
pub mod handle_keys;
pub mod poll_events;
pub mod print_controls;
pub mod render;
pub mod run;

pub struct App {
	// Configuration:
	fractal:  Fractal,

	canvas_width:  u32,
	canvas_height: u32,
	scale:         u32,

	centre: Complex,
	extra:  Complex,
	zoom:   Float,

	max_iter_count: u32,

	palette:      Palette,
	colour_range: f32,

	// Flags:
	do_render:           bool,
	do_textual_feedback: bool,
}
