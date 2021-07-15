# KVS Protocol Serde Serialization/Deserialization

## Serialization Format

The serialization is UTF-8 for all non-binary (String) data. Numeric Values, including lengths of data, are represented as their textual (ASCII) form. Floating point values are represented as proscribed by IEEE floating point specifications for string representations of floating point values. A single newline (0x0A) terminates all types except for Maps, Sequences, and Structures (see the details of the individual types for elaboration).

Each type is indicated by a single byte-length ASCII character that identifies the type of data which follows. Some types include a *Length* parameter which immediately follows the indicator and is itself followed by a newline (0x0A) before the data. Some types (boolean) have indicators that indicate the type and the value simultaneously without further data.

The following types are supported for serialization:

  * **$** - Simple String (utf-8 encoded, containing no new-lines, limited to 8k in byte-length)
  * **&** - String (utf-8 encoded, containing any character, length up to u32::MAX)
  * **c** - Character (unicode 32-bit code-point)
  * **%** - Binary Byte Array
  * **b,w,i,d,q** - Signed-Integer (8,16,32,64,128 bits)
  * **B,W,I,D,Q** - Unsigned-Integer (8,16,32,64,128 bits)
  * **f,F** - Floating-Point Value (32,64 bits)
  * **0,1** - Boolean Value (false, true)
  * **@,^,#,** - Enum Variant (Unit, Tuple, Struct)
  * **`** - Homogenous Sequence (Arrays)
  * **~** - Heterogenous Sequence (Tuples)
  * **:** - Named Heterogenous Sequence (Named Tuple, aka Tuple Struct)
  * **{** - Map (Homogenous Keys & Values)
  * **}** - Named Structure (Homogenous Keys, Heterogenous Values)
  * **!** - Nil/None/Null type/value
  * **=** - Identifier (Enum Variant, Field Name, etc.)

    NOTE: In the examples below, individual bytes are represented as " 0x00" hexadecimal representation. The actual data would not include the leading space and the hex value would just be a single byte.

### Simple String

  * Indicator: **$** (0x24)
  * Data: utf-8 encoded string containing no newline (0xA) characters with a maximum length of 8k bytes
  * Terminated By: newline (0xA)
  * Example: **$ This is a test 0x0A**
  * Error Condition: greater than 8k bytes is invalid

### String

  * Indicator: **&** (0x26)
  * Data:
    * Length: Numeric maximum 32-bit unsigned integer
    * Separator: newline (0xA)
    * String Data: utf-8 encoded string data of exactly *Length* bytes
  * Terminated By: newline (0xA)
  * Example: **&19 0x0A This is also a test 0x0A**
  * Error Condition: newline (0xA) does not immediately follow at position *Length + 1*

### Character

  * Indicator: **c**
  * Data: 32-bit unicode code-point value
  * Terminated By: newline (0xA)
  * Example: **!48 0x0A** - represents the character 0 (ASCII value 48)
  * Error Condition: value larger than 32-bit integer or outside the valid Unicode range

### Binary Bytes Array

  * Indicator: **%**
  * Data:
    * Length: 0 to Numeric maximum 32-bit unsigned integer
    * Separator: newline (0x0A)
    * Binary Data: binary bytes of exactly *Length* bytes (not encoded)
  * Terminated By: newline (0x0A)
  * Example: **%7 0x0A 0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x0A**

### Signed/Unsigned Integer

  * Indicators:
    * Signed:
      * **b** - 8-bit
      * **w** - 16-bit
      * **i** - 32-bit
      * **d** - 64-bit
      * **q** - 128-bit
    * Unsigned:
      * **B** - 8-bit
      * **W** - 16-bit
      * **I** - 32-bit
      * **D** - 64-bit
      * **Q** - 128-bit
  * Data: Numeric textual value
  * Terminated By: newline (0x0A)
  * Example: **w-3978 0x0A** (16-bit value -3978)

### Floating-Point

  * Indicator:
    * 32-Bit: **f**
    * 64-Bit: **F**
  * Data:
  * Terminated By: newline (0x0A)
  * Example: **__**

### Boolean

  * Indicator:
    * true: **1**
    * false: **0**
  * Data: (N/A)
  * Terminated By: newline (0x0A)
  * Example: **1 0x0A** (true)

### Enum Variant

#### Unit Enum Variant

  * Indicator: **@**
  * Data:
  * Terminated By: newline (0x0A)
  * Example: **@EnumName 0x0A VariantName 0x0A** EnumName::VariantName

#### Tuple Enum Variant

  * Indicator: **^**
  * Data:
  * Terminated By: newline (0x0A)
  * Example: **^3 0x0A EnumName 0x0A VariantName 0x0A I32 0x0A $TestString 0x0A 1 0x0A** EnumName::VariantName(u32, String, bool)

#### Struct Enum Variant

  * Indicator: **#**
  * Data:
  * Terminated By: newline (0x0A)
  * Example: **#2 0x0A EnumName 0x0A VariantName 0x0A $Field1 0x0A $Value1 0x0A $Field2 0x0A $Value2 0x0A** EnumName::VariantName{Field1:String,Field2:String}

### Homogenous Sequence (Array Slice, Vec, Set, etc.)

  * Indicator: **`**
  * Data:
  * Terminated By: newline (0x0A)
  * Example: **__**

### Heterogenous Sequence/Homogenous Fixed Length Sequence (Tuple or Array)

  * Indicator: **~**
  * Data:
  * Terminated By: newline (0x0A)
  * Example: **__**

### Named Heterogenous Sequence (Tuple Struct)

  * Indicator: **:**
  * Data:
  * Terminated By: newline (0x0A)
  * Example: **__**

### Map

  * Indicator: **{**
  * Data:
    * Length: 0 to 32-bit integer max
    * Separator: newline (0x0A)
    * Key-Value Pairs: *Length* number of Map Key-Value Pairs
  * Terminated By: (N/A)
  * Example: **{2 0x0A b1 0x0A $Test 0x0A b2 0x0A $Test2 0x0A** (map of length 2 with byte keys and string values)

### Structure

  * Indicator: **}**
    * Field Indicator: **]**
  * Data:
    * Length: 0 to 32-bit integer max
    * Separator: newline (0x0A)
    * Name: text name of structure
    * Separator: newline (0x0A)
    * Fields: *Length* number of Field Name/Value Pairs
  * Terminated By: newline (N/A)
  * Example: **}3 0x0A Struct1 0x0A ]Field1 0x0A $Value1 0x0A ]Field2 0x0A $Value2 0x0A ]Field3 0x0A $Value3 0x0A** (structure named "Struct1" with 3 fields named Field1, Field2, Field3 with values Value1, Value2, Value (Strings))

### Nil/None/Null

  * Indicator: **!**
  * Data: (N/A)
  * Terminated By: newline (0x0A)
  * Example: **! 0x0A**
