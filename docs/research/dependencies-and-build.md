# Logos dependency and build inventory

Snapshot date: 2026-11-04. Primary source is [`logos-liblogos` revision `af19e519`](https://github.com/logos-co/logos-liblogos/tree/af19e519c9b579a1dab86e8dc3960925fc28b526), including its `flake.nix`, `flake.lock`, CMake files, Nix expressions, and README.

## Build entry points and declared targets

| Layer | Build definition / inputs | Evidence and limitation |
|---|---|---|
| Runtime | Nix flake; CMake; C++17; Qt Core, Network, RemoteObjects; OpenSSL; spdlog; nlohmann_json; Boost.Process and Boost.Filesystem | `CMakeLists.txt`, `src/CMakeLists.txt`, `nix/default.nix`, and README. Standard outputs include runtime binaries, shared libraries, headers, bundled modules, and tests. |
| Runtime system matrix | `x86_64-linux`, `aarch64-linux`, `x86_64-darwin`, `aarch64-darwin`; plus `x86_64-windows` using a Linux build system | `flake.nix` defines these explicitly. Android triples such as `aarch64-linux-android` and Android NDK/API settings are absent from the inspected package matrix. `platforms.unix` metadata is not Android build evidence. |
| Default module container | `logos-container-subprocess` Nix/CMake; Boost.Process v2, Boost.Asio, spdlog, Qt local sockets, container interface | Pinned in liblogos lockfile at `df1fbecf2f35910e7efe14d7a8208959f2f20ae1`. Source uses POSIX `spawn.h`/`posix_spawn`, pipes, and process management. |
| Default module loader | `logos-module-loader-qt` Nix/CMake; Qt plugin host, Qt/C++ SDK stack, Boost.DLL | Pinned at `3eee855362c238052eb66833ec14782c9ab2d07e`. Host is an executable that loads Qt plugins. No Android packaging setup was located in the pinned source. |
| Logos CLI | Separate Nix flake frontend, `logosctl` and portable bundle outputs | Inspected checkout says macOS/Linux support. It is not the runtime library and the inspected checkout was not a pinned liblogos input. |
| Mobile layer in this repository | No Rust workspace, Cargo manifest, Gradle project, Android manifest, NDK config, or module code exists yet | This is intentional: Phase 0 records upstream integration facts before choosing mobile dependencies. |

## Version and provenance snapshot

These are immutable Git revisions selected directly by the root node of the liblogos lockfile, not semver releases. This avoids confusing root-selected inputs with duplicate nested lock nodes that can point at other revisions.

| Input | Revision |
|---|---|
| `default-container` (`logos-container-subprocess`) | `df1fbecf2f35910e7efe14d7a8208959f2f20ae1` |
| `default-module-loader` (`logos-module-loader-qt`) | `3eee855362c238052eb66833ec14782c9ab2d07e` |
| `logos-container` | `641d21108781a353eed9f0567f15776fee06af40` |
| `logos-module-loader` | `3628b97a32cb2edb30e7d26c0c5113a487e3e993` |
| `logos-capability-module` | `1a1b8b5a1167931afbf41ebaafbe76034e6357a0` |
| `logos-cpp-sdk` | `3bf5365c5412f89bbbf30e3f46237cd68cff0599` |
| `logos-module` | `ceee1e35649cc938c213515c672fa40df0a32469` |
| `logos-modules-state-module` | `ed0f4ba534520542764775efc1d0ef705a02e616` |
| `logos-nix` | `f55bf91b8a723ee5c27c39c773b034255538d280` |
| `nixpkgs` (follows `logos-nix/nixpkgs`) | `e9f00bd893984bc8ce46c895c3bf7cac95331127` |
| `logos-package-manager` | `1c8ea4a84a310773968c7b338320f84d182278c1` |
| `logos-plugin-qt` | `f9e969e7b62ffed543660024c62198b61894de9c` |
| `logos-protocol` | `d8389858fe4fc9a91113bc8dd66721bf99699173` |
| `logos-qt-sdk` | `b0c57af85a9600bfe29be536696863f002eb6bdd` |
| `process-stats` | `5bff72cd99ad533885a3c64ae9c8fe65fbc9caf5` |

The root `nixpkgs` input follows `logos-nix/nixpkgs`; the inherited resolved revision is recorded in the table.

Evaluating `legacyPackages.x86_64-linux.qt6.qtbase.version` at this revision returns **Qt 6.9.2**, matching the `pkgs.qt6.qtbase` and `qtremoteobjects` packages used by liblogos. This is the host Nix Qt package version; it does not mean the locked Nix derivation contains Qt for Android.

The source checkout at `/tmp/logos-liblogos` was obtained by shallow `git clone` after the `opensrc` fetch failed in this environment (GitHub API rate limit, then a generic request error). Its `HEAD` was verified as the revision above. The root `flake.lock` was parsed to obtain its direct input pins. Source/build details were inspected in liblogos and the pinned default container and loader; the other direct inputs are revision-mapped here but were not individually audited for Android support. No fetched source was added to this repository.

## Android integration implications

- There is no upstream Android target in the liblogos Nix system list and no Android NDK/Gradle instructions in the inspected liblogos, default-container, and default-loader build files. The remaining transitive/direct repositories have not each been independently audited for Android support, so their compatibility remains unknown.
- Qt 6.9's official Android matrix lists `arm64-v8a`, Android API 28–35, NDK r26b or r27c, JDK 17, and Gradle 8.10/AGP 8.6.0. The locked host Qt version is 6.9.2, but a compatible Qt Android package and an Android cross-build for Logos' other native dependencies have not been established. See [Qt 6.9 supported platforms](https://doc.qt.io/archives/qt-6.9/supported-platforms.html) and [Android NDK installation guidance](https://developer.android.com/studio/projects/install-ndk).
- The default execution model requires launching `logos_host_qt`/module processes and resolving native Qt plugins and dependent shared libraries. Android executable loading, executable file placement/permissions, dynamic linker lookup, Qt availability, and process supervision all need explicit experiments; desktop ELF support cannot establish these properties.
- `logos-container-subprocess` uses POSIX process APIs. This is a portability concern to test against Android's Bionic libc and policy, not proof of failure by itself.
- The liblogos build uses Qt 6 and additional native dependencies; a Rust Android target alone would not cross-build or package this C++ dependency graph.
- No API level, minimum device version, Android ABI, Rust target, Gradle plugin version, or NDK revision is selected. They remain unknown until the upstream path or an approved alternate boundary is proven.

## Reproduction references

- [`logos-liblogos` README/build guide](https://github.com/logos-co/logos-liblogos/blob/af19e519c9b579a1dab86e8dc3960925fc28b526/README.md)
- [`logos-liblogos` Nix system/output matrix](https://github.com/logos-co/logos-liblogos/blob/af19e519c9b579a1dab86e8dc3960925fc28b526/flake.nix)
- [`logos-liblogos` CMake dependencies and test platform guards](https://github.com/logos-co/logos-liblogos/blob/af19e519c9b579a1dab86e8dc3960925fc28b526/CMakeLists.txt)
- [`logos-container-subprocess` README](https://github.com/logos-co/logos-container-subprocess/blob/df1fbecf2f35910e7efe14d7a8208959f2f20ae1/README.md)
- [`logos-module-loader-qt` README](https://github.com/logos-co/logos-module-loader-qt/blob/3eee855362c238052eb66833ec14782c9ab2d07e/README.md)
