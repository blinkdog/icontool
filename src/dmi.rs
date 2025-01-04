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

use image::{DynamicImage, ImageFormat, ImageReader};
use std::path::Path;
use std::{fs::File, io::BufReader};

use crate::constant::ZTXT_KEYWORD;
use crate::error::{IconToolError, MissingMetadata, Result};

pub fn read_image(path: &Path) -> Result<DynamicImage> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let image = ImageReader::with_format(reader, ImageFormat::Png).decode()?;
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
    // use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }
}
