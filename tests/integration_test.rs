// copyright 2023 Remi Bernotavicius

use serde_dis::{DeserializeWithDiscriminant, SerializeWithDiscriminant};
use serde_test::{assert_de_tokens, assert_ser_tokens, Token};
use std::marker::PhantomData;

#[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
#[repr(u32)]
enum Foo {
    A = 1,
    B = 2,
    C = 3,
}

#[test]
fn serialize_unit_enum() {
    assert_ser_tokens(
        &Foo::A,
        &[
            Token::Struct {
                name: "Foo",
                len: 1,
            },
            Token::Str("discriminant"),
            Token::U32(1),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &Foo::B,
        &[
            Token::Struct {
                name: "Foo",
                len: 1,
            },
            Token::Str("discriminant"),
            Token::U32(2),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &Foo::C,
        &[
            Token::Struct {
                name: "Foo",
                len: 1,
            },
            Token::Str("discriminant"),
            Token::U32(3),
            Token::StructEnd,
        ],
    );
}

#[test]
fn deseralize_unit_enum() {
    assert_de_tokens(
        &Foo::A,
        &[Token::Seq { len: None }, Token::U32(1), Token::SeqEnd],
    );
    assert_de_tokens(
        &Foo::B,
        &[Token::Seq { len: None }, Token::U32(2), Token::SeqEnd],
    );
    assert_de_tokens(
        &Foo::C,
        &[Token::Seq { len: None }, Token::U32(3), Token::SeqEnd],
    );
}

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

#[test]
fn serialize_fielded_enum() {
    assert_ser_tokens(
        &Bar::A("hello".into()),
        &[
            Token::Struct {
                name: "Bar",
                len: 2,
            },
            Token::Str("discriminant"),
            Token::U16(1),
            Token::Str("field0"),
            Token::String("hello".into()),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &Bar::B { a: 36, b: 74 },
        &[
            Token::Struct {
                name: "Bar",
                len: 3,
            },
            Token::Str("discriminant"),
            Token::U16(12),
            Token::Str("a"),
            Token::I32(36),
            Token::Str("b"),
            Token::U32(74),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &Bar::C(36, 74),
        &[
            Token::Struct {
                name: "Bar",
                len: 3,
            },
            Token::Str("discriminant"),
            Token::U16(7),
            Token::Str("field0"),
            Token::I32(36),
            Token::Str("field1"),
            Token::U64(74),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &Bar::D,
        &[
            Token::Struct {
                name: "Bar",
                len: 1,
            },
            Token::Str("discriminant"),
            Token::U16(9),
            Token::StructEnd,
        ],
    );
}

#[test]
fn deserialize_fielded_enum() {
    assert_de_tokens(
        &Bar::A("hello".into()),
        &[
            Token::Seq { len: None },
            Token::U16(1),
            Token::String("hello".into()),
            Token::SeqEnd,
        ],
    );

    assert_de_tokens(
        &Bar::C(36, 74),
        &[
            Token::Seq { len: None },
            Token::U16(7),
            Token::I32(36),
            Token::U64(74),
            Token::SeqEnd,
        ],
    );

    assert_de_tokens(
        &Bar::D,
        &[Token::Seq { len: None }, Token::U16(9), Token::SeqEnd],
    );

    // Because of the `other` attribute, `D` will deserialize as any unknown discriminant
    assert_de_tokens(
        &Bar::D,
        &[Token::Seq { len: None }, Token::U16(700), Token::SeqEnd],
    );
}

#[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
#[repr(u32)]
enum Baz<T> {
    A(T) = 3,
    B = 9,
}

#[test]
fn serialize_generic_fielded_enum() {
    assert_ser_tokens(
        &Baz::A(String::from("hello")),
        &[
            Token::Struct {
                name: "Baz",
                len: 2,
            },
            Token::Str("discriminant"),
            Token::U32(3),
            Token::Str("field0"),
            Token::String("hello".into()),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &Baz::<String>::B,
        &[
            Token::Struct {
                name: "Baz",
                len: 1,
            },
            Token::Str("discriminant"),
            Token::U32(9),
            Token::StructEnd,
        ],
    );
}

#[test]
fn deserialize_generic_fielded_enum() {
    assert_de_tokens(
        &Baz::A(String::from("hello")),
        &[
            Token::Seq { len: None },
            Token::U32(3),
            Token::String("hello".into()),
            Token::SeqEnd,
        ],
    );

    assert_de_tokens(
        &Baz::<String>::B,
        &[Token::Seq { len: None }, Token::U32(9), Token::SeqEnd],
    );
}

#[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
#[repr(u32)]
enum CrazyGenerics<'a, const S: usize> {
    B(PhantomData<(&'a (), [(); S])>) = 9,
}
