// decompile.rs
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

use crate::cmdline::DecompileArgs;

pub fn decompile(_args: &DecompileArgs) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }

    #[test]
    fn test_compile_default() {
        let args = DecompileArgs {
            output: None,
            file: String::from("tests/data/neck.dmi"),
        };
        decompile(&args);
    }

    #[test]
    fn test_compile_output() {
        let args = DecompileArgs {
            output: Some(String::from("tests/data/neckbeard.dmi.yml")),
            file: String::from("tests/data/neck.dmi"),
        };
        decompile(&args);
    }
}
