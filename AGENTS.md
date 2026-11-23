# Repository rules

- This repository owns declaration files and their package contract only.
- Preserve existing work and notices. Add no game assets, credentials, compilers,
  runtime implementation or speculative APIs.
- Document authored code with Doxygen-style `@summary` and relevant annotations;
  implement complete behavior without TODO placeholders.
- Keep investigation evidence under ignored `.analysis/`; maintained guides belong
  in documentation files. Prefix shell commands with `rtk` in this workspace.
- Validate hashes and installation after changes; declaration changes require
  runtime compatibility evidence and consumer pin updates.
- L2/L3 changes require independent audit and explicit human acceptance. This file
  grants no permissions, remote publication authority or approval exceptions.
