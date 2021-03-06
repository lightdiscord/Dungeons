use crate::error::{Result, Error};
use crate::types::Var;
use serde::de::{self, Visitor, DeserializeSeed};
use bytes::Buf;

use super::Deserializer;

type StaticStr = &'static str;

macro_rules! deserialize_unimplemented {
    ($($fn:ident$(($($type:ty),*))*),*) => {
        $(
            fn $fn<V>(self, $($(_: $type),*,)* _: V) -> Result<V::Value>
            where
                V: Visitor<'de>
            {
                unimplemented!(stringify!($fn))
            }
        )*
    };
}

macro_rules! deserialize_simple {
    ($(($deserialize:ident, $visit:ident, $get:ident)),*) => {
        $(
            fn $deserialize<V>(self, visitor: V) -> Result<V::Value>
            where
                V: Visitor<'de>
            {
                visitor.$visit((self.0).$get())
            }
        )*
    }
}

struct SeqAccess<'a>(&'a mut Deserializer);

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>
    {
        Ok(Some(T::deserialize(seed, &mut *self.0)?))
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        visitor.visit_bool((self.0).get_u8() != 0)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        let length = *self.deserialize::<Var<i32>>()? as usize;
        let bytes = self.0.split_to(length);
        let string = std::str::from_utf8(&bytes)?;
        visitor.visit_str(string)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        visitor.visit_seq(SeqAccess(self))
    }

    fn deserialize_unit_struct<V>(self, _: StaticStr, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple<V>(self, _: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_struct<V>(self, _: StaticStr, _: &'static [StaticStr], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        let length = *self.deserialize::<Var<i32>>()? as usize;
        visitor.visit_bytes(&self.0.split_to(length))
    }

    deserialize_unimplemented! {
        deserialize_any,
        deserialize_char,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i8,
        deserialize_identifier,
        deserialize_ignored_any,
        deserialize_map,
        deserialize_option,
        deserialize_u32,
        deserialize_bytes,
        deserialize_unit,
        deserialize_newtype_struct(StaticStr),
        deserialize_tuple_struct(StaticStr, usize),
        deserialize_enum(StaticStr, &'static [StaticStr])
    }

    deserialize_simple! {
        (deserialize_u8, visit_u8, get_u8),
        (deserialize_u16, visit_u16, get_u16),
        (deserialize_u64, visit_u64, get_u64),
        (deserialize_f32, visit_f32, get_f32),
        (deserialize_f64, visit_f64, get_f64)
    }
}
