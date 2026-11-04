# ADR-0001: Logos Mobile integration boundary

- **Status:** Proposed; requires review in issue [#2](https://github.com/YodHeVauHe/logos-mobile/issues/2)
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

**Confidence:** medium that this is the right sequencing decision; low on Android feasibility because the decisive build/runtime experiment is unavailable. This ADR remains proposed until issue #2 review.

## Smallest next milestone proposal

After issue #2 is reviewed, create or revise the next issue to cover only an Android ARM64 build and packaging spike for the pinned runtime closure. Acceptance evidence should include the exact SDK/NDK/Qt/CMake/API versions, successful `arm64-v8a` outputs for the runtime and one real module (initial candidate: `logos-capability-module` at `1a1b8b5a1167931afbf41ebaafbe76034e6357a0`), APK native artifact inventory, and a runtime launch/load observation on an emulator. If the build fails, preserve the first failing dependency and diagnostics and revise the architecture proposal. Keep this issue blocked until #2 is reviewed; do not start a Rust supervisor or broad UI work before it.

## Consequences and unresolved questions

- Phase 0 can recommend only a gated Android experiment, not a compatible production architecture.
- Android API level, ABI build configuration, NDK/Qt versions, module packaging, linker paths, and process strategy remain unresolved.
- A successful cross-build alone will not prove runtime execution; emulator and physical-device checks remain separate evidence levels.
- Android background rules require explicit user-visible operation and lifecycle testing; continuous 24/7 availability is not a supported claim.
- The next implementation issue must exist and be open before any spike work starts. Its PR must link that issue.
