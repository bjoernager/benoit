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

use crate::benoit::image::Image;
use crate::benoit::render::Render;
use crate::benoit::script::Script;

impl Script {
	#[must_use]
	pub(super) fn still(&self) -> i32 {
		let mut render = Render::allocate(self.canvas_width, self.canvas_height);
		let mut image  = Image::allocate( self.canvas_width, self.canvas_height);

		const FRAME_NAME: &str = "render";

		Script::dump_frame(
			self.dump_path.as_str(),
			FRAME_NAME,
			&mut image,
			&mut render,
			self.fractal,
			self.palette,
			&self.centre,
			&self.extra,
			&self.zoom,
			self.max_iter_count,
			self.colour_range,
			self.image_format,
		);

		return 0x0;
	}
}
