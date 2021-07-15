use super::*;

macro_rules! test_integer {
    (min $min:expr, mid $mid:expr, max $max:expr, delim $delim:expr) => {{
        let input = format!(
            "{delim}{}\n{delim}{}\n{delim}{}\n",
            $min,
            $mid,
            $max,
            delim = $delim,
        );
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!($min, from_reader(reader)?);
        assert_eq!($mid, from_reader(reader)?);
        assert_eq!($max, from_reader(reader)?);

        Ok(())
    }};
}

macro_rules! test_float {
    (for $type:ty, min $min:expr, mid $mid:expr, max $max:expr, epsilon $epsilon:expr, delim $delim:expr) => {{
        let input = format!(
            "{delim}{}\n{delim}{}\n{delim}{}\n",
            $min,
            $mid,
            $max,
            delim = $delim
        );
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert!(($min - from_reader::<_, $type>(reader)?).abs() <= $epsilon);
        assert!(($mid - from_reader::<_, $type>(reader)?).abs() <= $epsilon);
        assert!(($max - from_reader::<_, $type>(reader)?).abs() <= $epsilon);

        Ok(())
    }};
}

#[test]
fn test_bool() -> Result<()> {
    let expect_true = true;
    let expect_false = false;
    let reader = &mut io::BufReader::new("1\n0\n0\n1\n".as_bytes());

    assert_eq!(expect_true, from_reader(reader)?);
    assert_eq!(expect_false, from_reader(reader)?);
    assert_eq!(expect_false, from_reader(reader)?);
    assert_eq!(expect_true, from_reader(reader)?);

    Ok(())
}

#[test]
fn test_i8() -> Result<()> {
    test_integer!( min i8::MIN, mid 0_i8, max i8::MAX, delim 'b')
}

#[test]
fn test_i16() -> Result<()> {
    test_integer!( min i16::MIN, mid 0_i16, max i16::MAX, delim 'w')
}

#[test]
fn test_i32() -> Result<()> {
    test_integer!( min i32::MIN, mid 0_i32, max i32::MAX, delim 'i')
}

#[test]
fn test_i64() -> Result<()> {
    test_integer!( min i64::MIN, mid 0_i64, max i64::MAX, delim 'd')
}

#[test]
fn test_i128() -> Result<()> {
    test_integer!( min i128::MIN, mid 0_i128, max i128::MAX, delim 'q')
}

#[test]
fn test_u8() -> Result<()> {
    test_integer!( min u8::MIN, mid 0_u8, max u8::MAX, delim 'B')
}

#[test]
fn test_u16() -> Result<()> {
    test_integer!( min u16::MIN, mid 0_u16, max u16::MAX, delim 'W')
}

#[test]
fn test_u32() -> Result<()> {
    test_integer!( min u32::MIN, mid 0_u32, max u32::MAX, delim 'I')
}

#[test]
fn test_u64() -> Result<()> {
    test_integer!( min u64::MIN, mid 0_u64, max u64::MAX, delim 'D')
}

#[test]
fn test_u128() -> Result<()> {
    test_integer!( min u128::MIN, mid 0_u128, max u128::MAX, delim 'Q')
}

#[test]
fn test_f32() -> Result<()> {
    test_float!(for f32, min f32::MIN, mid 0_f32, max f32::MAX, epsilon f32::EPSILON, delim 'f')
}

#[test]
fn test_f64() -> Result<()> {
    test_float!(for f64, min f64::MIN, mid 0_f64, max f64::MAX, epsilon f64::EPSILON, delim 'F')
}

#[test]
fn test_char() -> Result<()> {
    let min = '\0';
    let max = char::MAX;
    let input = format!("c{}\nc{}\n", min, max);
    let reader = &mut io::BufReader::new(input.as_bytes());

    assert_eq!(min, from_reader(reader)?);
    assert_eq!(max, from_reader(reader)?);

    Ok(())
}

#[test]
fn test_string() -> Result<()> {
    let string1 = "This is a test".to_owned();
    let string2 = "This is also\r\na test...∑, 𖿢".to_owned();
    let input = format!("${}\n&{}\n{}\n", string1, string2.len(), string2);
    let reader = &mut io::BufReader::new(input.as_bytes());

    assert_eq!(string1, from_reader::<_, String>(reader)?);
    assert_eq!(string2, from_reader::<_, String>(reader)?);

    Ok(())
}

