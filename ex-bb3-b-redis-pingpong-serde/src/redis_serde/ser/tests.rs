use std::fmt::Display;

use super::*;

fn test_integer<T: Display + Serialize>(value: T) -> Result<()> {
    let expected = format!(":{}\r\n", value);
    let mut actual = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut actual), &value)?;
    assert_eq!(expected.as_bytes(), actual.as_slice());
    Ok(())
}

fn test_float<T: Display + Serialize>(value: T) -> Result<()> {
    let expected = format!("+{}\r\n", value);
    let mut actual = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut actual), &value)?;
    assert_eq!(expected.as_bytes(), actual.as_slice());
    Ok(())
}

#[test]
fn test_bool() -> Result<()> {
    let mut buf = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut buf), &true)?;
    assert_eq!(":1\r\n".as_bytes(), buf.as_slice());

    let mut buf = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut buf), &false)?;
    assert_eq!(":0\r\n".as_bytes(), buf.as_slice());

    Ok(())
}

#[test]
fn test_i8() -> Result<()> {
    test_integer(i8::MIN)?;
    test_integer(-1_i8)?;
    test_integer(0_i8)?;
    test_integer(1_i8)?;
    test_integer(i8::MAX)
}

#[test]
fn test_i16() -> Result<()> {
    test_integer(i16::MIN)?;
    test_integer(-1_i16)?;
    test_integer(0_i16)?;
    test_integer(1_i16)?;
    test_integer(i16::MAX)
}

#[test]
fn test_i32() -> Result<()> {
    test_integer(i32::MIN)?;
    test_integer(-1_i32)?;
    test_integer(0_i32)?;
    test_integer(1_i32)?;
    test_integer(i32::MAX)
}

#[test]
fn test_i64() -> Result<()> {
    test_integer(i64::MIN)?;
    test_integer(-1_i64)?;
    test_integer(0_i64)?;
    test_integer(1_i64)?;
    test_integer(i64::MAX)
}

#[test]
fn test_u8() -> Result<()> {
    test_integer(u8::MIN)?;
    test_integer(1_u8)?;
    test_integer(u8::MAX)
}

#[test]
fn test_u16() -> Result<()> {
    test_integer(u16::MIN)?;
    test_integer(1_u16)?;
    test_integer(u16::MAX)
}

#[test]
fn test_u32() -> Result<()> {
    test_integer(u32::MIN)?;
    test_integer(1_u32)?;
    test_integer(u32::MAX)
}

#[test]
fn test_u64() -> Result<()> {
    test_integer(u64::MIN)?;
    test_integer(1_u64)?;
    test_integer(u64::MAX)
}

#[test]
fn test_f32() -> Result<()> {
    test_float(f32::MIN)?;
    test_float(-1_f32)?;
    test_float(0_f32)?;
    test_float(1_f32)?;
    test_float(f32::MAX)
}

#[test]
fn test_f64() -> Result<()> {
    test_float(f64::MIN)?;
    test_float(-1_f64)?;
    test_float(0_f64)?;
    test_float(1_f64)?;
    test_float(f64::MAX)
}

#[test]
fn test_char() -> Result<()> {
    let chars_to_test = [
        '\0', '\t', '\r', '\n', 'A', 'Z', 'a', 'z', '0', '9', '!', ')', '~', '∑', '𖿢',
    ];
    let mut buf = Vec::<u8>::new();
    for char in chars_to_test {
        let expected = format!(":{}\r\n", char as u32);
        to_writer(&mut io::BufWriter::new(&mut buf), char)?;
        assert_eq!(expected.as_bytes(), buf.as_slice());
        buf.clear();
    }
    Ok(())
}

