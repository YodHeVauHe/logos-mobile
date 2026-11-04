# Android build experiment record

Date: 2026-11-04. Outcome: **not run; prerequisites and upstream target are absent**. This is an environment/source inspection result, not an Android build failure and not a simulated pass.

## Inputs inspected

- `logos-liblogos` revision `af19e519c9b579a1dab86e8dc3960925fc28b526`.
- Default subprocess container revision `df1fbecf2f35910e7efe14d7a8208959f2f20ae1`.
- Default Qt module loader revision `3eee855362c238052eb66833ec14782c9ab2d07e`.
- Tool availability in the research environment: Nix `2.35.3`; Cargo present; no `ANDROID_HOME`/`ANDROID_NDK_HOME` configured; `adb` not present.
- Source result: liblogos `flake.nix` names `x86_64-linux`, `aarch64-linux`, `x86_64-darwin`, `aarch64-darwin`, and a Windows package target. It does not declare `aarch64-linux-android` or any Android cross toolchain/API configuration.

## Attempt and result

No Android-target command was run because there is no declared Android derivation/toolchain in the inspected upstream build and this environment has no Android SDK/NDK or device bridge. Running a desktop Nix build would not answer Android compatibility. We did not run one and do not report a host build as Android evidence.

Classification: **source inspection**. Android cross-build, emulator, and real-device evidence: **none**.

## Reproducible next attempt

Once upstream supports an Android target or an issue approves a concrete cross-build patch:

1. Provision and record exact Android SDK command-line tools, NDK, CMake/Ninja, Qt Android package, Rust toolchain (if the mobile layer is in scope), and emulator image versions.
2. Pin the Logos source revisions from [the source map](logos-repository-map.md).
3. Build the runtime closure for `arm64-v8a` at an explicitly chosen Android API level. Record the NDK target triple (`aarch64-linux-android<API>`), CMake toolchain file, linker flags, Qt modules, and every dependency output.
4. Preserve full configure/build logs, exit status, APK contents, native library inventory, and `readelf`/Android package inspection output.
5. If a dependency fails, record the first failing target and exact compiler/linker diagnostics. Do not replace that dependency with a mock and call the runtime integrated.
6. Install on an emulator and then a physical device. Record launch, plugin/module load, local observable behavior, process death/restart and cleanup separately.

This plan deliberately leaves API level and NDK version unselected: upstream has not supplied a supported combination, and guessing those values would not make the experiment reproducible.
