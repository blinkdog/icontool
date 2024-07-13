// main.rs
// Copyright 2024 Patrick Meade.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//---------------------------------------------------------------------------

pub mod cmdline;
pub mod compile;
pub mod constant;
pub mod decompile;
pub mod dmi;
pub mod error;
pub mod indexmap_helper;
pub mod metadata;
pub mod parser;

use clap::Parser;
use std::process::ExitCode;

use crate::cmdline::{Cli, Commands};
use crate::compile::compile;
use crate::decompile::decompile;
use crate::error::get_error_message;
use crate::metadata::{flatten_metadata, output_metadata};

#[cfg(not(tarpaulin_include))]
fn main() -> ExitCode {
    // parse what the user provided on the command line
    let cli = Cli::parse();

    // depending on what subcommand the user provided
    let result = match &cli.command {
        // compile a .dmi.yml -> .dmi
        Commands::Compile(args) => compile(args),
        // decompile a .dmi -> .dmi.yml
        Commands::Decompile(args) => decompile(args),
        // flatten metadata into .yml format
        Commands::Flat(args) => flatten_metadata(args),
        // output metadata for a .dmi
        Commands::Metadata(args) => output_metadata(args),
    };

    // if the operation failed for some reason
    if let Err(x) = result {
        // print a friendly message on stderr
        eprintln!("{}", get_error_message(x));
        // exit (with non-zero to indicate an error)
        return ExitCode::FAILURE;
    }

    // exit (with zero to indicate no error)
    ExitCode::SUCCESS
}

//---------------------------------------------------------------------------
//---------------------------------------------------------------------------
//---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }
}
