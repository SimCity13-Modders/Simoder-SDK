"""@summary Generate candidate-only C declarations from one numeric/layout input; no product implementation."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent

INPUTS = ['schema.json', 'generate.py', 'joint-contract.md',
          'native-boundary.md', 'value-contract.md', 'conformance.md',
          'abi_v1.h',
          'baseline-schema.json']

def identity():
    """@summary Hash the complete normative input bundle. @return Hashes and canonical map digest."""
    hashes = {name: hashlib.sha256((HERE / name).read_bytes()).hexdigest() for name in INPUTS}
    canonical = json.dumps(hashes, sort_keys=True, separators=(',', ':')).encode('utf-8')
    return {'algorithm': 'SHA256 of sorted compact UTF8 JSON path-to-SHA256 map',
            'input_hashes': hashes, 'digest': hashlib.sha256(canonical).hexdigest()}

def render():
    """@summary Render deterministic candidate declarations. @return UTF-8 C header text."""
    raw = (HERE / 'schema.json').read_bytes()
    s = json.loads(raw)
    lines = ['/* @summary U2a candidate only, not a deployed SDK or implemented export. */',
             '#ifndef SIMODER_U2A_SEMANTIC_CONTRACT_H', '#define SIMODER_U2A_SEMANTIC_CONTRACT_H',
             '#include "' + s['baseline'] + '"',
             '/* @summary Complete normative bundle identity, excluding generated outputs. */',
             '#define SD_CONTRACT_SHA256_HEX "' + identity()['digest'] + '"',
             '#define SD_U2A_HOST_VERSION ' + str(s['host_version']) + 'u',
             '#define SD_SEMANTIC_VERSION ' + str(s['semantic_version']) + 'u',
             '#define SD_BASE_CONTRACT_VERSION ' + str(s['base_contract_version']) + 'u']
    for name, value in s['result_extensions'].items():
        lines.append('/* @summary Additional explicit result; existing0..11 unchanged. */\n#define SD_' + name + ' ((SdResult)' + str(value) + ')')
    for name, value in s['task_phases'].items():
        lines.append('/* @summary Task phase, independent of API result and commit flag. */\n#define SD_TASK_' + name + ' ' + str(value) + 'u')
    for name, value in s['task_flags'].items():
        lines.append('/* @summary Task commit reservation flag. */\n#define SD_TASK_' + name + ' ' + str(value) + 'u')
    lines.append('''#pragma pack(push, 4)
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
/* @summary Root version2, leaf tables version1; null facets are unavailable, never permission fallback. */
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
#pragma pack(pop)''')
    for name, size in s['sizes'].items():
        lines.append('/* @summary Compile-only x86 size assertion. */\ntypedef char check_' + name + '[(sizeof(' + name + ')==' + str(size) + ')?1:-1];')
    lines += ['/* @summary Pack4 offset assertion for unaligned uint64 field. */',
              'typedef char check_task_revision[(offsetof(SdTaskInfo,revision)==36)?1:-1];',
              '/* @summary Root semantic facet offset. */',
              'typedef char check_semantic_offset[(offsetof(SdHostApiV2,semantic)==28)?1:-1];', '#endif', '']
    return '\n'.join(lines)

if __name__ == '__main__':
    (HERE / 'native.h').write_bytes(render().encode('utf-8'))
    (HERE / 'identity.json').write_bytes((json.dumps(identity(), indent=2) + '\n').encode('utf-8'))