#[test]
fn test_byte_buf() -> Result<()> {
    let string1 = "This is a test".to_owned();
    let string2 = "This is also\r\na test...∑, 𖿢".to_owned();
    let input = format!(
        "%{}\n{}\n%{}\n{}\n",
        string1.len(),
        string1,
        string2.len(),
        string2
    );
    let reader = &mut io::BufReader::new(input.as_bytes());

    assert_eq!(
        string1.as_bytes(),
        from_reader::<_, serde_bytes::ByteBuf>(reader)?.into_vec()
    );
    assert_eq!(
        string2.as_bytes(),
        from_reader::<_, serde_bytes::ByteBuf>(reader)?.into_vec()
    );

    Ok(())
}

#[test]
fn test_option() -> Result<()> {
    let string1 = "This is a test".to_owned();
    let string2 = "This is also\r\na test...∑, 𖿢".to_owned();
    let num1 = 8_u8;
    let num2 = 16_u16;
    let num3 = 32_u32;
    let num4 = 64_u64;
    let input = format!(
        "${string1}\n!\n&{len_string2}\n{string2}\n!\nB{num1}\nW{num2}\n!\nI{num3}\nD{num4}\n",
        string1 = string1,
        len_string2 = string2.len(),
        string2 = string2,
        num1 = num1,
        num2 = num2,
        num3 = num3,
        num4 = num4
    );
    let reader = &mut io::BufReader::new(input.as_bytes());

    assert_eq!(Some(string1), from_reader(reader)?);
    assert_eq!(None, from_reader::<_, Option<String>>(reader)?);
    assert_eq!(Some(string2), from_reader(reader)?);
    assert_eq!(None, from_reader::<_, Option<u8>>(reader)?);
    assert_eq!(Some(num1), from_reader(reader)?);
    assert_eq!(Some(num2), from_reader(reader)?);
    assert_eq!(None, from_reader::<_, Option<f64>>(reader)?);
    assert_eq!(Some(num3), from_reader(reader)?);
    assert_eq!(Some(num4), from_reader(reader)?);

    Ok(())
}

#[test]
#[allow(clippy::unit_cmp)]
fn test_unit() -> Result<()> {
    let input = "~0\n";
    let reader = &mut io::BufReader::new(input.as_bytes());

    assert_eq!((), from_reader(reader)?);

    Ok(())
}

mod test_unit_struct {
    use super::super::*;

    #[derive(Eq, PartialEq, Deserialize, Debug)]
    struct UnitStruct;

    #[test]
    fn test_unit_struct() -> Result<()> {
        let input = "}0\nUnitStruct\n";
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(UnitStruct {}, from_reader(reader)?);

        Ok(())
    }
}

mod test_newtype_struct {
    use super::super::*;

    #[derive(Eq, PartialEq, Deserialize, Debug)]
    struct NewTypeStructU32(u32);

    #[derive(Eq, PartialEq, Deserialize, Debug)]
    struct NewTypeStructString(String);

    #[derive(Eq, PartialEq, Deserialize, Debug)]
    struct NewTypeStructBool(bool);

    #[test]
    fn test_newtype_struct() -> Result<()> {
        let expected_u32 = u32::MAX / 2;
        let expected_string = "This is a test".to_owned();
        let expected_bool = true;
        let input = format!(
            ":1\nNewTypeStructU32\nI{}\n:1\nNewTypeStructString\n${}\n:1\nNewTypeStructBool\n1\n",
            expected_u32, expected_string
        );
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(NewTypeStructU32(expected_u32), from_reader(reader)?);
        assert_eq!(NewTypeStructString(expected_string), from_reader(reader)?);
        assert_eq!(NewTypeStructBool(expected_bool), from_reader(reader)?);

        Ok(())
    }
}

mod test_seq {
    use super::super::*;

