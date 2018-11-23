use super::*;
use std::fmt;

use serde::{Serializer, Deserializer, de::Visitor};
use serde_bytes::ByteBuf;

pub fn serialize<S: Serializer>(bit_set: &BitSet, serializer: S) -> Result<S::Ok, S::Error> {
    let bit_vec = bit_set.get_ref();
    let bytes = bit_vec.to_bytes();
    let buffer = ByteBuf::from(bytes);

    serializer.serialize_bytes(&buffer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BitSet, D::Error> {
    struct MyVisitor { }

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = BitSet;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a BitSet")
        }

        fn visit_byte_buf<E>(self, buffer: Vec<u8>) -> Result<Self::Value, E> {
            Ok(BitSet::from_bytes(&buffer))
        }
    }

    deserializer.deserialize_byte_buf(MyVisitor { })
}
