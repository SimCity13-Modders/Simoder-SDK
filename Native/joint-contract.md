# Joint DLL / base / SDK contract

Format version1; contract R22-U2A-1; candidate for independent audit and human
acceptance. This is an authored contract, not an implemented public API. Source
facts/gaps are in discovery.md; numeric declarations in semantic-schema.json and
generated semantic-contract.h are candidate-only. BASE-001..009 remain normative.

## Ownership and placement

| Owner | Single responsibility |
| --- | --- |
| DLL | Native trust/admission, unforgeable identity bindings, handles/epochs/grants, copied bounded transport, execution lanes, native lifetime and result access |
| base, entirely Luau | One semantic manager registry, explicit service dependencies, domain validation/orchestration, task/facade experience and state reconciliation |
| SDK | One canonical declaration/schema input, generated projections, conformance fixtures, no Core implementation |
| DevTools/MCP | Optional native consumer, protocol/transport/catalog/auth adaptation, no authority mint or duplicate managers |

Recommend one isolated base authority VM per runtime, plus host-created local
facades in separate mod VMs, on one qualified serial script scheduler with fair
per-VM mailboxes. Singleton means one semantic authority, not one shared mutable
Lua global. Copy immutable value envelopes through DLL; keep manager decisions
in Luau. Native transport never becomes an all-purpose semantic manager.

Alternatives: copying base into every mod duplicates authority/state (reject);
one shared mod VM removes current isolation (reject); a separate base thread
preserves isolation but adds concurrency/shutdown complexity without measured need
(defer). A separate base process adds a game-side executable and is outside target.
Dedicated VM is recommended, not claimed empirically superior in performance.

## Origins and authority

Distinguish executing provider/native owner, originating mod+generation, base
generation, optional authenticated agent principal/session/channel, and runtime /
region / city / entity context. They are not interchangeable. Ordinary script mods
do not require an agent login; agent-originated operations additionally require
the separately qualified agent identity and applicable consent. Neither native
trust nor being base supplies another mod's permissions.

DLL creates the facade binding from the actual registered VM/generation, not
simoder.modId text, Lua arguments, a thread-local ambient caller or a module-supplied
principal. Invocation records are immutable host-issued opaque references bound
to one method descriptor, originating owner, execution provider, context and lease.
Every cross-VM hop carries that record; callbacks/await resume the original origin,
never base's authority or another mod's last callback context. No public mint or
grant setter. Production provider absence remains NOT_READY/DENIED.

Effective authorization is the intersection of origin permissions, provider's
allowed routing role and operation constraints. Applicable agent consent is
session-wide OR entity-specific, with mandatory gates still conjunctive. Recheck
at reservation/commit and before exposing a result: owner/generation, base/runtime
epoch, context, expiry, operation allowlist, native trust as applicable, principal /
agent session/channel/grant generation as applicable, readiness, SP/MP/sandbox and
state revision. A base view is neither a credential nor a transferable permission.

Use explicit cooperation APIs to exchange data. Facades, invocation records,
closures and VM userdata cannot be serialized to another mod. Entity references
cross owners only through an explicitly approved cooperation route that mints a
recipient-bound reference with narrowed rights; no generic transferable authority.
Native code is unisolated and may subvert the process; this contract is not a
hostile-native sandbox or secret-isolation guarantee.

## Execution and snapshots

Three lanes: script scheduler for all base/mod VM entries and continuations;
verified game-operation adapters on their qualified owning thread; native worker
validation/copying with no VM/game entry. U1 ordered lifecycle notifications must
enqueue copied facts; their publisher thread never calls Lua. Never hold a VM,
manager, authority or registry lock while entering another VM or calling user code.

Synchronous public reads use already-published immutable copied snapshots, never
a blocking cross-VM/game call. Missing cached data returns CapabilityUnavailable /
NotReady. Metadata includes runtime epoch, base generation, typed context identity /
epoch, state revision, publication sequence and monotonic observation time.
Historical observations are labelled historical; freshness is not live-game proof.
Public 64-bit counters/IDs use opaque handles or canonical decimal strings, never
lossy Luau/JS floating-point coercion. Copy/deep-freeze supported values; reject
cycles, functions, foreign userdata, raw addresses and secrets.

