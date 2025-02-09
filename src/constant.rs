// constant.rs
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

pub const DMI_METADATA_KEY: &str = "__dmi_metadata";

pub const DMI_PATH_KEY: &str = "__dmi_path";

pub const IMAGE_HEIGHT_KEY: &str = "__image_height";

pub const IMAGE_WIDTH_KEY: &str = "__image_width";

pub const ICONTOOL_KEYS: [&str; 4] = [
    DMI_METADATA_KEY,
    DMI_PATH_KEY,
    IMAGE_HEIGHT_KEY,
    IMAGE_WIDTH_KEY,
];

pub const MAX_IMAGE_HEIGHT: u32 = 6144;

pub const MAX_IMAGE_WIDTH: u32 = 6144;

pub const ZTXT_KEYWORD: &str = "Description";

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
    fn test_dmi_metadata_key() {
        assert_eq!("__dmi_metadata", DMI_METADATA_KEY);
    }

    #[test]
    fn test_dmi_path_key() {
        assert_eq!("__dmi_path", DMI_PATH_KEY);
    }

    #[test]
    fn test_image_height_key() {
        assert_eq!("__image_height", IMAGE_HEIGHT_KEY);
    }

    #[test]
    fn test_image_width_key() {
        assert_eq!("__image_width", IMAGE_WIDTH_KEY);
    }

    #[test]
    fn test_max_image_height() {
        assert_eq!(6144, MAX_IMAGE_HEIGHT);
    }

    #[test]
    fn test_max_image_width() {
        assert_eq!(6144, MAX_IMAGE_WIDTH);
    }

    #[test]
    fn test_ztxt_keyword() {
        assert_eq!("Description", ZTXT_KEYWORD);
    }
}
