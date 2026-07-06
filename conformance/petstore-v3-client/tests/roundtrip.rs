//! Serde round-trip tests for the generated Petstore v3 types.
//!
//! These are the cheap, hermetic correctness checks: they never touch the
//! network, only the generated `#[derive(Serialize, Deserialize)]` types. Each
//! test deserializes a representative JSON payload, asserts the semantic shape,
//! and proves the type's serde is self-consistent (either exact-value fidelity,
//! or deserialize/serialize idempotence where a field's textual form is allowed
//! to normalize). A regression in field renames, enum mapping, or the
//! `AbsentOr` optional-field handling fails here.
//!
//! `serde_json` and `AbsentOr` come from the crate's own re-export of
//! `ploidy-util`, so this test file adds no dependencies.

use petstore_v3_client::types::order::types::Status as OrderStatus;
use petstore_v3_client::types::pet::types::Status as PetStatus;
use petstore_v3_client::types::{Category, Order, Pet, Tag, User};
use petstore_v3_client::util::absent::AbsentOr;
use petstore_v3_client::util::serde_json::{self, Value};

// MARK: Pet

#[test]
fn test_pet_round_trips_with_nested_types_and_enum() {
    let input: Value = serde_json::from_str(
        r#"{
            "id": 10,
            "name": "doggie",
            "category": { "id": 1, "name": "Dogs" },
            "photoUrls": ["https://example.com/a.png", "https://example.com/b.png"],
            "tags": [{ "id": 0, "name": "puppy" }],
            "status": "available"
        }"#,
    )
    .unwrap();

    let pet: Pet = serde_json::from_value(input.clone()).unwrap();

    // Semantic shape: renames, nesting, arrays, and the string enum all decode.
    assert_eq!(pet.id, AbsentOr::Present(10));
    assert_eq!(pet.name, "doggie");
    assert_eq!(
        pet.category,
        AbsentOr::Present(Category {
            id: AbsentOr::Present(1),
            name: AbsentOr::Present("Dogs".to_owned())
        })
    );
    assert_eq!(pet.photo_urls.len(), 2);
    assert_eq!(
        pet.tags,
        AbsentOr::Present(vec![Tag {
            id: AbsentOr::Present(0),
            name: AbsentOr::Present("puppy".to_owned())
        }])
    );
    assert_eq!(pet.status, AbsentOr::Present(PetStatus::Available));

    // Exact-value fidelity: re-serializing reproduces the input (no datetime
    // fields here, and `Value` compares order-independently).
    let output: Value = serde_json::to_value(&pet).unwrap();
    assert_eq!(output, input);
}

#[test]
fn test_pet_status_enum_maps_known_and_unknown_variants() {
    assert_eq!(
        serde_json::from_str::<PetStatus>(r#""available""#).unwrap(),
        PetStatus::Available
    );
    assert_eq!(
        serde_json::from_str::<PetStatus>(r#""pending""#).unwrap(),
        PetStatus::Pending
    );
    assert_eq!(
        serde_json::from_str::<PetStatus>(r#""sold""#).unwrap(),
        PetStatus::Sold
    );

    // Unknown values are preserved in the catch-all variant rather than failing,
    // and serialize back verbatim.
    let unknown = serde_json::from_str::<PetStatus>(r#""adopted""#).unwrap();
    assert_eq!(unknown, PetStatus::OtherStatus("adopted".to_owned()));
    assert_eq!(
        serde_json::to_value(&unknown).unwrap(),
        Value::String("adopted".to_owned())
    );
}

// MARK: Store

#[test]
fn test_order_round_trips_including_datetime() {
    let input: Value = serde_json::from_str(
        r#"{
            "id": 1,
            "petId": 7,
            "quantity": 2,
            "shipDate": "2020-02-20T12:00:00Z",
            "status": "placed",
            "complete": false
        }"#,
    )
    .unwrap();

    let order: Order = serde_json::from_value(input).unwrap();
    assert_eq!(order.pet_id, AbsentOr::Present(7));
    assert_eq!(order.status, AbsentOr::Present(OrderStatus::Placed));
    assert_eq!(order.complete, AbsentOr::Present(false));

    // Idempotence rather than exact-input match: chrono may normalize the
    // `shipDate` textual form, so assert the value survives a full round-trip.
    let reparsed: Order = serde_json::from_value(serde_json::to_value(&order).unwrap()).unwrap();
    assert_eq!(order, reparsed);
}

// MARK: User

#[test]
fn test_user_round_trips_with_renamed_fields() {
    let input: Value = serde_json::from_str(
        r#"{
            "id": 1,
            "username": "alice",
            "firstName": "Alice",
            "lastName": "Liddell",
            "email": "alice@example.com",
            "password": "hunter2",
            "phone": "555-0100",
            "userStatus": 1
        }"#,
    )
    .unwrap();

    let user: User = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(user.username, AbsentOr::Present("alice".to_owned()));
    assert_eq!(user.first_name, AbsentOr::Present("Alice".to_owned()));
    assert_eq!(user.user_status, AbsentOr::Present(1));

    let output: Value = serde_json::to_value(&user).unwrap();
    assert_eq!(output, input);
}

// MARK: Optional fields

#[test]
fn test_absent_optional_fields_are_omitted_on_reserialize() {
    // Only the two required fields are present.
    let input: Value = serde_json::from_str(r#"{ "name": "nameless", "photoUrls": [] }"#).unwrap();

    let pet: Pet = serde_json::from_value(input).unwrap();
    assert!(pet.id.is_absent());
    assert!(pet.category.is_absent());
    assert!(pet.tags.is_absent());
    assert!(pet.status.is_absent());

    // Absent optionals must not reappear as `null`; the round-trip stays minimal.
    let output: Value = serde_json::to_value(&pet).unwrap();
    let Value::Object(map) = output else {
        panic!("expected a JSON object; got `{output:?}`");
    };
    let mut keys = map.keys().collect::<Vec<_>>();
    keys.sort();
    assert_eq!(keys, ["name", "photoUrls"]);
}
