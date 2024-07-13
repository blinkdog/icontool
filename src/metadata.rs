// metadata.rs
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

use indexmap::IndexMap;
use serde_yml::Value;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

use crate::cmdline::{FlatArgs, MetadataArgs};
use crate::constant::DMI_METADATA_KEY;
use crate::dmi::read_metadata;
use crate::error::Result;

pub fn flatten_metadata(args: &FlatArgs) -> Result<()> {
    // read the metadata from the file
    let metadata_path = PathBuf::from(&args.file);
    let mut metadata_file = File::open(&metadata_path)?;
    let mut contents = String::new();
    metadata_file.read_to_string(&mut contents)?;

    // convert it to flat yml format
    let mut data = IndexMap::new();
    data.insert(DMI_METADATA_KEY.to_string(), Value::from(contents));
    let yaml = serde_yml::to_string(&data)?;
    println!("{}", yaml);

    Ok(())
}

pub fn output_metadata(args: &MetadataArgs) -> Result<()> {
    let metadata_path = PathBuf::from(&args.file);
    let metadata_text = read_metadata(&metadata_path)?;

    // if the user provided an output file
    if let Some(output) = &args.output {
        // if the user provided an output file
        let output_path = PathBuf::from(output);
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);
        let _written_amount = writer.write(metadata_text.as_bytes())?;
        writer.flush()?;
        return Ok(());
    }

    // otherwise, just print it to the console
    println!("{}", metadata_text);
    Ok(())
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
