/**
 * @summary TypeScript declarations for the mandatory Simoder-Base Luau abstraction layer (`base`),
 * usable across Luau, TypeScript, and JavaScript mods in Simoder Beta (SDK 0.2.0).
 * Not an executable module or standalone permission issuer.
 */

type BaseStatus =
    | "Ok"
    | "Denied"
    | "Stale"
    | "NotReady"
    | "Busy"
    | "Conflict"
    | "Cancelled"
    | "Indeterminate"
    | "BufferSmall"
    | "Invalid"
    | "ResourceLimit"
    | "MailboxOverflow"
    | "UnsupportedCapability"
    | "Revoked"
    | "Expired";

/** @summary Native immutable VM-owned copied value; obtain only via `base.values.validate` or a task result. */
declare class BaseCopiedValue {}

/** @summary Frozen task outcome; `committed` is independent of terminal state. */
interface BaseOutcome {
    readonly state: "queued" | "running" | "succeeded" | "failed" | "cancelled" | "indeterminate";
    readonly committed: boolean;
    readonly result?: BaseCopiedValue;
    readonly error: BaseStatus;
}

/** @summary Frozen original-task facade exported by `Simoder-Base`. */
interface BaseTask {
    /** @summary Observe original task status without re-executing work. */
    poll(): BaseOutcome | undefined;
    /** @summary Request cancellation without rollback; committed tasks return `"Busy"`. */
    cancel(): BaseStatus;
    /** @summary Register a completion callback on a later owner scheduler turn. */
    onCompleted(callback: (outcome: BaseOutcome) => void): boolean | undefined;
    /** @summary Register a nonsuccess or access-denial callback on a later owner scheduler turn. */
    onFailed(callback: (outcome?: BaseOutcome, denial?: BaseStatus) => void): boolean | undefined;
    /** @summary Suspend a host-registered `base.run` coroutine until terminal outcome or denial. */
    await(): BaseOutcome | undefined;
}

/** @summary Frozen context identity encoded as exact decimal strings to preserve 64-bit precision. */
interface BaseContext {
    readonly slot: string;
    readonly generation: string;
    readonly epoch: string;
}

/** @summary Frozen lifecycle observation; never acts as a reusable permission lease. */
interface BaseSnapshot {
    readonly baseEpoch: string;
    readonly baseGeneration: string;
    readonly generation: string;
    readonly runtimeEpoch: string;
    readonly revision: string;
    readonly sequence: string;
    readonly monotonicNs: string;
    readonly ready: boolean;
    readonly singlePlayerKnown: boolean;
    readonly contexts: {
        readonly runtime: BaseContext;
        readonly region: BaseContext;
        readonly city: BaseContext;
        readonly entity: BaseContext;
    };
}

/** @summary Frozen metadata for an explicitly reviewed Base semantic operation; discovery grants no access. */
interface BaseDescriptor {
    readonly id: string;
    readonly kind: "task" | "snapshot";
    readonly inputSchema: string;
    readonly lane: string;
    readonly context: string;
    readonly capability: string;
    readonly postcondition: string;
    readonly maxResultBytes: number;
}

/** @summary Frozen subscription handle; call `unsubscribe()` to close delivery. */
interface BaseSubscription {
    unsubscribe(): BaseStatus;
}

/** @summary Frozen mod-local permission view; never a reusable authorization token. */
interface BasePermissionView {
    readonly available: boolean;
    readonly selected: boolean;
    readonly granted: boolean;
    readonly effective: boolean;
    readonly restricted: boolean;
}

/** @summary Frozen `base` facade installed by `Simoder-Base` via the native host bridge. */
interface BaseFacade {
    readonly permissions: {
        /** @summary Inspect selected versus effective rights for a closed capability name without minting authority. */
        query(name: string): BasePermissionView | undefined;
    };
    readonly values: {
        /** @summary Decode a canonical tagged envelope into an immutable VM-owned copied value. */
        validate(wire: string, schema: string): BaseCopiedValue | undefined;
        /** @summary Serialize an original VM-owned copied value back into canonical wire text. */
        wire(value: BaseCopiedValue): string | undefined;
    };
    /** @summary Queue a reviewed Base semantic task without implying immediate completion. */
    request(method: string, value: BaseCopiedValue): BaseTask | undefined;
    /** @summary Register a coroutine for scheduling on a later owner-lane turn. */
    run(callback: () => void): boolean | undefined;
    /** @summary Read the current copied lifecycle snapshot. */
    snapshot(): BaseSnapshot | undefined;
    /** @summary Subscribe atomically to initial snapshot state followed by ordered changes. */
    subscribe(callback: (snapshot?: BaseSnapshot, denial?: BaseStatus) => void): BaseSubscription | undefined;
    /** @summary Discover declared metadata descriptors without granting authority. */
    descriptors(): readonly BaseDescriptor[] | undefined;
}

/** @summary Global facade exposed by the mandatory `Simoder-Base` Luau mod to Luau, TypeScript, and JavaScript mods. */
declare var base: BaseFacade | undefined;
