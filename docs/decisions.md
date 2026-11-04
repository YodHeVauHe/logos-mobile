# Architecture Decision Index

This index links decisions to their evidence and current status. Proposed decisions remain open until reviewed; superseded decisions retain a link to their replacement.

| Decision | Status | Phase | Evidence / record |
|---|---|---|---|
| Logos runtime integration boundary for Android | Proposed; awaits issue #2 review | 0 | [ADR-0001](adr/0001-logos-mobile-integration-boundary.md), informed by the [source map](research/logos-repository-map.md), [build inventory](research/dependencies-and-build.md), [Android assessment](research/android-compatibility.md), and [experiment record](research/android-build-experiment.md). |

## Open questions

- Can the actual Logos runtime and a real module build and execute for Android ARM64?
- What process, linker, filesystem, packaging, and module-loading model does the selected runtime require on Android?
- Which official Logos runtime and module revisions provide the smallest viable Android proof of concept?
- What Android lifecycle and background-execution behavior can be verified on the target devices?

## Phase 0 recommendation

Do not start an Android app shell or Rust supervisor yet. The next issue, after Phase 0 review, should isolate an Android ARM64 build/packaging spike for the pinned Logos runtime closure and a real module. The proposed candidate and evidence gates are in [ADR-0001](adr/0001-logos-mobile-integration-boundary.md). No Android build was possible in the current environment, and upstream does not declare an Android target in the inspected snapshot.

Answers must cite official documentation, pinned source/configuration, or a recorded experiment. Unknowns remain marked as unknown until evidence resolves them.
