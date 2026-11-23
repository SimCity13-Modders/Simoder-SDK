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

## IDE Setup & Installation

`Simoder-SDK` provides authoritative definitions for **Visual Studio Code**, **Cursor**, **VSCodium**, **Visual Studio**, and other LSP-enabled editors.

### 1. Visual Studio Code / Cursor (TypeScript & JavaScript)

For mods written in JavaScript (QuickJS ES2023) or TypeScript:

1. Copy or submodule `Simoder-SDK` into your mod project (e.g. `vendor/Simoder-SDK`).
2. Add a `tsconfig.json` or `jsconfig.json` in your mod project root:

```json
{
  "compilerOptions": {
    "target": "ES2023",
    "module": "CommonJS",
    "moduleResolution": "node",
    "checkJs": true,
    "strict": true
  },
  "include": [
    "scripts/**/*",
    "vendor/Simoder-SDK/TypeScript/**/*"
  ]
}
```

This gives you immediate **IntelliSense**, parameter documentation, and type safety for `simoder.*` and `base.*` APIs.

---

### 2. Visual Studio Code / Cursor (Luau)

For mods written in Luau:

1. Install the [**Luau Language Server**](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.luau-lsp) extension (`JohnnyMorganz.luau-lsp`).
2. In your workspace `.vscode/settings.json`, point to the SDK definition files:

```json
{
  "luau-lsp.platform.type": "standard",
  "luau-lsp.types.definitionFiles": [
    "${workspaceFolder}/vendor/Simoder-SDK/Luau/simoder.d.luau",
    "${workspaceFolder}/vendor/Simoder-SDK/Luau/base.d.luau"
  ]
}
```

Disable Roblox-specific type checking (`"luau-lsp.platform.type": "standard"`). You will receive full autocomplete and hover documentation for all host and base methods.

---

### 3. Visual Studio / CLion / C++ (Native ABI v2 DLL Mods)

For native C/C++ mods (`.dll` compiled for `i686-pc-windows-msvc` x86):

1. Add `Native/` to your compiler include directories (`/I path/to/Simoder-SDK/Native` or CMake `target_include_directories`).
2. Include the native headers:
   ```c
   #include "native.h"
   ```
3. Export the standard ABI v2 lifecycle hooks: `SimoderModule_QueryV2`, `SimoderModule_PrepareV2`, `SimoderModule_AdmitV2`, `SimoderModule_QuiesceV2`, and `SimoderModule_DrainV2`.

---

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