#[test]
fn test_str() -> Result<()> {
    let strs_to_test = [
        "this is a test",
        "THIS",
        "This is also\ra test",
        "This is a\ntest too!",
        "This is\r\nanother test",
        "This is a test with a special char and a chinese char ∑, 𖿢",
        "This is another test\r\nwith chinese char and a special char 𖿢, ∑ too!",
    ];
    let mut buf = Vec::<u8>::new();
    for str in strs_to_test {
        let expected = if str.contains(|c| c == '\r' || c == '\n') {
            format!("${}\r\n{}\r\n", str.len(), str)
        } else {
            format!("+{}\r\n", str)
        };
        to_writer(&mut io::BufWriter::new(&mut buf), str)?;
        assert_eq!(expected.as_bytes(), buf.as_slice());
        buf.clear();
    }
    Ok(())
}

#[test]
fn test_bytes() -> Result<()> {
    let byte_slices_to_test = [
        "this is a test".as_bytes(),
        "THIS".as_bytes(),
        "This is also\ra test".as_bytes(),
        "This is a\ntest too!".as_bytes(),
        "This is\r\nanother test".as_bytes(),
        "This is a test with a special char and a chinese char ∑, 𖿢".as_bytes(),
        "This is another test\r\nwith chinese char and a special char 𖿢, ∑ too!".as_bytes(),
    ];
    let mut buf = Vec::<u8>::new();
    for bytes in byte_slices_to_test {
        let mut expected = format!("${}\r\n", bytes.len()).as_bytes().to_vec();
        expected.append(&mut bytes.to_vec());
        expected.append(&mut "\r\n".as_bytes().to_vec());
        to_writer(
            &mut io::BufWriter::new(&mut buf),
            serde_bytes::ByteBuf::from(bytes),
        )?;
        assert_eq!(expected.as_slice(), buf.as_slice());
        buf.clear();
    }
    Ok(())
}

#[test]
fn test_none() -> Result<()> {
    let expected = "$-1\r\n";
    let mut buf = Vec::<u8>::new();
    to_writer::<_, Option<()>>(&mut io::BufWriter::new(&mut buf), None)?;
    assert_eq!(expected.as_bytes(), buf.as_slice());
    Ok(())
}

#[test]
fn test_some() -> Result<()> {
    let expected = "+This is a test\r\n";
    let mut buf = Vec::<u8>::new();
    to_writer::<_, Option<&str>>(&mut io::BufWriter::new(&mut buf), Some("This is a test"))?;
    assert_eq!(expected.as_bytes(), buf.as_slice());
    Ok(())
}

#[test]
fn test_unit() -> Result<()> {
    let expected = "*0\r\n";
    let mut buf = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut buf), ())?;
    assert_eq!(expected.as_bytes(), buf.as_slice());
    Ok(())
}

mod test_unit_struct {
    use super::super::*;

    #[derive(Serialize)]
    struct Unit;

    #[test]
    fn test_unit_struct() -> Result<()> {
        let expected = "*0\r\n";
        let mut buf = Vec::<u8>::new();
        to_writer(&mut io::BufWriter::new(&mut buf), Unit)?;
        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}

mod test_unit_variant {
    use super::super::*;

    #[derive(Serialize)]
    #[allow(clippy::enum_variant_names, dead_code)]
    enum ContainsUnitVariants {
        Unit1,
        Unit2,
        Unit3,
        NonUnit(i32),
        Unit4,
    }

    #[test]
    fn test_unit_variant() -> Result<()> {
        let mut buf = Vec::<u8>::new();
        to_writer(
            &mut io::BufWriter::new(&mut buf),
            ContainsUnitVariants::Unit1,
        )?;
        to_writer(
            &mut io::BufWriter::new(&mut buf),
            ContainsUnitVariants::Unit2,
        )?;
        to_writer(
            &mut io::BufWriter::new(&mut buf),
            ContainsUnitVariants::Unit3,
        )?;
        to_writer(
            &mut io::BufWriter::new(&mut buf),
            ContainsUnitVariants::Unit4,
        )?;
        assert_eq!(":0\r\n:1\r\n:2\r\n:4\r\n".as_bytes(), buf.as_slice());
        Ok(())
    }
}

mod test_newtype_struct {
    use super::super::*;

