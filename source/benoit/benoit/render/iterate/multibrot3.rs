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

extern crate rug;

use rug::Float;

pub fn multibrot3(za: &mut Float, zb: &mut Float, ca: &Float, cb: &Float) {
	let za_temporary = za.clone(); // a

	//   (a+bi)^3
	// = (a+bi)(a+bi)(a+bi)
	// = (a^2-b^2+2abi)(a+bi)
	// = a^3+(a^2)bi-ab^2-(b^3)i+2(a^2)bi-2ab^2
	// = a^3+3(a^2)bi-3ab^2-(b^3)i
	//
	// <=> z_a = a^3-3ab^2
	//     z_b = 3(a^2)b-b^3

	let mut tmp0 = Float::with_val(PRECISION, &*zb * &*zb); // b^2

	let tmp1 = Float::with_val(PRECISION, &tmp0 * &*zb); // b^3

	tmp0 *= &*za; // ab^2
	tmp0 *= 0x3;  // 3ab^2

	za.square_mut(); // a^2

	*zb *= &*za; // (a^2)b

	*za *= &za_temporary; // a^3
	*za -= &tmp0;         // a^3-3ab^2
	*za += ca;            // a^3-3ab^2+Re(c)

	*zb *= 3.0;   // 3(a^2)b
	*zb -= &tmp1; // 3(a^2)b-b^3
	*zb += cb;    // 3(a^2)b-b^3+Im(c)
}
