//! JavaScript Object Notation.

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{any::type_name, fs, path::Path};

/// Load an object from a JSON file.
#[inline]
#[must_use]
pub fn load<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de>,
{
    read(
        &fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}.", path.display())),
    )
}

/// Deserialize a JSON string into an object.
#[inline]
#[must_use]
pub fn read<T>(s: &str) -> T
where
    for<'de> T: Deserialize<'de>,
{
    json5::from_str(s).unwrap_or_else(|_| {
        panic!(
            "Failed to create type {} from JSON string.",
            type_name::<T>()
        )
    })
}

/// Save an object to a JSON file.
#[inline]
pub fn save<T: Serialize>(instance: &T, path: &Path) {
    fs::write(path, write(instance))
        .unwrap_or_else(|_| panic!("Failed to write file: {}.", path.display()));
}

/// Serialise an object into a JSON string.
#[inline]
#[must_use]
pub fn write<T: Serialize>(instance: &T) -> String {
    to_string(instance)
        .unwrap_or_else(|_| panic!("Failed to write type {} to JSON string.", type_name::<T>()))
}
