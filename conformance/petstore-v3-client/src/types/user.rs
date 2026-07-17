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
pub struct User {
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub id: ::ploidy_util::absent::AbsentOr<i64>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub username: ::ploidy_util::absent::AbsentOr<::std::string::String>,
    #[serde(
        rename = "firstName",
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    #[ploidy(pointer(rename = "firstName"))]
    pub first_name: ::ploidy_util::absent::AbsentOr<::std::string::String>,
    #[serde(
        rename = "lastName",
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    #[ploidy(pointer(rename = "lastName"))]
    pub last_name: ::ploidy_util::absent::AbsentOr<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub email: ::ploidy_util::absent::AbsentOr<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub password: ::ploidy_util::absent::AbsentOr<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    pub phone: ::ploidy_util::absent::AbsentOr<::std::string::String>,
    /// User Status
    #[serde(
        rename = "userStatus",
        default,
        skip_serializing_if = "::ploidy_util::absent::AbsentOr::is_absent"
    )]
    #[ploidy(pointer(rename = "userStatus"))]
    pub user_status: ::ploidy_util::absent::AbsentOr<i32>,
}
