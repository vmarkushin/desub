// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-desub.
//
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

pub mod decoder;
mod error;
#[allow(unused, dead_code)] // TODO: refactor to not need this attribute
pub mod metadata;

#[cfg(test)]
mod test_suite;

use serde::{Serialize, Deserialize};

pub trait TypeDetective {
    fn get(ty: &str) -> &Decodable;
}

pub trait Decodable {
    fn as_string(&self) -> String;
    fn as_str(&self) -> &str;
    fn as_generic_struct(&self) -> GenericStruct;
    fn as_primitive(&self) -> PrimitiveField;
    fn as_bytes(&self) -> Vec<u8>;
    fn as_encoded_bytes(&self) -> Vec<u8>;


    fn is_str(&self) -> bool;
    fn is_bytes(&self) -> bool;
    fn is_generic_struct(&self) -> bool;
    fn is_primitive(&self) -> bool;
}

// tuples may be represented as anonymous structs
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericStruct {
    name: String,
    fields: Vec<StructOrPrimitive>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimitiveField {
    name: Option<String>,
    field: RustTypeMarker
}

#[derive(Debug, Serialize, Deserialize)]
enum StructOrPrimitive {
    Struct(GenericStruct),
    Primitive(PrimitiveField)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RustTypeMarker {
    /// name of a type
    Pointer(String),
    /// Some Struct
    Struct(GenericStruct),
    /// Some Enum
    Enum(GenericStruct),
    /// A sized array
    Array {
        size: usize,
        ty: String
    },
    /// primitive unsigned 8 bit integer
    U8,
    /// primtiive unsigned 16 bit integer
    U16,
    /// primitive unsigned 32 bit integer
    U32,
    /// primitive unsigned 64 bit integer
    U64,
    /// primitive unsigned 128 bit integer
    U128,
    /// primitive unsigned word-sized integer
    USize,

    /// primitive signed 8 bit integer
    I8,
    /// primitive signed 16 bit integer
    I16,
    /// primitive signed 32 bit integer
    I32,
    /// primitive signed 64 bit integer
    I64,
    /// primitive signed 128 bit integer
    I128,
    /// primitive signed word-sized integer
    ISize,

    /// primitive IEEE-spec 32-bit floating-point number
    F32,
    /// primitive IEEE-spec 64-bit floating-point number
    F64,
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
