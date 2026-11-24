/* @summary U2a candidate only, not a deployed SDK or implemented export. */
#ifndef SIMODER_U2A_SEMANTIC_CONTRACT_H
#define SIMODER_U2A_SEMANTIC_CONTRACT_H
#include "abi_v1.h"
/* @summary Complete normative bundle identity, excluding generated outputs. */
#define SD_CONTRACT_SHA256_HEX "00c98e36f669c6c4f16f7ffd9073130ae9798543b3d0759d7c1937f443e6e236"
#define SD_U2A_HOST_VERSION 2u
#define SD_SEMANTIC_VERSION 1u
#define SD_BASE_CONTRACT_VERSION 1u
/* @summary Additional explicit result; existing0..11 unchanged. */
#define SD_CAPABILITY_UNAVAILABLE ((SdResult)12)
/* @summary Additional explicit result; existing0..11 unchanged. */
#define SD_CONTEXT_EXPIRED ((SdResult)13)
/* @summary Additional explicit result; existing0..11 unchanged. */
#define SD_OPERATION_FAILED ((SdResult)14)
/* @summary Additional explicit result; existing0..11 unchanged. */
#define SD_RESOURCE_LIMIT ((SdResult)15)
/* @summary Additional explicit result; existing0..11 unchanged. */
#define SD_DEADLINE ((SdResult)16)
/* @summary Task phase, independent of API result and commit flag. */
#define SD_TASK_QUEUED 0u
/* @summary Task phase, independent of API result and commit flag. */
#define SD_TASK_RUNNING 1u
/* @summary Task phase, independent of API result and commit flag. */
#define SD_TASK_SUCCEEDED 2u
/* @summary Task phase, independent of API result and commit flag. */
#define SD_TASK_FAILED 3u
/* @summary Task phase, independent of API result and commit flag. */
#define SD_TASK_CANCELLED 4u
/* @summary Task phase, independent of API result and commit flag. */
#define SD_TASK_INDETERMINATE 5u
/* @summary Task commit reservation flag. */
#define SD_TASK_COMMITTED 1u
#pragma pack(push, 4)
/** @summary Original copied task observation; error is SdResult domain, flags only defined bits.
 * @remarks context identifies this task full execution lease, scoped to owner/provider generation and task lifetime; informational, not a Bind target or credential.
 * Sequence starts1 on insertion and changes only on tuple publication; revision retains original admission or exact commit writer revision.
 */
typedef struct SdTaskInfo { SdHeader header; SdHandle task; SdHandle context; uint32_t phase; uint32_t error; uint32_t flags; uint64_t revision; uint64_t sequence; } SdTaskInfo;
/* @summary Metadata for the immutable snapshot returned in this call; identities are observations, not authority. */
typedef struct SdSnapshotInfo { SdHeader header; uint64_t runtime_epoch; uint64_t base_generation; SdHandle context; uint64_t context_epoch; uint64_t revision; uint64_t sequence; uint64_t observed_monotonic_ms; } SdSnapshotInfo;
/* @summary Host-mediated value interface; owner+invocation binding is revalidated on every call. */
typedef struct SdSemanticApi {
    SdHeader header;
    /* @summary Resolve existing host authority, never grant it. @param owner Host executing owner/role. @param session Verified agent session, mandatory for agent-provider role; zero only for native-mod role. @param method Reviewed descriptor. @param target Owned typed context reference. @param revision Expected host revision. @param invocation Zeroed on failure. @return Bound immutable reference or explicit denial. */
    SdResult (SD_CALL *bind_invocation)(SdHandle owner, SdHandle session, uint32_t method, SdHandle target, uint64_t revision, SdHandle *invocation);
    /* @summary Read cached immutable values, no VM/game entry. @param owner Executing native owner. @param invocation Host-issued origin. @param method Reviewed descriptor. @param metadata Caller-sized output. @param output Caller storage. @param capacity Bytes available. @param required Authorized required bytes. @return Status, no partial payload. */
    SdResult (SD_CALL *snapshot)(SdHandle owner, SdHandle invocation, uint32_t method, SdSnapshotInfo *metadata, uint8_t *output, uint32_t capacity, uint32_t *required);
    /* @summary Copy and enqueue without claiming completion. @param owner Executing owner. @param invocation Bound origin. @param method Reviewed descriptor. @param input Copied bounded values. @param task Zeroed on error. @return Admission status only. */
    SdResult (SD_CALL *invoke)(SdHandle owner, SdHandle invocation, uint32_t method, SdBytes input, SdHandle *task);
    /* @summary Observe task; never execute/replay it. @param owner Executing owner. @param invocation Original bound origin. @param task Owned task. @param info Caller-sized phase output. @param output Caller result storage. @param capacity Bytes available. @param required Authorized bytes. @return Access/buffer status distinct from phase. */
    SdResult (SD_CALL *poll)(SdHandle owner, SdHandle invocation, SdHandle task, SdTaskInfo *info, uint8_t *output, uint32_t capacity, uint32_t *required);
    /* @summary Request cancellation without undo. @param owner Executing owner. @param invocation Original origin. @param task Owned task. @return OK if cancelled, BUSY if committed, or explicit denial. */
    SdResult (SD_CALL *cancel)(SdHandle owner, SdHandle invocation, SdHandle task);
    /* @summary Release only owned retained storage, no result disclosure. @param owner Issuing owner. @param task Owned task. @return BUSY while retained by work; STALE after release. */
    SdResult (SD_CALL *release)(SdHandle owner, SdHandle task);
    /* @summary Release owned invocation without revealing result. @param owner Issuing owner. @param invocation Owned reference. @return BUSY until all retaining tasks/subscriptions are released; OK invalidates every later use as STALE. */
    SdResult (SD_CALL *release_invocation)(SdHandle owner, SdHandle invocation);
} SdSemanticApi;
/* @summary Root version2, leaf tables version1; null facets are unavailable, never permission fallback.
 * @remarks Extended host semantic table (`SdSemanticHostApiV2`, version 3) appends optional trailing fields
 * (`eval_ui_js` at v1, `log_message` at v2, `list_mod_tools` + `invoke_mod_tool(name, args, grants, out)` at v3)
 * gated by `header.size` and `header.version` checks; `SdHostApiV2` size and `SD_CONTRACT_SHA256_HEX` remain unchanged.
 */
