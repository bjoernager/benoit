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
use crate::benoit::fractal::Fractal;

extern crate rug;
extern crate toml;

use rug::Float;
use std::fs::read;
use std::str::FromStr;
use toml::{Table, Value};

impl Configuration {
	pub fn load(path: &str) -> Configuration {
		eprintln!("loading configuration at \"{path}\"");

		let mut configuration = Configuration::default();

		configuration.interactive = false;

		let configuration_text = match read(path) {
			Ok(content) => String::from_utf8_lossy(&content).to_string(),
			Err(..)     => {
				eprintln!("unable to read configuration file");
				return configuration;
			},
		};

		let configuration_table = Table::from_str(configuration_text.as_str()).expect("unable to parse configuration");

		let get_integer = |buffer: &mut u32, table: &Table, name: &str| {
			if !table.contains_key(name) { return }

			match &configuration_table[name] {
				Value::Integer(value) => *buffer = *value as u32,
				_                     => panic!("mismatched type of {name}"),
			};
		};

		let get_float = |buffer: &mut Float, table: &Table, name: &str| {
			if !table.contains_key(name) { return }

			match &configuration_table[name] {
				Value::String(string) => {
					*buffer = match Float::parse(string) {
						Ok(value) => Float::with_val(PRECISION, value),
						_         => panic!("invalid format of {name}"),
					}
				},
				_ => panic!("mismatched type of {name}"),
			};
		};

		let get_string = |table: &Table, name: &str| -> Option<&String> {
			if !table.contains_key(name) { return None }

			match &configuration_table[name] {
				Value::String(value) => return Some(value),
				_                    => panic!("mismatched type of {name}"),
			};
		};

		get_integer(&mut configuration.thread_count, &configuration_table, "thread_count");

		configuration.fractal = if let Some(name) = get_string(&configuration_table, "fractal") {
			match name.as_str() {
				"burningship" => Fractal::BurningShip,
				"mandelbrot"  => Fractal::Mandelbrot,
				"tricorn"    => Fractal::Tricorn,
				name          => panic!("invalid fractal name {name}"),
			}
		} else {
			configuration.fractal
		};

		get_integer(&mut configuration.canvas_width,  &configuration_table, "canvas_width");
		get_integer(&mut configuration.canvas_height, &configuration_table, "canvas_height");
		get_integer(&mut configuration.scale,         &configuration_table, "scale");
		get_integer(&mut configuration.frame_count,   &configuration_table, "frame_count");

		get_float(  &mut configuration.centre_real,             &configuration_table, "real");
		get_float(  &mut configuration.centre_imaginary,        &configuration_table, "imaginary");
		get_float(  &mut configuration.zoom,                    &configuration_table, "zoom");
		get_integer(&mut configuration.maximum_iteration_count, &configuration_table, "maximum_iteration_count");

		return configuration;
	}
}
