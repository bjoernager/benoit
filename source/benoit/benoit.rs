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

extern crate rug;

use rug::Float;

pub mod app;
pub mod configuration;
pub mod fractal;
pub mod iteration;
pub mod task;
pub mod video;

pub struct Version<T> {
	major: T,
	minor: T,
	patch: T,
}

pub const VERSION: Version::<u32> = Version::<u32> {
	major: 0x1,
	minor: 0x1,
	patch: 0x0,
};

pub const PRECISION: u32 = 0x80;

pub struct FeedbackInfo<'a> {
	prev_centre_real: &'a Float,
	prev_centre_imag: &'a Float,
	prev_zoom:        &'a Float,
	next_centre_real:      &'a Float,
	next_centre_imag:      &'a Float,
	next_zoom:             &'a Float,
}
