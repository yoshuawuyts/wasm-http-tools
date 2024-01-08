# Type System

Definitions are for [OpenAPI v3.1.0](https://spec.openapis.org/oas/latest.html):

| JSON Schema Type | OpenAPI Format Modifier | WIT Type    |
| :--------------- | :---------------------- | :---------- |
| `null`           | -                       | `option<T>` |
| `boolean`        | -                       | `bool`      |
| `object`         | -                       | `record`    |
| `array`          | -                       | `list<T>`   |
| `number`         | -                       | `float64`   |
| `number`         | `float`                 | `float32`   |
| `number`         | `double`                | `float64`   |
| `string`         | -                       | `string`    |
| `string`         | `password`              | `string`    |
| `integer`        | `int32`                 | `s32`       |
| `integer`        | `int64`                 | `s64`       |

## Discussion

### `number`

The base `number` type without any qualifier here is mapped to `float64`. The
JSON Schema specification mentions it should actually be:

> An arbitrary-precision, base-10 decimal number value, from the JSON "number"
> value.

But we want to be able to use `serde-json` for this, which defines `number` [as
an
`f64`](https://docs.rs/serde_json/latest/serde_json/value/struct.Number.html#method.from_f64).
WIT does not (yet?) support arbitrary-precision floating point numbers, so in
the absence of any constraining information we should just pick the biggest
number we can support.

### `null`

WIT also has no notion of `null` in the type system; instead it uses the
`option` type to indicate a value may or may not be present. A field which can
only ever be `null` should be rejected in any encoding - in Rust terms we'd want
to translate that to a type of `Option<!>` (only `None` can be constructed) - or
`Option<Unreachable>` (an empty enum akin to `!`). But in WIT we can't do either of these things, so a field that's hard-coded to only ever be `none` should be rejected to prevent any ambiguities in the encoding.

### `password`

Regarding password strings: for the sake of simplicity we should treat this as a
regular string for the time being. However in the future we may want to define a
generic type `sensitive<T>` which can hold strings whose contents are sensitive
and should never be printed or displayed. This will require a non-trivial type
system addition though, as we'll also want to consider secret encoding /
decoding schemes, if only to ensure we're making the right tradeoffs. So to not
have that conversation right now, we're just treating password strings as
regular strings.

## See Also

- [OpenAPI `v3.1.0` specifcation](https://spec.openapis.org/oas/latest.html#openapi-document)
- [`webidl-wit`](https://github.com/MendyBerger/webidl-wit/tree/main)
- [`serde_json::value::Number`](https://docs.rs/serde_json/1.0.61/serde_json/value/struct.Number.html)
- [json schema draft RFC](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-01#section-4.2.1)
