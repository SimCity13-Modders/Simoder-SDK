# Simoder-SDK

[![Download Latest (Nightly)](https://img.shields.io/github/last-commit/SimCity13-Modders/Simoder-SDK/main?label=Download%20latest%20(nightly)&logo=github)](https://github.com/SimCity13-Modders/Simoder-SDK/archive/refs/heads/main.zip)
[![Download Latest Release](https://img.shields.io/github/v/release/SimCity13-Modders/Simoder-SDK?label=Download%20latest%20release&logo=github)](https://github.com/SimCity13-Modders/Simoder-SDK/releases/latest)
[![Star](https://img.shields.io/github/stars/SimCity13-Modders/Simoder-SDK?style=social)](https://github.com/SimCity13-Modders/Simoder-SDK/stargazers)

**Simoder-SDK** is the authoritative declaration, header, and contract matrix repository for **Simoder** mod authors. It ships typed definitions for **Luau** (`simoder.d.luau`, `base.d.luau`), **TypeScript / QuickJS ES2023** (`simoder.d.ts`), and **Native C/C++ `ABI v2`** (`Native/native.h`), together with `contracts-matrix.json` and an offline Rust verifier (`simoder-sdk-verifier`).

> [!NOTE]
> `Simoder-SDK` provides API/ABI declarations and IDE type definitions only — it contains no runtime binaries, compilers, or permission grants.

## Dependencies & Requirements

- **Target Runtime**: [Simoder](https://github.com/SimCity13-Modders/Simoder) + [Simoder-Base](https://github.com/SimCity13-Modders/Simoder-Base)
- **IDE Support (optional)**:
  - **Luau**: VS Code / Cursor with [`JohnnyMorganz.luau-lsp`](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.luau-lsp)
  - **TypeScript / JS**: VS Code / Cursor / any TS language server
  - **Native C/C++**: Any `i686-pc-windows-msvc` (32-bit x86) C/C++ toolchain
- **Verifier Tests (optional)**: Rust toolchain (`edition = "2024"`) for `crates/simoder-sdk-verifier`

## Installation & IDE Setup

Download the SDK from the **Release** or **Nightly** badge above (or add it as a Git submodule at `vendor/Simoder-SDK`), then configure your editor:

### 1. Luau (VS Code / Cursor)

1. Install **Luau Language Server** (`JohnnyMorganz.luau-lsp`).
2. Add to `.vscode/settings.json`:

```json
{
  "luau-lsp.platform.type": "standard",
  "luau-lsp.types.definitionFiles": [
    "${workspaceFolder}/vendor/Simoder-SDK/Luau/simoder.d.luau",
    "${workspaceFolder}/vendor/Simoder-SDK/Luau/base.d.luau"
  ]
}
```

### 2. TypeScript & JavaScript (QuickJS ES2023)

Add a `tsconfig.json` or `jsconfig.json` in your mod project root:

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

### 3. Native C/C++ (`ABI v2` x86 DLL Mods)

1. Add `vendor/Simoder-SDK/Native` to your compiler include directories (`/I` or CMake `target_include_directories`).
2. Include `#include "native.h"` and compile for 32-bit `i686-pc-windows-msvc`.
3. Export `SimoderModule_QueryV2`, `SimoderModule_PrepareV2`, `SimoderModule_AdmitV2`, `SimoderModule_QuiesceV2`, and `SimoderModule_DrainV2`.

---

## Contents & Contracts

- [Luau/BASE_API.md](Luau/BASE_API.md) & [Luau/README.md](Luau/README.md) — `base-luau-alpha-v1` and host Luau API guides
- [Native/README.md](Native/README.md) & [Native/native.h](Native/native.h) — Native `ABI v2` host/module structs and lifecycle contract
- [CONTRACTS.md](CONTRACTS.md) & [`contracts-matrix.json`](contracts-matrix.json) — Canonical cross-runtime capability matrix
- `sdk-manifest.json` — Exact source hashes and provenance

## Verification

```powershell
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
```
