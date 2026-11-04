# Android ARM64 Logos Runtime Build Spike Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Determine whether the pinned Logos runtime and one real module can be built, packaged, and started on Android ARM64, or produce a reproducible first blocker.

**Architecture:** Use the upstream `logos-liblogos` lockfile and default runtime components as the source of truth. Select an Android toolchain only after checking the exact Qt/runtime requirements. Attempt the narrowest supported cross-build before creating APK packaging or app integration; if the upstream build has no Android derivation, record that concrete boundary and stop rather than inventing a passing substitute.

**Tech Stack:** Logos C++/Qt runtime, Nix/CMake, Android NDK/SDK and Qt for Android if a supported dependency path is available; Android emulator for runtime evidence.

**Spec:** [Phase 0 integration-boundary ADR](../../adr/0001-logos-mobile-integration-boundary.md), [Android compatibility report](../../research/android-compatibility.md), and [issue #3](https://github.com/YodHeVauHe/logos-mobile/issues/3).

## Global Constraints

- Use issue #3 for all work and link every PR to it.
- Use the pinned upstream revisions in the [source map](../../research/logos-repository-map.md); do not substitute mocks or unpinned dependency heads.
- Do not infer Android ARM64 support from Linux AArch64 support.
- Separate source inspection, cross-build, APK packaging, emulator execution, and module-operation evidence.
- Do not implement a Rust supervisor, broad UI, or Logos protocol functionality in this spike.
- If an upstream dependency blocks the supported path, preserve the first failure and scope remediation in a separate issue.

## Review Focus

- A Nix package set for Linux AArch64 is not an Android cross toolchain.
- Qt Android binaries must match the Qt version and NDK used by the consuming runtime.
- `liblogos_core`, child host binaries, plugins, and transitive libraries all need compatible Android ABI and packaging/linker behavior.
- The capability module's successful token flow requires both a real caller and reachable target module; loading the module alone does not prove the operation.
- Emulator startup, module load, operation, and restart are separate evidence levels.

---

## Task 1: Record the exact source and toolchain baseline

**Files:**
- Modify: `docs/research/android-build-experiment.md`
- Modify: `docs/research/dependencies-and-build.md`

**Interfaces:**
- Consumes: pinned revisions from the Phase 0 source map and the active issue #3 checklist.
- Produces: exact host tools and the root-selected Qt/NDK/Android target requirements, or an explicit unknown where no upstream value exists.

- [x] Verify the local `logos-liblogos` checkout is at `af19e519c9b579a1dab86e8dc3960925fc28b526` and record `nix --version`, host OS/architecture, SDK/NDK availability, Qt tools, CMake, Ninja, Java, and emulator availability.
- [x] Trace the locked Qt and Nix inputs to the resolved Qt major/minor and build derivation; record the commands and exact output. The locked host `qtbase` resolves to 6.9.2 from Nixpkgs `e9f00bd893984bc8ce46c895c3bf7cac95331127`.
- [x] Read the official Qt Android support matrix matching that Qt release and record its ABI, API, NDK, JDK, and Gradle requirements: Qt 6.9 lists API 28–35, `arm64-v8a`, NDK r26b/r27c, JDK 17, Gradle 8.10, and AGP 8.6.0.
- [x] Read the official Android NDK/CMake target documentation and record the target triple and packaging constraints relevant to the chosen ABI: `arm64-v8a` maps to `aarch64-linux-android<minSdkVersion>` and CMake's NDK toolchain requires `ANDROID_ABI=arm64-v8a` plus `ANDROID_PLATFORM`. Since Qt 6.9's minimum supported Android API is 28, `aarch64-linux-android28` is the candidate compiler target; the exact build remains gated on the full Logos dependency closure.
- [x] Do not choose a Qt/NDK combination that differs from the runtime's supported dependency version without documenting the required rebuild boundary.

## Task 2: Reproduce the upstream target support boundary

**Files:**
- Modify: `docs/research/android-build-experiment.md`

**Interfaces:**
- Consumes: Task 1 baseline.
- Produces: a command-level result showing whether the pinned upstream flake exposes an Android package or cross target.

- [x] Query the pinned flake's package system list and preserve the output. Nix evaluation returned only desktop Linux/macOS and Windows systems.
- [x] Query the Android system package attr for `aarch64-linux-android` using `nix eval`; record the exact command, exit code, and error/output.
- [x] The target is absent. Identify the earliest source/build boundary that lacks Android configuration and stop before building a desktop target as a substitute. The exact Nix error is recorded in the experiment report.
- [x] A supported target does not exist in the pinned flake. Created enabling follow-up issue [#7](https://github.com/YodHeVauHe/logos-mobile/issues/7) for a reproducible Android dependency cross-build path.

## Task 3: Attempt the narrowest Android cross-build

**Files:**
- Modify: `docs/research/android-build-experiment.md`
- Modify: `docs/research/dependencies-and-build.md`

**Interfaces:**
- Consumes: Task 1's pinned toolchain and Task 2's verified target path.
- Produces: Android-target build outputs or first-failure diagnostics for the pinned runtime closure.

- [ ] Provision only the officially required SDK/NDK/Qt/CMake/Java components if a supported path is established; record versions and licenses accepted. Blocked on #7.
- [ ] Build `logos-liblogos` and the default container/loader dependency closure for `arm64-v8a` with the exact upstream revisions. Blocked on #7.
- [ ] Preserve complete configure/build logs, command lines, environment variables relevant to the toolchain, first failing dependency, and exit status.
- [x] Do not patch upstream or introduce alternate dependencies without a separate issue authorizing that scope.

## Task 4: Package and run the minimum real-module proof

**Files:**
- Modify: `docs/research/android-build-experiment.md`
- Modify: `docs/research/android-module-poc-plan.md`
- Create: minimal Android harness/artifact only if required by the issue and supported by Tasks 1–3.

**Interfaces:**
- Consumes: successful Android build outputs and the pinned `logos-capability-module` source.
- Produces: APK inventory and named emulator observations separated by stage.

- [ ] Package the runtime, Qt/plugin dependency closure, module host, and candidate module into a minimal APK. Blocked on #7.
- [ ] Record APK ABI contents, executable/module files, linker dependencies, and app-private file locations.
- [ ] Install and run on a named emulator; record app launch, runtime startup, and module load separately. Blocked on #7 and missing local Android SDK/emulator.
- [ ] For a capability operation, run a real caller and target module, verify authenticated target behavior, and preserve logs/results. If this is not possible, limit the claim to module load.
- [ ] Record stop and restart observations; do not claim continuous background execution.

## Task 5: Update the decision and close the issue with evidence

**Files:**
- Modify: `docs/adr/0001-logos-mobile-integration-boundary.md`
- Modify: `docs/decisions.md`
- Modify: `docs/roadmap.md`
- Modify: issue #3 checklist/status through the linked PR review process.

**Interfaces:**
- Consumes: recorded source, toolchain, build, packaging, and emulator evidence.
- Produces: reproducible go/no-go result and separately scoped follow-up issues for blockers.

- [x] If blocked, name the first failing dependency/configuration, link diagnostics, and create a separate issue for any upstream support work before starting it. The flake target boundary is documented and #7 is open/blocked on #3.
- [x] Update the roadmap and ADR; keep all later issues blocked until issue #3 is reviewed and merged.
- [ ] After issue #7 provides a build path, return to the remaining package/emulator work if the blocker is resolved; otherwise update this issue's acceptance outcome with review evidence.
- [ ] Run documentation/link/whitespace checks and commit with the repository's required issue-number, `Changes:` and `Verification:` structure.

## Handoff boundary

This plan does not authorize work on issues #4 or #5. It ends after the issue #3 build/packaging spike is reviewed and merged.
