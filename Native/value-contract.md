# U2a copied-value contract, version 1

Candidate, not deployed. This document defines `SdBytes` semantic payloads; it does not define MCP messages or executable scripts.

An envelope is strict UTF-8 JSON with exactly the unique keys `v`, `schema`, `value`. `v` is the integer literal 1; `schema` is a reviewed method schema identifier such as `test.echo@1`; `value` is one tagged value below. Unknown fields, duplicate keys, invalid UTF-8, unpaired surrogates and embedded NUL are rejected. JSON key order and insignificant whitespace do not affect meaning; wire bytes are not a signature or canonical identity. Fixtures use compact UTF-8 JSON for reproducibility. No evaluation, object instantiation or automatic coercion is permitted.

| Tag | Exact representation |
| --- | --- |
| null | `["null"]` |
| bool | `["bool",true]` or false |
| string | `["string","text"]` |
| f64 | `["f64","16 lowercase hexadecimal digits"]`, IEEE754 binary64 bits in big-endian order; finite values only, signed zero preserved |
| u64 / i64 | `["u64","canonical decimal"]` or i64; exact unsigned/signed 64-bit range; no plus, leading zeros or negative zero |
| bytes | `["bytes","lowercase even-length hexadecimal"]` |
| list | `["list",[tagged values]]` |
| map | `["map",[["key",tagged value],...]]`; unique keys strictly ascending by UTF-8 bytes |
| ref | `["ref","runtime|region|city|entity","slot","generation"]`; slot and generation canonical nonzero uint32 decimals |

No other numeric JSON values are accepted inside tagged values. References convey identity, never authority; host owner, generation, type and context checks still apply. Invocation references, credentials, closures, VM pointers and facades cannot be serialized. Cross-mod reference sharing requires the explicit cooperation API and cannot widen permissions.

All limits apply together: envelope at most 1 MiB; tagged nesting depth at most 16 (root depth 1); at most 4096 tagged nodes; each map at most 128 pairs, list at most 1024 entries, string/key at most 65536 UTF-8 bytes, decoded bytes at most 65536 bytes. Existing transport or method limits remain stricter where applicable, including existing fixed packets. Validate before scheduling; over-limit input returns RESOURCE_LIMIT without partial work. Malformed/schema-invalid input returns INVALID, with empty outputs.

One shared host codec serves native transport adapters and the private Luau binding; base owns domain semantics, not another wire parser. Library selection and parser implementation are outside U2a. The semantic facet remains unavailable until the codec, authority and scheduler are qualified; U2b fixtures may test opaque copied buffers in a test-only host, without advertising production semantic support.
