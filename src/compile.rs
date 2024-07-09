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
use png::Encoder;
use serde_yml::Value;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::cmdline::CompileArgs;
use crate::constant::*;
use crate::error::Result;
use crate::metadata::parse_metadata;

pub fn compile(args: &CompileArgs) -> Result<()> {
    // determine the path to the provided .dmi.yml file
    let path = PathBuf::from(&args.file);

    // read the yaml data from the provided file
    let file = File::open(path)?;
    let yaml_data: IndexMap<String, Value> = serde_yml::from_reader(file)?;

    // parse dmi metadata
    let metadata_value = yaml_data
        .get(DMI_METADATA_KEY)
        .expect("Provided .dmi.yml does not contain metadata key");
    let metadata_str = metadata_value
        .as_str()
        .expect("Provided .dmi.yml has malformed metadata value that cannot be parsed as a string");
    let dmi_metadata = parse_metadata(metadata_str)?;
    let icon_width = dmi_metadata.width;
    let icon_height = dmi_metadata.height;

    // create image to begin reconstruction
    let image_width_value = yaml_data
        .get(IMAGE_WIDTH_KEY)
        .expect("Provided .dmi.yml does not contain image width key");
    let image_width = image_width_value.as_u64().expect(
        "Provided .dmi.yml has malformed image width value that cannot be parsed as a number",
    ) as u32;
    let image_height_value = yaml_data
        .get(IMAGE_HEIGHT_KEY)
        .expect("Provided .dmi.yml does not contain image height key");
    let image_height = image_height_value.as_u64().expect(
        "Provided .dmi.yml has malformed image height value that cannot be parsed as a number",
    ) as u32;
    let mut image = DynamicImage::new_rgba8(image_width as u32, image_height as u32);

    // as we iterate, we need to keep track of our position
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    // for each icon_state in the metadata
    for state in &dmi_metadata.states {
        // read the frame data from the yaml
        let frames_value = yaml_data.get(&state.name).unwrap_or_else(|| {
            panic!(
                "Provided .dmi.yml does not contain icon_state {}",
                state.name
            )
        });
        let frames_base64_joined = frames_value
            .as_str()
            .expect("Provided .dmi.yml has malformed icon_state that cannot be parsed as a string");
        let frames_base64: Vec<String> = frames_base64_joined
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        // for each frame
        for frame_base64 in frames_base64 {
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

    // write the .dmi file
    let output_path = get_output_path(args);
    write_dmi_file(&output_path, ZTXT_KEYWORD, metadata_str, &image)?;

    // return success to the caller
    Ok(())
}

fn get_output_path(args: &CompileArgs) -> PathBuf {
    match &args.output {
        Some(output) => PathBuf::from(output),
        None => {
            let file_stem = Path::new(&args.file)
                .file_stem()
                .expect("Failed to get file stem")
                .to_str()
                .expect("Failed to convert file stem to string");

            let mut file_path = Path::new(&args.file)
                .parent()
                .expect("Failed to get parent directory")
                .to_path_buf();

            file_path.push(file_stem);
            file_path.set_extension("dmi");

            file_path
        }
    }
}

fn write_dmi_file(path: &PathBuf, keyword: &str, text: &str, image: &DynamicImage) -> Result<()> {
    // create the .dmi file
    let file = File::create(path)?;
    let bufwriter = BufWriter::new(file);

    // use the PNG encoder to create the metadata
    let mut encoder = Encoder::new(bufwriter, 256, 256);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.add_ztxt_chunk(keyword.to_string(), text.to_string())?;

    // write the PNG header and image data
    let mut writer = encoder.write_header()?;
    writer.write_image_data(image.as_bytes())?;

    // flush the correctness verified PNG out to disk
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
}
