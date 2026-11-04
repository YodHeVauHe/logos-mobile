# Logos Mobile Roadmap

This roadmap summarizes the phase order and current status. Each phase issue is the source of truth for its detailed checklist, discussion, and linked PRs. Work starts only from an open issue, and a phase remains blocked until its dependencies are complete and reviewed.

## Phases

| Phase | Issue | Status | Dependencies | Exit criteria |
|---|---|---|---|---|
| 0 — Repository foundation and Android feasibility | [#2](https://github.com/YodHeVauHe/logos-mobile/issues/2) | In progress | None | Repository guidance and progress records are in place; Logos runtime/module sources, dependencies, build requirements, and Android ARM64 constraints are documented; a reproducible build experiment is recorded where feasible; an evidence-backed integration recommendation and minimal real-module Android proof-of-concept plan are reviewed. |
| 1 — Rust mobile foundation | [#3](https://github.com/YodHeVauHe/logos-mobile/issues/3) | Blocked | #2 | The Rust engine builds and tests independently of Android UI; role assessments are deterministic and explainable using verified constraints. |
| 2 — Android execution foundation | [#4](https://github.com/YodHeVauHe/logos-mobile/issues/4) | Blocked | #2, #3 | The Rust engine executes locally inside an Android application and survives the defined lifecycle tests. |
| 3 — Real Logos runtime integration | [#5](https://github.com/YodHeVauHe/logos-mobile/issues/5) | Blocked | #2, #4 | At least one actual Logos module runs locally on Android. Blockchain synchronization is claimed only after the actual protocol integration and its evidence are verified. |

## Progress rules

- Issues #2–#5 contain the scope checklist and acceptance criteria for their phases.
- Every pull request links to the issue it addresses and reports verification evidence.
- Update a phase status only when its issue checklist and review evidence support that status.
- Keep later issues blocked until all listed dependencies are complete; do not start later-phase work in parallel with an unresolved prerequisite.
- Distinguish source inspection, mocked behavior, host builds, Android-target builds, emulator runs, and real-device runs. Compilation alone does not establish runtime or protocol integration.

## Scope boundary

The infrastructure contribution or reward economy remains outside these initial phases. It should be considered only after the underlying services and independent work-verification mechanisms are understood.
