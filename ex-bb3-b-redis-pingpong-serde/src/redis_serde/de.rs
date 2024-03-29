#[cfg(test)]
mod tests;

use super::error;

use std::io::{self, BufRead, Read};

use serde::{
    de::{self, IntoDeserializer},
    Deserialize,
};

use error::{Error, ErrorKind, Result};

struct Deserializer<'reader, R: io::Read> {
    reader: &'reader mut io::BufReader<R>,
}

pub fn from_reader<'reader, R: io::Read, T>(reader: &'reader mut io::BufReader<R>) -> Result<T>
where
    T: Deserialize<'reader>,
{
    let mut deserializer = Deserializer { reader };
    T::deserialize(&mut deserializer)
}

impl<'a, R: io::Read> Deserializer<'a, R> {
    fn peek(&mut self) -> Result<Option<u8>> {
        let buf = self.peekn(1)?;
        match buf {
            [b] => Ok(Some(*b)),
            _ => Ok(None),
        }
    }
    fn peekn(&mut self, num: u8) -> Result<&[u8]> {
        let buf = self.reader.fill_buf()?;
        Ok(&buf[..(num as usize).min(buf.len())])
    }

    fn consume(&mut self, num: u8) {
        self.reader.consume(num as usize);
    }

    fn read_line(&mut self) -> Result<String> {
        let mut line = String::new();
        let _ = self.reader.read_line(&mut line)?;
        if line.ends_with("\r\n") {
            line.pop();
            line.pop();
            Ok(line)
        } else {
            Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "End of input reached with missing or incorrect CR\\LF pair. Input is: {}",
                    line
                ),
            })
        }
    }

    fn parse_u64(&mut self) -> Result<u64> {
        match self.peek()? {
            #[allow(clippy::char_lit_as_u8)]
            Some(b':') => {
                self.consume(1);
                Ok(self.read_line()?.parse::<u64>()?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected ':' for input of u64, found: {:?}", input),
            }),
        }
    }

    fn parse_i64(&mut self) -> Result<i64> {
        match self.peek()? {
            #[allow(clippy::char_lit_as_u8)]
            Some(b':') => {
                self.consume(1);
                Ok(self.read_line()?.parse::<i64>()?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected ':' for input of i64, found: {:?}", input),
            }),
        }
    }

    fn parse_f64(&mut self) -> Result<f64> {
        match self.peek()? {
            #[allow(clippy::char_lit_as_u8)]
            Some(b'+') => {
                self.consume(1);
                Ok(self.read_line()?.parse::<f64>()?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected ':' for input of f64, found: {:?}", input),
            }),
        }
    }

    fn parse_f32(&mut self) -> Result<f32> {
        match self.peek()? {
            #[allow(clippy::char_lit_as_u8)]
            Some(b'+') => {
                self.consume(1);
                Ok(self.read_line()?.parse::<f32>()?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected ':' for input of f32, found: {:?}", input),
            }),
        }
    }

    fn parse_char(&mut self) -> Result<char> {
        let parsed_u64 = self.parse_u64()?;
        char::from_u32(parsed_u64 as u32).ok_or(Error {
            kind: ErrorKind::DataError,
            message: format!(
                "Expected a char value in char (Unicode, 32-bit) range between 0 and {}, found {}",
                char::MAX,
                parsed_u64
            ),
        })
    }

    #[allow(clippy::char_lit_as_u8)]
    fn parse_string(&mut self) -> Result<String> {
        match self.peek()? {
            Some(b'+') => {
                self.consume(1);
                Ok(self.read_line()?)
            }
            Some(b'$') => Ok(String::from_utf8(self.parse_bytes()?)?),
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected '+' OR '$' for input of String, found: {:?}",
                    input
                ),
            }),
        }
    }

    #[allow(clippy::char_lit_as_u8)]
    fn parse_bytes(&mut self) -> Result<Vec<u8>> {
        match self.peek()? {
            Some(b'$') => {
                self.consume(1);
                let len = self.read_line()?.parse::<usize>()?;
                let mut buf = Vec::<u8>::with_capacity(len);
                buf.resize(len, Default::default());
                self.reader.read_exact(buf.as_mut())?;
                let final_delimiter = self.peekn(2)?;
                match final_delimiter {
                    [0xD, 0xA] => {
                        self.consume(2);
                        Ok(buf)
                    }
                    input => Err(Error {
                        kind: ErrorKind::DataError,
                        message: format!(
                            "Expected ending delimiter 'CR LF' for input of Bytes, found: {:?}",
                            input
                        ),
                    }),
                }
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected '$' for input of Bytes, found: {:?}", input),
            }),
        }
    }
}

macro_rules! parse_number_and_apply_visitor {
    (using $parser:ident.$parser_func:ident from $from:ident to $to:ident with $visitor:ident.$visitor_func:ident) => {{
        let value = match $parser.$parser_func()? {
            v if ($to::MIN as $from..=$to::MAX as $from).contains(&v) => v as $to,
            v => {
                return Err(Error {
                    kind: ErrorKind::DataError,
                    message: format!(
                        "Only values {} to {} permitted. Found value {}",
                        $to::MIN,
                        $to::MAX,
                        v
                    ),
                })
            }
        };
        $visitor.$visitor_func(value)
    }};
}

