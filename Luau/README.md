# Simoder Luau editor types

Configure Luau LSP definition files with the installed `simoder.d.luau` for the
legacy host and, when targeting a semantically bound Base host, `base.d.luau`.
Disable Roblox-specific types for this standalone host. Use `.luau` sources.
These are editor definitions: do not require them or put them in mod scripts.

```json
{
  "luau-lsp.platform.type": "standard",
  "luau-lsp.types.definitionFiles": [
    "C:/path/to/SDK/Luau/simoder.d.luau",
    "C:/path/to/SDK/Luau/base.d.luau"
  ]
}
```

This is opt-in guidance, not an installer that changes IDE settings. Check the
installed LSP's settings for other editors. Runtime capabilities remain native;
`simoder.hasCapability` checks explicit legacy capability, and PROP reads are
copied historical observations that may be nil. Lifecycle exports remain
onEnable, onTick(dt) and onDisable.

[Base API](BASE_API.md) documents SDK0.2.0/profile base-luau-alpha-v1, optional
availability, dot calls, nil/code and exception paths. Frozen/opaque native
ownership is enforced at runtime, not by structurally typed editor declarations.
Parsing definitions does not prove analyzer inference, editor UI or gameplay.
