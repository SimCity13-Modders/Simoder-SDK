# Base Luau profile

SDK **0.2.0**, profile **base-luau-alpha-v1**, describes the current native-bound
Base presentation. This is an alpha contract, not a public Beta compatibility
promise. Add `base.d.luau` to editor definition files alongside the legacy
`simoder.d.luau`. Do not require either declaration at runtime.

`base` exists only when the native host binds the verified Base factory to that
mod VM. A legacy/unavailable host may expose no `base`; check it before use.
Preparation can expose the facade while operations return `nil, "NotReady"`.
Every function uses **dot calls**, including task and subscription functions.
Tables are frozen and copied values are opaque, immutable native userdata owned
by the original VM. Editor shapes do not enforce runtime ownership or grant access.

```luau
local api = base
if api == nil then return {} end
return {onEnable = function()
    local snapshot, reason = api.snapshot()
    if snapshot == nil then print(reason); return end
    print(snapshot.generation) -- exact decimal string
end}
```

## Values, permissions, tasks and delivery

`permissions.query(name)` returns a frozen `BasePermissionView` (`available`,
`selected`, `granted`, `effective`, `restricted` booleans) for the closed
capability set (`game.read`, `game.write`, `runtime.observe`, `runtime.mutate`,
`filesystem.read`, `filesystem.write`), or `nil, "UnsupportedCapability"` (or
another sanitized denial code such as `"Revoked"` or `"Expired"`). During
restricted mode (`onEnable` / preparation), `effective` is always `false`. The
returned view is informational only and never acts as a reusable authorization token.

`values.validate(wire, schema)` returns a native copied value or `nil, code`.
Use the canonical tagged envelope and reviewed exact schema; integers/references
do not silently become floating point or rebind to a new context.
`values.wire(value)` returns its canonical wire; forged or foreign values throw
`Invalid owner value`. Serialization is not authorization.

`request(method, value)` returns a frozen task or `nil, code`. Acceptance is
queueing, not completion. The production catalog may be empty; this profile
declares no callable `fixture.echo`, navigation manager or public descriptor issuer.
Task `poll()` returns an outcome or `nil, code`; outcome state is queued, running,
succeeded, failed, cancelled or indeterminate. `committed` is independent of
terminal state, `result` is optional opaque userdata, and `error` is a status code.
`cancel()` returns a status string; after commitment it returns Busy, without rollback.

`onCompleted(callback)` and `onFailed(callback)` return admission `true` or
`nil, code`, not registration/unsubscribe handles. Delivery occurs on a later
original-owner lane turn, at most once: completion receives succeeded outcomes;
failure receives nonsuccess outcomes or `nil, code` on access denial. Permissions,
context and original generation are checked again when accessing/delivering results.

`run(callback)` likewise returns admission `true` or `nil, code`, not a cancellable
coroutine object. `task.await()` yields only inside a host-registered `base.run`
coroutine; ordinary `coroutine.create` does not qualify. A stopped/replaced
generation does not transfer old continuations to its successor.

## Lifecycle and discovery

`snapshot()` returns a frozen observation, not a permission lease. baseEpoch,
baseGeneration, generation, runtimeEpoch, revision, sequence, monotonicNs and
every context slot/generation/epoch are exact **decimal strings**. `ready` and
`singlePlayerKnown` are booleans; there is no declared singlePlayer field.
Contexts are runtime, region, city and entity.

`subscribe(callback)` returns a frozen unsubscribe handle or `nil, code`, with
an atomic initial snapshot followed by ordered changes. Callback access denial
may supply `nil, code`. A faulted subscription reports `nil, "MailboxOverflow"`
when its bounded ordered-delivery mailbox overflows; it is not a successful
snapshot or permission decision. `unsubscribe()` returns Ok, Busy while executing, or Stale;
it is not boolean completion. Native bounded admission/overflow may reject work.
`descriptors()` returns frozen metadata, possibly an empty sequence, or `nil, code`.
Metadata has id, kind=task, inputSchema, lane, context, capability, postcondition,
and bounded numeric maxResultBytes. Discovery never grants authority.

## Failures and verification limits

Status strings are Ok, Denied, Stale, NotReady, Busy, Conflict, Cancelled,
Indeterminate, BufferSmall, Invalid and selected presentation admission
ResourceLimit and subscription-delivery MailboxOverflow. Not every function emits every code. Successful single-value
returns have an absent second return; editor optional pairs intentionally permit
the runtime multi-return paths without inventing a dependent type system.
Invalid callback throws BASE_CALLBACK_INVALID; admission exhaustion may throw
ResourceLimit. Await outside a registered/yieldable boundary throws. Exceptions
are separate from nil/code results; the declarations do not turn exceptions into values.

Declaration parsing validates grammar, not full static analysis/editor inference.
Actual API compatibility requires canonical-file execution against the verified
Base factory/native endpoint in isolated host tests, including permission/context
denial, copied-value ownership and continuation cleanup. No game qualification is
implied by package tests. Full analyzer evidence must be labelled separately.

This package installs no compiler, runtime, downloader, Node.js or JavaScript
engine. Existing TypeScript declarations describe the legacy TypeScriptToLua
pipeline; they do not promise Base TypeScript/JavaScript support.