    #[derive(Serialize)]
    struct NewTypeBool(bool);

    #[derive(Serialize)]
    struct NewTypeU8(u8);

    #[derive(Serialize)]
    struct NewTypeI64(i64);

    #[derive(Serialize)]
    struct NewTypeString(String);

    #[test]
    fn test_newtype_struct_bool() -> Result<()> {
        let expected = ":1\r\n:0\r\n:0\r\n:1\r\n".as_bytes();
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, NewTypeBool(true))?;
            to_writer(&mut buf_writer, NewTypeBool(false))?;
            to_writer(&mut buf_writer, NewTypeBool(false))?;
            to_writer(&mut buf_writer, NewTypeBool(true))?;
        }
        assert_eq!(expected, buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_newtype_struct_u8() -> Result<()> {
        let expected = ":0\r\n:1\r\n:127\r\n:255\r\n".as_bytes();
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, NewTypeU8(u8::MIN))?;
            to_writer(&mut buf_writer, NewTypeU8(1))?;
            to_writer(&mut buf_writer, NewTypeU8(127))?;
            to_writer(&mut buf_writer, NewTypeU8(u8::MAX))?;
        }
        assert_eq!(expected, buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_newtype_struct_i64() -> Result<()> {
        let expected = format!(
            ":{}\r\n:{}\r\n:{}\r\n:{}\r\n:{}\r\n",
            i64::MIN,
            -1_i64,
            0_i64,
            1_i64,
            i64::MAX
        );
        let expected = expected.as_bytes();
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, NewTypeI64(i64::MIN))?;
            to_writer(&mut buf_writer, NewTypeI64(-1))?;
            to_writer(&mut buf_writer, NewTypeI64(0))?;
            to_writer(&mut buf_writer, NewTypeI64(1))?;
            to_writer(&mut buf_writer, NewTypeI64(i64::MAX))?;
        }
        assert_eq!(expected, buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_newtype_struct_string() -> Result<()> {
        let expected = format!(
            "+{}\r\n+{}\r\n+{}\r\n+{}\r\n+{}\r\n${}\r\n{}\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
            "",
            " ",
            "   ",
            "  Test  ",
            "This is a test...∑, 𖿢",
            "This is a\r\ntest...∑, 𖿢".len(),
            "This is a\r\ntest...∑, 𖿢",
            "This is a\rtest...∑, 𖿢".len(),
            "This is a\rtest...∑, 𖿢",
            "This is a\ntest...∑, 𖿢".len(),
            "This is a\ntest...∑, 𖿢"
        );
        let expected = expected.as_bytes();
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, NewTypeString("".into()))?;
            to_writer(&mut buf_writer, NewTypeString(" ".into()))?;
            to_writer(&mut buf_writer, NewTypeString("   ".into()))?;
            to_writer(&mut buf_writer, NewTypeString("  Test  ".into()))?;
            to_writer(
                &mut buf_writer,
                NewTypeString("This is a test...∑, 𖿢".into()),
            )?;
            to_writer(
                &mut buf_writer,
                NewTypeString("This is a\r\ntest...∑, 𖿢".into()),
            )?;
            to_writer(
                &mut buf_writer,
                NewTypeString("This is a\rtest...∑, 𖿢".into()),
            )?;
            to_writer(
                &mut buf_writer,
                NewTypeString("This is a\ntest...∑, 𖿢".into()),
            )?;
        }
        assert_eq!(expected, buf.as_slice());
        Ok(())
    }
}

mod test_newtype_variant {
    use super::super::*;

    #[derive(Serialize)]
    enum NewTypeVariants {
        Bool(bool),
        String(String),
    }

