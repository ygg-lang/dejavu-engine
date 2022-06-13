use std::collections::BTreeMap;

use diagnostic_quick::{QError, QResult};
use rust_decimal::Decimal;
use serde::Serializer;

use crate::SahaNode;

pub struct SahaSerializer {}

pub struct SeqEx {
    vec: Vec<SahaNode>,
}
pub struct SeqVariant {
    name: String,
    vec: Vec<SahaNode>,
}

pub struct MapEx {
    map: BTreeMap<String, SahaNode>,
}

pub struct MapVariant {
    name: String,
    map: BTreeMap<String, SahaNode>,
}

impl Serializer for SahaSerializer {
    type Ok = SahaNode;
    type Error = QError;

    type SerializeSeq = SeqEx;
    type SerializeTuple = SeqEx;
    type SerializeTupleStruct = SeqEx;
    type SerializeTupleVariant = SeqVariant;
    type SerializeMap = MapEx;
    type SerializeStruct = MapEx;
    type SerializeStructVariant = MapVariant;

    #[inline]
    fn serialize_bool(self, value: bool) -> QResult<SahaNode> {
        Ok(SahaNode::boolean(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> QResult<SahaNode> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> QResult<SahaNode> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> QResult<SahaNode> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> QResult<SahaNode> {
        Ok(Value::Number(value.into()))
    }

    #[cfg(feature = "arbitrary_precision")]
    fn serialize_i128(self, value: i128) -> QResult<SahaNode> {
        Ok(Value::Number(value.into()))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> QResult<SahaNode> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> QResult<SahaNode> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> QResult<SahaNode> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> QResult<SahaNode> {
        Ok(Value::Number(value.into()))
    }

    #[cfg(feature = "arbitrary_precision")]
    fn serialize_u128(self, value: u128) -> QResult<SahaNode> {
        Ok(Value::Number(value.into()))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> QResult<SahaNode> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> QResult<SahaNode> {
        Ok(Number::from_f64(value).map_or(Value::Null, Value::Number))
    }

    #[inline]
    fn serialize_char(self, value: char) -> QResult<SahaNode> {
        let mut s = String::new();
        s.push(value);
        Ok(Value::String(s))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> QResult<SahaNode> {
        Ok(Value::String(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> QResult<SahaNode> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }

    #[inline]
    fn serialize_unit(self) -> QResult<SahaNode> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> QResult<SahaNode> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> QResult<SahaNode> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> QResult<SahaNode>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> QResult<SahaNode>
    where
        T: ?Sized + Serialize,
    {
        let mut values = Map::new();
        values.insert(String::from(variant), tri!(to_value(value)));
        Ok(Value::Object(values))
    }

    #[inline]
    fn serialize_none(self) -> QResult<SahaNode> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> QResult<SahaNode>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec { vec: Vec::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SeqVariant { name: String::from(variant), vec: Vec::with_capacity(len) })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapEx::Map { map: Map::new(), next_key: None })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(MapEx::Number { out_value: None }),
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(MapEx::RawValue { out_value: None }),
            _ => self.serialize_map(Some(len)),
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(MapVariant { name: String::from(variant), map: Map::new() })
    }

    fn collect_str<T>(self, value: &T) -> QResult<SahaNode>
    where
        T: ?Sized + Display,
    {
        Ok(Value::String(value.to_string()))
    }
}