typedef struct SdHostApiV2 { SdHeader header; SdHandle owner; const SdLifecycleApi *lifecycle; const SdDispatchApi *dispatch; const SdCredentialApi *credentials; const SdSemanticApi *semantic; } SdHostApiV2;
/* @summary Exact module contract identity; header version2, ABI/base versions checked, reserved must be zero. */
typedef struct SdModuleDescriptorV2 { SdHeader header; uint32_t host_version; uint32_t base_contract_version; uint32_t reserved0; uint32_t reserved1; uint8_t contract_sha256[32]; } SdModuleDescriptorV2;
/* @summary Query after native trust/load, not a pre-load sandbox. @param host_version Requested root ABI. @param descriptor Caller-sized output. @return VERSION on mismatch with no admitted work. */
typedef SdResult (SD_CALL *SdModuleQueryV2)(uint32_t host_version, SdModuleDescriptorV2 *descriptor);
/* @summary Stage only, outside loader/manager locks. @param host Borrowed through safe drain. @return Explicit status, no C++ exception escapes. */
typedef SdResult (SD_CALL *SdModulePrepareV2)(const SdHostApiV2 *host);
/* @summary Postcommit readiness admission. @param owner Fresh host generation. @return Explicit status. */
typedef SdResult (SD_CALL *SdModuleAdmitV2)(SdHandle owner);
/* @summary Nonblocking close admission; no forced teardown. @param owner Owned generation. @return Explicit status. */
typedef SdResult (SD_CALL *SdModuleQuiesceV2)(SdHandle owner);
/* @summary BUSY retains code/resources, never permission to force-unload. @param owner Owned generation. @return Drain observation only. */
typedef SdResult (SD_CALL *SdModuleDrainV2)(SdHandle owner);
#pragma pack(pop)
/* @summary Compile-only x86 size assertion. */
typedef char check_SdTaskInfo[(sizeof(SdTaskInfo)==52)?1:-1];
/* @summary Compile-only x86 size assertion. */
typedef char check_SdSnapshotInfo[(sizeof(SdSnapshotInfo)==64)?1:-1];
/* @summary Compile-only x86 size assertion. */
typedef char check_SdSemanticApi[(sizeof(SdSemanticApi)==36)?1:-1];
/* @summary Compile-only x86 size assertion. */
typedef char check_SdHostApiV2[(sizeof(SdHostApiV2)==32)?1:-1];
/* @summary Compile-only x86 size assertion. */
typedef char check_SdModuleDescriptorV2[(sizeof(SdModuleDescriptorV2)==56)?1:-1];
/* @summary Pack4 offset assertion for unaligned uint64 field. */
typedef char check_task_revision[(offsetof(SdTaskInfo,revision)==36)?1:-1];
/* @summary Root semantic facet offset. */
typedef char check_semantic_offset[(offsetof(SdHostApiV2,semantic)==28)?1:-1];
#endif
