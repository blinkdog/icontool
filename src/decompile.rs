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

use base64::prelude::*;
use image::{DynamicImage, GenericImageView, Pixel};
use indexmap::IndexMap;
use lz4_flex::block::compress_prepend_size;
use serde_yml::Value;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::cmdline::DecompileArgs;
use crate::constant::{DMI_METADATA_KEY, DMI_PATH_KEY, IMAGE_HEIGHT_KEY, IMAGE_WIDTH_KEY};
use crate::dmi::{read_image, read_metadata};
use crate::error::Result;
use crate::parser::{parse_metadata, DreamMakerIconMetadata};

struct IconStatePixels {
    key: String,
    value: Value,
}

pub fn decompile(args: &DecompileArgs) -> Result<()> {
    // determine the path to the provided dmi file
    let path = PathBuf::from(&args.file);

    // read the image data from the provided dmi file
    let image = read_image(&path)?;
    // read the dmi metadata from the provided dmi file
    let metadata_text = read_metadata(&path)?;
    // parse dmi metadata
    let dmi_metadata = parse_metadata(&metadata_text)?;

    // decompile the icon to an indexmap
    let data = decompile_icon(&path, &image, &metadata_text, &dmi_metadata);

    // output yaml to file
    let output_path = get_output_path(args);
    let file = File::create(output_path)?;
    serde_yml::to_writer(file, &data)?;

    // return success to the caller
    Ok(())
}

fn decompile_icon(
    path: &Path,
    image: &DynamicImage,
    text: &str,
    dmi: &DreamMakerIconMetadata,
) -> IndexMap<String, Value> {
    // this is the data structure that we'll build
    let mut data = IndexMap::new();

    // put the filename of the dmi at the top of the yaml
    let path_str = path.to_str().expect("Failed to convert path to string");
    data.insert(DMI_PATH_KEY.to_string(), Value::from(path_str));

    // save the image dimensions
    data.insert(IMAGE_WIDTH_KEY.to_string(), Value::from(image.width()));
    data.insert(IMAGE_HEIGHT_KEY.to_string(), Value::from(image.height()));

    // for each icon_state, add the name and pixels to the yaml
    let icon_states = extract_icon_states(image, dmi);
    for icon_state in icon_states {
        data.insert(icon_state.key, icon_state.value);
    }

    // put the dmi metadata at the bottom of the yaml
    data.insert(DMI_METADATA_KEY.to_string(), Value::from(text));

    // return the indexmap to the caller
    data
}

fn extract_icon_states(image: &DynamicImage, dmi: &DreamMakerIconMetadata) -> Vec<IconStatePixels> {
    // build up a nice list for the caller
    let mut icon_states = Vec::new();

    // make some nice aliases
    let DreamMakerIconMetadata {
        width: icon_width,
        height: icon_height,
        ..
    } = *dmi;
    let (image_width, _image_height) = image.dimensions();

    // as we iterate, we need to keep track of our position
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    // for each icon_state in the icon
    for state in &dmi.states {
        // we'll collect up each frame of the icon here
        let mut icon_frames = Vec::new();
        // determine how many frames we need to extract
        let num_frames = state.frames * state.dirs;
        // for each frame we need to extract
        for _ in 0..num_frames {
            // extract the pixel data
            let pixel_data = extract_pixel_data(image, cursor_x, cursor_y, icon_width, icon_height);
            // stringify the pixel data
            let pixel_text = stringify_pixel_data(&pixel_data);
            // add the pixel data to the icon_state
            icon_frames.push(pixel_text);
            // update the cursor
            cursor_x += icon_width;
            if cursor_x >= image_width {
                cursor_y += icon_height;
                cursor_x = 0;
            }
        }
        // collect up all the frames into a single value
        let frames = Value::String(icon_frames.join("\n"));
        // turn this into an icon_state
        let icon_state = IconStatePixels {
            key: state.name.clone(),
            value: frames,
        };
        // add it to our list of icon_states
        icon_states.push(icon_state);
    }

    // return the list of icon states to the caller
    icon_states
}

fn extract_pixel_data(
    image: &DynamicImage,
    tile_x: u32,
    tile_y: u32,
    tile_width: u32,
    tile_height: u32,
) -> Vec<u8> {
    // allocate a vector to hold the pixels
    let num_bytes: usize = tile_width as usize * tile_height as usize * 4;
    let mut pixel_data = Vec::with_capacity(num_bytes);

    // extract the RGBA values for each pixel in the requested region
    for y in tile_y..tile_y + tile_height {
        for x in tile_x..tile_x + tile_width {
            let pixel = image.get_pixel(x, y).to_rgba();
            for i in 0..4 {
                pixel_data.push(pixel[i]);
            }
        }
    }

    // return the RGBA pixel data to the caller
    pixel_data
}

fn get_output_path(args: &DecompileArgs) -> PathBuf {
    match &args.output {
        Some(output) => PathBuf::from(output),
        None => {
            let mut file_path = PathBuf::from(&args.file);
            file_path.set_extension("dmi.yml");
            file_path
        }
    }
}

fn stringify_pixel_data(pixel_data: &[u8]) -> String {
    // compress the pixel data with lz4
    let compressed = compress_prepend_size(pixel_data);
    // encode the compressed data into a base64 string
    BASE64_STANDARD.encode(compressed)
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
    fn test_decompile_default() {
        let args = DecompileArgs {
            output: None,
            file: String::from("tests/data/decompile/neck.dmi"),
        };
        let _ = decompile(&args);
    }

    #[test]
    fn test_decompile_output() {
        let args = DecompileArgs {
            output: Some(String::from("tests/data/decompile/neckbeard.dmi.yml")),
            file: String::from("tests/data/decompile/neck.dmi"),
        };
        let _ = decompile(&args);
    }

    #[test]
    fn test_get_output_path_default() {
        let args = DecompileArgs {
            output: None,
            file: String::from("tests/data/decompile/neck.dmi"),
        };
        let output_path = get_output_path(&args);
        assert_eq!(
            PathBuf::from("tests/data/decompile/neck.dmi.yml"),
            output_path
        );
    }

    #[test]
    fn test_get_output_path_override() {
        let args = DecompileArgs {
            output: Some(String::from("tests/data/decompile/neckbeard.dmi.yml")),
            file: String::from("tests/data/decompile/neck.dmi"),
        };
        let output_path = get_output_path(&args);
        assert_eq!(
            PathBuf::from("tests/data/decompile/neckbeard.dmi.yml"),
            output_path
        );
    }
}
