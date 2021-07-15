use std::fmt::Display;

use super::*;

fn test_integer<T: Display + Serialize>(indicator: char, value: T) -> Result<()> {
    let expected = format!("{}{}\n", indicator, value);
    let mut actual = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut actual), &value)?;
    assert_eq!(expected.as_bytes(), actual.as_slice());
    Ok(())
}

fn test_float<T: Display + Serialize>(indicator: char, value: T) -> Result<()> {
    let expected = format!("{}{}\n", indicator, value);
    let mut actual = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut actual), &value)?;
    assert_eq!(expected.as_bytes(), actual.as_slice());
    Ok(())
}

#[test]
fn test_bool() -> Result<()> {
    let mut buf = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut buf), &true)?;
    assert_eq!("1\n".as_bytes(), buf.as_slice());

    let mut buf = Vec::<u8>::new();
    to_writer(&mut io::BufWriter::new(&mut buf), &false)?;
    assert_eq!("0\n".as_bytes(), buf.as_slice());

    Ok(())
}

#[test]
fn test_i8() -> Result<()> {
    test_integer('b', i8::MIN)?;
    test_integer('b', -1_i8)?;
    test_integer('b', 0_i8)?;
    test_integer('b', 1_i8)?;
    test_integer('b', i8::MAX)
}

#[test]
fn test_i16() -> Result<()> {
    test_integer('w', i16::MIN)?;
    test_integer('w', -1_i16)?;
    test_integer('w', 0_i16)?;
    test_integer('w', 1_i16)?;
    test_integer('w', i16::MAX)
}

#[test]
fn test_i32() -> Result<()> {
    test_integer('i', i32::MIN)?;
    test_integer('i', -1_i32)?;
    test_integer('i', 0_i32)?;
    test_integer('i', 1_i32)?;
    test_integer('i', i32::MAX)
}

#[test]
fn test_i64() -> Result<()> {
    test_integer('d', i64::MIN)?;
    test_integer('d', -1_i64)?;
    test_integer('d', 0_i64)?;
    test_integer('d', 1_i64)?;
    test_integer('d', i64::MAX)
}

#[test]
fn test_i128() -> Result<()> {
    test_integer('q', i128::MIN)?;
    test_integer('q', -1_i128)?;
    test_integer('q', 0_i128)?;
    test_integer('q', 1_i128)?;
    test_integer('q', i128::MAX)
}

#[test]
fn test_u8() -> Result<()> {
    test_integer('B', u8::MIN)?;
    test_integer('B', 1_u8)?;
    test_integer('B', u8::MAX)
}

#[test]
fn test_u16() -> Result<()> {
    test_integer('W', u16::MIN)?;
    test_integer('W', 1_u16)?;
    test_integer('W', u16::MAX)
}

#[test]
fn test_u32() -> Result<()> {
    test_integer('I', u32::MIN)?;
    test_integer('I', 1_u32)?;
    test_integer('I', u32::MAX)
}

#[test]
fn test_u64() -> Result<()> {
    test_integer('D', u64::MIN)?;
    test_integer('D', 1_u64)?;
    test_integer('D', u64::MAX)
}

#[test]
fn test_u128() -> Result<()> {
    test_integer('Q', u128::MIN)?;
    test_integer('Q', 1_u128)?;
    test_integer('Q', u128::MAX)
}

#[test]
fn test_f32() -> Result<()> {
    test_float('f', f32::MIN)?;
    test_float('f', -1_f32)?;
    test_float('f', 0_f32)?;
    test_float('f', 1_f32)?;
    test_float('f', f32::MAX)
}

#[test]
fn test_f64() -> Result<()> {
    test_float('F', f64::MIN)?;
    test_float('F', -1_f64)?;
    test_float('F', 0_f64)?;
    test_float('F', 1_f64)?;
    test_float('F', f64::MAX)
}

