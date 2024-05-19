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

use png::EncodingError;
use rug::float::ParseFloatError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;

/// Denotes an error.
#[derive(Debug)]
pub enum Error {
	BadComplexSeperator { expr: String },

	BigFloatParseFailure { source: ParseFloatError },

	FileCreationFailure { path: PathBuf },

	ImageEncodingFailure { source: EncodingError },

	MissingImaginaryUnit { expr: String },

	MissingRenderGeneration,

	MissingRenderPlot,

	NanColourParameter { name: &'static str },

	UnknownFractal { string: String },

	UnknownPalette { string: String },

	ZeroLengthPaletteData,

	ZeroLengthRender,
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		#[allow(clippy::enum_glob_use)]
		use Error::*;

		match *self {
			BadComplexSeperator { ref expr } => {
				write!(f, "bad complex seperator ('+' or '-') in expression \"{expr}\"")
			},

			BigFloatParseFailure { source } => {
				write!(f, "unable to parse bigfloat: \"{source}\"")
			},

			FileCreationFailure { ref path } => {
				write!(f, "unable to create file at \"{}\"", path.display())
			},

			ImageEncodingFailure { ref source } => {
				write!(f, "unable to encode image: \"{source}\"")
			},

			MissingImaginaryUnit { ref expr } => {
				write!(f, "missing imaginary unit 'i' in expression \"{expr}\"")
			},

			MissingRenderGeneration => {
				write!(f, "no render has been generated yet")
			},

			MissingRenderPlot => {
				write!(f, "no image has been plotted")
			},

			NanColourParameter { name } => {
				write!(f, "parameter `{name}` cannot be nan")
			},

			UnknownFractal { ref string } => {
				write!(f, "unknown fractal \"{string}\"")
			},

			UnknownPalette { ref string } => {
				write!(f, "unknown palette \"{string}\"")
			},

			ZeroLengthPaletteData => {
				write!(f, "palette data cannot have a length of zero")
			},

			ZeroLengthRender => {
				write!(f, "total render size must be non-zero")
			},
		}
	}
}

impl From<ParseFloatError> for Error {
	fn from(value: ParseFloatError) -> Self {
		Self::BigFloatParseFailure { source: value }
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		#[allow(clippy::enum_glob_use)]
		use Error::*;

		#[allow(clippy::wildcard_enum_match_arm)]
		match *self {
			BigFloatParseFailure { ref source } => Some(source),

			ImageEncodingFailure { ref source } => Some(source),

			_ => None,
		}
	}
}
