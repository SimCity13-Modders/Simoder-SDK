# Candidate native boundary and R19 reconciliation

Format version1; R22-U2A-ABI. Not installed SDK/export/loader or runtime acceptance.
R19 candidate.h/contract.json are immutable baseline inputs, hashed in contract-identity.json.
R20 fresh-only guard and accepted U1 retention supersede R19's abandoned-owner
recovery discussion. Never copy that obsolete recovery path into implementation.

## Version and representation

Keep R19 fixed-width x86 PE32/__cdecl/pack4, SdHeader8, SdHandle8, SdContext64,
SdState40 and v1 lifecycle/dispatch/credential tables. Preserve result0..11 and
readiness1/2/4. Internal C++ enum order is different: explicit mapping is mandatory,
never static-cast ScopeResult to SdResult. No STL, exceptions, C++ ownership,
allocator, OS/game handle, Lua state/closure or foreign VM userdata in the ABI.

R19 root host table28 bytes had no semantic bridge. Candidate v2 root is32 bytes,
with a separately versioned semantic table. This deliberately supersedes the
unpublished v1 root, not a shipped compatibility promise. v1 root is rejected,
not silently interpreted as v2. Leaf v1 tables remain unchanged. Exact size/version
and zero reserved fields required; no inferred tail compatibility. Unknown method,
flag or enum fails closed before side effects. Alpha does not need a migration shim.
Single numeric registry is semantic-schema.json; generate.py owns the declaration
field/signature template and creates the header/static layout checks. Both belong
to the canonical identity bundle together with joint/native/value contracts,
conformance vectors and immutable R19 declaration/schema inputs. Contract identity
is SHA256 of a sorted canonical UTF-8 JSON file->SHA256 map (no spaces/newline),
not schema hash alone. Generated header/identity output are excluded from inputs
to avoid cycles. Paths are logical relative identifiers and bytes are hashed exactly.
SDK promotion requires revalidation/rebinding, not silent path/template rewriting.

Candidate added result12..16: CapabilityUnavailable, ContextExpired, OperationFailed,
ResourceLimit, Deadline. The old statuses retain their meaning; stale native handle
is distinct from an expired city/region context. Task phase/commit flags and copied
snapshot metadata are defined in the generated header and joint-contract.md.

## Root negotiation and module lifecycle

Required exact undecorated exports (verified .def names in U2b):
SimoderModule_QueryV2, SimoderModule_PrepareV2, SimoderModule_AdmitV2,
SimoderModule_QuiesceV2, SimoderModule_DrainV2.
Query takes host ABI integer and caller-sized descriptor; descriptor declares
ABI2/base-contract1 and the exact canonical contract SHA256. Host rejects mismatch,
reserved fields and unsupported minimum dependencies, not guessing compatibility.
Prepare borrows immutable host tables until successful drain; zeroed outputs on
failure. No public table mutates trust, grants, readiness or provisioning.

Native trust/content/search policy must be approved **before LoadLibrary/Query**:
DllMain/TLS/static initializers already execute during load. Host admission cannot
sandbox a malicious DLL or prevent it directly opening sockets/reading memory.
Prepare/no-listener is a cooperative module requirement; API admission gates
protect host operations, not arbitrary native machine code. Double native-consent
and staged private integrity/public Beta publisher policy remain prerequisites.

Prepare stages only; persistence/registry postcommit, critical base readiness and
all module-specific trust/policy gates precede Admit. Module scope states remain
PREPARED -> ACTIVE -> STOPPING -> DRAINED, with failed prepare/admit quiesced and
drained, no second ACTIVE service. Quiesce closes admission and is nonblocking /
idempotent; Drain BUSY retains code/resources. Native callback invocations must
contain C++ exceptions internally. Cooperative nonblocking contracts are not hard
preemption: malicious or hung native code can hang/crash the process.

V2 optional facets: lifecycle for permitted facts; dispatch only for reviewed
low-level capabilities; credentials absent until qualified authentication provider;
semantic absent until qualified base bridge. Null means unavailable, never fallback.
Missing production authority continues to deny dispatch. Native trust alone grants
no agent session. Base readiness is private host-managed dependency state, not
something a module/facade may assert via a boolean.

## Semantic transport table

Native invoke/snapshot/poll/cancel take executing owner plus a host-issued opaque
invocation reference and method/task identity. The invocation reference binds the
full origin/provider/agent/context semantics in joint-contract.md; a valid handle
alone is insufficient. Registered VM adapters issue script bindings privately.
Native bind_invocation resolves existing host authority: an agent-provider owner
must supply its authenticated owner/channel-bound session from credential.authenticate;
zero session is denied. A reviewed native-mod owner uses zero session and only its
own existing native-mod permissions; it cannot assert another mod or agent identity.
The role comes from trusted host registration, never a caller-selected origin flag.
Official MCP always has agent-provider role and cannot downgrade to native-mod
authority. The host fills identity/grant/expiry bindings; unqualified providers deny
binding. This function provisions no grant or credential. Target handles and expected
revision are checked against the method/context. No credential
proof is required from an ordinary script facade, and no agent route can masquerade
as an ordinary mod to bypass consent. Low-level native diagnostics remain separately
scoped and are not automatically exposed as powerful Luau manager methods.
release_invocation is issuing-owner cleanup, not result access or cancellation.
It returns BUSY while any task/subscription retains the invocation, including
terminal tasks until task release. After successful invocation release every use
is STALE; slots are not reused while retained. Tasks keep the original live binding
for authorized poll/cancel/cleanup. All
retained task/operation/invocation references share the128-slot generation cap.
Public slot32+generation32 maps to host records; never truncate/bit-cast U1's
internal64-bit generation/serial identity into that representation.