impl<'de, 'a, R: io::Read> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = match self.parse_u64()? {
            0 => false,
            1 => true,
            v => {
                return Err(Error {
                    kind: ErrorKind::DataError,
                    message: format!("Only 0 or 1 permitted as boolean value. Found value {}", v),
                })
            }
        };
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_i64 from i64 to i8 with visitor.visit_i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_i64 from i64 to i16 with visitor.visit_i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_i64 from i64 to i32 with visitor.visit_i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_i64 from i64 to i64 with visitor.visit_i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_u64 from u64 to u8 with visitor.visit_u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_u64 from u64 to u16 with visitor.visit_u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_u64 from u64 to u32 with visitor.visit_u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_u64 from u64 to u64 with visitor.visit_u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_f32 from f32 to f32 with visitor.visit_f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        parse_number_and_apply_visitor!(using self.parse_f64 from f64 to f64 with visitor.visit_f64)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!("Deserialization of unowned strings is not supported with this deserializer")
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!(
            "Deserialization of unowned byte arrays is not supported with this deserializer"
        )
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.parse_bytes()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peekn(5)? {
            b"$-1\r\n" => {
                self.consume(5);
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peekn(4)? {
            b"*0\r\n" => {
                self.consume(4);
                visitor.visit_unit()
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected 0 length sequence for unit tuple/struct, found input: {:?}",
                    input
                ),
            }),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'*') => {
                self.consume(1);
                let element_count = self.read_line()?.parse::<u32>()?;
                visitor.visit_seq(DeserializerSeqElements {
                    de: self,
                    element_count,
                })
            }
            Some(input) => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected * for input for beginning of sequence, found: {:?}",
                    input
                ),
            }),
            None => Err(Error {
                kind: ErrorKind::DataError,
                message:
                    "Expected * for input for beginning of sequence. Empty input/EOF found instead."
                        .into(),
            }),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'*') => {
                self.consume(1);
                let element_count = self.read_line()?.parse::<u32>()?;
                if len != element_count as usize {
                    return Err(Error {
                        kind: ErrorKind::DataError,
                        message: format!(
                            "Expected tuple of length {}, found length {}",
                            len, element_count
                        ),
                    });
                }
                visitor.visit_seq(DeserializerSeqElements {
                    de: self,
                    element_count,
                })
            }
            Some(input) => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected * for input for beginning of tuple, found: {:?}",
                    input
                ),
            }),
            None => Err(Error {
                kind: ErrorKind::DataError,
                message:
                    "Expected * for input for beginning of tuple. Empty input/EOF found instead."
                        .into(),
            }),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'*') => {
                self.consume(1);
                let element_count = self.read_line()?.parse::<u32>()?;
                visitor.visit_map(DeserializerSeqElements {
                    de: self,
                    element_count,
                })
            }
            Some(input) => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected * for input for beginning of Map, found: {:?}",
                    input
                ),
            }),
            None => Err(Error {
                kind: ErrorKind::DataError,
                message:
                    "Expected * for input for beginning of Map. Empty input/EOF found instead."
                        .into(),
            }),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b':') => visitor.visit_enum(variants[self.parse_u64()? as usize].into_deserializer()),
            Some(b'*') => match self.peekn(4)? {
                b"*2\r\n" => {
                    self.consume(4);
                    Ok(visitor.visit_enum(DeserializeEnum{de:self})?)
                },
                input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected *2CR\\LF for input at beginning of Non-Unit Enum, found: {:?}",
                    input
                ),
            })
            },
            Some(input) => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected * or : for input for beginning of Enum, found: {:?}",
                    input
                ),
            }),
            None => Err(Error {
                kind: ErrorKind::DataError,
                message:
                    "Expected * or : for input for beginning of Enum. Empty input/EOF found instead."
                        .into(),
            }),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'+') => self.deserialize_string(visitor),
            Some(b':') => self.deserialize_u32(visitor),
            Some(input) => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected + or : for input of Identifier, found: {:?}",
                    input
                ),
            }),
            None => Err(Error {
                kind: ErrorKind::DataError,
                message: "Expected + or : for input of Identifier. Empty input/EOF found instead."
                    .into(),
            }),
        }
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

struct DeserializerSeqElements<'a, 'de: 'a, R: io::Read> {
    de: &'a mut Deserializer<'de, R>,
    element_count: u32,
}

impl<'de, 'a, R: io::Read> de::SeqAccess<'de> for DeserializerSeqElements<'a, 'de, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.element_count == 0 {
            return Ok(None);
        }
        self.element_count -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de, 'a, R: io::Read> de::MapAccess<'de> for DeserializerSeqElements<'a, 'de, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.element_count == 0 {
            return Ok(None);
        }
        self.element_count -= 1;
        match self.de.peek()? {
            Some(b'*') => {
                self.de.consume(1);
                match self.de.read_line()?.parse::<u32>()? {
                    2 => seed.deserialize(&mut *self.de).map(Some),
                    input => Err(Error {
                        kind: ErrorKind::DataError,
                        message: format!(
                            "Expected len 2 for pair/entry of map, Found input: {:?}",
                            input
                        ),
                    }),
                }
            }
            Some(input) => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected * at beginning of map pair/entry, Found input: {:?}",
                    input
                ),
            }),
            None => Err(Error {
                kind: ErrorKind::DataError,
                message: "Expected * at beginning of map pair/entry, Found Nothing/EOF".into(),
            }),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct DeserializeEnum<'a, 'de: 'a, R: io::Read> {
    de: &'a mut Deserializer<'de, R>,
}

impl<'de, 'a, R: io::Read> de::EnumAccess<'de> for DeserializeEnum<'a, 'de, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a, R: io::Read> de::VariantAccess<'de> for DeserializeEnum<'a, 'de, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        unimplemented!("should never be called - unit variants handled immediately")
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}
