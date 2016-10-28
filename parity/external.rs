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

//! External binaries

extern crate ethstore;
extern crate ethkey;

use std::fmt;

use docopt::Docopt;
use cli::Args;

#[derive(Debug, PartialEq)]
pub enum Tool {
	EthStore,
	EthKey,
	EvmBin,
}

pub fn to_external(args: &Args) -> Option<Tool> {
	if args.cmd_evmbin {
		Some(Tool::EvmBin)
	} else if args.cmd_ethstore {
		Some(Tool::EthStore)
	} else if args.cmd_ethkey {
		Some(Tool::EthKey)
	} else {
		None
	}
}

pub fn to_string<T: fmt::Debug>(t: T) -> String {
	format!("{:?}", t)
}

pub fn execute(tool: Tool, mut cli: Vec<String>) -> Result<String, String> {
	// skip parity
	cli.remove(0);
	// skip tools
	cli.remove(0);

	println!("Running: {:?} with {:?}", tool, cli);

	// Execute tool
	match tool {
		Tool::EthStore => {
			let args = Docopt::new(ethstore::cli::USAGE)
				.and_then(|d| d.argv(cli).decode())
				.unwrap_or_else(|e| e.exit());
			ethstore::cli::execute(args).map_err(to_string)
		},
		Tool::EthKey => {
			let args = Docopt::new(ethkey::cli::USAGE)
				.and_then(|d| d.argv(cli).decode())
				.unwrap_or_else(|e| e.exit());
			ethkey::cli::execute(args).map_err(to_string)
		},
		_ => {
			unimplemented!()
		}
	}
}
