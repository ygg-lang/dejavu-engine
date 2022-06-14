use std::{collections::BTreeMap, fmt::Display};

use diagnostic_quick::{QError, QResult};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{ser::SerializeSeq, Serialize, Serializer};

use crate::SahaNode;

pub struct SahaSerializer {}

pub struct SeqEx {
    vec: Vec<SahaNode>,
}

impl SerializeSeq for SeqEx {
    type Ok = ();
    type Error = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeSeq for SeqEx {
    type Ok = ();
    type Error = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
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
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> QResult<SahaNode> {
        Ok(SahaNode::number(Decimal::from(value)))
    }
    #[inline]
    fn serialize_i64(self, value: i64) -> QResult<SahaNode> {
        Ok(SahaNode::number(Decimal::from(value)))
    }
    #[inline]
    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        match Decimal::from_i128(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> QResult<SahaNode> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> QResult<SahaNode> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> QResult<SahaNode> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> QResult<SahaNode> {
        Ok(SahaNode::number(Decimal::from(value)))
    }
    #[inline]
    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        match Decimal::from_u128(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> QResult<SahaNode> {
        match Decimal::from_f32(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> QResult<SahaNode> {
        match Decimal::from_f64(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_char(self, value: char) -> QResult<SahaNode> {
        todo!("{value}")
    }

    #[inline]
    fn serialize_str(self, value: &str) -> QResult<SahaNode> {
        todo!("{value}")
    }

    fn serialize_bytes(self, value: &[u8]) -> QResult<SahaNode> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }

    #[inline]
    fn serialize_unit(self) -> QResult<SahaNode> {
        Ok(SahaNode::null())
    }

    #[inline]
    #[allow(unused_variables)]
    fn serialize_unit_struct(self, name: &'static str) -> QResult<SahaNode> {
        self.serialize_unit()
    }

    #[inline]
    #[allow(unused_variables)]
    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> QResult<SahaNode> {
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
        name: &'static str,
        variant_index: u32,
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

    fn serialize_seq(self, len: Option<usize>) -> QResult<Self::SerializeSeq> {
        Ok(SerializeVec { vec: Vec::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> QResult<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> QResult<Self::SerializeTupleStruct> {
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

    fn serialize_map(self, len: Option<usize>) -> QResult<Self::SerializeMap> {
        Ok(MapEx { map: Map::new(), next_key: None })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> QResult<Self::SerializeStruct> {
        match name {
            _ => self.serialize_map(Some(len)),
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> QResult<Self::SerializeStructVariant> {
        Ok(MapVariant { name: String::from(variant), map: Map::new() })
    }

    fn collect_str<T>(self, value: &T) -> QResult<SahaNode>
    where
        T: ?Sized + Display,
    {
        Ok(Value::String(value.to_string()))
    }
}
