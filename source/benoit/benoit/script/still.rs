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
	pub(super) fn still(&self) -> Result<(), String> {
		let frame_count = self.stop.frame - self.start.frame + 0x1;
		assert!(frame_count == 0x1);

		let mut render = Render::allocate(self.canvas_width, self.canvas_height)?;
		let mut image  = Image::allocate( self.canvas_width, self.canvas_height)?;

		const FRAME_NAME: &str = "render";

		eprintln!("");
		eprintln!("rendering still: the {}", self.fractal.name());
		eprintln!("    width:           {}", self.canvas_width);
		eprintln!("    height:          {}", self.canvas_height);
		eprintln!("    re(c):           {}", self.stop.centre.real);
		eprintln!("    im(c):           {}", self.stop.centre.imag);
		eprintln!("    re(w):           {}", self.stop.extra.real);
		eprintln!("    im(w):           {}", self.stop.extra.imag);
		eprintln!("    zoom:            {}", self.stop.zoom);
		eprintln!("    max. iter count: {}", self.stop.max_iter_count);
		eprintln!("    col. range:      {}", self.stop.colour_range);
		eprintln!("");

		Script::dump_frame(
			self.dump_path.as_str(),
			FRAME_NAME,
			&mut image,
			&mut render,
			self.fractal,
			self.palette,
			&self.stop.centre,
			&self.stop.extra,
			&self.stop.zoom,
			self.stop.max_iter_count,
			self.stop.colour_range,
			self.image_format,
		)?;

		return Ok(());
	}
}
