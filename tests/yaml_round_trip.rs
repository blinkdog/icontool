// yaml_round_trip.rs
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
use serde::{Deserialize, Serialize};
use serde_yml::{self, Value};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TestData {
    field1: String,
    field2: i32,
    field3: Vec<f64>,
}

#[test]
fn test_yaml_round_trip() {
    // create some sample data
    let original_data = TestData {
        field1: String::from("Hello, world!"),
        field2: 42,
        field3: vec![1.1, 2.2, 3.3],
    };

    // serialize the struct to a yaml string
    let yaml_string = serde_yml::to_string(&original_data).expect("Failed to serialize data");
    // eprintln!("{}", &yaml_string);

    // deserialize the yaml string back to a struct
    let deserialized_data: TestData =
        serde_yml::from_str(&yaml_string).expect("Failed to deserialize data");

    // check that the deserialized data matches the original sample
    assert_eq!(original_data, deserialized_data);
}

#[test]
fn test_yaml_with_ordered_dynamic_fields() {
    // create some sample data
    let mut data = IndexMap::new();
    data.insert(
        "field1".to_string(),
        Value::String("Hello, world!".to_string()),
    );
    data.insert(
        "opened".to_string(),
        Value::String("Lorem ipsum dolor sit amet.".to_string()),
    );
    data.insert(
        "closed".to_string(),
        Value::String("Consectetur adipiscing elit.".to_string()),
    );
    data.insert(
        "field3".to_string(),
        Value::Sequence(vec![
            Value::Number(1.1.into()),
            Value::Number(2.2.into()),
            Value::Number(3.3.into()),
        ]),
    );
    data.insert(
        "enchanted".to_string(),
        Value::String(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n\
         Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\
         Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris\n\
         nisi ut aliquip ex ea commodo consequat."
                .to_string(),
        ),
    );
    data.insert(
        "'".to_string(),
        Value::String("Single quote stuff.".to_string()),
    );
    data.insert(
        "\"".to_string(),
        Value::String("Double quotes also in fashion.".to_string()),
    );
    data.insert("field2".to_string(), Value::Number(42.into()));

    // serialize the IndexMap to a yaml string
    let yaml_string = serde_yml::to_string(&data).expect("Failed to serialize data");
    // eprintln!("Serialized YAML: {}", yaml_string);

    // deserialize the yaml string back to an IndexMap
    let deserialized_data: IndexMap<String, Value> =
        serde_yml::from_str(&yaml_string).expect("Failed to deserialize data");
    // eprintln!("Deserialized data: {:?}", deserialized_data);

    // check that the deserialized data matches the original sample
    assert_eq!(data, deserialized_data);
}
