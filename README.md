# Logos Mobile

Logos Mobile is a Rust-first Android platform for running real Logos infrastructure locally on smartphones and managing it safely as device conditions change. It is not a remote node dashboard or a replacement for existing Logos protocols.

> **Status: research and foundation.** Android compatibility has not been established, and no Logos module is currently claimed to run on Android through this project.

## Project direction

The engineering rule is **integrate Logos; do not rebuild Logos**. Rust will own mobile infrastructure management. Existing Logos runtimes and modules remain the source of protocol functionality. Android-specific lifecycle and execution support will stay behind a narrow platform boundary.

Work happens in two modes:

1. **Research mode** verifies official Logos sources, dependencies, build targets, runtime behavior, and Android constraints. Findings label verified facts, assumptions, and open questions.
2. **Implementation mode** builds one independently verifiable component at a time from reviewed evidence. Pull requests state what is real, mocked, incomplete, or blocked and include verification evidence.

Compilation alone does not establish that a real Logos module runs on Android. Mocks, host builds, Android builds, emulator runs, and real-device results must be reported separately.

## Current phase

Phase 0 is tracked in [issue #2](https://github.com/YodHeVauHe/logos-mobile/issues/2). Later phases have their own issues and remain blocked until their stated dependencies are complete:

- [Phase 1: Rust mobile foundation](https://github.com/YodHeVauHe/logos-mobile/issues/3)
- [Phase 2: Android execution foundation](https://github.com/YodHeVauHe/logos-mobile/issues/4)
- [Phase 3: real Logos runtime integration](https://github.com/YodHeVauHe/logos-mobile/issues/5)

See the [roadmap](docs/roadmap.md) for phase status and exit criteria, and the [engineering specification](logos-mobile-engineering-specification.md) for the proposed architecture and initial audit assignment.

The platform-independent Rust workspace is rooted at `crates/logos-mobile-core`.
It provides timestamped capability measurements, deterministic resource budgets,
evidence-gated role assessment, and a pure node lifecycle state machine. These
APIs are policy primitives only; they do not collect real device measurements,
control Logos processes, or establish Android runtime support. See the [Rust
core architecture](docs/architecture/rust-core.md) for the current contracts.

## Contributing

All work starts from an open GitHub issue, and every pull request links to the issue it addresses. Work proceeds through issue checklists and reviewed pull requests; do not start a blocked phase. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md) before working.

## License

Dual-licensed under MIT or Apache-2.0, at your option. See [LICENSE](LICENSE) and [LICENSE-APACHE](LICENSE-APACHE).
