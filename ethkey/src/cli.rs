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

use std::fmt;
use std::num::ParseIntError;
use rustc_serialize::hex::{FromHex, FromHexError};
use {KeyPair, Random, Brain, Prefix, Error as EthkeyError, Generator, sign, verify_public, verify_address};

pub const USAGE: &'static str = r#"
Ethereum keys generator.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethkey info <secret> [options]
    ethkey generate random [options]
    ethkey generate prefix <prefix> <iterations> [options]
    ethkey generate brain <seed> [options]
    ethkey sign <secret> <message>
    ethkey verify public <public> <signature> <message>
    ethkey verify address <address> <signature> <message>
    ethkey [-h | --help]

Options:
    -h, --help         Display this message and exit.
    -s, --secret       Display only the secret.
    -p, --public       Display only the public.
    -a, --address      Display only the address.

Commands:
    info               Display public and address of the secret.
    generate           Generates new ethereum key.
    random             Random generation.
    prefix             Random generation, but address must start with a prefix
    brain              Generate new key from string seed.
    sign               Sign message using secret.
    verify             Verify signer of the signature.
"#;

#[derive(Debug, RustcDecodable)]
pub struct Args {
	cmd_info: bool,
	cmd_generate: bool,
	cmd_random: bool,
	cmd_prefix: bool,
	cmd_brain: bool,
	cmd_sign: bool,
	cmd_verify: bool,
	cmd_public: bool,
	cmd_address: bool,
	arg_prefix: String,
	arg_iterations: String,
	arg_seed: String,
	arg_secret: String,
	arg_message: String,
	arg_public: String,
	arg_address: String,
	arg_signature: String,
	flag_secret: bool,
	flag_public: bool,
	flag_address: bool,
}

#[derive(Debug)]
pub enum Error {
	Ethkey(EthkeyError),
	FromHex(FromHexError),
	ParseInt(ParseIntError),
}

impl From<EthkeyError> for Error {
	fn from(err: EthkeyError) -> Self {
		Error::Ethkey(err)
	}
}

impl From<FromHexError> for Error {
	fn from(err: FromHexError) -> Self {
		Error::FromHex(err)
	}
}

impl From<ParseIntError> for Error {
	fn from(err: ParseIntError) -> Self {
		Error::ParseInt(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Error::Ethkey(ref e) => write!(f, "{}", e),
			Error::FromHex(ref e) => write!(f, "{}", e),
			Error::ParseInt(ref e) => write!(f, "{}", e),
		}
	}
}

enum DisplayMode {
	KeyPair,
	Secret,
	Public,
	Address,
}

impl DisplayMode {
	fn new(args: &Args) -> Self {
		if args.flag_secret {
			DisplayMode::Secret
		} else if args.flag_public {
			DisplayMode::Public
		} else if args.flag_address {
			DisplayMode::Address
		} else {
			DisplayMode::KeyPair
		}
	}
}

fn display(keypair: KeyPair, mode: DisplayMode) -> String {
	match mode {
		DisplayMode::KeyPair => format!("{}", keypair),
		DisplayMode::Secret => format!("{:?}", keypair.secret()),
		DisplayMode::Public => format!("{:?}", keypair.public()),
		DisplayMode::Address => format!("{:?}", keypair.address()),
	}
}

pub fn execute(args: Args) -> Result<String, Error> {
	if args.cmd_info {
		let display_mode = DisplayMode::new(&args);
		let secret = try!(args.arg_secret.parse().map_err(|_| EthkeyError::InvalidSecret));
		let keypair = try!(KeyPair::from_secret(secret));
		Ok(display(keypair, display_mode))
	} else if args.cmd_generate {
		let display_mode = DisplayMode::new(&args);
		let keypair = if args.cmd_random {
			Random.generate()
		} else if args.cmd_prefix {
			let prefix = try!(args.arg_prefix.from_hex());
			let iterations = try!(usize::from_str_radix(&args.arg_iterations, 10));
			Prefix::new(prefix, iterations).generate()
		} else if args.cmd_brain {
			Brain::new(args.arg_seed).generate()
		} else {
			unreachable!();
		};
		Ok(display(try!(keypair), display_mode))
	} else if args.cmd_sign {
		let secret = try!(args.arg_secret.parse().map_err(|_| EthkeyError::InvalidSecret));
		let message = try!(args.arg_message.parse().map_err(|_| EthkeyError::InvalidMessage));
		let signature = try!(sign(&secret, &message));
		Ok(format!("{}", signature))
	} else if args.cmd_verify {
		let signature = try!(args.arg_signature.parse().map_err(|_| EthkeyError::InvalidSignature));
		let message = try!(args.arg_message.parse().map_err(|_| EthkeyError::InvalidMessage));
		let ok = if args.cmd_public {
			let public = try!(args.arg_public.parse().map_err(|_| EthkeyError::InvalidPublic));
			try!(verify_public(&public, &signature, &message))
		} else if args.cmd_address {
			let address = try!(args.arg_address.parse().map_err(|_| EthkeyError::InvalidAddress));
			try!(verify_address(&address, &signature, &message))
		} else {
			unreachable!();
		};
		Ok(format!("{}", ok))
	} else {
		unreachable!();
	}
}
