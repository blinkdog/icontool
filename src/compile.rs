// compile.rs
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

use crate::cmdline::CompileArgs;
use crate::error::Result;

pub fn compile(_args: &CompileArgs) -> Result<()> {
    // read yaml from file
    // use indexmap to generate dmi
    // output dmi

    Ok(())
}

//---------------------------------------------------------------------------
//---------------------------------------------------------------------------
//---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }

    #[test]
    fn test_compile_default() {
        let args = CompileArgs {
            output: None,
            file: String::from("tests/data/compile/neck.dmi.yml"),
        };
        let _ = compile(&args);
    }

    #[test]
    fn test_compile_output() {
        let args = CompileArgs {
            output: Some(String::from("tests/data/compile/neckbeard.dmi")),
            file: String::from("tests/data/compile/neck.dmi.yml"),
        };
        let _ = compile(&args);
    }
}
