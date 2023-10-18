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
use crate::benoit::complex::Complex;
use crate::benoit::video::Video;

extern crate rug;

use rug::Float;

impl App {
	pub(super) fn draw_feedback(&self, video: &mut Video, prev_centre: &Complex, prev_zoom: &Float) {
		if {
			// Don't draw translation feedback if rendering a
			// Julia set or if we haven't done any viewport
			// translations.

			   &self.centre.real != &prev_centre.real
			|| &self.centre.imag != &prev_centre.imag
			|| &self.zoom        != prev_zoom
		}{
			video.draw_translation_feedback(self.canvas_width, self.canvas_height, self.scale, prev_centre, prev_zoom, &self.centre, &self.zoom);
		}

		if self.do_textual_feedback { video.draw_textual_feedback(&self.centre, &self.zoom, self.max_iter_count) };
	}
}