# Simoder Beta Supported Contracts Matrix

This document and the machine-readable [`contracts-matrix.json`](contracts-matrix.json) define the canonical API/ABI surface for Simoder Beta (`SDK 0.2.0`), pinned to the approved `Simoder` and `Simoder-Base` implementations.

Every entry specifies:
- **Version**: Semantic or schema contract version.
- **Owner**: Authoritative repository (`Simoder`, `Simoder-Base`, `Simoder-SDK`, `Simoder-DevTools-Authoring`).
- **Status**: `supported` (implemented and verified) or `unavailable` (reserved in layout/schema, fails closed at runtime).

## 1. Legacy & Core Luau Host Contract (`simoder-luau-v1`)

| Contract Item | Version | Owner | Status | Declaration |
| --- | --- | --- | --- | --- |
| `simoder.apiVersion` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |
| `simoder.modId` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |
| `simoder.log` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |
| `simoder.time` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |
| `simoder.hasCapability` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |
| `simoder.props.getFloat` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |
| `simoder.props.setFloat` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |
| `simoder.props.clearFloat` | `1.0.0` | `Simoder` | `supported` | `Luau/simoder.d.luau` |

## 2. Base Semantic Luau Facade (`base-luau-alpha-v1`, Catalogue Contract `2`)

| Contract Item | Version | Owner | Status | Declaration |
| --- | --- | --- | --- | --- |
| `base.permissions.query` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `base.values.validate` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `base.values.wire` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `base.request` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `base.run` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `base.snapshot` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `base.subscribe` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `base.descriptors` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `BaseTask.poll` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `BaseTask.cancel` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `BaseTask.onCompleted` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `BaseTask.onFailed` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `BaseTask.await` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |
| `BaseSubscription.unsubscribe` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `Luau/base.d.luau` |

## 3. Native Module Host ABI (`native-abi-v2`)

| Contract Item | Version | Owner | Status | Declaration |
| --- | --- | --- | --- | --- |
| `SimoderModule_QueryV2` | `2.0.0` | `Simoder` | `supported` | `Native/native.h` |
| `SimoderModule_PrepareV2` | `2.0.0` | `Simoder` | `supported` | `Native/native.h` |
| `SimoderModule_AdmitV2` | `2.0.0` | `Simoder` | `supported` | `Native/native.h` |
| `SimoderModule_QuiesceV2` | `2.0.0` | `Simoder` | `supported` | `Native/native.h` |
| `SimoderModule_DrainV2` | `2.0.0` | `Simoder` | `supported` | `Native/native.h` |
| `SdHostApiV2.lifecycle` | `2.0.0` | `Simoder` | `supported` | `Native/native.h` |
| `SdHostApiV2.dispatch` | `2.0.0` | `Simoder` | `unavailable` | `Native/native.h` |
| `SdHostApiV2.credentials` | `2.0.0` | `Simoder` | `unavailable` | `Native/native.h` |
| `SdHostApiV2.semantic` | `2.0.0` | `Simoder` | `unavailable` | `Native/native.h` |

## 4. JavaScript (QuickJS ES2020) & TypeScript-to-JS Pipeline (`js-ts-beta-v1`)

| Contract Item | Version | Owner | Status | Declaration |
| --- | --- | --- | --- | --- |
| `simoder.apiVersion` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `simoder.modId` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `simoder.log` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `simoder.time` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `simoder.hasCapability` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `simoder.props.getFloat` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `simoder.props.setFloat` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `simoder.props.clearFloat` | `1.0.0` | `Simoder` | `supported` | `TypeScript/simoder.d.ts` |
| `base.permissions.query` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `base.values.validate` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `base.values.wire` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `base.request` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `base.run` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `base.snapshot` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `base.subscribe` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `base.descriptors` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `BaseTask.poll` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `BaseTask.cancel` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `BaseTask.onCompleted` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `BaseTask.onFailed` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
| `BaseTask.await` | `0.2.0-alpha` | `Simoder-Base` | `supported` | `TypeScript/base.d.ts` |
