# Logos source map

Research snapshot: 2026-11-04. This is an upstream source inspection, not evidence of Android compatibility. Revisions below come from `logos-liblogos`'s `flake.lock` at [`af19e519`](https://github.com/logos-co/logos-liblogos/tree/af19e519c9b579a1dab86e8dc3960925fc28b526), except where marked as the current CLI checkout.

## Official documentation

- [Logos developer documentation](https://docs.logos.co/) is the official documentation entry point. Its [coding-agent guide](https://docs.logos.co/get-started/coding-with-agents) describes the Markdown and `llms.txt` indexes.
- [Build and run a Logos Core module](https://docs.logos.co/core/build-modules/build-and-run-a-logos-core-module) describes a Qt 6 based module build and documents Linux x86_64/aarch64, macOS arm64, and Windows under WSL2 prerequisites. It does not list Android.
- [Run a node](https://docs.logos.co/run-a-node) describes `logosctl` managing the blockchain, storage, and delivery module processes on a Linux host.

## Runtime path

| Repository | Pinned revision | Role and evidence |
|---|---|---|
| [logos-liblogos](https://github.com/logos-co/logos-liblogos/tree/af19e519c9b579a1dab86e8dc3960925fc28b526) | `af19e519c9b579a1dab86e8dc3960925fc28b526` | Core runtime library. Its README identifies `liblogos_core` (C API) and `logos_host`, and names Basecamp and logosctl as frontends. Entry points include `logos_core_init`, `logos_core_start`, module directory configuration, load/unload, and module queries in `src/logos_core/logos_core.h`. |
| [logos-container](https://github.com/logos-co/logos-container/tree/641d21108781a353eed9f0567f15776fee06af40) | `641d21108781a353eed9f0567f15776fee06af40` | Runtime abstraction for the module execution container; pinned as a liblogos flake input. |
| [logos-container-subprocess](https://github.com/logos-co/logos-container-subprocess/tree/df1fbecf2f35910e7efe14d7a8208959f2f20ae1) | `df1fbecf2f35910e7efe14d7a8208959f2f20ae1` | Default container in liblogos. README and `src/subprocess_container.cpp` describe one child OS process per module, supervised with Boost.Process and Unix-domain-socket token handoff. |
| [logos-module-loader](https://github.com/logos-co/logos-module-loader/tree/3628b97a32cb2edb30e7d26c0c5113a487e3e993) | `3628b97a32cb2edb30e7d26c0c5113a487e3e993` | Module-format loader interface pinned by liblogos. |
| [logos-module-loader-qt](https://github.com/logos-co/logos-module-loader-qt/tree/3eee855362c238052eb66833ec14782c9ab2d07e) | `3eee855362c238052eb66833ec14782c9ab2d07e` | Default loader. Its README says the child `logos_host_qt` loads a Qt plugin in its own process; the parent library resolves the host executable and prepares arguments. |
| [logos-cpp-sdk](https://github.com/logos-co/logos-cpp-sdk/tree/3f34c0b216c8fbdbf2093056b9f0fc53d89f05a1) | `3f34c0b216c8fbdbf2093056b9f0fc53d89f05a1` | C++ module SDK input used by the Qt host and module builds. |
| [logos-qt-sdk](https://github.com/logos-co/logos-qt-sdk/tree/03e489b16f9401526e52c0fdbc1e72cbf9fb1144) | `03e489b16f9401526e52c0fdbc1e72cbf9fb1144` | Qt integration SDK pinned by liblogos. |
| [logos-logoscore-cli](https://github.com/logos-co/logos-logoscore-cli/tree/66d098d3af1367cf831b69c13a5aa8345edeb682) | inspected checkout `66d098d3af1367cf831b69c13a5aa8345edeb682` | Headless `logosctl` frontend. Its README lists only macOS and Linux supported platforms and describes the session under `~/.logosctl`. This checkout was not pinned by the liblogos lockfile, so it is context, not a transitive version claim. |

The liblogos lockfile also pins protocol, Qt host/plugin, capability, state, package-manager, and process-stat dependencies. Exact revisions and build inputs are recorded in [Dependencies and build](dependencies-and-build.md). A lock pin proves dependency selection only; it does not establish that the dependency builds or runs on Android.

## Findings and limits

The documented default runtime path is an executable host plus per-module child processes and Qt plugin loading. That is materially different from embedding one self-contained Rust library. An alternate `ModuleContainer` or module-loader implementation is an upstream architectural seam, but no Android implementation was found in the inspected source. Do not infer that the seam itself makes the current runtime Android-compatible.

The official docs and inspected source establish Linux/macOS/Windows or desktop Nix targets. They do not provide an Android ABI/API level, Android NDK configuration, Android packaging recipe, or Android CI result. Android support therefore remains unverified, not disproved.
