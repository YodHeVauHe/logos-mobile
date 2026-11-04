# Architecture Decision Index

This index links decisions to their evidence and current status. Proposed decisions remain open until reviewed; superseded decisions retain a link to their replacement.

| Decision | Status | Phase | Evidence / record |
|---|---|---|---|
| Logos runtime integration boundary for Android | Accepted for sequencing; Android feasibility remains unverified | 0 | [ADR-0001](adr/0001-logos-mobile-integration-boundary.md), approved after Phase 0 PRs #1 and #6 merged; evidence is in the [source map](research/logos-repository-map.md), [build inventory](research/dependencies-and-build.md), [Android assessment](research/android-compatibility.md), and [experiment record](research/android-build-experiment.md). |

## Open questions

- Can the actual Logos runtime and a real module build and execute for Android ARM64?
- What process, linker, filesystem, packaging, and module-loading model does the selected runtime require on Android?
- Which official Logos runtime and module revisions provide the smallest viable Android proof of concept?
- What Android lifecycle and background-execution behavior can be verified on the target devices?
- Can the pinned Nix/CMake dependency closure be cross-built with Qt 6.9.2's Android toolchain support? Tracked by issue #7.

## Phase 0 decision

Do not start an Android app shell or Rust supervisor yet. Issue [#3](https://github.com/YodHeVauHe/logos-mobile/issues/3) now isolates an Android ARM64 build/packaging spike for the pinned Logos runtime closure and a real module. The candidate and evidence gates are in [ADR-0001](adr/0001-logos-mobile-integration-boundary.md). No Android build was possible during Phase 0, and upstream does not declare an Android target in the inspected snapshot.

Answers must cite official documentation, pinned source/configuration, or a recorded experiment. Unknowns remain marked as unknown until evidence resolves them.
