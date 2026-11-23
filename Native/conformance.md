# U2a conformance matrix

All behavioral cases below are required future tests, currently NOT_RUN. Header compilation establishes layout only. Test-only providers cannot establish production base, authority or game readiness.

| ID | Setup / action | Required oracle | Stage |
| --- | --- | --- | --- |
| C01 / BASE001 | Separate base and two mod VMs; attempt cross-VM closure/pointer transfer | Rejected; only copied values and checked references cross boundary | U3 |
| C02 / BASE002 | Call native provider with spoofed origin, missing agent session or base-mediated wider permission | DENIED; base cannot launder origin; session grant OR entity grant still subject to mandatory safety gates | U4 |
| C03 / BASE003, BASE006 | Read cached snapshot while game lane is busy; replace city then use old ref | Read never enters VM/game; old context returns CONTEXT_EXPIRED, no rebinding | U3/U6 |
| C04 / BASE004 | Queue, cancel before commit, cancel RUNNING before commit, cancel after commit | First two cancellation paths end CANCELLED; committed cancellation BUSY, no undo or replay | U3 |
| C05 / BASE004 | Commit then lose postcondition visibility | INDETERMINATE, never fabricated success or retry | U3/U6 |
| C06 / BASE005 | Register completion late, throw inside handler, revoke before result delivery | Later scheduler turn only; isolated failure; no privileged result after revoke; stored outcome unchanged | U3/U4 |
| C07 / BASE005 | await from raw/unregistered coroutine; stop during suspended await | Controlled AwaitOutsideScheduler; stopped VM never resumed | U3 |
| C08 / BASE007 | Subscribe during state transition; overflow mailbox; unsubscribe while callback runs | Atomic initial snapshot then ordered sequence; explicit fault/close on overflow; BUSY while inflight | U3 |
| C09 / BASE008, BASE009 | Dependency cycle, missing dependency, optional failure with critical descendant | Validate before effects; deterministic order/reverse unwind; critical readiness closed | U3 |
| C10 / BASE004, BASE005 | Two mods hit resource limits and long callbacks | Bounded storage, fair scheduling evidence; no claim of hard 5ms cycle from per-callback limit | U3 |
| C11 / BASE009 | Stop/reload base while operation committed and native callback inflight | Admission closes; safe drain/retention; stale generations denied; no forced unload | U2b/U3 |
| C12 | Terminal task still retained; release invocation; release task then invocation; reuse old reference | First release BUSY; final release OK; every subsequent use STALE, no generation alias | U3 |
| C13 | Wrong header size/version/reserved bits/architecture/contract digest | Reject before admission; x64 compile rejects explicitly | U2a layout, U2b behavior |
| C14 | Untrusted DLL, invalid imports/dependency, changed bytes between validation/load | Trust before LoadLibrary; approved immutable identity only; no PATH fallback | U2b |
| C15 | Prepare fails, Drain BUSY, retained image budget exhausted | Code stays mapped to process exit; no forced unload; RESTART_REQUIRED before new execution | U2b |
| C16 | Sizing call capacity0/null, then undersized and denied request | BUFFER_SMALL may report required bytes; other errors disclose zero payload/required bytes | U3 |
| C17 | Exact u64 above 2^53, signed zero, duplicate JSON keys, NaN bits, bad ref, depth/byte overflow | Exact round-trip for valid values; INVALID or RESOURCE_LIMIT before effects for invalid values | U3 |
| C18 | Agent provider calls bind with zero session; native mod requests provider role | DENIED; role fixed by host, never selected by caller | U4 |

Fixtures for C17 must include u64 `18446744073709551615`, i64 `-9223372036854775808`, f64 `8000000000000000`, reject u64 `18446744073709551616`, decimal `01`, f64 `7ff0000000000000`, duplicate envelope keys, duplicate map keys, and zero-generation refs. Boundary tests exercise every inclusive limit and one over it. Native fixtures run in a disposable host before game use. U6 then repeats applicable effects against the approved sandbox region and preserves evidence.

## Native task observation additions

These are required conformance cases, not execution evidence or production grants.

| ID | Setup / action | Required oracle | Stage |
| --- | --- | --- | --- |
| C19 | Queue, prepare, observe committed running, then terminal; poll repeatedly after unrelated writer advance | Original sequence starts1 and changes only on tuple publication; original admission/commit revision retained; terminal immutable | U4 |
| C20 | Exhaust sequence capacity before commit; exercise normal and prepared writer paths | Reject before effects; reserve terminal capacity; no wrap, fabricated result or extra domain increment | U4 |
| C21 | Observe two tasks against the same route, input-added context facts, release/reuse, foreign owner/provider and wrong native kinds | Distinct original execution-context observations; every original fact/epoch validated; no task/invocation alias or stale generation reuse | U4 |
| C22 | Size/copy terminal envelope, undersize buffer, revoke or replace context immediately before delivery | Required-only BUFFER_SMALL; whole original tuple/envelope on success; sanitized failure with no replay or stored-outcome rewrite | U4 |

Typed Bind target provenance is a separate production qualification; a private
provider's own kind checks or numeric equality do not prove a global target issuer.
