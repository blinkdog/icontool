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

use base64::prelude::*;
use image::{DynamicImage, Rgba};
use indexmap::IndexMap;
use lz4_flex::block::decompress_size_prepended;
use num_integer::Roots;
use png::Encoder;
use serde_yml::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::cmdline::CompileArgs;
use crate::constant::*;
use crate::error::{IconToolError, Result};
use crate::indexmap_helper::IndexMapHelper;
use crate::parser::{parse_metadata, DreamMakerIconMetadata};

pub fn compile(args: &CompileArgs) -> Result<()> {
    // determine the path to the provided .dmi.yml file
    let path = PathBuf::from(&args.file);

    // read the yaml data from the provided file
    let file = File::open(path)?;
    let yaml_data: IndexMap<String, Value> = serde_yml::from_reader(file)?;

    // parse dmi metadata
    let yaml_metadata = yaml_data.get_string(DMI_METADATA_KEY)?;
    let dmi_metadata = parse_metadata(&yaml_metadata)?;

    // measure the dimensions of the image to create our canvas
    let (image_width, image_height) = get_image_dimensions(&yaml_data, &dmi_metadata)?;
    let mut image = DynamicImage::new_rgba8(image_width, image_height);

    // warn if any icon states specified in the yaml will not be used to paint
    warn_for_unused_icon_states(&yaml_data, &dmi_metadata);

    // paint frames to the DynamicImage canvas
    paint_frames(&yaml_data, &dmi_metadata, &mut image)?;

    // write the .dmi file
    let output_path = get_output_path(args)?;
    write_dmi_file(&output_path, ZTXT_KEYWORD, &yaml_metadata, &image)?;

    // return success to the caller
    Ok(())
}

fn get_image_dimensions(
    yaml: &IndexMap<String, Value>,
    dmi: &DreamMakerIconMetadata,
) -> Result<(u32, u32)> {
    // measure the dimensions of the icon
    let icon_width = dmi.width;
    let icon_height = dmi.height;

    // measure the original width and height of the image
    let mut image_width = yaml.get_u32(IMAGE_WIDTH_KEY)?;
    let mut image_height = yaml.get_u32(IMAGE_HEIGHT_KEY)?;

    // determine how many frames we need
    let mut frames_needed = 0;
    for state in &dmi.states {
        frames_needed += state.dirs * state.frames;
    }

    // determine how many frames we have available
    let frames_per_row = image_width / icon_width;
    let rows_per_image = image_height / icon_height;
    let frames_available = frames_per_row * rows_per_image;

    // if we need more frames than we've got available
    if frames_needed >= frames_available {
        // emit a warning to the user
        eprintln!("Image dimensions {image_width}x{image_height} are not sufficient for {frames_needed} frames of icons sized {icon_width}x{icon_height}");

        // calculate the new dimensions of the image
        let pixels_square_needed = icon_width * icon_height * frames_needed;
        let pixels_needed = pixels_square_needed.sqrt();
        let frames_needed_per_row = (pixels_needed / icon_width) + 1;
        let pixels_needed_per_row = frames_needed_per_row * icon_width;
        image_width = pixels_needed_per_row; // note: always a multiple of icon_width
        let rows_needed = (frames_needed / frames_needed_per_row) + 1;
        image_height = rows_needed * icon_height; // note: always a multiple of icon_height

        // tell the user that we've increased the dimensions
        eprintln!("Image dimensions increased to {image_width}x{image_height}");
    }

    // do a final sanity check
    if image_width > 1024 || image_height > 1024 {
        return Err(IconToolError::TooManyIconStates(image_width, image_height));
    }

    // return the dimensions to the caller
    Ok((image_width, image_height))
}

fn get_output_path(args: &CompileArgs) -> Result<PathBuf> {
    // if we were provided an output, just use it
    if let Some(output) = &args.output {
        return Ok(PathBuf::from(output));
    }

    // otherwise, compute an output path based on the input path
    let file_stem = Path::new(&args.file)
        .file_stem()
        .ok_or_else(|| IconToolError::PathError("Failed to get file stem".to_string()))?
        .to_str()
        .ok_or_else(|| IconToolError::PathError("Failed to convert file stem".to_string()))?;

    let mut file_path = Path::new(&args.file)
        .parent()
        .ok_or_else(|| IconToolError::PathError("Failed to get parent directory".to_string()))?
        .to_path_buf();

    file_path.push(file_stem);
    file_path.set_extension("dmi");

    Ok(file_path)
}

