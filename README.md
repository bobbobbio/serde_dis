# serde_dis

This crate provides two macros for generating a `serde` `Serialize` and
`Deserialize` implementation for enums.

These are `SerializeWithDiscriminant` and `DeserializeWithDiscriminant`

The implementations serializes and deserializes an enum as a struct with the
discriminant as the first field and the variant fields as the following fields.

The `Deserialize` implementation expects the deserializer to call
`visit_seq` for structs, this is mainly because the deserializers I was
targeting do things this way. `visit_map` could be implemented but there is a
caveat here which is that this strategy would only work if the map being
deserialized always has the discriminant as the first entry.

Here are some examples

This enum will be serialized / deserialized as a u32
```Rust
  #[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
  #[repr(u32)]
  enum Foo {
      A = 1,
      B = 2,
      C = 3,
  }
```

It also supports enums with fields

This enum will first serialize / deserialize its discriminant value (a `u16`) then any fields
```Rust
  #[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
  #[repr(u16)]
  enum Bar {
      A(String) = 1,
      B {
          a: i32,
          b: u32,
      } = 12,
      C(i32, u64) = 7,
      #[serde(other)]
      D = 9,
  }
```

## supported `serde` attributes

 - `other` this makes it so any unknown discriminant deserializes as this given
   variant
 - `rename` the container attribute. This lets you rename the enum.
