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

These are immutable Git revisions selected by the liblogos lockfile, not semver releases:

| Input | Revision |
|---|---|
| `default-container` (`logos-container-subprocess`) | `df1fbecf2f35910e7efe14d7a8208959f2f20ae1` |
| `default-module-loader` (`logos-module-loader-qt`) | `3eee855362c238052eb66833ec14782c9ab2d07e` |
| `logos-container` | `641d21108781a353eed9f0567f15776fee06af40` |
| `logos-module-loader` | `3628b97a32cb2edb30e7d26c0c5113a487e3e993` |
| `logos-cpp-sdk` | `3f34c0b216c8fbdbf2093056b9f0fc53d89f05a1` |
| `logos-qt-sdk` | `03e489b16f9401526e52c0fdbc1e72cbf9fb1144` |
| `logos-protocol` | `8bbc027c99505c3c2f043265f8c1ae8ff899bdff` |
| `logos-plugin-qt` | `3a471be14af66d099827ee712ec8c40ead701340` |
| `logos-package-manager` | `d3af2972f51d9c542537d80d60ad8d20282ddcd1` |
| `logos-module` | `9812dc8c69cfbfed3eea06510a5c1ff33c1fcf7f` |

The source checkout at `/tmp/logos-liblogos` was obtained by shallow `git clone` after the `opensrc` fetch failed in this environment (GitHub API rate limit, then a generic request error). Its `HEAD` was verified as the revision above. No fetched source was added to this repository.

## Android integration implications

- There is no upstream Android target in the liblogos Nix system list and no Android NDK/Gradle instructions in the inspected runtime/default-container/default-loader build files.
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
