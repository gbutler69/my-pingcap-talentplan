use super::*;

macro_rules! test_integer {
    (min $min:expr, mid $mid:expr, max $max:expr) => {{
        let input = format!(":{}\r\n:{}\r\n:{}\r\n", $min, $mid, $max);
        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!($min, from_reader(reader)?);
        assert_eq!($mid, from_reader(reader)?);
        assert_eq!($max, from_reader(reader)?);

        Ok(())
    }};
}

macro_rules! test_float {
    (for $type:ty, min $min:expr, mid $mid:expr, max $max:expr, epsilon $epsilon:expr) => {{
        let input = format!("+{}\r\n+{}\r\n+{}\r\n", $min, $mid, $max);
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
    let reader = &mut io::BufReader::new(":1\r\n:0\r\n:0\r\n:1\r\n".as_bytes());

    assert_eq!(expect_true, from_reader(reader)?);
    assert_eq!(expect_false, from_reader(reader)?);
    assert_eq!(expect_false, from_reader(reader)?);
    assert_eq!(expect_true, from_reader(reader)?);

    Ok(())
}

#[test]
fn test_i8() -> Result<()> {
    test_integer!( min i8::MIN, mid 0_i8, max i8::MAX)
}

#[test]
fn test_i16() -> Result<()> {
    test_integer!( min i16::MIN, mid 0_i16, max i16::MAX)
}

#[test]
fn test_i32() -> Result<()> {
    test_integer!( min i32::MIN, mid 0_i32, max i32::MAX)
}

#[test]
fn test_i64() -> Result<()> {
    test_integer!( min i64::MIN, mid 0_i64, max i64::MAX)
}

#[test]
fn test_u8() -> Result<()> {
    test_integer!( min u8::MIN, mid 0_u8, max u8::MAX)
}

#[test]
fn test_u16() -> Result<()> {
    test_integer!( min u16::MIN, mid 0_u16, max u16::MAX)
}

#[test]
fn test_u32() -> Result<()> {
    test_integer!( min u32::MIN, mid 0_u32, max u32::MAX)
}

#[test]
fn test_u64() -> Result<()> {
    test_integer!( min u64::MIN, mid 0_u64, max u64::MAX)
}

#[test]
fn test_f32() -> Result<()> {
    test_float!(for f32, min f32::MIN, mid 0_f32, max f32::MAX, epsilon f32::EPSILON)
}

#[test]
fn test_f64() -> Result<()> {
    test_float!(for f64, min f64::MIN, mid 0_f64, max f64::MAX, epsilon f64::EPSILON)
}

#[test]
fn test_char() -> Result<()> {
    let min = '\0';
    let max = char::MAX;
    let input = format!(":{}\r\n:{}\r\n", min as u32, max as u32);
    let reader = &mut io::BufReader::new(input.as_bytes());

    assert_eq!(min, from_reader(reader)?);
    assert_eq!(max, from_reader(reader)?);

    Ok(())
}

#[test]
fn test_string() -> Result<()> {
    let string1 = "This is a test".to_owned();
    let string2 = "This is also\r\na test...∑, 𖿢".to_owned();
    let input = format!("+{}\r\n${}\r\n{}\r\n", string1, string2.len(), string2);
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
        "${}\r\n{}\r\n${}\r\n{}\r\n",
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
        "+{string1}\r\n$-1\r\n${len_string2}\r\n{string2}\r\n$-1\r\n:{num1}\r\n:{num2}\r\n$-1\r\n:{num3}\r\n:{num4}\r\n",
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
    let input = "*0\r\n";
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
        let input = "*0\r\n";
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
        let input = format!(":{}\r\n+{}\r\n:1\r\n", expected_u32, expected_string);
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
            "*2\r\n:{u32_0}\r\n:{u32_1}\r\n\r\n\
            *3\r\n+{string1}\r\n+{string2}\r\n${string3_len}\r\n{string3}\r\n\r\n\
            *5\r\n:{bool1}\r\n:{bool2}\r\n:{bool3}\r\n:{bool4}\r\n:{bool5}\r\n\r\n",
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
            "*10\r\n\
            :{u32_0}\r\n:{u32_1}\r\n\
            +{string1}\r\n+{string2}\r\n${string3_len}\r\n{string3}\r\n\
            :{bool1}\r\n:{bool2}\r\n:{bool3}\r\n:{bool4}\r\n:{bool5}\r\n\
            \r\n",
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
            "*10\r\n\
            :{u32_0}\r\n:{u32_1}\r\n\
            +{string1}\r\n+{string2}\r\n${string3_len}\r\n{string3}\r\n\
            :{bool1}\r\n:{bool2}\r\n:{bool3}\r\n:{bool4}\r\n:{bool5}\r\n\
            \r\n",
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

        let input = "*5\r\n\
        *2\r\n:1\r\n+test1\r\n\r\n\
        *2\r\n:2\r\n+test2\r\n\r\n\
        *2\r\n:3\r\n+test3\r\n\r\n\
        *2\r\n:4\r\n+test4\r\n\r\n\
        *2\r\n:5\r\n+test5\r\n\r\n\
        \r\n";

        let reader = &mut io::BufReader::new(input.as_bytes());

        assert_eq!(expected_map, from_reader(reader)?);

        Ok(())
    }
}