    #[test]
    fn test_newtype_variant_bool() -> Result<()> {
        let expected =
            "*2\r\n:0\r\n:1\r\n*2\r\n:0\r\n:0\r\n*2\r\n:0\r\n:0\r\n*2\r\n:0\r\n:1\r\n".as_bytes();
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, NewTypeVariants::Bool(true))?;
            to_writer(&mut buf_writer, NewTypeVariants::Bool(false))?;
            to_writer(&mut buf_writer, NewTypeVariants::Bool(false))?;
            to_writer(&mut buf_writer, NewTypeVariants::Bool(true))?;
        }
        assert_eq!(expected, buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_newtype_variant_string() -> Result<()> {
        let string1 = "This is a test".to_owned();
        let string2 = "This is\r\nalso a test".to_owned();
        let string3 = "This is another test...∑, 𖿢".to_owned();
        let expected = format!(
            "*2\r\n:1\r\n+{}\r\n*2\r\n:1\r\n${}\r\n{}\r\n*2\r\n:1\r\n+{}\r\n",
            string1,
            string2.len(),
            string2,
            string3
        );
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, NewTypeVariants::String(string1))?;
            to_writer(&mut buf_writer, NewTypeVariants::String(string2))?;
            to_writer(&mut buf_writer, NewTypeVariants::String(string3))?;
        }
        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}

mod test_seq {
    use super::super::*;

    #[test]
    fn test_seq_bool() -> Result<()> {
        let bools = [true, false, false, true];
        let expected = "*4\r\n:1\r\n:0\r\n:0\r\n:1\r\n";
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, bools)?;
        }
        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_seq_i16() -> Result<()> {
        let i16s = [i16::MIN, -1, 0, 1, i16::MAX];
        let expected = format!(
            "*5\r\n:{}\r\n:{}\r\n:{}\r\n:{}\r\n:{}\r\n",
            i16s[0], i16s[1], i16s[2], i16s[3], i16s[4]
        );
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, i16s)?;
        }
        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_seq_string() -> Result<()> {
        let strings = ["Test1", "Test\r\n2", "Test\r3", "Test4"];
        let expected = format!(
            "*4\r\n+{}\r\n${}\r\n{}\r\n${}\r\n{}\r\n+{}\r\n",
            strings[0],
            strings[1].len(),
            strings[1],
            strings[2].len(),
            strings[2],
            strings[3],
        );
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, strings)?;
        }
        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}

#[test]
fn test_tuple() -> Result<()> {
    let tuple = (
        true,
        '\0',
        char::MAX,
        u8::MIN,
        u16::MAX,
        u32::MIN,
        u64::MAX,
        i8::MIN,
        i16::MAX,
        i32::MIN,
        i64::MAX,
        "Test this is",
        "This is a\r\ntest",
        (5, 6, 7, "Test Also"),
        [8, 9, 10],
    );
    let mut expected = "*15\r\n:1\r\n".to_owned();
    expected += format!(":{}\r\n", tuple.1 as u32).as_str();
    expected += format!(":{}\r\n", tuple.2 as u32).as_str();
    expected += format!(":{}\r\n", tuple.3).as_str();
    expected += format!(":{}\r\n", tuple.4).as_str();
    expected += format!(":{}\r\n", tuple.5).as_str();
    expected += format!(":{}\r\n", tuple.6).as_str();
    expected += format!(":{}\r\n", tuple.7).as_str();
    expected += format!(":{}\r\n", tuple.8).as_str();
    expected += format!(":{}\r\n", tuple.9).as_str();
    expected += format!(":{}\r\n", tuple.10).as_str();
    expected += format!("+{}\r\n", tuple.11).as_str();
    expected += format!("${}\r\n{}\r\n", tuple.12.len(), tuple.12).as_str();
    expected += "*4\r\n";
    expected += format!(":{}\r\n", tuple.13 .0).as_str();
    expected += format!(":{}\r\n", tuple.13 .1).as_str();
    expected += format!(":{}\r\n", tuple.13 .2).as_str();
    expected += format!("+{}\r\n", tuple.13 .3).as_str();
    expected += "*3\r\n";
    expected += format!(":{}\r\n", tuple.14[0]).as_str();
    expected += format!(":{}\r\n", tuple.14[1]).as_str();
    expected += format!(":{}\r\n", tuple.14[2]).as_str();

    let mut buf = Vec::new();
    {
        let mut buf_writer = io::BufWriter::new(&mut buf);
        to_writer(&mut buf_writer, tuple)?;
    }

    assert_eq!(expected.as_bytes(), buf.as_slice());
    Ok(())
}