Async admission returns a task immediately after bounded copy/reservation, not a
successful game result. Each method has one reviewed schema, readiness/context,
permission and postcondition descriptor. Existing fixed session adapters remain
build/signature/thread gated. HWND observation is not a universal game-thread ABI.
No new game method, arbitrary eval/address/hook interface or extension catalog is
introduced here. Exact game payload bindings remain their parity gates.

## Task state and result delivery

Task phases: QUEUED -> RUNNING -> SUCCEEDED / FAILED / INDETERMINATE;
QUEUED -> CANCELLED; RUNNING(uncommitted) -> CANCELLED. RUNNING with commit flag unset may still cancel by atomic
reservation; once committed, cancellation returns BUSY and cannot undo/replay.
Phase RUNNING alone does not assert commit. Terminal phase/result never changes;
monotonic task sequence distinguishes observations. Success requires the method's
verified postcondition, not enqueue, native return or accepted UI request alone.
Unknown outcome at deadline is INDETERMINATE, not cancelled or safely retryable.

Callbacks and await use the same terminal/result-access rules. onCompleted receives
only a permitted successful value; onFailed receives a structured non-success /
delivery error. await returns value,nil or nil,error. Each registration fires at
most once; registering after terminal schedules a later scheduler turn, never
inline reentry. Multiple registrations remain separately bounded/owned. Callback
failure follows existing DEV/REGULAR policy; it does not roll back committed work.

Result access is separate from stored operation outcome. If consent is revoked
after success but before delivery, do not expose the result. A still-active owner
may receive a sanitized PermissionDenied diagnostic through callbacks/await; the
internal operation remains SUCCEEDED. Revocation does not rewrite history. Status
and poll return access errors rather than privileged cached metadata. Cleanup
release remains owner-bound and reveals no result after revocation.

Await is allowed only in a host-registered schedulable coroutine of the current
generation. A raw coroutine.create does not register it. Unsupported context yields
a controlled AwaitOutsideScheduler error; no blocking thread wait. Host retains
the coroutine reference, origin and continuation token, resumes only on its VM
lane, and checks generation/authority before resume. Native C-boundary yield/resume
must be proven before advertising await. Current lua_pcall is not that proof.

## Generation retirement and stale context

Close admission first; mark ordinary completions/subscriptions/continuations
ineligible; cancel uncommitted work. A callback already executing retains its VM
and dependencies until return; no new ordinary callback begins after this fence.
A bounded host-scheduled onDisable teardown may run once while the retained VM is
RETIRING, with no new task/game-mutation admission. It is a distinct cleanup entry,
not a completion callback, and must finish before final VM destruction. After the
final callback fence no entry, including teardown, is allowed.

Remove queued old callbacks and release stopped coroutines without resuming an old
VM. Already committed native work may finish into host-owned bounded records;
never call old-generation code to report it. Timeout retains resources/code with
admission closed; no forced thread stop or executing-code unload. Base failure/
reload also revokes dependent bindings and services; no temporary second authority.

Prepare/entry/onEnable cannot start privileged tasks or listeners. New task calls
before activation return NotReady. Existing legacy staged PROP initialization is
declarative preparation, not authorization or a promise it already uses this API;
its migration/parity must preserve behavior explicitly. After postcommit readiness,
operational callbacks run on the qualified scheduler, replacing the current mixed
caller-thread lifecycle path only in a later reviewed implementation.

Handles bind issuing owner generation, kind and relevant context epoch. City/region
replacement, unload or identity loss invalidates relevant handles even if the same
numeric object reappears. Never automatically rebind. Runtime-wide facade discovery
can survive city changes; a context-specific facade/handle remains stale and must
be explicitly obtained anew. Unknown SP/MP blocks mutation; MP adds its separate
approval and verified operation policy, not a permissive default.

## Ordered state and resource bounds

