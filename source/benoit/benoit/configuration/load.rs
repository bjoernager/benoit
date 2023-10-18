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
	pub fn load(path: &str) -> Configuration {
		eprintln!("loading configuration at \"{path}\"");

		let mut configuration = Configuration::default();

		let configuration_text = match read(path) {
			Ok(content) => String::from_utf8_lossy(&content).to_string(),
			Err(..)     => panic!("unable to read configuration file"),
		};

		let configuration_table = Table::from_str(configuration_text.as_str()).expect("unable to parse configuration");

		get_integer(&mut configuration.thread_count, &configuration_table, "thread_count");

		if let Some(name) = get_string(&configuration_table, "fractal") {
			configuration.fractal.set_kind(match name.as_str() {
				"burningship" => FractalKind::BurningShip,
				"mandelbrot"  => FractalKind::Mandelbrot,
				"multibrot3"  => FractalKind::Multibrot3,
				"multibrot4"  => FractalKind::Multibrot4,
				"tricorn"     => FractalKind::Tricorn,
				name          => panic!("invalid fractal kind \"{name}\""),
			});
		}

		{
			let mut inverse = false;
			get_boolean(&mut inverse, &configuration_table, "inverse");
			configuration.fractal.set_inverse(inverse);
		}

		get_integer(&mut configuration.canvas_width,  &configuration_table, "canvas_width");
		get_integer(&mut configuration.canvas_height, &configuration_table, "canvas_height");
		get_integer(&mut configuration.frame_start,   &configuration_table, "frame_start");
		get_integer(&mut configuration.frame_stop,    &configuration_table, "frame_stop");

		get_bigfloat(&mut configuration.centre_real, &configuration_table, "real");
		get_bigfloat(&mut configuration.centre_imag, &configuration_table, "imaginary");
		get_bigfloat(&mut configuration.extra_real,  &configuration_table, "extra_real");
		get_bigfloat(&mut configuration.extra_imag,  &configuration_table, "extra_imaginary");
		get_bigfloat(&mut configuration.zoom,        &configuration_table, "zoom");

		get_integer(&mut configuration.max_iter_count, &configuration_table, "maximum_iteration_count");

		if let Some(name) = get_string(&configuration_table, "palette") {
			configuration.palette = match name.as_str() {
				"ancient"   => Palette::Ancient,
				"fire"      => Palette::Fire,
				"greyscale" => Palette::Greyscale,
				"hsv"       => Palette::Hsv,
				"lch"       => Palette::Lch,
				"sapphire"  => Palette::Sapphire,
				name        => panic!("invalid palette \"{name}\""),
			};
		}

		get_float(&mut configuration.colour_range, &configuration_table, "colour_range");

		if let Some(path) = get_string(&configuration_table, "dump_path") {
			configuration.dump_path = path.clone();
		}

		if let Some(name) = get_string(&configuration_table, "image_format") {
			configuration.image_format = match name.as_str() {
				"png"  => ImageFormat::Png,
				"webp" => ImageFormat::Webp,
				name   => panic!("invalid image format \"{name}\""),
			};
		}

		match check_configuration(&configuration) {
			Err(message) => panic!("invalid configuration: {message}"),
			_            => {},
		}

		return configuration;
	}
}

fn check_configuration(configuration: &Configuration) -> Result<(), &str> {
	// We allow thread counts of zero as those signal
	// automatic thread count detection.

	if configuration.canvas_width == 0x0 {
		return Err("only non-zero values for canvas_width are allowed");
	}

	if configuration.scale == 0x0 {
		return Err("only non-zero values for scale are allowed");
	}

	if configuration.frame_start > configuration.frame_stop {
		return Err("frame_start may not be greater than frame_stop");
	}

	if configuration.max_iter_count == 0x0 {
		return Err("only non-zero values for maximum_iteration_count are allowed");
	}

	return Ok(());
}

fn get_value<'a>(table: &'a Table, name: &str) -> Option<&'a Value> {
	if !table.contains_key(name) { return None };

	return Some(&table[name]);
}

fn get_boolean(buffer: &mut bool, table: &Table, name: &str) {
	match get_value(table, name) {
		Some(Value::Boolean(value)) => *buffer = *value,
		Some(_)                     => panic!("\"{name}\" should be a boolean"),
		_                           => {},
	};
}

fn get_integer(buffer: &mut u32, table: &Table, name: &str) {
	match get_value(table, name) {
		Some(Value::Integer(value)) => *buffer = (*value) as u32,
		Some(_)                     => panic!("\"{name}\" should be an integer"),
		_                           => {},
	};
}

fn get_float(buffer: &mut f32, table: &Table, name: &str) {
	match get_value(table, name) {
		Some(Value::Float(value)) => *buffer = (*value) as f32,
		Some(_)                   => panic!("\"{name}\" should be a float"),
		_                         => {},
	};
}

fn get_bigfloat(buffer: &mut Float, table: &Table, name: &str) {
	return match get_value(table, name) {
		Some(Value::String(string)) => {
			*buffer = match Float::parse(string) {
				Ok(value) => Float::with_val(PRECISION, value),
				_         => panic!("invalid format of \"{name}\""),
			}
		},
		Some(_) => panic!("\"{name}“ should be a quoted float"),
		_ => {},
	};
}

fn get_string(table: &Table, name: &str) -> Option<String> {
	return match get_value(table, name) {
		Some(Value::String(value)) => Some(value.clone()),
		Some(_)                    => panic!("\"{name}\" should be a string"),
		_                          => None,
	};
}
