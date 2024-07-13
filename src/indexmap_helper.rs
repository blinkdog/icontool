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

use indexmap::IndexMap;
use serde_yml::Value;

use crate::error::{IconToolError, Result};

// IndexMapHelper adds a few convenience methods to IndexMap to handle
// all the grunt work of missing keys and type thunking between a generic
// serde_yml::Value down to a useful type
pub trait IndexMapHelper {
    fn get_icon_state_frames(&self, key: &str) -> Result<Vec<String>>;
    fn get_string(&self, key: &str) -> Result<String>;
    fn get_u32(&self, key: &str) -> Result<u32>;
}

impl IndexMapHelper for IndexMap<String, Value> {
    fn get_icon_state_frames(&self, key: &str) -> Result<Vec<String>> {
        // if there is a Value stored under the provided key
        if let Some(value) = self.get(key) {
            // and we can convert it to a &str
            if let Some(value_str) = value.as_str() {
                // split the string into each individual frame
                let frames_base64: Vec<String> =
                    value_str.split('\n').map(|s| s.to_string()).collect();
                // convert it to an owned String
                return Ok(frames_base64);
            }
            // return an error if we couldn't convert it to a Vec<String>
            return Err(IconToolError::InvalidType(format!(
                "Under key {key}, Value {value:?} cannot be converted to list of base64 encoded icon_state"
            )));
        }
        // return an error if the key was missing
        Err(IconToolError::MissingKey(format!("Key {key} is missing")))
    }

    fn get_string(&self, key: &str) -> Result<String> {
        // if there is a Value stored under the provided key
        if let Some(value) = self.get(key) {
            // and we can convert it to a &str
            if let Some(value_str) = value.as_str() {
                // convert it to an owned String
                return Ok(value_str.to_string());
            }
            // return an error if we couldn't convert it to a string
            return Err(IconToolError::InvalidType(format!(
                "Under key {key}, Value {value:?} cannot be converted to a String"
            )));
        }
        // return an error if the key was missing
        Err(IconToolError::MissingKey(format!("Key {key} is missing")))
    }

    fn get_u32(&self, key: &str) -> Result<u32> {
        // if there is a Value stored under the provided key
        if let Some(value) = self.get(key) {
            // and we can convert it to a u64
            if let Some(value_u64) = value.as_u64() {
                if value_u64 > u32::MAX as u64 {
                    // return an error if the value doesn't fit in u32
                    return Err(IconToolError::InvalidType(format!(
                        "Under key {key}, Value {value:?} cannot be converted to a u32"
                    )));
                }
                // convert it to a u32
                return Ok(value_u64 as u32);
            }
            // return an error if the value couldn't be converted to a u64
            return Err(IconToolError::InvalidType(format!(
                "Under key {key}, Value {value:?} cannot be converted to a u64"
            )));
        }
        // return an error if the key was missing
        Err(IconToolError::MissingKey(format!("Key {key} is missing")))
    }
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
