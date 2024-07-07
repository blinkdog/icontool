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
pub mod decompile;

use clap::Parser;

use crate::cmdline::{Cli, Commands};
use crate::compile::compile;
use crate::decompile::decompile;

#[cfg(not(tarpaulin_include))]
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compile(args) => {
            compile(args);
        }
        Commands::Decompile(args) => {
            decompile(args);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }
}
