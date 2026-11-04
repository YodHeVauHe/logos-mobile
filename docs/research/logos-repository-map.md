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
| [logos-cpp-sdk](https://github.com/logos-co/logos-cpp-sdk/tree/3bf5365c5412f89bbbf30e3f46237cd68cff0599) | `3bf5365c5412f89bbbf30e3f46237cd68cff0599` | C++ module SDK input selected by the root lockfile for this liblogos snapshot. |
| [logos-qt-sdk](https://github.com/logos-co/logos-qt-sdk/tree/b0c57af85a9600bfe29be536696863f002eb6bdd) | `b0c57af85a9600bfe29be536696863f002eb6bdd` | Qt integration SDK selected by the root lockfile. |
| [logos-capability-module](https://github.com/logos-co/logos-capability-module/tree/1a1b8b5a1167931afbf41ebaafbe76034e6357a0) | `1a1b8b5a1167931afbf41ebaafbe76034e6357a0` | Runtime-bundled module input; candidate for the later real-module experiment, not yet tested on Android. |
| [logos-modules-state-module](https://github.com/logos-co/logos-modules-state-module/tree/ed0f4ba534520542764775efc1d0ef705a02e616) | `ed0f4ba534520542764775efc1d0ef705a02e616` | Runtime-bundled module input. |
| [logos-module](https://github.com/logos-co/logos-module/tree/ceee1e35649cc938c213515c672fa40df0a32469) | `ceee1e35649cc938c213515c672fa40df0a32469` | Logos module contract input. |
| [logos-protocol](https://github.com/logos-co/logos-protocol/tree/d8389858fe4fc9a91113bc8dd66721bf99699173) | `d8389858fe4fc9a91113bc8dd66721bf99699173` | Protocol library input selected by the root lockfile. |
| [logos-plugin-qt](https://github.com/logos-co/logos-plugin-qt/tree/f9e969e7b62ffed543660024c62198b61894de9c) | `f9e969e7b62ffed543660024c62198b61894de9c` | Qt host/plugin runtime input. |
| [logos-package-manager](https://github.com/logos-co/logos-package-manager/tree/1c8ea4a84a310773968c7b338320f84d182278c1) | `1c8ea4a84a310773968c7b338320f84d182278c1` | Package-manager library input to the runtime build. |
| [logos-nix](https://github.com/logos-co/logos-nix/tree/f55bf91b8a723ee5c27c39c773b034255538d280) | `f55bf91b8a723ee5c27c39c773b034255538d280` | Logos Nix build helpers; the root `nixpkgs` input follows `logos-nix/nixpkgs`. |
| [process-stats](https://github.com/logos-co/process-stats/tree/5bff72cd99ad533885a3c64ae9c8fe65fbc9caf5) | `5bff72cd99ad533885a3c64ae9c8fe65fbc9caf5` | Process statistics library input. |
| [logos-logoscore-cli](https://github.com/logos-co/logos-logoscore-cli/tree/66d098d3af1367cf831b69c13a5aa8345edeb682) | inspected checkout `66d098d3af1367cf831b69c13a5aa8345edeb682` | Headless `logosctl` frontend. Its README lists only macOS and Linux supported platforms and describes the session under `~/.logosctl`. This checkout was not pinned by the liblogos lockfile, so it is context, not a transitive version claim. |

The table records the root-selected revisions for the direct `logos-liblogos` flake inputs. The lockfile contains additional transitive nodes and nested references; those are not separate root-selected runtime inputs. A lock pin proves dependency selection only; it does not establish that the dependency builds or runs on Android. See [Dependencies and build](dependencies-and-build.md) for the complete direct-input list and the scope of source inspection.

## Findings and limits

The documented default runtime path is an executable host plus per-module child processes and Qt plugin loading. That is materially different from embedding one self-contained Rust library. An alternate `ModuleContainer` or module-loader implementation is an upstream architectural seam, but no Android implementation was found in the inspected source. Do not infer that the seam itself makes the current runtime Android-compatible.

The official docs and inspected source establish Linux/macOS/Windows or desktop Nix targets. They do not provide an Android ABI/API level, Android NDK configuration, Android packaging recipe, or Android CI result. Android support therefore remains unverified, not disproved.
