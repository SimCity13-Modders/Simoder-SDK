/** @noSelfInFile @summary Describes the host API shared by native Luau and transpiled TypeScript. */
/** @noSelf @summary Calls host functions without a Lua method receiver. */
interface SimoderApi {
    readonly apiVersion: 1;
    readonly modId: string;
    /** @summary Appends a bounded message attributed to the current mod. */
    log(message: string): void;
    /** @summary Returns elapsed monotonic runtime seconds; it is not simulation time. */
    time(): number;
    /** @summary Tests an actually implemented host capability. */
    hasCapability(name: string): boolean;
    /** @summary Queues owner-scoped PROP operations and returns historical observations without game pointers. */
    readonly props: SimoderProps;
}
/** @summary Describes historical resource evidence; it is not a synchronous read of live game memory. */
interface SimoderFloatObservation {
    readonly value: number;
    readonly vanilla: number;
    readonly historical: true;
    readonly generation: number;
    readonly observedAtUnixMs: number;
    readonly instances: number;
    readonly status: string;
}
/** @noSelf @summary Uses plain host calls without Lua method receivers; writes commit after successful callbacks. */
interface SimoderProps {
    /** @summary Returns historical evidence, undefined before observation, or an error for incompatible/ambiguous data. */
    getFloat(resource: string, property: number | string): SimoderFloatObservation | undefined;
    /** @summary Queues an absolute finite float32 override owned by this mod. */
    setFloat(resource: string, property: number | string, value: number): void;
    /** @summary Removes this mod's scripted override, retaining other mods and declarative operations. */
    clearFloat(resource: string, property: number | string): void;
}
/** @summary Exposes the host-owned API table; members remain read-only except callable methods. */
declare var simoder: SimoderApi;
