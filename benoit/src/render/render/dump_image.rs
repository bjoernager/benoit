/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::error::Error;
use crate::render::Render;

use png::{
	BitDepth,
	ColorType,
	Compression,
	Encoder,
	SrgbRenderingIntent,
};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;


impl Render {
	/// Dumps the stored image at the provided location.
	/// # Errors
	///
	/// Yields an error if dump fails, which could include:
	///
	/// If no image has been plotted (as by [`Render::plot`]), an [`Err`] object is also returned instead.
	pub fn dump_image(&self, path: &Path) -> Result<(), Error> {
		let Some(data) = self.image_as_bytes() else {
			return Err(Error::MissingRenderPlot);
		};

		let Ok(file) = File::create(path) else {
			return Err(Error::FileCreationFailure { path: path.to_owned() })
		};

		let w = BufWriter::new(file);

		let mut encoder = Encoder::new(w, self.size.0, self.size.1);
		encoder.set_color(ColorType::Rgba);
		encoder.set_depth(BitDepth::Sixteen);
		encoder.set_srgb(SrgbRenderingIntent::Perceptual);

		let compression = if cfg!(debug_assertions) {
			Compression::Fast
		} else {
			Compression::Best
		};
		encoder.set_compression(compression);

		let mut w = encoder.write_header().map_err(
			|e| Error::ImageEncodingFailure { source: e }
		)?;

		w.write_image_data(data).map_err(
			|e| Error::ImageEncodingFailure { source: e }
		)?;

		Ok(())
	}
}