    #[test]
    fn test_seq() -> Result<()> {
        let expected_u32s = [u32::MIN, u32::MAX];
        let expected_strings = [
            "This is a test".to_owned(),
            "Also a test".into(),
            "Another\r\nTest".into(),
        ];
        let expected_bools = [true, false, false, true, false];

        let input = format!(
            "`2\nI{u32_0}\nI{u32_1}\n\
             `3\n${string1}\n${string2}\n&{string3_len}\n{string3}\n\
             `5\n{bool1}\n{bool2}\n{bool3}\n{bool4}\n{bool5}\n",
            u32_0 = expected_u32s[0],
            u32_1 = expected_u32s[1],
            string1 = expected_strings[0],
            string2 = expected_strings[1],
            string3_len = expected_strings[2].len(),
            string3 = expected_strings[2],
            bool1 = if expected_bools[0] { 1 } else { 0 },
            bool2 = if expected_bools[1] { 1 } else { 0 },
            bool3 = if expected_bools[2] { 1 } else { 0 },
            bool4 = if expected_bools[3] { 1 } else { 0 },
            bool5 = if expected_bools[4] { 1 } else { 0 },
        );
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected_u32s.to_vec(), from_reader::<_, Vec<_>>(reader)?);
        assert_eq!(
            expected_strings.to_vec(),
            from_reader::<_, Vec<String>>(reader)?
        );
        assert_eq!(expected_bools.to_vec(), from_reader::<_, Vec<_>>(reader)?);

        Ok(())
    }
}

mod test_tuple {
    use super::super::*;

    #[test]
    fn test_tuple() -> Result<()> {
        let expected = (
            u32::MIN,
            u32::MAX,
            "This is a test".to_owned(),
            "Also a test".to_owned(),
            "Another\r\nTest".to_owned(),
            true,
            false,
            false,
            true,
            false,
        );

        let input = format!(
            "~10\n\
             I{u32_0}\nI{u32_1}\n\
             ${string1}\n${string2}\n&{string3_len}\n{string3}\n\
             {bool1}\n{bool2}\n{bool3}\n{bool4}\n{bool5}\n",
            u32_0 = expected.0,
            u32_1 = expected.1,
            string1 = expected.2,
            string2 = expected.3,
            string3_len = expected.4.len(),
            string3 = expected.4,
            bool1 = if expected.5 { 1 } else { 0 },
            bool2 = if expected.6 { 1 } else { 0 },
            bool3 = if expected.7 { 1 } else { 0 },
            bool4 = if expected.8 { 1 } else { 0 },
            bool5 = if expected.9 { 1 } else { 0 },
        );
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected, from_reader(reader)?);

        Ok(())
    }
}

mod test_tuple_struct {
    use super::super::*;

    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct TupleStruct(
        u32,
        u32,
        String,
        String,
        String,
        bool,
        bool,
        bool,
        bool,
        bool,
    );

    #[test]
    fn test_tuple_struct() -> Result<()> {
        let expected = TupleStruct(
            u32::MIN,
            u32::MAX,
            "This is a test".to_owned(),
            "Also a test".to_owned(),
            "Another\r\nTest".to_owned(),
            true,
            false,
            false,
            true,
            false,
        );

        let input = format!(
            ":10\nTupleStruct\n\
             I{u32_0}\nI{u32_1}\n\
             ${string1}\n${string2}\n&{string3_len}\n{string3}\n\
             {bool1}\n{bool2}\n{bool3}\n{bool4}\n{bool5}\n",
            u32_0 = expected.0,
            u32_1 = expected.1,
            string1 = expected.2,
            string2 = expected.3,
            string3_len = expected.4.len(),
            string3 = expected.4,
            bool1 = if expected.5 { 1 } else { 0 },
            bool2 = if expected.6 { 1 } else { 0 },
            bool3 = if expected.7 { 1 } else { 0 },
            bool4 = if expected.8 { 1 } else { 0 },
            bool5 = if expected.9 { 1 } else { 0 },
        );
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected, from_reader(reader)?);

        Ok(())
    }
}

mod test_map {
    use std::collections::HashMap;

    use super::super::*;

    #[test]
    fn test_map() -> Result<()> {
        let mut expected_map = HashMap::new();
        expected_map.insert(1, "test1".to_owned());
        expected_map.insert(2, "test2".into());
        expected_map.insert(3, "test3".into());
        expected_map.insert(4, "test4".into());
        expected_map.insert(5, "test5".into());
        let expected_map = expected_map;

        let input = "{5\n\
                          i1\n$test1\n\
                          i2\n$test2\n\
                          i3\n$test3\n\
                          i4\n$test4\n\
                          i5\n$test5\n";

        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected_map, from_reader(reader)?);

