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

use crate::benoit::configuration::Configuration;

extern crate toml;

use std::fs::read;
use std::str::FromStr;
use toml::{Table, Value};

use std::time::Duration;

impl Configuration {
	pub fn load(path: &str) -> Configuration {
		eprintln!("loading configuration at \"{path}\"");

		let mut configuration = Configuration::default();

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

			match configuration_table[name] {
				Value::Integer(value) => *buffer = value as u32,
				_                     => panic!("mismatched type for {name}"),
			};
		};

		let get_float = |buffer: &mut f64, table: &Table, name: &str| {
			if !table.contains_key(name) { return }

			match configuration_table[name] {
				Value::Float(value) => *buffer = value,
				_                   => panic!("mismatched type for {name}"),
			};
		};

		get_integer(&mut configuration.canvas_width,  &configuration_table, "canvas_width");
		get_integer(&mut configuration.canvas_height, &configuration_table, "canvas_height");
		get_integer(&mut configuration.scale,         &configuration_table, "scale");

		get_float(&mut configuration.position_x,  &configuration_table, "position_x");
		get_float(&mut configuration.position_y,  &configuration_table, "position_y");
		get_float(&mut configuration.zoom,        &configuration_table, "zoom");

		get_integer(&mut configuration.maximum_iteration_count, &configuration_table, "maximum_iteration_count");

		return configuration;
	}
}
