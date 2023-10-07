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

use crate::benoit::{ImageFormat, PRECISION};
use crate::benoit::fractal::Fractal;
use crate::benoit::palette::Palette;
use crate::benoit::rendering::Rendering;

extern crate rug;
extern crate toml;

use rug::Float;
use std::fs::read;
use std::str::FromStr;
use toml::{Table, Value};

pub struct Configuration {
	pub thread_count: u32,

	pub fractal:   Fractal,
	pub rendering: Rendering,

	pub canvas_width:  u32,
	pub canvas_height: u32,
	pub scale:         u32,
	pub frame_count:   u32,

	pub centre_real: Float,
	pub centre_imag: Float,
	pub zoom:        Float,

	pub max_iter_count: u32,

	pub inverse: bool,

	pub palette:      Palette,
	pub colour_range: f32,

	pub dump_path:    String,
	pub image_format: ImageFormat,

	pub interactive: bool,
}

impl Configuration {
	#[must_use]
	pub fn default() -> Configuration {
		return Configuration {
			thread_count: 0x0,

			fractal:   Fractal::Mandelbrot,
			rendering: Rendering::Normal,

			canvas_width:  0x100,
			canvas_height: 0xC0,
			scale:         0x2,
			frame_count:   0x10,

			centre_real: Float::with_val(PRECISION, 0.0),
			centre_imag: Float::with_val(PRECISION, 0.0),
			zoom:        Float::with_val(PRECISION, 1.0),

			max_iter_count: 0x100,

			inverse: false,

			palette:      Palette::Fire,
			colour_range: 64.0,

			dump_path:    "./render".to_string(),
			image_format: ImageFormat::Png,

			interactive: true,
		};
	}

	#[must_use]
	pub fn load(path: &str) -> Configuration {
		eprintln!("loading configuration at \"{path}\"");

		let mut configuration = Configuration::default();

		configuration.interactive = false;

		let configuration_text = match read(path) {
			Ok(content) => String::from_utf8_lossy(&content).to_string(),
			Err(..)     => panic!("unable to read configuration file"),
		};

		let configuration_table = Table::from_str(configuration_text.as_str()).expect("unable to parse configuration");

		get_integer(&mut configuration.thread_count, &configuration_table, "thread_count");

		if let Some(name) = get_string(&configuration_table, "fractal") {
			configuration.fractal = match name.as_str() {
				"burningship" => Fractal::BurningShip,
				"mandelbrot"  => Fractal::Mandelbrot,
				"multibrot3"  => Fractal::Multibrot3,
				"tricorn"     => Fractal::Tricorn,
				name          => panic!("invalid fractal kind \"{name}\""),
			}
		}

		if let Some(name) = get_string(&configuration_table, "rendering") {
			configuration.rendering = match name.as_str() {
				"julia"  => Rendering::Julia,
				"normal" => Rendering::Normal,
				name     => panic!("invalid rendering method \"{name}\""),
			};
		}

		get_integer(&mut configuration.canvas_width,  &configuration_table, "canvas_width");
		get_integer(&mut configuration.canvas_height, &configuration_table, "canvas_height");
		get_integer(&mut configuration.frame_count,   &configuration_table, "frame_count");

		get_bigfloat(&mut configuration.centre_real,    &configuration_table, "real");
		get_bigfloat(&mut configuration.centre_imag,    &configuration_table, "imaginary");
		get_bigfloat(&mut configuration.zoom,           &configuration_table, "zoom");

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
	} else if configuration.scale == 0x0 {
		return Err("only non-zero values for scale are allowed");
	} else if configuration.frame_count == 0x0 {
		return Err("only non-zero values for frame_count are allowed");
	} else if configuration.max_iter_count == 0x0 {
		return Err("only non-zero values for maximum_iteration_count are allowed");
	}

	return Ok(());
}

fn get_value<'a>(table: &'a Table, name: &str) -> Option<&'a Value> {
	if !table.contains_key(name) { return None };

	return Some(&table[name]);
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