fn paint_frames(
    yaml: &IndexMap<String, Value>,
    dmi: &DreamMakerIconMetadata,
    image: &mut DynamicImage,
) -> Result<()> {
    // measure the dimensions of the image
    let image_width = image.width();
    let image_height = image.height();

    // measure the dimensions of the icon
    let icon_width = dmi.width;
    let icon_height = dmi.height;

    // as we iterate, we need to keep track of our position
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    // for each icon_state in the dmi metadata
    for state in &dmi.states {
        // read the frame data from the yaml
        let frames_base64 = yaml.get_icon_state_frames(&state.name)?;
        // determine the number of frames we expect
        let expected_frames = (state.dirs * state.frames) as usize;
        // determine the number of frames we got
        let actual_frames = frames_base64.len();
        // if we didn't get what we expect
        if expected_frames != actual_frames {
            // tell the user which icon_state doesn't match between yaml and metadata
            return Err(IconToolError::FrameCountMismatch(
                state.name.to_string(),
                expected_frames,
                actual_frames,
            ));
        }

        // for each frame
        for frame_base64 in frames_base64 {
            // if cursor_y has already reached the complete height of the image
            if cursor_y >= image_height {
                // we have nowhere to paint this frame; so error out
                // NOTE: Seeing this error means there is a BUG in the get_image_dimensions
                // function. That function should have given us enough space for all the
                // frames, but it did not do so!
                return Err(IconToolError::TooManyFrames());
            }
            // decode the base64 to compressed pixel data
            let frame_pixel_data_compressed = BASE64_STANDARD.decode(frame_base64)?;
            // decompress pixel data to flat rgba pixel data
            let frame_pixel_data = decompress_size_prepended(&frame_pixel_data_compressed)?;
            // write the pixels of the frame to the image buffer
            let buffer = image.as_mut_rgba8().expect("Failed to convert to RGBA8");
            for y in 0..icon_height {
                for x in 0..icon_width {
                    let index = ((y * icon_width + x) * 4) as usize;
                    let pixel = Rgba([
                        frame_pixel_data[index],
                        frame_pixel_data[index + 1],
                        frame_pixel_data[index + 2],
                        frame_pixel_data[index + 3],
                    ]);
                    buffer.put_pixel(cursor_x + x, cursor_y + y, pixel);
                }
            }
            // update the cursor
            cursor_x += icon_width;
            if cursor_x >= image_width {
                cursor_y += icon_height;
                cursor_x = 0;
            }
        }
    }

    // tell the caller that all the frames were painted without error
    Ok(())
}

fn warn_for_unused_icon_states(yaml: &IndexMap<String, Value>, dmi: &DreamMakerIconMetadata) {
    // collect up all the keys from the yaml
    let mut keys: HashSet<String> = yaml.keys().cloned().collect();
    // remove keys used by icontool
    for key in ICONTOOL_KEYS {
        keys.remove(key);
    }
    // remove keys referenced by the dmi metadata
    for state in &dmi.states {
        keys.remove(&state.name);
    }
    // if there is anything left in our list
    if !keys.is_empty() {
        eprintln!(
            "icontool: {} icon_state(s) in the yaml are unused in the .dmi metadata: {:?}",
            keys.len(),
            keys
        );
    }
}

fn write_dmi_file(path: &PathBuf, keyword: &str, text: &str, image: &DynamicImage) -> Result<()> {
    // create the .dmi file
    let file = File::create(path)?;
    let bufwriter = BufWriter::new(file);

    // use the PNG encoder to create the metadata
    let width = image.width();
    let height = image.height();
    let mut encoder = Encoder::new(bufwriter, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.add_ztxt_chunk(keyword.to_string(), text.to_string())?;

    // write the PNG header and image data
    let mut writer = encoder.write_header()?;
    writer.write_image_data(image.as_bytes())?;

    // flush the correctness-verified PNG out to disk
    writer.finish()?;

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

    #[test]
    fn test_compile_failed_u32_conversion() {
        let args = CompileArgs {
            output: None,
            file: String::from("tests/data/compile/u33.dmi.yml"),
        };
        match compile(&args) {
            Err(x) => match x {
                IconToolError::InvalidType(_) => {
                    return;
                }
                _ => {
                    panic!("test_compile_failed_u32_conversion: Expected InvalidType error")
                }
            },
            _ => {
                panic!("test_compile_failed_u32_conversion: Expected IconToolError")
            }
        }
    }
}
