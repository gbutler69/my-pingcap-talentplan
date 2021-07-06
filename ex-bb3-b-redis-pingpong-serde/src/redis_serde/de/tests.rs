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
    // assert_eq!(Some(string2), from_reader(reader)?);
    assert_eq!(None, from_reader::<_, Option<u8>>(reader)?);
    // assert_eq!(Some(num1), from_reader(reader)?);
    // assert_eq!(Some(num2), from_reader(reader)?);
    // assert_eq!(None, from_reader::<_, Option<f64>>(reader)?);
    // assert_eq!(Some(num3), from_reader(reader)?);
    // assert_eq!(Some(num4), from_reader(reader)?);

    Ok(())
}
