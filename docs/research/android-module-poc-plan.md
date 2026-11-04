# Minimum real Logos module experiment

Status: **blocked on an Android-capable runtime build path**. This plan follows the official [module build guide](https://docs.logos.co/core/build-modules/build-and-run-a-logos-core-module) and pinned runtime inventory in [the source map](logos-repository-map.md). It does not claim a module has run on Android.

## Candidate and prerequisite gate

Use the upstream [`logos-capability-module`](https://github.com/logos-co/logos-capability-module/tree/1a1b8b5a1167931afbf41ebaafbe76034e6357a0) at revision `1a1b8b5a1167931afbf41ebaafbe76034e6357a0`, pinned by `logos-liblogos` `flake.lock` in the selected audit snapshot. Its [module specification](https://github.com/logos-co/logos-capability-module/blob/1a1b8b5a1167931afbf41ebaafbe76034e6357a0/docs/docs.md) documents `requestModule`, which issues a token and delivers it to a target. A successful operation therefore requires both a caller and a reachable target module; loading this module alone cannot prove the token flow. The later experiment issue must select/pin a suitable target and define a real caller or harness before claiming this operation works. Before implementation, verify the module build output is installable by the same runtime build. If this capability-module path is unsuitable, select the smallest upstream module with a self-contained observable operation and record the reason in that issue.

Do not begin the experiment until the runtime, default or approved alternate container, loader, and selected module all build for `arm64-v8a` with a recorded Android API level and NDK. That gate is currently unmet.

## Experiment sequence after the gate

1. Build the pinned Logos runtime and candidate module with the documented Android toolchain; save source SHAs, SDK/NDK/Qt/CMake versions, configure commands, and complete build logs.
2. Package the runtime executable/shared libraries, Qt/plugin dependencies, and module into an Android application using app-private native/module storage. Record the APK native library listing and module file permissions.
3. Start execution from an explicit user action in a visible activity or a platform-compliant foreground service. Capture process IDs, runtime logs, Android logcat, exit codes, and storage paths.
4. Load the module through the Logos runtime API. Start the pinned target module and a real caller or harness, request a capability, then use the returned token to make an authenticated call to the target. Verify the target's observable result plus an independent observation (runtime/module logs or token delivery metadata). If the caller/target pair is not defined and runnable, report only module-load evidence and do not claim capability-operation success. A mock response does not count.
5. Exercise stop/cleanup, activity backgrounding, OS process death, and user-initiated restart. Record each as a separate lifecycle observation; do not infer indefinite background availability.
6. Repeat on an emulator and at least one physical Android device before claiming device support. Record model, Android version, API, ABI, battery policy, and result.

## Evidence labels and acceptance

- **Source inspection:** confirms upstream interfaces and build configuration only.
- **Host build:** proves desktop compilation only.
- **Android cross-build:** proves ABI compilation/linking only.
- **Emulator run:** proves behavior on the named emulator image only.
- **Real-device run:** proves behavior on the named physical device/configuration only.
- **Mocked:** useful for app/controller tests, but proves no Logos runtime or protocol behavior.

The minimum integration acceptance is a non-mocked module loaded by the Android-packaged Logos runtime, an observable module operation, clean shutdown, and a recorded restart after one controlled process death. For the capability-module candidate, operation acceptance additionally requires a real caller-target token handoff and authenticated target call. Broader availability or blockchain synchronization claims require separate evidence and issues.