mod test_tuple_struct {
    use super::super::*;

    #[derive(Serialize)]
    struct TupleStruct<'a>(
        bool,
        char,
        char,
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        &'a str,
        &'a str,
        (u32, u8, i16, &'a str),
        [u32; 3],
    );

    #[test]
    fn test_tuple_struct() -> Result<()> {
        let tuple = TupleStruct(
            true,
            '\0',
            char::MAX,
            u8::MIN,
            u16::MAX,
            u32::MIN,
            u64::MAX,
            i8::MIN,
            i16::MAX,
            i32::MIN,
            i64::MAX,
            "Test this is",
            "This is a\r\ntest",
            (5, 6, 7, "Test Also"),
            [8, 9, 10],
        );
        let mut expected = "*15\r\n:1\r\n".to_owned();
        expected += format!(":{}\r\n", tuple.1 as u32).as_str();
        expected += format!(":{}\r\n", tuple.2 as u32).as_str();
        expected += format!(":{}\r\n", tuple.3).as_str();
        expected += format!(":{}\r\n", tuple.4).as_str();
        expected += format!(":{}\r\n", tuple.5).as_str();
        expected += format!(":{}\r\n", tuple.6).as_str();
        expected += format!(":{}\r\n", tuple.7).as_str();
        expected += format!(":{}\r\n", tuple.8).as_str();
        expected += format!(":{}\r\n", tuple.9).as_str();
        expected += format!(":{}\r\n", tuple.10).as_str();
        expected += format!("+{}\r\n", tuple.11).as_str();
        expected += format!("${}\r\n{}\r\n", tuple.12.len(), tuple.12).as_str();
        expected += "*4\r\n";
        expected += format!(":{}\r\n", tuple.13 .0).as_str();
        expected += format!(":{}\r\n", tuple.13 .1).as_str();
        expected += format!(":{}\r\n", tuple.13 .2).as_str();
        expected += format!("+{}\r\n", tuple.13 .3).as_str();
        expected += "*3\r\n";
        expected += format!(":{}\r\n", tuple.14[0]).as_str();
        expected += format!(":{}\r\n", tuple.14[1]).as_str();
        expected += format!(":{}\r\n", tuple.14[2]).as_str();

        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, tuple)?;
        }

        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}

mod test_tuple_variant {
    use super::super::*;

    #[derive(Serialize)]
    #[allow(dead_code)]
    enum WithTupleVariant<'a> {
        Something,
        TupleStruct(
            bool,
            char,
            char,
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            &'a str,
            &'a str,
            (u32, u8, i16, &'a str),
            [u32; 3],
        ),
    }

    #[test]
    fn test_tuple_struct() -> Result<()> {
        let tuple = WithTupleVariant::TupleStruct(
            true,
            '\0',
            char::MAX,
            u8::MIN,
            u16::MAX,
            u32::MIN,
            u64::MAX,
            i8::MIN,
            i16::MAX,
            i32::MIN,
            i64::MAX,
            "Test this is",
            "This is a\r\ntest",
            (5, 6, 7, "Test Also"),
            [8, 9, 10],
        );
        let mut expected = "*2\r\n:1\r\n*15\r\n:1\r\n".to_owned();
        match tuple {
            WithTupleVariant::TupleStruct(
                _,
                t1,
                t2,
                t3,
                t4,
                t5,
                t6,
                t7,
                t8,
                t9,
                t10,
                t11,
                t12,
                t13,
                t14,
            ) => {
                expected += format!(":{}\r\n", t1 as u32).as_str();
                expected += format!(":{}\r\n", t2 as u32).as_str();
                expected += format!(":{}\r\n", t3).as_str();
                expected += format!(":{}\r\n", t4).as_str();
                expected += format!(":{}\r\n", t5).as_str();
                expected += format!(":{}\r\n", t6).as_str();
                expected += format!(":{}\r\n", t7).as_str();
                expected += format!(":{}\r\n", t8).as_str();
                expected += format!(":{}\r\n", t9).as_str();
                expected += format!(":{}\r\n", t10).as_str();
                expected += format!("+{}\r\n", t11).as_str();
                expected += format!("${}\r\n{}\r\n", t12.len(), t12).as_str();
                expected += "*4\r\n";
                expected += format!(":{}\r\n", t13.0).as_str();
                expected += format!(":{}\r\n", t13.1).as_str();
                expected += format!(":{}\r\n", t13.2).as_str();
                expected += format!("+{}\r\n", t13.3).as_str();
                expected += "*3\r\n";
                expected += format!(":{}\r\n", t14[0]).as_str();
                expected += format!(":{}\r\n", t14[1]).as_str();
                expected += format!(":{}\r\n", t14[2]).as_str();
            }
            _ => unreachable!("should never happen"),
        }
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, tuple)?;
        }

        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}

