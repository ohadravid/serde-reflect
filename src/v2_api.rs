use std::{iter::Peekable, slice::Iter};

use serde::{
    Deserialize, Deserializer,
    de::{
        DeserializeOwned, Error, IntoDeserializer, Visitor,
        value::{self, BorrowedStrDeserializer, StrDeserializer},
    },
    forward_to_deserialize_any,
};

use crate::{meta, raw_api};

#[derive(Deserialize)]
#[serde(rename = "Win32_Fan")]
#[serde(rename_all = "PascalCase")]
pub struct Fan {
    pub name: String,
    pub active_cooling: bool,
    pub desired_speed: u64,
}

pub fn query<T: DeserializeOwned>() -> Result<Vec<T>, value::Error> {
    let (name, _fields) = meta::struct_name_and_fields::<T>()?;

    let mut res = vec![];

    for obj in raw_api::query(&format!("SELECT * FROM {name}")) {
        res.push(T::deserialize(ObjectDeserializer { obj })?)
    }

    Ok(res)
}

struct ObjectDeserializer {
    obj: raw_api::Object,
}

impl<'de> Deserializer<'de> for ObjectDeserializer {
    type Error = value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Self::Error::custom(
            "Only `struct` deserialization is supported",
        ))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        struct ObjectMapAccess {
            fields: Peekable<Iter<'static, &'static str>>,
            obj: raw_api::Object,
        }

        let map = ObjectMapAccess {
            fields: fields.iter().peekable(),
            obj: self.obj,
        };

        // `MapAccess` is provided to the `Visitor` to give it the ability to iterate
        // through entries of the map.
        impl<'de> serde::de::MapAccess<'de> for ObjectMapAccess {
            type Error = serde::de::value::Error;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
            where
                K: serde::de::DeserializeSeed<'de>,
            {
                if let Some(field) = self.fields.peek() {
                    let field_deser = StrDeserializer::new(field);
                    seed.deserialize(field_deser).map(Some)
                } else {
                    Ok(None)
                }
            }

            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let current_field = self
                    .fields
                    .next()
                    .expect("Next value should only be called after next_key returned something");

                let field_value = self.obj.get_attr(current_field);

                struct ValueDeserializer {
                    value: raw_api::Value,
                }

                impl<'de> Deserializer<'de> for ValueDeserializer {
                    type Error = serde::de::value::Error;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                    where
                        V: Visitor<'de>,
                    {
                        match self.value {
                            raw_api::Value::Null => visitor.visit_none(),
                            raw_api::Value::Bool(b) => visitor.visit_bool(b),
                            raw_api::Value::I1(v) => visitor.visit_i8(v),
                            raw_api::Value::I2(v) => visitor.visit_i16(v),
                            raw_api::Value::I4(v) => visitor.visit_i32(v),
                            raw_api::Value::I8(v) => visitor.visit_i64(v),
                            raw_api::Value::UI1(v) => visitor.visit_u8(v),
                            raw_api::Value::UI2(v) => visitor.visit_u16(v),
                            raw_api::Value::UI4(v) => visitor.visit_u32(v),
                            raw_api::Value::UI8(v) => visitor.visit_u64(v),
                            raw_api::Value::R4(v) => visitor.visit_f32(v),
                            raw_api::Value::R8(v) => visitor.visit_f64(v),
                            raw_api::Value::String(s) => visitor.visit_string(s),
                            other => Err(Self::Error::custom(format!(
                                "Unimplemented Value variant {:?}",
                                other
                            ))),
                        }
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map enum struct identifier ignored_any
                    }
                }

                seed.deserialize(ValueDeserializer { value: field_value })
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.fields.size_hint().0)
            }
        }

        visitor.visit_map(map)
    }
}
