# Android compatibility assessment

Assessment date: 2026-11-04. Status: **unverified; no Android build or runtime result**. See [source map](logos-repository-map.md), [build inventory](dependencies-and-build.md), and [build experiment record](android-build-experiment.md).

## Evidence matrix

| Area | Evidence | Assessment |
|---|---|---|
| Android ARM64 target | The pinned liblogos flake enumerates desktop Darwin/Linux systems and Windows, with no Android system, ABI, NDK, or API-level configuration. Qt 6.9 itself officially supports `arm64-v8a` for API 28–35 using NDK r26b/r27c, but this does not establish support for the Logos build closure. The default container and loader build files inspected also lack Android configuration; other transitive repositories have not each been independently audited. | Android ARM64 support for Logos is not established. Linux `aarch64-linux` does not imply Android `aarch64-linux-android` support. |
| Process model | Default container starts one OS process per module using Boost.Process v2; source includes POSIX `posix_spawn`, pipes, wait/termination logic, and Unix-domain-socket auth-token transfer. | Must prove process launch, child lifecycle and IPC on Android/Bionic. Android sandbox and app process ownership may constrain use. |
| Module loading | Default loader launches a host executable that loads a Qt plugin. Runtime artifact closure includes Qt and transitive native libraries. | Must prove Qt module build, plugin ABI compatibility, linker search paths, and APK/AAB packaging. No Android recipe was found. |
| Filesystem | Logos CLI currently uses a home-directory session (`~/.logosctl`); liblogos exposes a persistence base path and module directories. | Android integration must map state and modules to app-private storage and test path/permission behavior. Do not assume access to a conventional home directory. Android [app-specific storage guidance](https://developer.android.com/training/data-storage/app-specific) describes app-owned persistent/cache locations. |
| Background/lifecycle | Official Android guidance documents background service limits and restrictions on starting foreground services from the background; current foreground service rules also include type-specific limits. | Design for visible/user-started execution, explicit restart and recovery, and measured bounded sessions. Do not promise uninterrupted 24/7 execution. See [background execution limits](https://developer.android.com/about/versions/oreo/background.html), [foreground service start restrictions](https://developer.android.com/develop/background-work/services/fgs/restrictions-bg-start), and [foreground service timeouts](https://developer.android.com/develop/background-work/services/fgs/timeout). |
| Toolchain | No Android SDK, NDK, Gradle project, or `adb` is configured in the current environment. | A valid Android cross-build/run experiment cannot be performed here. Record tool installation and exact versions as a prerequisite rather than substituting a mock. |

## Current judgment

The documented desktop runtime cannot yet be treated as an Android-ready dependency. Proceeding directly to a Rust supervisor or app shell would risk selecting an integration boundary before proving the C++ runtime, Qt plugin host, and module process model. Compatibility of uninspected transitive dependencies is unknown. Phase 0 should therefore recommend a gated follow-up experiment: first add or identify a supported Android build path for the runtime dependency closure; then package and run one real module in a foreground, user-visible session.

This is a **no-go for claiming Android runtime integration now**, and a **go for a narrowly scoped compatibility experiment after the Android toolchain and build path are available**. It is not a claim that Android support is impossible.

## Unknowns that must remain explicit

- Minimum Android API level and exact ABI/toolchain versions.
- Whether upstream Qt 6 and the Logos dependency closure build for that ABI.
- Whether Boost.Process's selected implementation and `posix_spawn` behavior work under the supported Android NDK/Bionic combination.
- Whether module executables and Qt plugins can be packaged, found, loaded, and executed within the app sandbox.
- Which transport works locally and meets Android socket policy/lifecycle behavior.
- Which Android foreground-service type, notification and user controls fit a node session; runtime quotas vary by Android version and service type.
- Recovery behavior after process death, force-stop, reboot, battery restriction, and vendor-specific task management.
- Whether protocol modules can operate within realistic mobile CPU, memory, bandwidth, storage, and thermal budgets.
