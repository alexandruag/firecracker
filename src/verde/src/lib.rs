//! This is the original iteration of the `VersionedSerde` trait, which addressed both
//! the versioned serialization and deserialization aspects:
//!
//! ```
//! use serde::{Deserialize, Serialize};
//!
//! pub trait VersionedSerde<'de>: Sized {
//!     type Version: Serialize;
//!     type S: Serialize;
//!     type D: Deserialize<'de> + Into<Self>;
//!
//!     fn versioned_serialize(&self, target_version: Self::Version) -> Self::S;
//! }
//! ```
//!
//! Values of type `Self::Version` are used to denote different versions of the type implementing
//! the `VersionedSerde` trait.
//!
//! Values of type `Self::S` are constructed by the `versioned_serialize` method,
//! and when serialized implement the actual versioned serialization logic for the provided
//! target version.
//!
//! The `Self::D` associated type can be deserialized from the results of serializing `Self::S`
//! for any known `Self::Version`, and then can be converted into value of the current version
//! of `Self` via `Into`.
//!
//! Ultimately, the `VersionedSerde` trait has been split for two reasons:
//! - It sometimes makes sense to only implement (or have in scope) either serialization or
//!   deserialization, but not both.
//! - Most concrete implementations of `Self::S` tend to have an associated lifetime, because
//!   they include a `&'a Self` reference, which needs to be explicitly declared, and makes
//!   implementing the trait awkward in many situations. To go around this, we use the
//!   Paolo maneuver, where we implement the serialization half of the trait for `&'a T`
//!   instead of `T`, to get the lifetime from the type definition. The `Generic Associated Types`
//!   [proposal](https://github.com/rust-lang/rust/issues/44265) will help once implemented. The
//!   deserialize half makes use of the `Into<Self>` bound for `Self::D`, so we implement it
//!   for `T` itself.

use serde::{Deserialize, Serialize};

/// Implementors of this trait support versioned serialization.
pub trait VersionedSerialize {
    /// Type that represents the target version for serialization. Even though we only specify
    /// `Serialize` as a trait bound here, the concrete type must also implement something we
    /// call "stable serialization": the definition of `Self::Version` may change from one version
    /// of `Self` to another, but we must still be able to express older `Self::Version` values,
    /// and they must serialize to exactly the same representation as before, so older
    /// `Self::Version` implementations can still interpret them.
    ///
    /// As an example, the default (implemented by `serde_derive`) serialization for primitive
    /// types is stable irrespective of the backend in use, but this is no longer true for enums,
    /// where adding/reordering variants can change the output of serialization. The user is
    /// expected to forgo the default `Serialize` implementation in these cases, and provide one
    /// that adheres to the aforementioned invariants.
    type Version: Serialize;

    type S: Serialize;

    /// Build a `Self::S` value whose serialized representation is the same as the versioned
    /// serialized representation of `Self` for the provided target version.
    fn versioned_serialize(&self, target_version: Self::Version) -> Self::S;
}

