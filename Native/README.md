# Native host ABI version 2

This declaration-only package is the canonical source for the native module
boundary. Runtime compatibility is qualified by the separate DevTools native-module
fixtures; it is not a compiler, runtime implementation or execution permission.
U2b supports Query/Prepare/Quiesce/Drain and factual lifecycle observations.
Production Admit returns NOT_READY until a real base provider qualifies.
Dispatch, credentials and semantic facets are NULL: their declarations reserve the
accepted contract layout but expose no callable capability or new Luau/TypeScript API.

The alpha loading profile is PE32 x86, no entry point, TLS, resources, bound/delay
imports or CLR. Named kernel32 imports only; no CRT dependency. Explicit reviewed
content and native trust are required before loading; native code is unrestricted.
Images and borrowed host tables stay retained until process exit. This is not yet
the general native-mod installation workflow or Beta publisher authentication.

Numeric registry schema.json, generate.py, inherited abi_v1.h/baseline-schema.json
and the joint/native/value/conformance contracts form identity.json. Regeneration
changes the ABI identity when any normative input changes. Generated native.h and
identity.json are excluded from the input hash map. Relocation from accepted U2a
intentionally rebinds the identity; the original accepted bundle digest is
273d45c89b5605f012e6eff0fabb917e92dd0ad94507521c8980a1a1b8c0e3ea.
Consumers pin canonical generated file hashes and explicitly refresh copies.
The contract documents retain future-stage requirements; C01-C18 are not all
implemented or tested by U2b. Runtime behavioral evidence belongs to its stage report.
