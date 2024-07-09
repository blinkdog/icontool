// dmi.rs
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

use image::{io::Reader, DynamicImage, ImageFormat};
use indexmap::IndexMap;
use serde_yml::Value;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs::File, io::BufReader};

use crate::cmdline::{FlatArgs, MetadataArgs};
use crate::constant::{DMI_METADATA_KEY, ZTXT_KEYWORD};
use crate::error::{IconToolError, MissingMetadata, Result};

// TODO: these flatten_metadata and output_metadata functions should probably
// be refactored to a new metadata.rs module (after the existing one also gets
// refactored)

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

pub fn read_image(path: &Path) -> Result<DynamicImage> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let image = Reader::with_format(reader, ImageFormat::Png).decode()?;
    Ok(image)
}

pub fn read_metadata(path: &Path) -> Result<String> {
    // read the png data from the .dmi file
    let dmi_file = File::open(path)?;
    let decoder = png::Decoder::new(dmi_file);
    let reader = decoder.read_info()?;

    // for each zTXt chunk in the png file
    for text_chunk in &reader.info().compressed_latin1_text {
        // println!("{:?}", text_chunk.keyword);
        // println!("zTXt: {}", text_chunk.get_text().unwrap());

        // if the chunk has keyword 'Description'
        if text_chunk.keyword == ZTXT_KEYWORD {
            // extract the dmi metadata from the zTXt chunk
            let metadata = text_chunk.get_text()?;
            return Ok(metadata);
        }
    }

    // if we didn't find a zTXt chunk with dmi metadata
    let missing_metadata = MissingMetadata(path.into());
    Err(IconToolError::MissingMetadata(missing_metadata))
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
}
