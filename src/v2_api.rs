use std::{iter::Peekable, slice::Iter};

use serde::{
    Deserialize, Deserializer,
    de::{
        DeserializeOwned, IntoDeserializer, Visitor,
        value::{BorrowedStrDeserializer, StrDeserializer},
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

pub fn query<T: DeserializeOwned>() -> Vec<T> {
    let (name, _fields) = meta::struct_name_and_fields::<T>().unwrap();

    let mut res = vec![];

    for obj in raw_api::query(&format!("SELECT * FROM {name}")) {
        res.push(T::deserialize(ObjectDeserializer { obj }).unwrap())
    }

    res
}

struct ObjectDeserializer {
    obj: raw_api::Object,
}

impl<'de> Deserializer<'de> for ObjectDeserializer {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!()
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
                let current_field = self.fields.next().unwrap();

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
                            raw_api::Value::Bool(b) => visitor.visit_bool(b),
                            raw_api::Value::I1(v) => visitor.visit_i8(v),
                            // ..
                            raw_api::Value::UI8(v) => visitor.visit_u64(v),
                            raw_api::Value::String(s) => visitor.visit_string(s),
                            _ => todo!(),
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
