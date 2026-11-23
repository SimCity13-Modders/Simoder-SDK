/* @summary U0 candidate ONLY: not a deployed SDK or implemented export. */
#ifndef SIMODER_U0_CANDIDATE_H
#define SIMODER_U0_CANDIDATE_H
#include <stdint.h>
#include <stddef.h>
#if !defined(_WIN32) || !defined(_M_IX86)
#error Candidate must be compiled for Windows x86
#endif
#define SD_CALL __cdecl
#define SD_ABI_VERSION 1u
#pragma pack(push, 4)
/* @summary Result codes; no C++ exceptions cross this boundary. */
typedef enum SdResult {
    SD_OK = 0,
    SD_INVALID = 1,
    SD_VERSION = 2,
    SD_STALE = 3,
    SD_DENIED = 4,
    SD_NOT_READY = 5,
    SD_BUSY = 6,
    SD_CONFLICT = 7,
    SD_CANCELLED = 8,
    SD_INDETERMINATE = 9,
    SD_RESTART_REQUIRED = 10,
    SD_BUFFER_SMALL = 11,
} SdResult;
/* @summary Canonical observed lifecycle values; separate from module-generation states. */
typedef enum SdLifecycleState {
    SD_STATE_COLD = 0,
    SD_STATE_STARTING = 1,
    SD_STATE_READY = 2,
    SD_STATE_STOPPING = 3,
    SD_STATE_STOPPED = 4,
    SD_STATE_FAULTED = 5,
} SdLifecycleState;
/* @summary Independent readiness facts, never inferred merely from bootstrap. */
typedef enum SdReadiness {
    SD_READY_RUNTIME = 1,
    SD_READY_MENU = 2,
    SD_READY_CITY = 4,
} SdReadiness;
/* @summary Host-allocated opaque identity; never a pointer or authority by itself. */
typedef struct SdHandle { uint32_t slot; uint32_t generation; } SdHandle;
/* @summary Structure negotiation prefix; exact v1 size required for this candidate. */
typedef struct SdHeader { uint32_t size; uint32_t version; } SdHeader;
/* @summary Host-issued immutable context; every field revalidated at commit. */
typedef struct SdContext {
    SdHeader header;
    SdHandle owner;
    SdHandle principal;
    SdHandle session;
    uint64_t epoch;
    uint64_t revision;
    uint64_t grant_generation;
    uint32_t opcode;
    uint32_t reserved;
} SdContext;
/* @summary Caller buffer; callee copies before returning and never retains data. */
typedef struct SdBytes { const uint8_t *data; uint32_t size; } SdBytes;
/* @summary Current lifecycle snapshot; flags are readiness facts, not game class IDs. */
typedef struct SdState {
    SdHeader header;
    uint64_t epoch;
    uint64_t sequence;
    uint64_t revision;
    uint32_t lifecycle;
    uint32_t readiness_flags;
} SdState;
/* @summary Callback borrows data until return; no blocking/mutating reentry; self-unsubscribe returns BUSY. */
typedef void (SD_CALL *SdStateCallback)(void *user, const SdState *state);
/* @summary Small lifecycle interface; unsubscribe closes admission before draining callbacks. */
typedef struct SdLifecycleApi {
    SdHeader header;
    SdResult (SD_CALL *snapshot)(SdHandle owner, SdState *out_state);
    SdResult (SD_CALL *subscribe_current)(SdHandle owner, SdStateCallback callback, void *user, SdHandle *out_subscription);
    SdResult (SD_CALL *unsubscribe)(SdHandle subscription);
} SdLifecycleApi;
/* @summary Bounded operation interface; opcode allowlist is core-authored, never a raw address. */
typedef struct SdDispatchApi {
    SdHeader header;
    SdResult (SD_CALL *submit)(const SdContext *context, uint32_t opcode, SdBytes request, SdHandle *out_operation);
    SdResult (SD_CALL *poll)(const SdContext *context, SdHandle operation, uint8_t *output, uint32_t capacity, uint32_t *required);
    SdResult (SD_CALL *cancel)(const SdContext *context, SdHandle operation);
    SdResult (SD_CALL *release)(SdHandle owner, SdHandle operation);
} SdDispatchApi;
/* @summary Credential verification only: no grant mint, principal assertion or protocol parsing. */
typedef struct SdCredentialApi {
    SdHeader header;
    /* @summary Verify locally provisioned opaque credential and bound proof.
     * @param owner Host-issued native owner identity.
     * @param credential Copied opaque secret; never a principal or grant assertion.
     * @param channel_proof Copied proof verified under the qualified host profile.
     * @param out_session Owner-bound authenticated session; zeroed on failure.
     * @return OK or explicit denial/error without secret-bearing diagnostics. */
    SdResult (SD_CALL *authenticate)(SdHandle owner, SdBytes credential, SdBytes channel_proof, SdHandle *out_session);
    /* @summary Resolve current host-issued context; rechecks never mint permissions.
     * @param owner Native owner bound to session.
     * @param session Authenticated host session, not an unverified client identifier.
     * @param opcode Existing qualified operation requested by this context.
     * @param out_context Caller-sized versioned output; host supplies identities.
     * @return OK only while all applicable identity/lease/operation gates hold. */
    SdResult (SD_CALL *obtain_context)(SdHandle owner, SdHandle session, uint32_t opcode, SdContext *out_context);
    /* @summary Invalidate owned authentication session and uncommitted contexts.
     * @param owner Issuing native owner; foreign-owner cleanup is forbidden.
     * @param session Host session to release without returning any secret/result.
     * @return OK or explicit stale/denied status; never replays committed work. */
    SdResult (SD_CALL *release_session)(SdHandle owner, SdHandle session);
} SdCredentialApi;
/* @summary Module entry gets only host-issued owner and borrowed interfaces; no registry copy. */
typedef struct SdHostApi {
    SdHeader header;
    SdHandle owner;
    const SdLifecycleApi *lifecycle;
    const SdDispatchApi *dispatch;
    const SdCredentialApi *credentials;
} SdHostApi;
/* @summary Candidate module lifecycle exports, called outside loader/manager locks.
 * @param host Borrowed through successful drain; prepare cannot listen or publish tools.
 * @return Explicit status; host catches no cross-boundary C++ exceptions. */