#[test]
fn test_char() -> Result<()> {
    let chars_to_test = [
        '\0', '\t', '\r', '\n', 'A', 'Z', 'a', 'z', '0', '9', '!', ')', '~', '∑', '𖿢',
    ];
    let mut buf = Vec::<u8>::new();
    for char in chars_to_test {
        let expected = format!("c{}\n", char);
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
        let expected = if str.contains(|c| c == '\n') {
            format!("&{}\n{}\n", str.len(), str)
        } else {
            format!("${}\n", str)
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
        let mut expected = format!("%{}\n", bytes.len()).as_bytes().to_vec();
        expected.append(&mut bytes.to_vec());
        expected.append(&mut "\n".as_bytes().to_vec());
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
    let expected = "!\n";
    let mut buf = Vec::<u8>::new();
    to_writer::<_, Option<()>>(&mut io::BufWriter::new(&mut buf), None)?;
    assert_eq!(expected.as_bytes(), buf.as_slice());
    Ok(())
}

#[test]
fn test_some() -> Result<()> {
    let expected = "$This is a test\n";
    let mut buf = Vec::<u8>::new();
    to_writer::<_, Option<&str>>(&mut io::BufWriter::new(&mut buf), Some("This is a test"))?;
    assert_eq!(expected.as_bytes(), buf.as_slice());
    Ok(())
}

#[test]
fn test_unit() -> Result<()> {
    let expected = "~0\n";
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
        let expected = "}0\nUnit\n";
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
        assert_eq!(
            "@ContainsUnitVariants\n$Unit1\n\
             @ContainsUnitVariants\n$Unit2\n\
             @ContainsUnitVariants\n$Unit3\n\
             @ContainsUnitVariants\n$Unit4\n"
                .as_bytes(),
            buf.as_slice()
        );
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
        let expected =
            ":1\nNewTypeBool\n1\n:1\nNewTypeBool\n0\n:1\nNewTypeBool\n0\n:1\nNewTypeBool\n1\n"
                .as_bytes();
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
        let expected = ":1\nNewTypeU8\nB0\n\
                              :1\nNewTypeU8\nB1\n\
                              :1\nNewTypeU8\nB127\n\
                              :1\nNewTypeU8\nB255\n"
            .as_bytes();
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
            ":1\nNewTypeI64\nd{}\n\
             :1\nNewTypeI64\nd{}\n\
             :1\nNewTypeI64\nd{}\n\
             :1\nNewTypeI64\nd{}\n\
             :1\nNewTypeI64\nd{}\n",
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
            ":1\nNewTypeString\n${}\n\
             :1\nNewTypeString\n${}\n\
             :1\nNewTypeString\n${}\n\
             :1\nNewTypeString\n${}\n\
             :1\nNewTypeString\n${}\n\
             :1\nNewTypeString\n&{}\n{}\n\
             :1\nNewTypeString\n${}\n\
             :1\nNewTypeString\n&{}\n{}\n",
            "",
            " ",
            "   ",
            "  Test  ",
            "This is a test...∑, 𖿢",
            "This is a\r\ntest...∑, 𖿢".len(),
            "This is a\r\ntest...∑, 𖿢",
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
        let expected = "^1\nNewTypeVariants\n$Bool\n1\n\
                              ^1\nNewTypeVariants\n$Bool\n0\n\
                              ^1\nNewTypeVariants\n$Bool\n0\n\
                              ^1\nNewTypeVariants\n$Bool\n1\n"
            .as_bytes();
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
            "^1\nNewTypeVariants\n$String\n${}\n\
             ^1\nNewTypeVariants\n$String\n&{}\n{}\n\
             ^1\nNewTypeVariants\n$String\n${}\n",
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
        let expected = "`4\n1\n0\n0\n1\n";
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, bools.to_vec())?;
        }
        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_seq_i16() -> Result<()> {
        let i16s = [i16::MIN, -1, 0, 1, i16::MAX];
        let expected = format!(
            "`5\nw{}\nw{}\nw{}\nw{}\nw{}\n",
            i16s[0], i16s[1], i16s[2], i16s[3], i16s[4]
        );
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, i16s.to_vec())?;
        }
        assert_eq!(expected.as_bytes(), buf.as_slice());
        Ok(())
    }

    #[test]
    fn test_seq_string() -> Result<()> {
        let strings = ["Test1", "Test\r\n2", "Test\r3", "Test4"];
        let expected = format!(
            "`4\n${}\n&{}\n{}\n${}\n${}\n",
            strings[0],
            strings[1].len(),
            strings[1],
            strings[2],
            strings[3],
        );
        let mut buf = Vec::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, strings.to_vec())?;
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
        [8, 9, 10].to_vec(),
    );
    let mut expected = "~15\n1\n".to_owned();
    expected += format!("c{}\n", tuple.1).as_str();
    expected += format!("c{}\n", tuple.2).as_str();
    expected += format!("B{}\n", tuple.3).as_str();
    expected += format!("W{}\n", tuple.4).as_str();
    expected += format!("I{}\n", tuple.5).as_str();
    expected += format!("D{}\n", tuple.6).as_str();
    expected += format!("b{}\n", tuple.7).as_str();
    expected += format!("w{}\n", tuple.8).as_str();
    expected += format!("i{}\n", tuple.9).as_str();
    expected += format!("d{}\n", tuple.10).as_str();
    expected += format!("${}\n", tuple.11).as_str();
    expected += format!("&{}\n{}\n", tuple.12.len(), tuple.12).as_str();
    expected += "~4\n";
    expected += format!("i{}\n", tuple.13 .0).as_str();
    expected += format!("i{}\n", tuple.13 .1).as_str();
    expected += format!("i{}\n", tuple.13 .2).as_str();
    expected += format!("${}\n", tuple.13 .3).as_str();
    expected += "`3\n";
    expected += format!("i{}\n", tuple.14[0]).as_str();
    expected += format!("i{}\n", tuple.14[1]).as_str();
    expected += format!("i{}\n", tuple.14[2]).as_str();

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
    struct TupleStruct(
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
        String,
        String,
        (u32, u8, i16, String),
        Vec<u32>,
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
            "Test this is".into(),
            "This is a\r\ntest".into(),
            (5, 6, 7, "Test Also".into()),
            [8, 9, 10].to_vec(),
        );
        let mut expected = ":15\nTupleStruct\n1\n".to_owned();
        expected += format!("c{}\n", tuple.1).as_str();
        expected += format!("c{}\n", tuple.2).as_str();
        expected += format!("B{}\n", tuple.3).as_str();
        expected += format!("W{}\n", tuple.4).as_str();
        expected += format!("I{}\n", tuple.5).as_str();
        expected += format!("D{}\n", tuple.6).as_str();
        expected += format!("b{}\n", tuple.7).as_str();
        expected += format!("w{}\n", tuple.8).as_str();
        expected += format!("i{}\n", tuple.9).as_str();
        expected += format!("d{}\n", tuple.10).as_str();
        expected += format!("${}\n", tuple.11).as_str();
        expected += format!("&{}\n{}\n", tuple.12.len(), tuple.12).as_str();
        expected += "~4\n";
        expected += format!("I{}\n", tuple.13 .0).as_str();
        expected += format!("B{}\n", tuple.13 .1).as_str();
        expected += format!("w{}\n", tuple.13 .2).as_str();
        expected += format!("${}\n", tuple.13 .3).as_str();
        expected += "`3\n";
        expected += format!("I{}\n", tuple.14[0]).as_str();
        expected += format!("I{}\n", tuple.14[1]).as_str();
        expected += format!("I{}\n", tuple.14[2]).as_str();

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
    enum WithTupleVariant {
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
            String,
            String,
            (u32, u8, i16, String),
            Vec<u32>,
        ),
    }

    #[test]
    fn test_tuple_variant() -> Result<()> {
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
            "Test this is".into(),
            "This is a\r\ntest".into(),
            (5, 6, 7, "Test Also".into()),
            [8, 9, 10].to_vec(),
        );
        let mut expected = "^15\nWithTupleVariant\n$TupleStruct\n1\n".to_owned();
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
                ref t11,
                ref t12,
                ref t13,
                ref t14,
            ) => {
                expected += format!("c{}\n", t1).as_str();
                expected += format!("c{}\n", t2).as_str();
                expected += format!("B{}\n", t3).as_str();
                expected += format!("W{}\n", t4).as_str();
                expected += format!("I{}\n", t5).as_str();
                expected += format!("D{}\n", t6).as_str();
                expected += format!("b{}\n", t7).as_str();
                expected += format!("w{}\n", t8).as_str();
                expected += format!("i{}\n", t9).as_str();
                expected += format!("d{}\n", t10).as_str();
                expected += format!("${}\n", t11).as_str();
                expected += format!("&{}\n{}\n", t12.len(), t12).as_str();
                expected += "~4\n";
                expected += format!("I{}\n", t13.0).as_str();
                expected += format!("B{}\n", t13.1).as_str();
                expected += format!("w{}\n", t13.2).as_str();
                expected += format!("${}\n", t13.3).as_str();
                expected += "`3\n";
                expected += format!("I{}\n", t14[0]).as_str();
                expected += format!("I{}\n", t14[1]).as_str();
                expected += format!("I{}\n", t14[2]).as_str();
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

        let mut expected = "{5\n".to_owned();
        for (k, v) in map.iter() {
            expected += format!("B{}\n", k).as_str();
            expected += format!("${}\n", v).as_str();
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

        let mut expected = "}4\nTestStruct\n".to_owned();
        expected += format!("$field1\nB{}\n", test_struct.field1).as_str();
        expected += format!("$field2\n{}\n", if test_struct.field2 { 1 } else { 0 }).as_str();
        expected += format!("$field3\n${}\n", test_struct.field3).as_str();
        expected += format!("$field4\nI{}\n", test_struct.field4).as_str();

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

        let mut expected = "#4\nWithStructVariant\n$TestStruct\n".to_owned();
        match test_struct {
            WithStructVariant::TestStruct {
                field1,
                field2,
                ref field3,
                field4,
            } => {
                expected += format!("$field1\nB{}\n", field1).as_str();
                expected += format!("$field2\n{}\n", if field2 { 1 } else { 0 }).as_str();
                expected += format!("$field3\n${}\n", field3).as_str();
                expected += format!("$field4\nI{}\n", field4).as_str();
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
