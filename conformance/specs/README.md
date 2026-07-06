# Vendored OpenAPI specs

Real-world Swagger Petstore specifications, checked in verbatim so the
conformance suite is reproducible and offline. Refetch with the commands below if
the upstream specs change.

| File | Source URL | OpenAPI | Fetched |
|------|------------|---------|---------|
| `petstore-v3.json` | <https://petstore3.swagger.io/api/v3/openapi.json> | 3.0.4 | 2026-07-05 |
| `petstore-v31.json` | <https://petstore31.swagger.io/api/v31/openapi.json> | 3.1.0 | 2026-07-05 |

```sh
curl -fsSL https://petstore3.swagger.io/api/v3/openapi.json  -o conformance/specs/petstore-v3.json
curl -fsSL https://petstore31.swagger.io/api/v31/openapi.json -o conformance/specs/petstore-v31.json
```

## What each spec exercises

- **`petstore-v3.json`** — the classic Petstore: 19 operations across the `pet`,
  `store`, and `user` tags. Broad operation surface (all HTTP verbs, array query
  parameters with `explode`, a header parameter, `application/xml` and
  form-urlencoded and `application/octet-stream` bodies), conventional
  `#/components/schemas/…` references. This is the breadth and compile target;
  ploidy generates and compiles a client from it today.

- **`petstore-v31.json`** — the OpenAPI 3.1 Petstore: only 3 operations plus a
  `newPet` webhook, but feature-dense (JSON Schema 2020-12 `$id`/`$anchor`,
  `readOnly`/`writeOnly`, `application/xml`, `oauth2`/`apiKey`/`mutualTLS`). Its
  `Pet.petDetailsId` uses an absolute-URI `$ref`
  (`/api/v31/components/schemas/petdetails#pet_details_id`), which ploidy does not
  support. This is the depth and known-limitations probe.

See `../GAPS.md` for what generates, compiles, and fails, and the prioritized list
of missing features.
