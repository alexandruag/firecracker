mod indirect_de;

use serde::Serialize;

pub use indirect_de::IndirectDeserialize;

pub trait Versioned {
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
}

/// Implementors of this trait support versioned serialization.
pub trait VersionedSerialize<'a>: Versioned {
    type S: Serialize;

    /// Build a `Self::S` value whose serialized representation is the same as the versioned
    /// serialized representation of `Self` for the provided target version.
    fn versioned_serialize<'b: 'a>(&'b self, target_version: Self::Version) -> Self::S;
}

/// Implementors of this trait support versioned deserialization.
pub trait VersionedDeserialize: Versioned + Sized {
    fn versioned_deserialize<'de, D: IndirectDeserialize<'de, Self>>(
        source_version: Self::Version,
        d: D,
    ) -> D::R;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt;

    use bincode;
    use serde::de::{Deserializer, Error, MapAccess, SeqAccess, Visitor};
    use serde::ser::{SerializeStruct, Serializer};
    use serde::Deserialize;
    use serde_json;

    use indirect_de::{MapIndirect, SeqIndirect};

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
            use AVersion::*;

            let a = self.0;
            let target_version = &self.1;

            // We'll ideally have tooling support to help generate the following code. The tricky
            // part comes when there's some semantic interpretation required as well. We might
            // change the existing interfaces if it turns out there are significant advantages for
            // automatic code generation when more contextual information is available; that's
            // something we have to explore first.
            match target_version {
                V1 => {
                    let mut state = serializer.serialize_struct("A", 2)?;
                    state.serialize_field("b", &a.b.versioned_serialize("v1"))?;
                    state.serialize_field("c", &a.c.versioned_serialize("v1"))?;
                    state.end()
                }
                V2 => {
                    let mut state = serializer.serialize_struct("A", 2)?;
                    state.serialize_field("b", &a.b.versioned_serialize("v2"))?;
                    state.serialize_field("c", &a.c.versioned_serialize("v2"))?;
                    state.end()
                }
                V3 => {
                    let mut state = serializer.serialize_struct("A", 2)?;
                    state.serialize_field("b", &a.b.versioned_serialize("v2"))?;
                    state.serialize_field("c", &a.c.versioned_serialize("v3"))?;
                    state.end()
                }
            }
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
                    let b = B::versioned_deserialize("v1", SeqIndirect(&mut seq))?
                        .ok_or_else(|| V::Error::missing_field("b"))?;

                    let c = C::versioned_deserialize("v1", SeqIndirect(&mut seq))?
                        .ok_or_else(|| V::Error::missing_field("c"))?;

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
                                b = Some(B::versioned_deserialize("v1", MapIndirect(&mut map))?);
                            }
                            Field::C => {
                                if c.is_some() {
                                    return Err(V::Error::duplicate_field("c"));
                                }
                                c = Some(C::versioned_deserialize("v1", MapIndirect(&mut map))?);
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

    impl From<ADV1> for A {
        fn from(a: ADV1) -> Self {
            a.0
        }
    }

    impl Versioned for A {
        type Version = AVersion;
    }

    impl<'a> VersionedSerialize<'a> for A {
        type S = AS<'a>;

        fn versioned_serialize<'b: 'a>(&'b self, target_version: Self::Version) -> Self::S {
            AS(self, target_version)
        }
    }

    impl VersionedDeserialize for A {
        fn versioned_deserialize<'de, D: IndirectDeserialize<'de, Self>>(
            source_version: Self::Version,
            mut d: D,
        ) -> D::R {
            use AVersion::*;
            match source_version {
                V1 => d.with_deserialize::<ADV1>(),
                _ => unreachable!(),
            }
        }
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

    impl Versioned for B {
        type Version = &'static str;
    }

    impl<'a> VersionedSerialize<'a> for B {
        type S = BS<'a>;

        fn versioned_serialize<'b: 'a>(&'b self, _target_version: Self::Version) -> Self::S {
            BS(self)
        }
    }

    impl VersionedDeserialize for B {
        fn versioned_deserialize<'de, D: IndirectDeserialize<'de, Self>>(
            _source_version: Self::Version,
            mut d: D,
        ) -> D::R {
            d.with_deserialize::<B>()
        }
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

    impl Versioned for C {
        type Version = &'static str;
    }

    impl<'a> VersionedSerialize<'a> for C {
        type S = CS<'a>;

        fn versioned_serialize<'b: 'a>(&'b self, _target_version: Self::Version) -> Self::S {
            CS(self)
        }
    }

    impl VersionedDeserialize for C {
        fn versioned_deserialize<'de, D: IndirectDeserialize<'de, Self>>(
            _source_version: Self::Version,
            mut d: D,
        ) -> D::R {
            d.with_deserialize::<C>()
        }
    }

    #[test]
    fn test_dummy() {
        let a = A {
            b: B { x: 11 },
            c: C { y: 123 },
        };

        struct BincodeIndirect<'b>(&'b [u8]);

        impl<'a, 'b: 'a, T: VersionedDeserialize> IndirectDeserialize<'a, T> for BincodeIndirect<'b> {
            type R = T;

            fn with_deserialize<D: Deserialize<'a> + Into<T>>(&mut self) -> Self::R {
                bincode::deserialize::<D>(self.0).unwrap().into()
            }
        }

        struct JsonIndirect<'b>(&'b str);

        impl<'a, 'b: 'a, T: VersionedDeserialize> IndirectDeserialize<'a, T> for JsonIndirect<'b> {
            type R = T;

            fn with_deserialize<D: Deserialize<'a> + Into<T>>(&mut self) -> Self::R {
                serde_json::from_str::<D>(self.0).unwrap().into()
            }
        }

        // Trying out bincode.
        {
            let v = bincode::serialize(&a.versioned_serialize(AVersion::V1)).unwrap();
            let hmm = A::versioned_deserialize(AVersion::V1, BincodeIndirect(v.as_slice()));
            assert_eq!(a, hmm);
        }

        // Trying out serde-json.
        {
            let j = serde_json::to_string(&a.versioned_serialize(AVersion::V1)).unwrap();
            let hmm = A::versioned_deserialize(AVersion::V1, JsonIndirect(j.as_str()));
            assert_eq!(a, hmm);
        }
    }
}