        Ok(())
    }
}

mod test_struct {
    use super::super::*;

    #[derive(Debug, PartialEq, Deserialize)]
    struct TestStruct {
        a_u32: u32,
        a_f64: f64,
        a_string: String,
        another_string: String,
        a_bool: bool,
    }

    #[test]
    fn test_struct() -> Result<()> {
        let expected = TestStruct {
            a_u32: 32,
            a_f64: -64.5,
            a_string: "This is a test".into(),
            another_string: "This is\r\nalso a test".into(),
            a_bool: true,
        };

        let input = format!(
            "}}5\nTestStruct\n\
             $a_u32\nI32\n\
             $a_f64\nF-64.5\n\
             $a_string\n${}\n\
             $another_string\n&{}\n{}\n\
             $a_bool\n1\n",
            expected.a_string,
            expected.another_string.len(),
            expected.another_string
        );

        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected, from_reader(reader)?);

        Ok(())
    }
}

mod test_enum {
    use super::super::*;

    #[derive(Debug, PartialEq, Deserialize)]
    enum SimpleEnum {
        Test1,
        Test2,
        Test3,
        Test4,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    enum ComplexEnum {
        Au32(u32),
        Af64(f64),
        Astring(String),
        Astruct {
            a_u32: u32,
            an_f64: f64,
            a_tuple: (u32, u64),
            an_array: Vec<u32>,
            an_enum: SimpleEnum,
            a_string: String,
            another_string: String,
        },
        Atuple(u32, f64, (u32, u64), Vec<u32>, SimpleEnum, String, String),
        AnotherString(String),
        Abool(bool),
    }

    #[test]
    fn test_enum_struct() -> Result<()> {
        let expected_simple = SimpleEnum::Test3;
        let expected_complex = ComplexEnum::Astruct {
            a_u32: 32,
            an_f64: -64.5,
            a_tuple: (32, 64),
            an_array: [5, 7, 9].to_vec(),
            an_enum: SimpleEnum::Test2,
            a_string: "test1".into(),
            another_string: "test\r\n2".into(),
        };

        let input = "@SimpleEnum\n$Test3\n\
                          #7\nComplexEnum\n$Astruct\n\
                          $a_u32\nI32\n\
                          $an_f64\nF-64.5\n\
                          $a_tuple\n~2\nI32\nD64\n\
                          $an_array\n`3\nI5\nI7\nI9\n\
                          $an_enum\n@SimpleEnum\n$Test2\n\
                          $a_string\n$test1\n\
                          $another_string\n&7\ntest\r\n2\n";

        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected_simple, from_reader(reader)?);
        assert_eq!(expected_complex, from_reader(reader)?);

        Ok(())
    }

    #[test]
    fn test_enum_tuple() -> Result<()> {
        let expected_simple = SimpleEnum::Test3;
        let expected_complex = ComplexEnum::Atuple(
            32,
            -64.5,
            (32, 64),
            [5, 7, 9].to_vec(),
            SimpleEnum::Test2,
            "test1".into(),
            "test\r\n2".into(),
        );

        let input = "@SimpleEnum\n$Test3\n\
                          ^7\nComplexEnum\n$Atuple\n\
                          I32\n\
                          F-64.5\n\
                          ~2\nI32\nD64\n\
                          `3\nI5\nI7\nI9\n\
                          @SimpleEnum\n$Test2\n\
                          $test1\n\
                          &7\ntest\r\n2\n";

        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected_simple, from_reader(reader)?);
        assert_eq!(expected_complex, from_reader(reader)?);

        Ok(())
    }

    #[test]
    fn test_enum_newtype() -> Result<()> {
        let expected_simple = SimpleEnum::Test3;
        let expected_complex = ComplexEnum::Af64(-64.5);

        let input = "@SimpleEnum\n$Test3\n\
                          ^7\nComplexEnum\n$Af64\n\
                          F-64.5\n";

        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected_simple, from_reader(reader)?);
        assert_eq!(expected_complex, from_reader(reader)?);

        Ok(())
    }
}