/// Implementors of this trait support versioned deserialization.
pub trait VersionedDeserialize<'de>: Sized {
    /// A type that can be deserialized starting from a versioned serialized representation of
    /// `Self`, and then consumed via `Into` to obtain the associated current version value
    /// of `Self`.
    type D: Deserialize<'de> + Into<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt;

    use bincode;
    use serde::de::{Error, MapAccess, SeqAccess, Visitor};
    use serde::ser::{SerializeStruct, SerializeTuple};
    use serde::{Deserializer, Serializer};
    use serde_json;

    enum AVersion {
        V1,
        V2,
        V3,
    }

    // `AVersion` is an enum, so we use custom`Serialize` and `Deserialize` implementations to
    // ensure the serialization is stable. Here's a simple approach where associated a textual
    // representation to each variant.
    impl Serialize for AVersion {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use AVersion::*;
            match self {
                V1 => "1",
                V2 => "2",
                V3 => "3",
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for AVersion {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            use AVersion::*;
            Ok(match <&str>::deserialize(deserializer)? {
                "1" => V1,
                "2" => V2,
                "3" => V3,
                _ => return Err(D::Error::custom("invalid version")),
            })
        }
    }

    #[derive(Debug, PartialEq)]
    struct A {
        b: B,
        c: C,
    }

    struct AS<'a>(&'a A, AVersion);

    impl<'a> Serialize for AS<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            // We serialize this as a (version, struct) tuple. Could also add magic numbers, etc.
            let mut tup = serializer.serialize_tuple(2)?;
            // Serialize the version.
            tup.serialize_element(&self.1)?;
            // Serialize the data for the target version.
            tup.serialize_element(&ASValue(self.0, &self.1))?;
            tup.end()
        }
    }

    // Internal helper struct. Implements serialization for a value of type A to the provided
    // target version.
    struct ASValue<'a>(&'a A, &'a AVersion);

    impl<'a> Serialize for ASValue<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use AVersion::*;

            let a = self.0;
            let target_version = self.1;

            // We'll ideally have tooling support to help generate the following code. The tricky
            // part comes when there's some semantic interpretation required as well. We might
            // change the existing interfaces if it turns out there are significant advantages for
            // automatic code generation when more contextual information is available; that's
            // something we have to explore first.
            match target_version {
                V1 => {
                    let mut state = serializer.serialize_struct("A", 2)?;
                    state.serialize_field("b", &(&a.b).versioned_serialize("v1"))?;
                    state.serialize_field("c", &(&a.c).versioned_serialize("v1"))?;
                    state.end()
                }
                V2 => {
                    let mut state = serializer.serialize_struct("A", 2)?;
                    state.serialize_field("b", &(&a.b).versioned_serialize("v2"))?;
                    state.serialize_field("c", &(&a.c).versioned_serialize("v2"))?;
                    state.end()
                }
                V3 => {
                    let mut state = serializer.serialize_struct("A", 2)?;
                    state.serialize_field("b", &(&a.b).versioned_serialize("v2"))?;
                    state.serialize_field("c", &(&a.c).versioned_serialize("v3"))?;
                    state.end()
                }
            }
        }
    }

    struct AD {
        inner: A,
    }

    // The deserializer follows the serialization steps in reverse. We first deserialize the
    // version, and based on that we pick the specific deserialization logic to use.
    impl<'de> Deserialize<'de> for AD {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct ADVisitor;

            impl<'de> Visitor<'de> for ADVisitor {
                type Value = A;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "some message here")
                }

                fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Self::Value, V::Error> {
                    use AVersion::*;
                    let version = seq
                        .next_element()?
                        .ok_or(V::Error::missing_field("version"))?;
                    // The deserialization logic for each (x -> current) version transition
                    // is implemented by a separate helper type.
                    Ok(match version {
                        V1 => {
                            seq.next_element::<ADV1>()?
                                .ok_or(V::Error::missing_field("value"))?
                                .0
                        }
                        V2 => unreachable!(),
                        V3 => unreachable!(),
                    })
                }
            }

            Ok(Self {
                inner: deserializer.deserialize_tuple(2, ADVisitor)?,
            })
        }
    }

    // Automatically provides `impl Into<A> for A_D` as well.
    impl From<AD> for A {
        fn from(source: AD) -> Self {
            source.inner
        }
    }

    // Deserializes an `a: A` saved for version `V1`.
    struct ADV1(A);

    impl<'de> Deserialize<'de> for ADV1 {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            #[derive(Deserialize)]
            #[serde(field_identifier, rename_all = "lowercase")]
            enum Field {
                B,
                C,
            };

            struct ADV1Visitor;

            impl<'de> Visitor<'de> for ADV1Visitor {
                type Value = A;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct A")
                }

                // This implementation is based on the example available
                // [here](https://serde.rs/deserialize-struct.html).

                // Most of this code should ideally be auto-generated, or at least written using
                // helper tools.

                fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let b = seq
                        .next_element::<<B as VersionedDeserialize>::D>()?
                        .ok_or_else(|| V::Error::missing_field("b"))?
                        .into();
                    let c = seq
                        .next_element::<<C as VersionedDeserialize>::D>()?
                        .ok_or_else(|| V::Error::missing_field("c"))?
                        .into();
                    Ok(A { b, c })
                }

                fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut b = None;
                    let mut c = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::B => {
                                if b.is_some() {
                                    return Err(V::Error::duplicate_field("b"));
                                }
                                b = Some(
                                    map.next_value::<<B as VersionedDeserialize>::D>()?.into(),
                                );
                            }
                            Field::C => {
                                if c.is_some() {
                                    return Err(V::Error::duplicate_field("c"));
                                }
                                c = Some(
                                    map.next_value::<<C as VersionedDeserialize>::D>()?.into(),
                                );
                            }
                        }
                    }

                    Ok(A {
                        b: b.ok_or_else(|| V::Error::missing_field("b"))?,
                        c: c.ok_or_else(|| V::Error::missing_field("c"))?,
                    })
                }
            }

            Ok(Self(deserializer.deserialize_struct(
                "A",
                &["b", "c"],
                ADV1Visitor,
            )?))
        }
    }

    impl<'a> VersionedSerialize for &'a A {
        type Version = AVersion;
        type S = AS<'a>;

        fn versioned_serialize(&self, target_version: Self::Version) -> Self::S {
            AS(self, target_version)
        }
    }

    impl<'de> VersionedDeserialize<'de> for A {
        type D = AD;
    }

    // For this dummy examples, types `B` and `C` have trivial implementations of
    // `VersionedSerialize` and `VersionedDeserialize`, i.e. they always call the default
    // `Serialize` and `Deserialize` implementations for any version.

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct B {
        x: u32,
    }

    struct BS<'a>(&'a B);

    impl<'a> Serialize for BS<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'a> VersionedSerialize for &'a B {
        type Version = &'static str;
        type S = BS<'a>;

        fn versioned_serialize(&self, _target_version: Self::Version) -> Self::S {
            BS(self)
        }
    }

    impl<'de> VersionedDeserialize<'de> for B {
        type D = Self;
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct C {
        y: u64,
    }

    struct CS<'a>(&'a C);

    impl<'a> Serialize for CS<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'a> VersionedSerialize for &'a C {
        type Version = &'static str;
        type S = CS<'a>;

        fn versioned_serialize(&self, _target_version: Self::Version) -> Self::S {
            CS(self)
        }
    }

    impl<'de> VersionedDeserialize<'de> for C {
        type D = Self;
    }

    #[test]
    fn test_dummy() {
        let a = A {
            b: B { x: 11 },
            c: C { y: 123 },
        };

        // Trying out bincode.
        {
            let v = bincode::serialize(&(&a).versioned_serialize(AVersion::V1)).unwrap();
            let hmm = bincode::deserialize::<<A as VersionedDeserialize>::D>(v.as_slice())
                .unwrap()
                .into();

            assert_eq!(a, hmm);
        }

        // Trying out serde-json.
        {
            let j = serde_json::to_string(&(&a).versioned_serialize(AVersion::V1)).unwrap();
            let hmm = serde_json::from_str::<<A as VersionedDeserialize>::D>(j.as_str())
                .unwrap()
                .into();

            assert_eq!(a, hmm);
        }
    }
}
