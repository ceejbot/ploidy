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
pub struct Category {
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub id: ::ploidy_util::absent::AbsentOr<i64>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub name: ::ploidy_util::absent::AbsentOr<::std::string::String>,
}