Atomically reserve subscription+initial immutable snapshot, then append increasing
sequence changes. Marshal both through the same per-owner queue; initial cannot
be overtaken. Carry context and generation; revalidate immediately before callback
reservation. Unsubscribe closes new reservations; BUSY means in-flight storage/code
must remain retained. Cross-VM adapter cannot promise stronger instantaneous
interruption of an already-running callback.

Reuse U1 ceilings: eight queued/running generic operations shared across scopes,
128 retained task/operation/invocation slots per generation, 64 retained resource slots,
1 MiB input/result envelope; stricter existing per-operation limits still apply.
Candidate mailbox cap:128 events and1 MiB aggregate copied bytes per generation;
callbacks/subscriptions together at most64 registrations and also subject to
resource capacity (VM/dependencies already consume slots). Overflow rejects new
admission explicitly. An ordered subscription faults/closes on overflow; never
silently drops events and continues a falsely contiguous stream. Explicit
resubscription obtains a new current snapshot.

Task terminal storage is reserved at admission and remains pollable under its
lease. Coalesced completion-ready tokens point to that bounded storage; event
backpressure never replays game work or loses the terminal record. Delivery has a
bounded deadline; expired delivery releases continuation references and reports
only a permitted delivery error. Scope stop releases all registrations at safe drain.

Base candidate allocation is8 MiB inside the existing128 MiB aggregate declared
VM budget, not an extra unaccounted VM. Preserve current per-VM2..64 MiB and
callback1..20ms limits; target default2ms. Existing scheduler5ms cycle is a target,
not a proven hard aggregate bound. U3 must qualify fairness, interrupt/resume and
actual aggregate latency before declaring performance. Native image count/bytes
must be finitely budgeted and measured in U2b before production admission; no
unmeasured DLL/memory/latency target becomes an accepted performance result.

## Initialization DAG and availability

Core ownership/dispatch exist before base; base can initialize without loading a
native mod. Base readiness then gates production native admission. No cycle where
base needs the very native MCP service that requires base readiness.

Candidate minimum authored graph (names are contract module IDs, not shipped APIs):
values -> tasks and lifecycle -> registry -> runtime-manager. All five are critical.
props-manager and regions-manager depend on registry+runtime-manager and are
optional startup domains while providers are unavailable. They wrap existing
capabilities only; unavailable methods report explicit availability/errors. Optional
startup status does not waive Beta/tool parity requirements or runtime fault policy.

Validate all graph edges/missing dependencies/cycles before initialization effects;
topological construction uses explicit injected dependencies and stable ID tie-break.
Unwind successful resources in reverse order on critical failure. An optional
failure blocks its dependents; a critical descendant makes base unavailable.
Independent optional domains may remain available only if critical base is healthy.
Criticality is reviewed package metadata, never an arbitrary mod's self-claim.
Core stops/drains dependent native services even if Luau cleanup fails.

## Acceptance and traceability

BASE001 placement/one registry;002 host origins/facades;003 cached reads/async lanes;
004 task/await contract;005 generation fences/drain;006 typed stale contexts;
007 atomic ordered marshaling;008 explicit DAG;009 dependency/criticality rollback.
Native ABI/version reconciliation and consumer gates are in native-boundary.md;
conformance vectors and U2b ownership plan are separate review artifacts.
Only independently audited, explicitly human-accepted identities may be promoted
to SDK/docs. Generated declaration matching is not behavioral conformance.

## Original task observation sequence and revision

The original retained task publishes sequence1 only after accepted queue insertion.
Sequence changes only when its externally visible phase/commit/outcome/result tuple
is published. Preparing is RUNNING without commit; the irreversible commit marker
is a further observation. A compound publication under the original task lock need
not invent an unobservable intermediate tuple. Polling, sizing and copying never
advance sequence. Terminal tuples are immutable until authorized release.

Revision initially identifies the original admitted operation-context revision.
On commit it becomes the exact original domain writer increment/Advance result;
later unrelated domain updates do not restamp the retained task. Precommit failure
or cancellation retains admission revision. Checked sequence capacity reserves the
terminal publication before any irreversible effect; no wrap, saturation or lost
postcommit observation may be presented as a new result.
