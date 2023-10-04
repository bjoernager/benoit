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

use crate::benoit::render::FactoriserFunction;
use crate::benoit::render::factorise;

use std::mem::transmute;

#[derive(Clone, Copy)]
pub enum Factorisation {
	Smooth,
	Stepped,
}

impl Factorisation {
	pub fn get_factoriser(self) -> FactoriserFunction {
		return match self {
			Factorisation::Smooth  => factorise::smooth,
			Factorisation::Stepped => factorise::stepped,
		};
	}

	pub fn cycle(self) -> Self {
		let raw = !(self as u8 != 0x0);

		let new: Self = unsafe { transmute(raw) };

		return new;
	}
}