Method IDs reference host-approved schema/operation descriptors, not function
addresses, free-form executable strings, MCP tool names or a new plugin registry.
This unit assigns no new gameplay opcode. U2b fixtures use test-only descriptors;
production semantic table remains unavailable until U3 and real descriptor mappings.
Operation-specific schemas/limits and verified existing adapters remain mandatory.

SdBytes values use the bounded nonexecutable value-contract.md encoding; generic
value transport is not JSON interpreted as code or a new MCP parser.
Caller input is copied before return. Outputs use caller storage, capacity0/null
for authorized sizing; BUFFER_SMALL writes no partial payload. Nonzero capacity
requires valid storage. required=0 and output handles zero on denial/error except
authorized BUFFER_SMALL returns required bytes; preserve
only valid size/version headers, no unauthorized length/metadata oracle. Pointer
validity is a trusted native caller obligation, not a hostile-pointer sandbox.
No caller buffer is retained after return. Host owns task/result storage and tracks
all leases. Release(owner,task) is owner-only cleanup after revocation, no result
disclosure and BUSY while committed work retains the record; double release is STALE.

Snapshot metadata refers to the immutable copy observed in that call. Two sizing/
copy calls can observe different publications; changed required size returns
BUFFER_SMALL again, never overflow. Callers bound retries; a stable pinned result
uses an admitted task. Poll returns an immutable terminal record until release,
rechecks result rights, and never executes the operation again. Task information
phase is separate from API return status. Poll metadata for pending/empty results
can succeed without BUFFER_SMALL when capacity0 suffices; failure/error output is
sanitized under the same permission gates.

Native v1 lifecycle callbacks retain their native storage/code through drain and
may run on a host publisher thread; they do not enter Lua/game. Script subscriptions
go through the qualified scheduler, never by casting a Lua closure to native callback.
No synchronous cross-VM call or blocking await is introduced by this table.

## Loading, retention and budgets for U2b

Freeze a safe search policy with exact dependency allowlist/import identity,
plain ancestors/reparse rejection, immutable content hash, use-time file identity,
loaded-image identity and dependency substitution checks. No current-directory /
PATH probing, implicit sibling DLL, unchecked imports or search fallback. Platform
APIs/flags must be verified against local SDK/official primary docs before coding.
Unsigned private artifacts need exact reviewed pins/integrity; Beta signed publisher
requirements are not waived or called implemented.

Conservative default: retain every loaded native image until process exit, including
failed preparation; stop/drain releases optional resources only. Do not infer that
returning from Drain proves arbitrary native thread quiescence or justifies
FreeLibrary. Core remains pinned and retains its hook trampolines. Logical reload
always gets a fresh owner generation; the module must scope mutable data by owner.
Same-content image reuse does not create a second authority. Changed content needs
a separately verified immutable load location; old images remain retained. Budget
exhaustion rejects loading/reload with RESTART_REQUIRED before executing new code.

U2b must freeze finite retained-image count/byte limits from its compiled fixtures
and measured dependencies before production admission. Its test profile may use two
images to exercise exhaustion; that is not a production performance target. Existing
U1 operation/resource caps and script aggregate budgets are not silently enlarged.

## Consumer gate ordering

U2b can implement native loading/retention in a standalone synthetic host fixture
without claiming base is implemented. Synthetic base/authority markers exist only
in independent test targets, excluded from core distribution. Production admission
requires real base and qualified authority; no dummy base-ready switch, test issuer,
anonymous native game call or fake agent credential in the product.
U3 supplies real Luau scheduling/semantics; U4 supplies MCP transport/auth; U5 cutover
and U6 actual game parity remain later gates. Missing later providers does not block
bounded U2b fixture implementation, but prevents their production consumers.

## Task execution-context observations

SdTaskInfo.context identifies the original task's full retained execution context,
including its unique operation, complete factual mask, typed facts and generations.
It is scoped to the issuing owner/provider generation and original task lifetime.
Different tasks therefore have different context-observation handles even when they
concern the same city or region. This is informational metadata, never authority,
a Bind target, a stable city identifier or a public context resolver. Task release
invalidates its mapping; reuse never aliases an old generation. Never truncate or
bit-cast the private64bit context identity. Task, invocation and context lookup kinds
remain distinct in their owning provider. Scalar bytes alone cannot prove foreign
target provenance; real typed target issuance/validation remains required before
production Bind attachment. No global numeric namespace is implied.

SdSnapshotInfo.context instead identifies the cached snapshot publisher's original
typed context. Do not substitute a task context or promise cross-task/snapshot handle
equality. Unqualified cached snapshot producers remain unavailable.

Successful native Poll delivers one authorized original task snapshot. API SD_OK
means observation delivered, not task success: phase, error and committed flag are
separate. Sequence/revision follow joint-contract.md; result bytes are the original
retained value-contract envelope, never replayed/recomputed. Empty queued/running
observations may succeed with capacity0 and no payload. BUFFER_SMALL discloses only
authorized required length; task metadata and payload remain zero on that return.
All other failed access clears metadata, payload and length while preserving the
caller size/version header. Revalidate original binding/context/authority before
delivery; revocation suppresses disclosure without rewriting retained outcomes.
