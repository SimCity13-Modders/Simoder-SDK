# Simoder SDK declarations

Version 0.2.0 adds the supported alpha `base-luau-alpha-v1` profile in
[Luau/base.d.luau](Luau/base.d.luau), with [API guidance](Luau/BASE_API.md).
The legacy Luau/TypeScript declarations and MIT notice were extracted byte-for-byte from Simoder-DevTools revision
`d1128dc40336ecd7371d15305411ab73fdae20db`.

`Luau/simoder.d.luau` describes the existing Luau host. See
[editor configuration](Luau/README.md), including opt-in Base definitions
without changing IDE settings.
`TypeScript/simoder.d.ts` describes the legacy TypeScriptToLua authoring pipeline.
These files are declarations, not executable runtime modules or permission grants.
This package provides no JavaScript engine, compiler, downloader,
runtime implementation or Base TypeScript/JavaScript support. The narrower TypeScript literal types
and broader Luau types are preserved from the existing implementation.

This repository is the authoritative declaration source. Consumers may retain
hash-verified vendored/generated copies to preserve existing offline editor and
compiler layouts; those copies must identify this package version and source
revision and must not be independently edited. `sdk-manifest.json` records exact
source hashes and upstream provenance. A local Git revision/hash verifies prepared
content; it does not authenticate a publisher or grant code execution permission.

DevTools and Mod-Examples retain pinned, hash-verified vendored compatibility
copies at their existing source paths. This deliberately preserves offline Node
compiler adjacency, runtime fingerprints and automatic editor discovery without
preparing a replacement toolchain or copying external compiler dependencies.
Consumer locks identify the SDK commit and hashes; consumers reject drift rather
than treating vendored files as separately editable API definitions. Canonical
payload paths use `-text` Git attributes to preserve exact checkout bytes.

Configure and verify with the offline Rust verification suite (`crates/simoder-sdk-verifier`):

```powershell
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
```

See [CONTRACTS.md](CONTRACTS.md) and [`contracts-matrix.json`](contracts-matrix.json) for the canonical Beta API/ABI matrix across Luau, Base Luau, Native ABI v2, and QuickJS/TypeScript-to-JS.

The package contains Luau/TypeScript and Base Luau declarations/guides,
the Native declaration bundle and its generation inputs, the Beta contracts matrix, this README, the manifest
and MIT license. No compiler is installed or fetched.
Tests verify package closure, pinned byte hashes, contract matrix completeness, and automated conformance against the `Simoder-Base` Luau implementation.
They do not prove editor UI behavior, gameplay or future language support.

Native ABI v2 declarations and supported/unavailable facets are described in [Native](Native/README.md). Native inputs and generated headers are installed under Native; no binary is installed.
