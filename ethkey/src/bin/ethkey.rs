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
extern crate ethkey;

use docopt::Docopt;

fn main() {
	match execute(env::args()) {
		Ok(ok) => println!("{}", ok),
		Err(err) => {
			println!("{}", err);
			process::exit(1);
		},
	}
}

fn execute<S, I>(command: I) -> Result<String, Error> where I: IntoIterator<Item=S>, S: AsRef<str> {
	let args: Args = Docopt::new(ethkey::cli::USAGE)
		.and_then(|d| d.argv(command).decode())
		.unwrap_or_else(|e| e.exit());

	ethkey::cli::execute(args)
}

#[cfg(test)]
mod tests {
	use super::execute;

	#[test]
	fn info() {
		let command = vec!["ethkey", "info", "17d08f5fe8c77af811caa0c9a187e668ce3b74a99acc3f6d976f075fa8e0be55"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected =
"secret:  17d08f5fe8c77af811caa0c9a187e668ce3b74a99acc3f6d976f075fa8e0be55
public:  689268c0ff57a20cd299fa60d3fb374862aff565b20b5f1767906a99e6e09f3ff04ca2b2a5cd22f62941db103c0356df1a8ed20ce322cab2483db67685afd124
address: 26d1ec50b4e62c1d1a40d16e7cacc6a6580757d5".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn brain() {
		let command = vec!["ethkey", "generate", "brain", "this is sparta"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected =
"secret:  17d08f5fe8c77af811caa0c9a187e668ce3b74a99acc3f6d976f075fa8e0be55
public:  689268c0ff57a20cd299fa60d3fb374862aff565b20b5f1767906a99e6e09f3ff04ca2b2a5cd22f62941db103c0356df1a8ed20ce322cab2483db67685afd124
address: 26d1ec50b4e62c1d1a40d16e7cacc6a6580757d5".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn secret() {
		let command = vec!["ethkey", "generate", "brain", "this is sparta", "--secret"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected = "17d08f5fe8c77af811caa0c9a187e668ce3b74a99acc3f6d976f075fa8e0be55".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn public() {
		let command = vec!["ethkey", "generate", "brain", "this is sparta", "--public"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected = "689268c0ff57a20cd299fa60d3fb374862aff565b20b5f1767906a99e6e09f3ff04ca2b2a5cd22f62941db103c0356df1a8ed20ce322cab2483db67685afd124".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn address() {
		let command = vec!["ethkey", "generate", "brain", "this is sparta", "--address"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected = "26d1ec50b4e62c1d1a40d16e7cacc6a6580757d5".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn sign() {
		let command = vec!["ethkey", "sign", "17d08f5fe8c77af811caa0c9a187e668ce3b74a99acc3f6d976f075fa8e0be55", "bd50b7370c3f96733b31744c6c45079e7ae6c8d299613246d28ebcef507ec987"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected = "c1878cf60417151c766a712653d26ef350c8c75393458b7a9be715f053215af63dfd3b02c2ae65a8677917a8efa3172acb71cb90196e42106953ea0363c5aaf200".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn verify_valid_public() {
		let command = vec!["ethkey", "verify", "public", "689268c0ff57a20cd299fa60d3fb374862aff565b20b5f1767906a99e6e09f3ff04ca2b2a5cd22f62941db103c0356df1a8ed20ce322cab2483db67685afd124", "c1878cf60417151c766a712653d26ef350c8c75393458b7a9be715f053215af63dfd3b02c2ae65a8677917a8efa3172acb71cb90196e42106953ea0363c5aaf200", "bd50b7370c3f96733b31744c6c45079e7ae6c8d299613246d28ebcef507ec987"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected = "true".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn verify_valid_address() {
		let command = vec!["ethkey", "verify", "address", "26d1ec50b4e62c1d1a40d16e7cacc6a6580757d5", "c1878cf60417151c766a712653d26ef350c8c75393458b7a9be715f053215af63dfd3b02c2ae65a8677917a8efa3172acb71cb90196e42106953ea0363c5aaf200", "bd50b7370c3f96733b31744c6c45079e7ae6c8d299613246d28ebcef507ec987"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected = "true".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}

	#[test]
	fn verify_invalid() {
		let command = vec!["ethkey", "verify", "public", "689268c0ff57a20cd299fa60d3fb374862aff565b20b5f1767906a99e6e09f3ff04ca2b2a5cd22f62941db103c0356df1a8ed20ce322cab2483db67685afd124", "c1878cf60417151c766a712653d26ef350c8c75393458b7a9be715f053215af63dfd3b02c2ae65a8677917a8efa3172acb71cb90196e42106953ea0363c5aaf200", "bd50b7370c3f96733b31744c6c45079e7ae6c8d299613246d28ebcef507ec986"]
			.into_iter()
			.map(Into::into)
			.collect::<Vec<String>>();

		let expected = "false".to_owned();
		assert_eq!(execute(command).unwrap(), expected);
	}
}
