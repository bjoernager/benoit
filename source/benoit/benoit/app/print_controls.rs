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

use crate::benoit::app::App;

impl App {
	pub(super) fn print_controls() {
		println!("");
		println!("controls:");
		println!("- \u{1B}[1mW\u{1B}[0m            translate +Im");
		println!("- \u{1B}[1mA\u{1B}[0m            translate -Re");
		println!("- \u{1B}[1mS\u{1B}[0m            translate -Im");
		println!("- \u{1B}[1mD\u{1B}[0m            translate +Re");
		println!();
		println!("- \u{1B}[1mQ\u{1B}[0m            zoom out");
		println!("- \u{1B}[1mE\u{1B}[0m            zoom in");
		println!();
		println!("- \u{1B}[1mR\u{1B}[0m            decrease max. iteration count");
		println!("- \u{1B}[1mF\u{1B}[0m            increase max. iteration count");
		println!();
		println!("- \u{1B}[1mLEFT ALT\u{1B}[0m     cycle to previous fractal");
		println!("- \u{1B}[1mRIGHT ALT\u{1B}[0m    cycle to next fractal");
		println!("- \u{1B}[1mTAB\u{1B}[0m          toggle Julia");
		println!("- \u{1B}[1mLEFT CTRL\u{1B}[0m    toggle inverse");
		println!();
		println!("- \u{1B}[1mLEFT\u{1B}[0m         cycle to previous palette");
		println!("- \u{1B}[1mRIGHT\u{1B}[0m        cycle to next palette");
		println!("- \u{1B}[1mUP\u{1B}[0m           increase colour range");
		println!("- \u{1B}[1mDOWN\u{1B}[0m         decrease colour range");
		println!();
		println!("- \u{1B}[1mF1\u{1B}[0m           toggle textual feedback");
		println!("- \u{1B}[1mZ\u{1B}[0m            print centre value (c)");
		println!();
		println!("- \u{1B}[1mC\u{1B}[0m            render frame");
		println!();
		println!("Controls (holding \u{1B}[1mSHIFT\u{1B}[0m):");
		println!("- \u{1B}[1mW\u{1B}[0m            perturbate/translate +Im");
		println!("- \u{1B}[1mA\u{1B}[0m            perturbate/translate -Re");
		println!("- \u{1B}[1mS\u{1B}[0m            perturbate/translate -Im");
		println!("- \u{1B}[1mD\u{1B}[0m            perturbate/translate +Re");
		println!();
		println!("- \u{1B}[1mC\u{1B}[0m            render frame");
		println!();
	}
}