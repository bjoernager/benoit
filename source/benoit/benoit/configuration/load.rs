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

use crate::benoit::PRECISION;
use crate::benoit::configuration::Configuration;
use crate::benoit::fractal::FractalKind;
use crate::benoit::image::ImageFormat;
use crate::benoit::palette::Palette;

extern crate rug;
extern crate toml;

use rug::Float;
use std::fs::read;
use std::str::FromStr;
use toml::{Table, Value};

impl Configuration {
	#[must_use]
	pub fn load(path: &str) -> Result<Configuration, String> {
		eprintln!("loading configuration at \"{path}\"");

		let mut configuration = Configuration::default();

		let configuration_text = match read(path) {
			Ok( content) => String::from_utf8_lossy(&content).to_string(),
			Err(_)       => return Err("unable to read configuration file".to_string()),
		};

		let base_table = match Table::from_str(configuration_text.as_str()) {
			Ok( table) => table,
			Err(_)     => return Err("unable to parse configuration".to_string()),
		};

		let start_table = get_table(&base_table, "start")?;
		let stop_table  = get_table(&base_table, "stop")?;

		get_integer(&mut configuration.thread_count, &base_table, "thread_count")?;

		get_boolean(&mut configuration.fractal.inverse, &base_table, "inverse")?;
		get_boolean(&mut configuration.fractal.julia,   &base_table, "julia")?;

		get_integer(&mut configuration.canvas_width,  &base_table, "canvas_width")?;
		get_integer(&mut configuration.canvas_height, &base_table, "canvas_height")?;

		if let Some(start_table) = start_table {
			get_integer( &mut configuration.start_frame,          &start_table, "frame")?;
			get_bigfloat(&mut configuration.start_centre_real,    &start_table, "real")?;
			get_bigfloat(&mut configuration.start_centre_imag,    &start_table, "imaginary")?;
			get_bigfloat(&mut configuration.start_extra_real,     &start_table, "extra_real")?;
			get_bigfloat(&mut configuration.start_extra_imag,     &start_table, "extra_imaginary")?;
			get_bigfloat(&mut configuration.start_zoom,           &start_table, "zoom")?;
			get_integer( &mut configuration.start_max_iter_count, &start_table, "maximum_iteration_count")?;
			get_float(   &mut configuration.start_colour_range,   &start_table, "colour_range")?;
		}

		if let Some(stop_table) = stop_table {
			get_integer( &mut configuration.stop_frame,          &stop_table, "frame")?;
			get_bigfloat(&mut configuration.stop_centre_real,    &stop_table, "real")?;
			get_bigfloat(&mut configuration.stop_centre_imag,    &stop_table, "imaginary")?;
			get_bigfloat(&mut configuration.stop_extra_real,     &stop_table, "extra_real")?;
			get_bigfloat(&mut configuration.stop_extra_imag,     &stop_table, "extra_imaginary")?;
			get_bigfloat(&mut configuration.stop_zoom,           &stop_table, "zoom")?;
			get_integer( &mut configuration.stop_max_iter_count, &stop_table, "maximum_iteration_count")?;
			get_float(   &mut configuration.stop_colour_range,   &stop_table, "colour_range")?;
		}

		// Strings:
		if let Some(name) = get_string(&base_table, "fractal")? {
			configuration.fractal.kind = FractalKind::from_str(name.as_str())?;
		}

		if let Some(name) = get_string(&base_table, "palette")? {
			configuration.palette = Palette::from_str(name.as_str())?;
		}

		if let Some(path) = get_string(&base_table, "dump_path")? {
			configuration.dump_path = path.clone();
		}

		if let Some(name) = get_string(&base_table, "image_format")? {
			configuration.image_format = ImageFormat::from_str(name.as_str())?;
		}

		return Ok(configuration);
	}
}

fn get_value<'a>(table: &'a Table, name: &str) -> Option<&'a Value> {
	if !table.contains_key(name) { return None };

	return Some(&table[name]);
}

fn get_table<'a>(table: &'a Table, name: &str) -> Result<Option<&'a Table>, String> {
	return match get_value(table, name) {
		Some(Value::Table(table)) => Ok(Some(table)),
		Some(_)                   => Err(format!("\"{name}\" should be a section")),
		_                         => Ok(None),
	};
}

fn get_boolean(buffer: &mut bool, table: &Table, name: &str) -> Result<(), String> {
	match get_value(table, name) {
		Some(Value::Boolean(value)) => *buffer = *value,
		Some(_)                     => return Err(format!("\"{name}\" should be a boolean")),
		_                           => {},
	};
	return Ok(())
}

fn get_integer(buffer: &mut u32, table: &Table, name: &str) -> Result<(), String> {
	match get_value(table, name) {
		Some(Value::Integer(value)) => *buffer = (*value) as u32,
		Some(_)                     => return Err(format!("\"{name}\" should be an integer")),
		_                           => {},
	};
	return Ok(())
}

fn get_float(buffer: &mut f32, table: &Table, name: &str) -> Result<(), String> {
	match get_value(table, name) {
		Some(Value::Float(value)) => *buffer = (*value) as f32,
		Some(_)                   => return Err(format!("\"{name}\" should be a float")),
		_                         => {},
	};
	return Ok(())
}

fn get_bigfloat(buffer: &mut Float, table: &Table, name: &str) -> Result<(), String> {
	match get_value(table, name) {
		Some(Value::String(string)) => {
			*buffer = match Float::parse(string) {
				Ok(value) => Float::with_val(PRECISION, value),
				_         => return Err(format!("invalid format of \"{name}\"")),
			}
		},
		Some(_) => return Err(format!("\"{name}“ should be a quoted float")),
		_       => {},
	};
	return Ok(())
}

fn get_string(table: &Table, name: &str) -> Result<Option<String>, String> {
	return match get_value(table, name) {
		Some(Value::String(value)) => Ok(Some(value.clone())),
		Some(_)                    => Err(format!("\"{name}\" should be a string")),
		_                          => Ok(None),
	};
}
