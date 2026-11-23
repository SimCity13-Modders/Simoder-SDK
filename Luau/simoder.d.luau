-- @summary Luau LSP definitions for the API installed by Simoder's sandbox host.
type FloatObservation = {
    value: number,
    vanilla: number,
    observedAtUnixMs: number,
    generation: number,
    instances: number,
    status: string,
    historical: boolean,
}

declare simoder: {
    apiVersion: number,
    modId: string,
    -- @summary Queue a bounded message under the current mod's identity.
    log: (message: string) -> (),
    -- @summary Read monotonic host time; this is not wall-clock time.
    time: () -> number,
    -- @summary Check an explicit capability rather than assuming game APIs exist.
    hasCapability: (capability: string) -> boolean,
    props: {
        -- @summary Read copied historical evidence, or nil when no resource was observed.
        getFloat: (resource: string, property: string | number) -> FloatObservation?,
        -- @summary Queue an owner-scoped absolute override for a successful callback.
        setFloat: (resource: string, property: string | number, value: number) -> (),
        -- @summary Remove only this script owner's override.
        clearFloat: (resource: string, property: string | number) -> (),
    },
}
