# Android build experiment record

Date: 2026-11-04. Outcome: **Android build blocked at the upstream flake target boundary**. An exact Nix evaluation confirmed there is no Android package system in the pinned liblogos flake. We did not attempt a cross-build or install SDK components after that check; no Android build failure or success is claimed.

## Inputs inspected

- `logos-liblogos` revision `af19e519c9b579a1dab86e8dc3960925fc28b526`.
- Default subprocess container revision `df1fbecf2f35910e7efe14d7a8208959f2f20ae1`.
- Default Qt module loader revision `3eee855362c238052eb66833ec14782c9ab2d07e`.
- Tool availability in the research environment: Nix `2.35.3`, CMake and Ninja present; no Java/JDK, Android SDK/NDK environment, or `adb` present.
- Source result: liblogos `flake.nix` names `x86_64-linux`, `aarch64-linux`, `x86_64-darwin`, `aarch64-darwin`, and a Windows package target. It does not declare `aarch64-linux-android` or any Android cross toolchain/API configuration.
- Locked host Qt result: Nixpkgs revision `e9f00bd893984bc8ce46c895c3bf7cac95331127` resolves `pkgs.qt6.qtbase.version` to `6.9.2`. Qt's official 6.9 matrix supports Android `arm64-v8a`, API 28–35, NDK r26b/r27c, JDK 17, and Gradle 8.10/AGP 8.6.0. This covers Qt's supported configuration only, not the Logos dependency closure.

## Attempt and result

The pinned flake's declared systems were queried with:

```sh
nix eval --json --expr 'builtins.attrNames ((builtins.getFlake "github:logos-co/logos-liblogos/af19e519c9b579a1dab86e8dc3960925fc28b526").packages)'
```

Exit: `0`. Output: `[` `"aarch64-darwin"`, `"aarch64-linux"`, `"x86_64-darwin"`, `"x86_64-linux"`, `"x86_64-windows"` `]`.

The Android target was queried with:

```sh
nix eval --json --expr 'builtins.attrNames ((builtins.getFlake "github:logos-co/logos-liblogos/af19e519c9b579a1dab86e8dc3960925fc28b526").packages."aarch64-linux-android")'
```

Exit: `1`. Nix reports `attribute 'aarch64-linux-android' missing` at the `packages = forAllTargets` output in `flake.nix:101`. This is the first reproducible blocker: the pinned upstream flake exposes no Android package target, so it cannot reach dependency compilation through its declared build path. Nixpkgs' Qt package is desktop Qt 6.9.2; a separate Android Qt build and Android builds for the other dependency inputs are not configured here.

No Android cross-build, APK packaging, emulator, or real-device run followed. Those require a supported Android dependency/build path; the environment also lacks SDK/NDK/JDK/adb. A desktop Nix build would not answer Android compatibility and is not reported as Android evidence.

Classification: **source inspection**. Android cross-build, emulator, and real-device evidence: **none**.

## Reproducible next attempt

After a separate issue defines a concrete Android cross-build path:

1. Provision and record exact Android SDK command-line tools, NDK, CMake/Ninja, Qt Android package, Rust toolchain (if the mobile layer is in scope), and emulator image versions.
2. Pin the Logos source revisions from [the source map](logos-repository-map.md).
3. Build the runtime closure for `arm64-v8a` at an explicitly chosen Android API level. Record the NDK target triple (`aarch64-linux-android<API>`), CMake toolchain file, linker flags, Qt modules, and every dependency output.
4. Preserve full configure/build logs, exit status, APK contents, native library inventory, and `readelf`/Android package inspection output.
5. If a dependency fails, record the first failing target and exact compiler/linker diagnostics. Do not replace that dependency with a mock and call the runtime integrated.
6. Install on an emulator and then a physical device. Record launch, plugin/module load, local observable behavior, process death/restart and cleanup separately.

For Qt 6.9.2, Qt's official 6.9 support matrix supplies candidate bounds (Android API 28–35 and NDK r26b/r27c); Android's NDK documentation maps `arm64-v8a` to `aarch64-linux-android<minSdkVersion>` and uses `ANDROID_ABI=arm64-v8a` with `ANDROID_PLATFORM` in CMake. The cross-build enablement follow-up is [issue #7](https://github.com/YodHeVauHe/logos-mobile/issues/7). It must establish which exact Qt Android package, NDK, and API level can be used consistently across the whole Logos dependency closure before a build attempt. These Qt values do not by themselves select or prove the Logos target.
