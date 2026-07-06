#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    ::ploidy_util::serde::Serialize,
    ::ploidy_util::serde::Deserialize,
    ::ploidy_util::pointer::JsonPointee,
    ::ploidy_util::pointer::JsonPointerTarget,
)]
#[serde(crate = "::ploidy_util::serde")]
#[ploidy(pointer(crate = "::ploidy_util::pointer"))]
pub struct Pet {
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub id: ::ploidy_util::absent::AbsentOr<i64>,
    pub name: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub category: ::ploidy_util::absent::AbsentOr<crate::types::Category>,
    #[serde(rename = "photoUrls")]
    #[ploidy(pointer(rename = "photoUrls"))]
    pub photo_urls: ::std::vec::Vec<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub tags: ::ploidy_util::absent::AbsentOr<::std::vec::Vec<crate::types::Tag>>,
    /// pet status in the store
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub status: ::ploidy_util::absent::AbsentOr<crate::types::pet::types::Status>,
}
pub mod types {
    /// pet status in the store
    #[derive(
        Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        ::ploidy_util::pointer::JsonPointee,
        ::ploidy_util::pointer::JsonPointerTarget,
    )]
    #[ploidy(pointer(crate = "::ploidy_util::pointer"))]
    pub enum Status {
        Available,
        Pending,
        Sold,
        OtherStatus(String),
    }
    impl ::std::default::Default for Status {
        fn default() -> Self {
            Self::OtherStatus(::std::string::String::default())
        }
    }
    impl ::std::fmt::Display for Status {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(match self {
                Self::Available => "available",
                Self::Pending => "pending",
                Self::Sold => "sold",
                Self::OtherStatus(s) => s.as_str(),
            })
        }
    }
    impl ::std::str::FromStr for Status {
        type Err = ::std::convert::Infallible;
        fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
            ::std::result::Result::Ok(match s {
                "available" => Self::Available,
                "pending" => Self::Pending,
                "sold" => Self::Sold,
                _ => Self::OtherStatus(s.to_owned()),
            })
        }
    }
    impl<'de> ::ploidy_util::serde::Deserialize<'de> for Status {
        fn deserialize<D: ::ploidy_util::serde::Deserializer<'de>>(
            deserializer: D,
        ) -> ::std::result::Result<Self, D::Error> {
            struct Visitor;
            impl<'de> ::ploidy_util::serde::de::Visitor<'de> for Visitor {
                type Value = Status;
                fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str("a variant of `Status`")
                }
                fn visit_str<E: ::ploidy_util::serde::de::Error>(
                    self,
                    s: &str,
                ) -> ::std::result::Result<Self::Value, E> {
                    let ::std::result::Result::Ok(v) = ::std::str::FromStr::from_str(s);
                    Ok(v)
                }
            }
            ::ploidy_util::serde::Deserializer::deserialize_str(deserializer, Visitor)
        }
    }
    impl ::ploidy_util::serde::Serialize for Status {
        fn serialize<S: ::ploidy_util::serde::Serializer>(
            &self,
            serializer: S,
        ) -> ::std::result::Result<S::Ok, S::Error> {
            serializer.collect_str(self)
        }
    }
}
