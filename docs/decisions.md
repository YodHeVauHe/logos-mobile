# Architecture Decision Index

This index links decisions to their evidence and current status. Proposed decisions remain open until reviewed; superseded decisions retain a link to their replacement.

| Decision | Status | Phase | Evidence / record |
|---|---|---|---|
| Logos runtime integration boundary for Android | Pending Phase 0 evidence | 0 | To be recorded as ADR-0001 after the repository, dependency, build, and Android compatibility audit in issue #2. |

## Open questions

- Can the actual Logos runtime and a real module build and execute for Android ARM64?
- What process, linker, filesystem, packaging, and module-loading model does the selected runtime require on Android?
- Which official Logos runtime and module revisions provide the smallest viable Android proof of concept?
- What Android lifecycle and background-execution behavior can be verified on the target devices?

Answers must cite official documentation, pinned source/configuration, or a recorded experiment. Unknowns remain marked as unknown until evidence resolves them.