typedef SdResult (SD_CALL *SdPrepare)(const SdHostApi *host);
/* @summary Called only after native trust, base readiness and activation commit. */
typedef SdResult (SD_CALL *SdAdmit)(SdHandle owner);
/* @summary Close listener and reject new work immediately; idempotent and nonblocking. */
typedef SdResult (SD_CALL *SdQuiesce)(SdHandle owner);
/* @summary Confirm no running callbacks; BUSY retains module/dependencies, never force-unloads. */
typedef SdResult (SD_CALL *SdDrain)(SdHandle owner);
#pragma pack(pop)
/* @summary Compile-time x86 wire layout checks; structs are in-process, not network serialization. */
typedef char sd_handle_size[(sizeof(SdHandle)==8)?1:-1];
typedef char sd_context_size[(sizeof(SdContext)==64)?1:-1];
typedef char sd_state_size[(sizeof(SdState)==40)?1:-1];
typedef char sd_bytes_size[(sizeof(SdBytes)==8)?1:-1];
typedef char sd_host_size[(sizeof(SdHostApi)==28)?1:-1];
typedef char sd_lifecycle_size[(sizeof(SdLifecycleApi)==20)?1:-1];
typedef char sd_dispatch_size[(sizeof(SdDispatchApi)==24)?1:-1];
typedef char sd_credential_size[(sizeof(SdCredentialApi)==20)?1:-1];
typedef char sd_epoch_offset[(offsetof(SdContext,epoch)==32)?1:-1];
typedef char sd_state_flags_offset[(offsetof(SdState,readiness_flags)==36)?1:-1];
#endif
