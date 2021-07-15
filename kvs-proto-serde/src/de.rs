#[cfg(test)]
mod tests;

use super::error;

use std::{
    io::{self, BufRead, Read},
    str,
};

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

macro_rules! parse_number {
    (from $self:ident type $type:ident indicated by $indicator:expr) => {{
        match $self.peek()? {
            Some($indicator) => {
                $self.consume(1);
                Ok($self.read_line()?.parse::<$type>()?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected '{}' for input of {}, found: {:?}",
                    stringify!($indicator),
                    stringify!($type),
                    input
                ),
            }),
        }
    }};
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
        if line.ends_with('\n') {
            line.pop();
            Ok(line)
        } else {
            Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "End of input reached with missing or incorrect ending LF. Input is: {}",
                    line
                ),
            })
        }
    }

    fn read_and_verify_name(&mut self, name: &str) -> Result<()> {
        let the_name = self.read_line()?;
        if name != "*" && the_name != name {
            return Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected struct name, {}, for tuple struct found: {}",
                    name, the_name
                ),
            });
        }
        Ok(())
    }

    fn read_and_verify_length(&mut self, len: usize, looking_for: &'static str) -> Result<()> {
        let element_count = self.read_length()?;
        self.verify_length(len, element_count as usize, looking_for)
    }

    fn read_length(&mut self) -> Result<u32> {
        Ok(self.read_line()?.parse::<u32>()?)
    }

    fn verify_length(
        &self,
        len: usize,
        element_count: usize,
        looking_for: &'static str,
    ) -> Result<()> {
        if len != element_count as usize {
            return Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected length for {} of {}, found length {}",
                    looking_for, len, element_count
                ),
            });
        }
        Ok(())
    }

    fn read_exact_given_discarding_ending_newline(&mut self) -> Result<Vec<u8>> {
        let len = self.read_line()?.parse::<usize>()?;
        let mut buf = Vec::<u8>::with_capacity(len);
        buf.resize(len, Default::default());
        self.reader.read_exact(buf.as_mut())?;
        match self.peek()? {
            Some(b'\n') => {
                self.consume(1);
                Ok(buf)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected ending delimiter 'LF' for input of Length given data, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn parse_bool(&mut self) -> Result<bool> {
        match self.peekn(2)? {
            b"1\n" => {
                self.consume(2);
                Ok(true)
            }
            b"0\n" => {
                self.consume(2);
                Ok(false)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected 1 or 0 for boolean followed by newline, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn parse_char(&mut self) -> Result<char> {
        parse_number!(from self type char indicated by b'c')
    }

    fn parse_u8(&mut self) -> Result<u8> {
        parse_number!(from self type u8 indicated by b'B')
    }

    fn parse_u16(&mut self) -> Result<u16> {
        parse_number!(from self type u16 indicated by b'W')
    }

    fn parse_u32(&mut self) -> Result<u32> {
        parse_number!(from self type u32 indicated by b'I')
    }

    fn parse_u64(&mut self) -> Result<u64> {
        parse_number!(from self type u64 indicated by b'D')
    }

    fn parse_u128(&mut self) -> Result<u128> {
        parse_number!(from self type u128 indicated by b'Q')
    }

    fn parse_i8(&mut self) -> Result<i8> {
        parse_number!(from self type i8 indicated by b'b')
    }

    fn parse_i16(&mut self) -> Result<i16> {
        parse_number!(from self type i16 indicated by b'w')
    }

    fn parse_i32(&mut self) -> Result<i32> {
        parse_number!(from self type i32 indicated by b'i')
    }

    fn parse_i64(&mut self) -> Result<i64> {
        parse_number!(from self type i64 indicated by b'd')
    }

    fn parse_i128(&mut self) -> Result<i128> {
        parse_number!(from self type i128 indicated by b'q')
    }

    fn parse_f32(&mut self) -> Result<f32> {
        parse_number!(from self type f32 indicated by b'f')
    }

    fn parse_f64(&mut self) -> Result<f64> {
        parse_number!(from self type f64 indicated by b'F')
    }

    fn parse_string(&mut self) -> Result<String> {
        match self.peek()? {
            Some(b'$') => {
                self.consume(1);
                Ok(self.read_line()?)
            }
            Some(b'&') => {
                self.consume(1);
                Ok(String::from_utf8(
                    self.read_exact_given_discarding_ending_newline()?,
                )?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected '$' OR '&' for input of String, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn parse_bytes(&mut self) -> Result<Vec<u8>> {
        match self.peek()? {
            Some(b'%') => {
                self.consume(1);
                Ok(self.read_exact_given_discarding_ending_newline()?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected '%' for input of Bytes, found: {:?}", input),
            }),
        }
    }
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
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i128(self.parse_i128()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u128(self.parse_u128()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
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
        match self.peekn(2)? {
            b"!\n" => {
                self.consume(2);
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peekn(3)? {
            b"~0\n" => {
                self.consume(3);
                visitor.visit_unit()
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected 0 length for unit tuple, found input: {:?}", input),
            }),
        }
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_struct(name, &[], visitor)
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple_struct(name, 1, visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'`') => {
                self.consume(1);
                let element_count = self.read_line()?.parse::<u32>()?;
                visitor.visit_seq(DeserializerSeqElements {
                    de: self,
                    element_count,
                })
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected ` for input at beginning of sequence, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'~') => {
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
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected ~ for input at beginning of tuple, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b':') => {
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
                self.read_and_verify_name(name)?;
                visitor.visit_seq(DeserializerSeqElements {
                    de: self,
                    element_count,
                })
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected ~ for input at beginning of tuple, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'{') => {
                self.consume(1);
                let element_count = self.read_line()?.parse::<u32>()?;
                visitor.visit_map(DeserializerSeqElements {
                    de: self,
                    element_count,
                })
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected {{ for input at beginning of Map, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'}') => {
                self.consume(1);
                self.read_and_verify_length(fields.len(), "tuple")?;
                self.read_and_verify_name(name)?;
                if fields.is_empty() {
                    visitor.visit_unit()
                } else {
                    visitor.visit_map(DeserializerSeqElements {
                        de: self,
                        element_count: fields.len() as u32,
                    })
                }
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected {{ for input at beginning of Map, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peekn(1)? {
            b"@" => {
                // Unit Variant
                self.consume(1);
                self.read_and_verify_name(name)?;
                let variant = self.parse_string()?;
                visitor.visit_enum(variant.into_deserializer())
            }
            b"^" => {
                // Tuple/New-Type Variant
                self.consume(1);
                let element_count = self.read_length()?;
                self.read_and_verify_name(name)?;
                Ok(visitor.visit_enum(DeserializeEnum {
                    de: self,
                    element_count,
                })?)
            }
            b"#" => {
                // Struct Variant
                self.consume(1);
                let element_count = self.read_length()?;
                self.read_and_verify_name(name)?;
                Ok(visitor.visit_enum(DeserializeEnum {
                    de: self,
                    element_count,
                })?)
            }
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected @, ^, or # for input at beginning of Enum, found: {:?}",
                    input
                ),
            }),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.peek()? {
            Some(b'$') => self.deserialize_string(visitor),
            input => Err(Error {
                kind: ErrorKind::DataError,
                message: format!("Expected $ for input of Identifier, found: {:?}", input),
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
        seed.deserialize(&mut *self.de).map(Some)
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
    element_count: u32,
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
        if self.element_count as usize != len {
            return Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected length {} for Enum Tuple Variant, found: {}",
                    len, self.element_count
                ),
            });
        }
        visitor.visit_seq(DeserializerSeqElements {
            de: self.de,
            element_count: len as u32,
        })
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.element_count as usize != fields.len() {
            return Err(Error {
                kind: ErrorKind::DataError,
                message: format!(
                    "Expected length {} for Enum Structure Variant, found: {}",
                    fields.len(),
                    self.element_count
                ),
            });
        }
        if fields.is_empty() {
            visitor.visit_unit()
        } else {
            visitor.visit_map(DeserializerSeqElements {
                de: self.de,
                element_count: self.element_count,
            })
        }
    }
}
