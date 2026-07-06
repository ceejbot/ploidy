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
pub struct ApiResponse {
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub code: ::ploidy_util::absent::AbsentOr<i32>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub r#type: ::ploidy_util::absent::AbsentOr<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub message: ::ploidy_util::absent::AbsentOr<::std::string::String>,
}
