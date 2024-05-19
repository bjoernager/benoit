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

use benoit::error::Error as LibError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::path::PathBuf;
use toml::de::Error as TomlError;

/// Denotes an error.
#[derive(Debug)]
pub enum Error {
	BenoitError { source: LibError },

	ConfigParseFailure { path: PathBuf, source: TomlError },

	ConfigReadFailure { path: PathBuf, source: IoError },

	InvalidConfig { message: String },

	InvalidKeyframe { message: String },

	InvalidPath { path: String, source: IoError },

	FieldLowerBounds { name: String, value: String, limit: String },

	FieldUpperBounds { name: String, value: String, limit: String },

	InfiniteField { name: String },

	MissingConfigField { name: String },

	MissingConfigPath,

	NegativeField { name: String },

	NonNumberField { name: String },

	TooManyCliArguments,

	WrongFieldType { name: String, ok_type: &'static str },
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		#[allow(clippy::enum_glob_use)]
		use Error::*;

		match *self {
			BenoitError { ref source } => {
				write!(f, "{source}")
			},

			ConfigParseFailure { ref path, ref source } => {
				write!(f, "unable to parse configuration at \"{}\": \"{source}\"", path.display())
			}

			ConfigReadFailure { ref path, ref source } => {
				write!(f, "unable to read configuration at \"{}\": \"{source}\"", path.display())
			},

			FieldLowerBounds { ref name, ref value, ref limit } => {
				write!(f, "field `{name}` cannot be less than ({limit}) but was ({value})")
			},

			FieldUpperBounds { ref name, ref value, ref limit } => {
				write!(f, "field `{name}` cannot be greater than ({limit}) but was ({value})")
			},

			InfiniteField { ref name } => {
				write!(f, "field `{name}` must be finite (was infinite)")
			},

			InvalidConfig { ref message } => {
				write!(f, "invalid configuration: {message}")
			},

			InvalidKeyframe { ref message } => {
				write!(f, "invalid keyframe: {message}")
			},

			InvalidPath { ref path, ref source } => {
				write!(f, "invalid path \"{path}\": \"{source}\"")
			},

			MissingConfigField { ref name } => {
				write!(f, "missing configuration field `{name}`")
			},

			MissingConfigPath => {
				write!(f, "missing configuration path")
			},

			NegativeField { ref name } => {
				write!(f, "field `{name}` cannot be negative")
			},

			NonNumberField { ref name } => {
				write!(f, "field `{name}` must be a number (was NaN)")
			},

			TooManyCliArguments => {
				write!(f, "too many arguments provided on the command line")
			},

			WrongFieldType { ref name, ok_type } => {
				write!(f, "type of field `{name} should'be been `{ok_type}`")
			},
		}
	}
}

impl From<LibError> for Error {
	fn from(value: LibError) -> Self {
		Self::BenoitError { source: value }
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		#[allow(clippy::enum_glob_use)]
		use Error::*;

		#[allow(clippy::match_same_arms, clippy::wildcard_enum_match_arm)]
		match *self {
			BenoitError { ref source } => Some(source),

			ConfigParseFailure { ref source, .. } => Some(source),

			ConfigReadFailure { ref source, .. } => Some(source),

			InvalidPath { ref source, .. } => Some(source),

			_ => None,
		}
	}
}
