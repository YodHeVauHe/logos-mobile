# Logos Mobile Roadmap

This roadmap summarizes the phase order and current status. Each phase issue is the source of truth for its detailed checklist, discussion, and linked PRs. Work starts only from an open issue, and a phase remains blocked until its dependencies are complete and reviewed.

## Phases

| Phase | Issue | Status | Dependencies | Exit criteria |
|---|---|---|---|---|
| 0 — Repository foundation and Android feasibility | [#2](https://github.com/YodHeVauHe/logos-mobile/issues/2) | Complete; PRs #1 and #6 merged | None | Repository workflow, pinned Logos source/build inventory, Android compatibility assessment, experiment record, and proposed integration boundary are documented and reviewed. Android compatibility remains unverified. |
| 1 — Android ARM64 Logos runtime build and packaging spike | [#3](https://github.com/YodHeVauHe/logos-mobile/issues/3) | Blocker reproduced; awaiting PR review | #2 | The pinned liblogos flake exports only desktop/Windows package systems; querying `aarch64-linux-android` fails at `flake.nix:101`. See [build experiment](research/android-build-experiment.md) and follow-up issue [#7](https://github.com/YodHeVauHe/logos-mobile/issues/7). |
| 2 — Android execution foundation | [#4](https://github.com/YodHeVauHe/logos-mobile/issues/4) | Blocked | #3, #7 | The Android runtime package launches on an emulator and survives the defined lifecycle checks. The original Rust supervisor scope must be revisited against the Android build result. |
| 3 — Real Logos runtime integration | [#5](https://github.com/YodHeVauHe/logos-mobile/issues/5) | Blocked | #4, #7 | At least one actual Logos module runs locally on Android. Blockchain synchronization is claimed only after the actual protocol integration and its evidence are verified. |

## Enabling follow-up

- [Issue #7](https://github.com/YodHeVauHe/logos-mobile/issues/7) tracks adding or identifying a reproducible Android ARM64 build path for the pinned Logos dependency closure. It is blocked until issue #3 is reviewed and merged.

## Progress rules

- Issues #2–#5 contain the scope checklist and acceptance criteria for their phases; enabling issue #7 tracks the Android cross-build prerequisite.
- Every pull request links to the issue it addresses and reports verification evidence.
- Update a phase status only when its issue checklist and review evidence support that status.
- Keep later issues blocked until all listed dependencies are complete; do not start later-phase work in parallel with an unresolved prerequisite.
- Distinguish source inspection, mocked behavior, host builds, Android-target builds, emulator runs, and real-device runs. Compilation alone does not establish runtime or protocol integration.

## Scope boundary

The infrastructure contribution or reward economy remains outside these initial phases. It should be considered only after the underlying services and independent work-verification mechanisms are understood.
