# ADR-0001: Logos Mobile integration boundary

- **Status:** Accepted for sequencing after Phase 0 PRs #1 and #6 merged; Android feasibility remains unverified
- **Date:** 2026-11-04
- **Decision owner:** Logos Mobile maintainers

## Context

The project aims to run existing Logos functionality locally on Android while keeping protocol behavior in upstream Logos components. The source audit is recorded in the [repository map](../research/logos-repository-map.md), [dependency/build inventory](../research/dependencies-and-build.md), [Android compatibility report](../research/android-compatibility.md), [Android build experiment](../research/android-build-experiment.md), and [real-module experiment plan](../research/android-module-poc-plan.md).

The inspected liblogos snapshot (`af19e519c9b579a1dab86e8dc3960925fc28b526`) provides a C API. Its locked default container (`df1fbecf2f35910e7efe14d7a8208959f2f20ae1`) creates child processes; its locked default module loader (`3eee855362c238052eb66833ec14782c9ab2d07e`) launches a Qt host process that loads a plugin. The source matrix contains desktop Linux/macOS and Windows targets, not Android. Official module documentation lists desktop prerequisites but no Android target. No Android build was run; the current environment has no configured Android SDK/NDK or `adb`.

## Options supported by inspected evidence

1. **Use the current liblogos default runtime unchanged.** This retains the documented upstream architecture and C API, but Android support for its Qt, process, plugin, and native dependency closure is unverified. There is no Android packaging/build recipe in the inspected source.
2. **Replace the default `ModuleContainer` implementation with an Android-capable implementation.** The upstream container abstraction is an architectural seam. This could address process-launch constraints, but would not alone solve runtime target support, Qt module loading, dependency packaging, or lifecycle behavior. No Android implementation was found.
3. **Replace the default module loader or module format.** The loader interface permits a different implementation, but the inspected source provides only the Qt plugin loader as the default. An alternate loader and compatible module format have not been selected or verified for Android.

Embedding `liblogos_core` and assuming its default child-process/Qt-plugin behavior works on Android is not an evidence-backed option today.

## Proposed decision

Do not select or implement a Rust-to-Logos API boundary yet. Keep Rust as a candidate for mobile lifecycle, resource policy, and app-to-runtime coordination, while treating the upstream Logos runtime and modules as the only source of protocol functionality. First resolve whether the existing runtime dependency closure can be built and packaged for Android, or identify the smallest upstream-compatible alternate boundary through a separately issued experiment.

**Confidence:** medium that this is the right sequencing decision; low on Android feasibility because the decisive build/runtime experiment is unavailable. The owner approved the sequencing by authorizing the Phase 0 merges and continuation to issue #3.

## Smallest next milestone proposal

Issue [#3](https://github.com/YodHeVauHe/logos-mobile/issues/3) reproduced the first blocker: the pinned liblogos Nix flake does not export an Android package target. Qt 6.9.2 itself supports Android `arm64-v8a`, but the Logos dependency closure has not been shown to cross-build. Follow-up [issue #7](https://github.com/YodHeVauHe/logos-mobile/issues/7) covers a concrete Android cross-build path. After #7, issue #4 can package/run the runtime on an emulator and issue #5 can validate the real module integration. Do not start a Rust supervisor or broad UI work before the native runtime path is proven.

## Consequences and unresolved questions

- Phase 0 can recommend only a gated Android experiment, not a compatible production architecture.
- Android API level, ABI build configuration, NDK/Qt versions, module packaging, linker paths, and process strategy remain unresolved.
- A successful cross-build alone will not prove runtime execution; emulator and physical-device checks remain separate evidence levels.
- Android background rules require explicit user-visible operation and lifecycle testing; continuous 24/7 availability is not a supported claim.
- Any further implementation work must exist in an open issue before it starts; its PR must link that issue.
