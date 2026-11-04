# Android Qt and Logos cross-build design

**Status:** Design approved in chat; written spec awaiting review

**Issue:** [#12 — Nixpkgs Android Qt cross-build support for Logos runtime](https://github.com/YodHeVauHe/logos-mobile/issues/12)

**Date:** 2026-11-04

## Goal

Provide a pinned, reproducible Nix cross-build path for the Android ARM64 Qt and Logos runtime dependency closure selected by the repository's pinned Logos sources. The build evidence must distinguish host tools from Android-target outputs and must identify the exact source pins, toolchain, patches, outputs, and any remaining upstream work.

## Constraints and current evidence

- The root `logos-liblogos` source is pinned at `af19e519c9b579a1dab86e8dc3960925fc28b526`; its flake declares desktop Linux/macOS and Windows package targets, but no Android target.
- The pinned `logos-nix` helper similarly enumerates desktop Linux/macOS and Windows targets. Several pinned runtime dependency flakes also hard-code that matrix, so changing Nixpkgs alone cannot expose an Android Logos package.
- The pinned Nixpkgs revision is `e9f00bd893984bc8ce46c895c3bf7cac95331127`. Its Android package manifest includes NDK r27c (`27.2.12479018`), while the cross target used in the earlier experiment selected r27b (`27.0.12077973`).
- Qt 6.9 supports Android `arm64-v8a`, API 28–35, and NDK r26b/r27c. The design pins the lower supported API (28) and NDK r27c to match Qt's supported toolchain and avoid relying on the Nixpkgs default r27b selection. See [Qt 6.9 supported platforms](https://doc.qt.io/archives/qt-6.9/supported-platforms.html).
- Nixpkgs Qt Base enables SCTP and adds `lksctp-tools` under generic non-Darwin/Linux conditions that also affect the Android cross target. No direct SCTP API references were found in the pinned `logos-liblogos`, default subprocess container, Qt module loader, or `logos-protocol` sources.
- Nixpkgs PCRE2 10.44 enables JIT automatically on this target; the Android attempt fails because Bionic does not declare `secure_getenv`.

## Design

### Cross-build entry point

Add a dedicated Android Nix build entry point under `nix/android/`. It will keep the existing host package path separate and expose the target as `aarch64-linux-android`, built on `x86_64-linux` using Nixpkgs' `aarch64-android` cross configuration. The configuration will pin Android API 28, NDK r27c (`27.2.12479018`), and the Qt 6.9.2 dependency version resolved by the existing Nixpkgs pin.

The entry point will pin all Logos source inputs to the revisions in `docs/research/logos-repository-map.md` and `docs/research/dependencies-and-build.md`. A maintained in-repository patch set will extend only the target enumeration and cross-package construction needed by the pinned Logos flakes in this runtime closure. The patch set will be applied to source snapshots at the exact locked commits before those patched sources are exposed as local flake inputs. Each patch will name its upstream repository, revision, and affected build expression. The Android target will use the Android cross package set; host-side build tools will remain host packages.

### Android-only Nixpkgs overrides

- Override PCRE2 10.44 for Android with JIT disabled. Keep the existing configuration for non-Android targets.
- Override Qt Base for Android with `QT_FEATURE_sctp=OFF` and remove `lksctp-tools` from the Android target inputs. Preserve SCTP behavior on platforms where it is supported.
- Keep the Nixpkgs and Logos source revisions locked. Do not replace a real dependency with a mock or an unpinned package.

If implementation evidence shows that a pinned Logos component requires SCTP, stop and revise this decision with that evidence instead of disabling the feature.

### Build and artifact evidence

Build in these stages so each boundary has independent evidence:

1. Evaluate the pinned Android target and verify the build host, target triple, API, NDK, and Qt version.
2. Cross-build PCRE2, Qt Base, and Qt RemoteObjects for Android ARM64.
3. Cross-build the pinned Logos SDK, protocol, container, loader, package-manager, runtime, and the pinned `logos-capability-module` using the patched target package definitions. The capability module is the selected real module for this proof.
4. Produce a deterministic staged runtime tree and a manifest of each artifact's source pin, Nix derivation, Android ELF machine, SONAME, dynamic dependencies, and destination path.

Keep full configure/build logs and exit statuses. Verify target artifacts with Android-aware ELF inspection and reject host Linux outputs from the Android staging tree. A successful build is cross-compilation evidence only; APK packaging, emulator execution, and device behavior remain outside issue #12.

### Blocker handling

If a new dependency or source-level target limitation prevents the full closure from building, preserve the first reproducible failure and its exact source and toolchain context. Do not describe Nix evaluation or a partial Qt build as a Logos runtime build. Create or update a separately scoped follow-up only after the blocker and required upstream change are established; record links and review status in the issue and build report.

## Scope

In scope: pinned Nix cross-build configuration, Android-only PCRE2/Qt overrides, the local target-support patch set required by the pinned Logos closure, Qt and Logos Android ARM64 artifacts, deterministic staging, and reproducible build evidence.

Out of scope: APK/application packaging, emulator or physical-device execution, Rust supervisor work, broad UI, Logos protocol implementation, and any claim of uninterrupted Android background operation.

## Acceptance checks

- Qt 6.9.2, Qt RemoteObjects, and the complete selected pinned Logos runtime closure, including `logos-capability-module`, build for Android `arm64-v8a` with API 28 and NDK r27c.
- The staged output contains Android ARM64 artifacts with verified ELF machine, SONAMEs, dynamic dependencies, and deterministic paths.
- The build can be reproduced from the committed pins and local patch set, with full commands, logs, and exit statuses recorded.
- Any required upstream changes are linked with their review/merge state; no mock or desktop artifact is presented as Android output.
- Issue #7 is updated with the verified unblock result and can resume its artifact-staging checklist.
