#[cfg(test)]
mod tests;

use super::error;

use std::io::{self, Write};

use serde::{ser, Serialize};

use error::{Error, Result};

struct Serializer<'writer, W: io::Write> {
    writer: &'writer mut io::BufWriter<W>,
}

pub fn to_writer<W, T>(writer: &mut io::BufWriter<W>, value: T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let mut serializer = Serializer { writer };
    value.serialize(&mut serializer)?;
    serializer.writer.flush()?;
    Ok(())
}

impl<'a, 'writer, W: io::Write> ser::Serializer for &'a mut Serializer<'writer, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.writer.write_all(
            {
                if v {
                    "1"
                } else {
                    "0"
                }
            }
            .as_bytes(),
        )?;
        self.writer.write_all("\n".as_bytes())?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.writer.write_all(format!("b{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.writer.write_all(format!("w{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.writer.write_all(format!("i{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.writer.write_all(format!("d{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        self.writer.write_all(format!("q{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.writer.write_all(format!("B{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.writer.write_all(format!("W{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.writer.write_all(format!("I{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.writer.write_all(format!("D{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        self.writer.write_all(format!("Q{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.writer.write_all(format!("f{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.writer.write_all(format!("F{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.writer.write_all(format!("c{}\n", v).as_bytes())?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        let to_write = if v.contains(|c| c == '\n') {
            format!("&{}\n{}\n", v.len(), v)
        } else {
            format!("${}\n", v)
        };
        self.writer.write_all(to_write.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer
            .write_all(format!("%{}\n", v.len()).as_bytes())?;
        self.writer.write_all(v)?;
        self.writer.write_all("\n".as_bytes())?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.writer.write_all("!\n".as_bytes())?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.writer.write_all("~0\n".as_bytes())?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.writer
            .write_all(format!("}}0\n{}\n", name).as_bytes())?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.writer
            .write_all(format!("@{}\n${}\n", name, variant).as_bytes())?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.writer
            .write_all(format!(":1\n{}\n", name).as_bytes())?;
        value.serialize(&mut *self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.writer
            .write_all(format!("^1\n{}\n${}\n", name, variant).as_bytes())?;
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(len) => {
                self.writer.write_all(format!("`{}\n", len).as_bytes())?
            },
            None => unimplemented!(
                "Sequences without a known length before iterating are not supported by this serialization format"
            ),
        };
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.writer.write_all(format!("~{}\n", len).as_bytes())?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.writer
            .write_all(format!(":{}\n{}\n", len, name).as_bytes())?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.writer
            .write_all(format!("^{}\n{}\n${}\n", len, name, variant).as_bytes())?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        match len {
            Some(len) => self.writer.write_all(format!("{{{}\n", len).as_bytes())?,
            None => unimplemented!(
                "Maps without a known length before iterating are not supported by this serialization format"
            ),
        };
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.writer
            .write_all(format!("}}{}\n{}\n", len, name).as_bytes())?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.writer
            .write_all(format!("#{}\n{}\n${}\n", len, name, variant).as_bytes())?;
        Ok(self)
    }
}

impl<'a, 'writer, W: io::Write> ser::SerializeSeq for &'a mut Serializer<'writer, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, 'writer, W: io::Write> ser::SerializeTuple for &'a mut Serializer<'writer, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, 'writer, W: io::Write> ser::SerializeTupleStruct for &'a mut Serializer<'writer, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, 'writer, W: io::Write> ser::SerializeTupleVariant for &'a mut Serializer<'writer, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        //self.writer.write_all("\r\n\r\n".as_bytes())?;
        Ok(())
    }
}

impl<'a, 'writer, W: io::Write> ser::SerializeMap for &'a mut Serializer<'writer, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, 'writer, W: io::Write> ser::SerializeStruct for &'a mut Serializer<'writer, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, 'writer, W: io::Write> ser::SerializeStructVariant for &'a mut Serializer<'writer, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
