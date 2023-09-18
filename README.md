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
