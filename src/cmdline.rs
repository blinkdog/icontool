// cmdline.rs
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

use clap::{crate_version, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "icontool")]
#[command(version = crate_version!())]
#[command(about = "Tool for working with BYOND DreamMaker Icon (.dmi) files", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// convert a .dmi.yml file to a .dmi file
    Compile(CompileArgs),
    /// convert a .dmi file to a .dmi.yml file
    Decompile(DecompileArgs),
}

#[derive(Args)]
pub struct CompileArgs {
    #[arg(short, long)]
    pub output: Option<String>,

    pub file: String,
}

#[derive(Args)]
pub struct DecompileArgs {
    #[arg(short, long)]
    pub output: Option<String>,

    pub file: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }

    #[test]
    fn test_compile_default() {
        let cli = Cli::parse_from(vec![
            "icontool",
            "compile",
            "icons/mob/clothing/neck.dmi.yml",
        ]);
        match &cli.command {
            Commands::Compile(args) => {
                assert_eq!("icons/mob/clothing/neck.dmi.yml", args.file);
                assert_eq!(None, args.output);
            }
            _ => panic!("Subcommand 'compile' was not parsed to Commands::Compile"),
        }
    }

    #[test]
    fn test_compile_output() {
        let cli = Cli::parse_from(vec![
            "icontool",
            "compile",
            "--output",
            "icons/mob/clothing/neckbeard.dmi",
            "icons/mob/clothing/neck.dmi.yml",
        ]);
        match &cli.command {
            Commands::Compile(args) => {
                assert_eq!("icons/mob/clothing/neck.dmi.yml", args.file);
                assert_eq!(
                    "icons/mob/clothing/neckbeard.dmi",
                    args.output.as_ref().unwrap()
                );
            }
            _ => panic!("Subcommand 'compile' was not parsed to Commands::Compile"),
        }
    }

    #[test]
    fn test_decompile_default() {
        let cli = Cli::parse_from(vec!["icontool", "decompile", "icons/mob/clothing/neck.dmi"]);
        match &cli.command {
            Commands::Decompile(args) => {
                assert_eq!("icons/mob/clothing/neck.dmi", args.file);
                assert_eq!(None, args.output);
            }
            _ => panic!("Subcommand 'decompile' was not parsed to Commands::Decompile"),
        }
    }

    #[test]
    fn test_decompile_output() {
        let cli = Cli::parse_from(vec![
            "icontool",
            "decompile",
            "--output",
            "icons/mob/clothing/neckbeard.dmi.yml",
            "icons/mob/clothing/neck.dmi",
        ]);
        match &cli.command {
            Commands::Decompile(args) => {
                assert_eq!("icons/mob/clothing/neck.dmi", args.file);
                assert_eq!(
                    "icons/mob/clothing/neckbeard.dmi.yml",
                    args.output.as_ref().unwrap()
                );
            }
            _ => panic!("Subcommand 'decompile' was not parsed to Commands::Decompile"),
        }
    }
}
