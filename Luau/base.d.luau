--[[ @summary Editor declarations for SDK 0.2.0 profile base-luau-alpha-v1; not an executable module or permission issuer. ]]
type BaseStatus = "Ok" | "Denied" | "Stale" | "NotReady" | "Busy" | "Conflict" | "Cancelled" | "Indeterminate" | "BufferSmall" | "Invalid" | "ResourceLimit" | "MailboxOverflow"
--[[ @summary Native immutable VM-owned copied value; obtain only through base.values.validate or a task result. ]]
declare class BaseCopiedValue
end
--[[ @summary Frozen task outcome; committed is independent of terminal state. ]]
type BaseOutcome = {
    state: "queued" | "running" | "succeeded" | "failed" | "cancelled" | "indeterminate",
    committed: boolean,
    result: BaseCopiedValue?,
    error: BaseStatus,
}
--[[ @summary Frozen original-task facade; use dot calls. ]]
type BaseTask = {
    --[[ @summary Observe original task access. @return Outcome or nil and denial. ]]
    poll: () -> (BaseOutcome?, BaseStatus?),
    --[[ @summary Request cancellation without rollback. @return Status code; committed tasks return Busy. ]]
    cancel: () -> BaseStatus,
    --[[ @summary Register successful outcome on a later owner turn. @param callback Consumer. @return True or nil and denial; not a registration handle. ]]
    onCompleted: (callback: (BaseOutcome) -> ()) -> (boolean?, BaseStatus?),
    --[[ @summary Register nonsuccess or access denial on a later owner turn. @param callback Consumer. @return True or nil and denial. ]]
    onFailed: (callback: (BaseOutcome?, BaseStatus?) -> ()) -> (boolean?, BaseStatus?),
    --[[ @summary Yield only from a host-registered base.run coroutine. @return Outcome or nil and denial. ]]
    await: () -> (BaseOutcome?, BaseStatus?),
}
--[[ @summary Frozen context identity, represented by exact decimal strings rather than doubles. ]]
type BaseContext = {slot: string, generation: string, epoch: string}
--[[ @summary Frozen lifecycle observation, not a permission lease. ]]
type BaseSnapshot = {
    baseEpoch: string, baseGeneration: string, generation: string,
    runtimeEpoch: string, revision: string, sequence: string, monotonicNs: string,
    ready: boolean, singlePlayerKnown: boolean,
    contexts: {runtime: BaseContext, region: BaseContext, city: BaseContext, entity: BaseContext},
}
--[[ @summary Frozen metadata for an explicitly reviewed native task; discovery grants no access. ]]
type BaseDescriptor = {
    id: string, kind: "task", inputSchema: string, lane: string, context: string,
    capability: string, postcondition: string, maxResultBytes: number,
}
--[[ @summary Frozen subscription view; unsubscribe uses a status code. ]]
type BaseSubscription = {unsubscribe: () -> BaseStatus}
--[[ @summary Frozen mod-local permission view; never a reusable authorization token. ]]
type BasePermissionView = {
    available: boolean,
    selected: boolean,
    granted: boolean,
    effective: boolean,
    restricted: boolean,
}
--[[ @summary Actual frozen Base presentation installed only by a semantically bound native host. ]]
type BaseFacade = {
    permissions: {
        --[[ @summary Inspect this mod's selected versus effective rights without minting authority. @param name Closed capability name. @return Frozen view or nil and denial. ]]
        query: (name: string) -> (BasePermissionView?, BaseStatus?),
    },
    values: {
        --[[ @summary Decode a canonical tagged envelope into opaque VM ownership. @param wire Tagged wire. @param schema Reviewed schema. @return Value or nil and denial. ]]
        validate: (wire: string, schema: string) -> (BaseCopiedValue?, BaseStatus?),
        --[[ @summary Encode an original VM-owned value. @param value Opaque native value. @return Tagged wire or nil and denial; forged/foreign values throw. ]]
        wire: (value: BaseCopiedValue) -> (string?, BaseStatus?),
    },
    --[[ @summary Request an available reviewed task without implying completion. @param method Exact method ID. @param value Original input. @return Task or nil and denial. ]]
    request: (method: string, value: BaseCopiedValue) -> (BaseTask?, BaseStatus?),
    --[[ @summary Register a coroutine for a later owner turn. @param callback Work. @return True or nil and denial, not a cancellable handle. ]]
    run: (callback: () -> ()) -> (boolean?, BaseStatus?),
    --[[ @summary Read lifecycle observation. @return Snapshot or nil and denial during preparation. ]]
    snapshot: () -> (BaseSnapshot?, BaseStatus?),
    --[[ @summary Subscribe atomically to initial snapshot and ordered changes. @param callback Consumer; access denial supplies nil/code. @return Handle or nil and denial. ]]
    subscribe: (callback: (BaseSnapshot?, BaseStatus?) -> ()) -> (BaseSubscription?, BaseStatus?),
    --[[ @summary Discover metadata without granting authority. @return Frozen sequence, which may be empty, or nil and denial. ]]
    descriptors: () -> ({BaseDescriptor}?, BaseStatus?),
}
--[[ @summary Optional outside the supported bound host; no declaration can create its native authority. ]]
declare base: BaseFacade?
