// error.rs
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

use std::path::PathBuf;

#[derive(Debug)]
pub struct MissingMetadata(pub PathBuf);

#[derive(Debug)]
pub enum IconToolError {
    DecodeError(base64::DecodeError),
    DecodingError(png::DecodingError),
    DecompressError(lz4_flex::block::DecompressError),
    EncodingError(png::EncodingError),
    ImageError(image::ImageError),
    IncompleteParseError(String),
    Io(std::io::Error),
    MissingMetadata(MissingMetadata),
    ParseError(String),
    Serialize(serde_yml::Error),
}

impl From<base64::DecodeError> for IconToolError {
    fn from(error: base64::DecodeError) -> Self {
        IconToolError::DecodeError(error)
    }
}

impl From<png::DecodingError> for IconToolError {
    fn from(error: png::DecodingError) -> Self {
        IconToolError::DecodingError(error)
    }
}

impl From<lz4_flex::block::DecompressError> for IconToolError {
    fn from(error: lz4_flex::block::DecompressError) -> Self {
        IconToolError::DecompressError(error)
    }
}

impl From<png::EncodingError> for IconToolError {
    fn from(error: png::EncodingError) -> Self {
        IconToolError::EncodingError(error)
    }
}

impl From<image::ImageError> for IconToolError {
    fn from(error: image::ImageError) -> Self {
        IconToolError::ImageError(error)
    }
}

impl From<std::io::Error> for IconToolError {
    fn from(error: std::io::Error) -> Self {
        IconToolError::Io(error)
    }
}

impl From<MissingMetadata> for IconToolError {
    fn from(error: MissingMetadata) -> Self {
        IconToolError::MissingMetadata(error)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for IconToolError {
    fn from(error: nom::Err<nom::error::Error<&str>>) -> Self {
        IconToolError::ParseError(error.to_string())
    }
}

impl From<serde_yml::Error> for IconToolError {
    fn from(error: serde_yml::Error) -> Self {
        IconToolError::Serialize(error)
    }
}

pub type Result<T> = std::result::Result<T, IconToolError>;

pub fn get_error_message(e: IconToolError) -> String {
    match e {
        IconToolError::DecodeError(x) => {
            format!("icontool: Unable to decode base64 data: {x}")
        }
        IconToolError::DecodingError(x) => {
            format!("icontool: Unable to decode .dmi file: {x}")
        }
        IconToolError::DecompressError(x) => {
            format!("icontool: Unable to decompress LZ4 data: {x}")
        }
        IconToolError::EncodingError(x) => {
            format!("icontool: Unable to encode .dmi file: {x}")
        }
        IconToolError::ImageError(x) => {
            format!("icontool: Error decoding .dmi image: {x}")
        }
        IconToolError::IncompleteParseError(x) => {
            format!("icontool: Incomplete parse of .dmi metadata: {x}")
        }
        IconToolError::Io(x) => {
            format!("icontool: I/O error: {x}")
        }
        IconToolError::MissingMetadata(x) => {
            format!("icontool: Unable to read metadata from .dmi file: {x:?}")
        }
        IconToolError::ParseError(x) => {
            format!("icontool: Error parsing .dmi metadata: {x}")
        }
        IconToolError::Serialize(x) => {
            format!("icontool: Unable to serialize YAML data: {x}")
        }
    }
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
