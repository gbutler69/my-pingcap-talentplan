mod test_complicated_serialization_deserialization_integrated {

    use std::{
        collections,
        io::{self},
    };

    use serde::{Deserialize, Serialize};

    use super::super::error::Result;

    use super::super::de::from_reader;
    use super::super::ser::to_writer;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum TestUnitEnum {
        Unit1,
        Unit2,
        Unit3,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum TestNewTypeEnum {
        NewTypeString(String),
        NewTypeU32(u32),
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum TestTupleEnum {
        Tuple1(String, u8, u32),
        Tuple2(char, u8),
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum TestStructEnum {
        Struct1 { a: u32, b: String },
        Struct2 { x: u8, y: u8, z: u8 },
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestUnitStruct;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestNewTypeStruct(String);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestTupleStruct(String, u64, u8, char);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        a_bool: bool,
        an_i8: i8,
        an_i16: i16,
        an_i32: i32,
        an_i64: i64,
        a_u8: u8,
        a_u16: u16,
        a_u32: u32,
        a_u64: u64,
        an_f32: f32,
        an_f64: f64,
        a_char: char,
        a_str: String,
        byte_array: [u8; 4],
        #[serde(with = "serde_bytes")]
        byte_array_as_bytes: Vec<u8>,
        a_none: Option<u32>,
        a_some: Option<String>,
        a_unit: (),
        a_unit_struct: TestUnitStruct,
        a_unit_enum: TestUnitEnum,
        a_newtype_struct: TestNewTypeStruct,
        a_newtype_enum: TestNewTypeEnum,
        an_array: [String; 3],
        a_tuple: (u32, String, char, u8),
        a_tuple_struct: TestTupleStruct,
        a_tuple_enum: TestTupleEnum,
        a_map: collections::HashMap<u32, String>,
        a_struct_enum: TestStructEnum,
    }

    #[test]
    fn test_all() -> Result<()> {
        let mut test_map = collections::HashMap::<u32, String>::new();
        test_map.insert(1, "TestString7_1".into());
        test_map.insert(2, "TestString7_2".into());
        test_map.insert(3, "TestString7_3".into());
        test_map.insert(4, "TestString7_4".into());
        test_map.insert(5, "TestString7_5".into());
        test_map.insert(6, "TestString7_6".into());
        test_map.insert(7, "TestString7_7".into());
        test_map.insert(8, "TestString7_8".into());

        let test_struct = TestStruct {
            a_bool: true,
            an_i8: -1,
            an_i16: 2,
            an_i32: -3,
            an_i64: 4,
            a_u8: 5,
            a_u16: 6,
            a_u32: 7,
            a_u64: 8,
            an_f32: -9.5,
            an_f64: 100000.5,
            a_char: 'c',
            a_str: "TestString1".into(),
            byte_array: [2, 4, 6, 8],
            byte_array_as_bytes: [1, 2, 3, 4, 5, 6, 7, 8].to_vec(),
            a_none: None,
            a_some: Some("TestString2".into()),
            a_unit: (),
            a_unit_struct: TestUnitStruct {},
            a_unit_enum: TestUnitEnum::Unit2,
            a_newtype_struct: TestNewTypeStruct("TestString3".into()),
            a_newtype_enum: TestNewTypeEnum::NewTypeU32(32),
            an_array: [
                "TestString4a".into(),
                "TestString4b".into(),
                "TestString4c".into(),
            ],
            a_tuple: (32, "TestString5".into(), 'd', 8),
            a_tuple_struct: TestTupleStruct("TestString6".into(), 64, 8, 'e'),
            a_tuple_enum: TestTupleEnum::Tuple2('f', 8),
            a_map: test_map,
            a_struct_enum: TestStructEnum::Struct2 { x: 1, y: 2, z: 3 },
        };

        let mut buf = Vec::<u8>::new();
        {
            let mut buf_writer = io::BufWriter::new(&mut buf);
            to_writer(&mut buf_writer, &test_struct)?;
        }

        let reader = &mut io::BufReader::new(buf.as_slice());

        assert_eq!(test_struct, from_reader(reader)?);

        Ok(())
    }
}