mod test_map {
    use std::collections::HashMap;

    use super::super::*;

    #[test]
    fn test_map() -> Result<()> {
        let map = &mut HashMap::<u8, String>::new();
        map.insert(1, "Test1".into());
        map.insert(2, "Test2".into());
        map.insert(3, "Test3".into());
        map.insert(4, "Test4".into());
        map.insert(5, "Test5".into());

        let mut expected = "*5\r\n".to_owned();
        for (k, v) in map.iter() {
            expected += "*2\r\n";
            expected += format!(":{}\r\n", k).as_str();
            expected += format!("+{}\r\n", v).as_str();
        }

        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, map)?;
        }

        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}

mod test_struct {

    use super::super::*;

    #[derive(Serialize)]
    struct TestStruct {
        field1: u8,
        field2: bool,
        field3: String,
        field4: u32,
    }

    #[test]
    fn test_struct() -> Result<()> {
        let test_struct = TestStruct {
            field1: 127,
            field2: true,
            field3: "This is a test".into(),
            field4: u32::MAX / 2,
        };

        let mut expected = "*4\r\n".to_owned();
        expected += format!("*2\r\n+field1\r\n:{}\r\n", test_struct.field1).as_str();
        expected += format!(
            "*2\r\n+field2\r\n:{}\r\n",
            if test_struct.field2 { 1 } else { 0 }
        )
        .as_str();
        expected += format!("*2\r\n+field3\r\n+{}\r\n", test_struct.field3).as_str();
        expected += format!("*2\r\n+field4\r\n:{}\r\n", test_struct.field4).as_str();

        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, test_struct)?;
        }

        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}

mod test_struct_variant {

    use super::super::*;

    #[derive(Serialize)]
    #[allow(dead_code)]
    enum WithStructVariant {
        Variant1,
        TestStruct {
            field1: u8,
            field2: bool,
            field3: String,
            field4: u32,
        },
    }

    #[test]
    fn test_struct_variant() -> Result<()> {
        let test_struct = WithStructVariant::TestStruct {
            field1: 127,
            field2: true,
            field3: "This is a test".into(),
            field4: u32::MAX / 2,
        };

        let mut expected = "*2\r\n:1\r\n*4\r\n".to_owned();
        match test_struct {
            WithStructVariant::TestStruct {
                ref field1,
                ref field2,
                ref field3,
                ref field4,
            } => {
                expected += format!("*2\r\n+field1\r\n:{}\r\n", field1).as_str();
                expected +=
                    format!("*2\r\n+field2\r\n:{}\r\n", if *field2 { 1 } else { 0 }).as_str();
                expected += format!("*2\r\n+field3\r\n+{}\r\n", field3).as_str();
                expected += format!("*2\r\n+field4\r\n:{}\r\n", field4).as_str();
            }
            _ => unreachable!("this will never happen"),
        }

        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, test_struct)?;
        }

        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }
}
