// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

extern crate docopt;
extern crate ethstore;

use docopt::Docopt;

fn main() {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.argv(env::args()).decode())
		.unwrap_or_else(|e| e.exit());

	match ethstore::cli::execute(args) {
		Ok(result) => println!("{}", result),
		Err(err) => {
			println!("{}", err);
			process::exit(1);
		}
	}
}
