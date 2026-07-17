# Petstore conformance findings

What ploidy generates, compiles, and fails to handle when driven against the
vendored Swagger Petstore specs (`specs/`). The point is proactive gap-finding:
this is a living to-do list of features to build ahead of needing them, and a
starting point for upstream (`linabutler/ploidy`) issues. Severities are a
judgment call about impact, not a roadmap.

Reproduce with `just conformance-full`, or by hand:

```sh
cargo run -p ploidy -- generate rust conformance/specs/petstore-v3.json  -o /tmp/pv3
cargo run -p ploidy -- generate rust conformance/specs/petstore-v31.json -o /tmp/pv31
```

## Per-spec outcome

### `petstore-v3.json` (OpenAPI 3.0.4) — generates and compiles

Ploidy emits 6 types (`ApiResponse`, `Category`, `Order`, `Pet`, `Tag`, `User`)
and 19 client methods, and the crate compiles against the in-tree `ploidy-util`
(the checked-in result is `petstore-v3-client/`). It degrades several features
**silently** — the client builds, but does less than the spec asks. Nothing
prints a warning, so these are easy to miss:

| Spec feature | What ploidy did | Evidence |
|---|---|---|
| `petstore_auth` (oauth2) + `api_key` (apiKey) **security** | No auth wiring; caller must set headers by hand | no `security` parsing |
| **`servers`** (`/api/v3`) | Ignored; base URL is caller-supplied | no `servers` parsing |
| 4xx / `default` responses (400, 404, 422) | Collapsed to `Error::Status(code)`; no typed error body | `error_for_status()?` |
| `application/xml` request bodies | Not offered; JSON alternative is picked when present | — |

Correctly handled: JSON request/response bodies (typed, e.g. `add_pet(request: impl Into<Pet>)`),
`application/octet-stream` request bodies (`upload_file(body: impl Into<reqwest::Body>)`)
and header parameters (`delete_pet(…, api_key: Option<&str>)`) — both fixed in this
branch, see below; array query parameters with `explode`
(`find_pets_by_status` → `QueryStyle::Form { exploded: true }`), the `status`
string enum with an open catch-all variant, path parameters, and optional fields
via `AbsentOr` (see `petstore-v3-client/tests/roundtrip.rs`).

**Fixed in this branch:**

- `application/octet-stream` request bodies previously degraded silently to a JSON
  `serde_json::Value` body. Ploidy now emits a `Binary` request kind — the method
  takes `body: impl Into<reqwest::Body>` and sends the raw bytes with an explicit
  `Content-Type: application/octet-stream`.
- Header parameters were silently dropped. They now become method arguments
  (`&str`, or `Option<&str>` when optional) set with `.header(name, value)`, so
  `delete_pet` gains its `api_key` header.

One ergonomic note: all 19 methods land in a single `default` resource. Ploidy
groups methods by the `x-resourceId` / `x-resource-name` extensions, not by
OpenAPI `tags`, so the spec's `pet` / `store` / `user` tags produce no module
split.

### `petstore-v31.json` (OpenAPI 3.1.0) — fails to parse

Generation exits non-zero before writing anything:

```
error: references must start with `#`; external references aren't supported
   Pet.petDetailsId → "$ref": "/api/v31/components/schemas/petdetails#pet_details_id"
```

`Pet.petDetailsId` references `PetDetails` by an absolute URI plus a JSON Schema
2020-12 `$anchor`-style fragment. Ploidy's `$ref` resolver accepts only
same-document `#/components/schemas/{name}` (`ploidy-core/src/parse/types.rs`,
`ComponentRef`/`SchemaRef`), so the whole document is rejected. Even if that ref
were fixed, this spec would further exercise: `$id`/`$anchor` document identity,
`$ref` alongside sibling keywords, `readOnly`/`writeOnly` (the `Pet`/`PetDetails`
split), `application/xml`, `mutualTLS`, and the `newPet` webhook — none of which
ploidy handles today.

## Prioritized gaps

| # | Gap | Severity | Where it bites in Petstore | Source | Direction |
|---|-----|----------|----------------------------|--------|-----------|
| 1 | ~~`application/octet-stream` bodies~~ **done** (`Binary` kind). `application/x-www-form-urlencoded`-only bodies still serialize as JSON | Medium (was High) | no form-only body in Petstore; form ops also offer JSON | `ir/spec.rs` request selection (`RequestContent::Any` fall-through) | Add a `Form(schema)` kind emitting `.form(&…)`; parse `requestBody.content.encoding` |
| 2 | External / `$id`-anchored `$ref` rejected | **High** (blocks whole specs) | v31 fails to parse | `parse/types.rs` `ComponentRef::from_str` | Resolve `$id`/`$anchor` document identity and same-document `$defs`; map absolute self-refs back to local schemas |
| 3 | ~~Header params dropped~~ **done** (`Header` kind → method arg + `.header()`). Cookie params still dropped | Low (was High) | no cookie param in Petstore | `ir/spec.rs` (`Cookie` → `None`) | Add a `Cookie` kind assembling a `Cookie` request header |
| 4 | Security schemes unmodeled | **Medium-High** | `petstore_auth`, `api_key` produce no auth API | `parse/types.rs` `SecurityScheme` placeholder; `Operation` doesn't parse `security` | Parse schemes + per-op `security`; generate typed auth setters (bearer/apiKey/basic) over the existing header plumbing |
| 5 | `servers` not parsed | **Medium** | `/api/v3` base path must be hand-supplied | no `servers` in `Document` | Parse `servers`; default `Client::new()` base URL, with server-variable templating |
| 6 | 4xx/5xx responses have no typed body | **Medium** | 400/404/422 collapse to `Error::Status` | `operation.rs` `error_for_status()?` | Model documented error schemas into a typed error enum variant per status |
| 7 | `readOnly`/`writeOnly` not modeled | **Medium** | v31 splits `Pet` (write) vs `PetDetails` (read) | `parse` `Schema` has no such fields | Track read/write visibility; consider request/response type variants |
| 8 | Resource grouping ignores OpenAPI `tags` | **Low** | 19 methods all in `default` | `graph`/`cargo.rs` resource derivation | Optionally fall back to `tags` when `x-resource*` is absent |
| 9 | `default` values, validation keywords, `const` ignored | **Low** | `Pet.status` default, numeric bounds unenforced | `parse` `Schema` | Parse and emit where they aid ergonomics/correctness |
| 10 | Webhooks / callbacks / links unmodeled | **Low** | v31 `newPet` webhook | no parsing | Out of scope for clients; revisit if needed |

The README support matrix (`README.md`) is also stale relative to this branch —
it still says non-primary response schemas are ignored, which the multi-2xx and
header-only-response work changed.
