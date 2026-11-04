# Logos Mobile Repository Foundation and Delivery Design

**Status:** Draft for review
**Date:** 2026-11-04
**Related specification:** [`logos-mobile-engineering-specification.md`](../../../logos-mobile-engineering-specification.md)
**Reference repository:** [YodHeVauHe/lez-wallet](https://github.com/YodHeVauHe/lez-wallet)

## 1. Purpose and success criteria

Logos Mobile aims to run real Logos infrastructure locally on Android and make its operation manageable for non-technical users. It is not a remote node dashboard or a replacement implementation of Logos protocol functionality.

The repository must support reviewable, evidence-based progress through pull requests. Contributors should be able to tell what is planned, what has been verified, what remains uncertain, and what each phase must demonstrate before the next phase begins.

The first technical success is evidence that an actual Logos runtime component or module can be built and executed locally in an Android application. Compilation alone, a mock node, or a remote service does not satisfy this criterion.

## 2. Operating model

Work proceeds in two modes:

1. **Research mode** verifies official Logos documentation and source, dependencies, build systems, supported targets, runtime behavior, and Android constraints. Findings distinguish verified facts, assumptions, and unresolved questions. Experiments may be used as evidence, but do not become production implementation by default.
2. **Implementation mode** follows the evidence and decisions from research. Work is split into independently reviewable issues and pull requests, with one working component at a time. Each change reports what is real, mocked, incomplete, or blocked, and includes appropriate verification evidence.

The central technical rule is to integrate existing Logos runtimes and modules rather than reimplement blockchain, storage, delivery, networking, or consensus functionality. Rust owns the mobile infrastructure management layer; Android-specific integration remains behind a narrow platform boundary.

## 3. Repository governance and progress tracking

Adopt the workflow patterns from `lez-wallet`, while keeping all technical scope and phase criteria specific to Logos Mobile:

- Maintain a roadmap with ordered phases, dependencies, measurable exit criteria, and current progress.
- Track one GitHub issue per phase. Each issue states its goal, scope, checklist, dependencies, acceptance checks, and evidence required for completion.
- Tie pull requests to the relevant issue. Keep each PR focused, include its checklist and verification evidence, and use review before integrating work.
- Use branches named `issue-<number>-<slug>`. Do not do feature work directly on `main`, force-push, or merge one's own PR.
- Use issue and PR templates to capture scope, acceptance criteria, and verification results consistently.
- Prefer separate documentation-only PRs when they can be reviewed independently.

The roadmap and issues are the progress record. A GitHub Projects board may be added later if the issue and roadmap views prove insufficient; it is not required for the initial foundation.

## 4. Foundation repository contents

The initial repository foundation should contain:

- `README.md`: project purpose, current status, scope, operating modes, and links to project documentation.
- `CONTRIBUTING.md`: issue-first workflow, branch and PR conventions, checks, and contributor expectations.
- `AGENTS.md`: repository-specific working rules for coding agents, including commit format and scope discipline.
- `LICENSE` and `LICENSE-APACHE`: dual licensing under MIT or Apache-2.0, matching the reference repository's licensing approach. The MIT copyright notice uses the GitHub repository owner's name, `YodHeVauHe`.
- `.gitignore`: generated Rust, Android, IDE, build, and local-secret artifacts as applicable to the actual chosen toolchain.
- `docs/roadmap.md`: phases, dependencies, status, and evidence-based exit criteria.
- `docs/decisions.md` and an ADR directory: decision index and durable records of architectural choices, including evidence and unresolved risks.
- GitHub issue and pull-request templates: phase scope, acceptance checklist, linked issue, and verification evidence.

The foundation must not create placeholder crates or assume Android compatibility before Phase 0 establishes the feasible integration boundary.

## 5. Phase 0: Logos Android feasibility audit

Phase 0 is the first engineering issue and follows the supplied engineering specification. It investigates:

1. Official Logos documentation, repositories, runtime components, and relevant module sources.
2. Dependency, native-library, build-system, and target-architecture requirements.
3. Android ARM64 compatibility based on actual build and runtime evidence, without inferring it from Linux AArch64 support.
4. Android process, linker, filesystem, packaging, background-execution, and module-loading constraints.
5. A proposed Rust-to-Logos integration boundary that preserves existing protocol implementations.
6. The smallest proof-of-concept path for executing one real Logos module locally in an Android app.
7. An architecture decision record with a feasibility recommendation, evidence, blockers, and unresolved questions.

**Phase 0 exit criteria:** the compatibility decision and its evidence are documented; relevant build/runtime dependencies and constraints are mapped; a reproducible build experiment is recorded where feasible; and the smallest next implementation milestone is proposed. Unknowns are recorded explicitly rather than presented as verified compatibility.

## 6. Post-audit implementation planning

The first implementation milestone is chosen only after Phase 0 evidence is reviewed. The plan should select the smallest independent step toward local Android execution, define its interfaces and acceptance checks, and identify whether it needs host-only tests, Android-target builds, or real-device evidence.

The roadmap may retain the engineering specification's broader milestones as provisional direction, but detailed architecture and crate boundaries must follow the audit findings. The design must not grow into contribution rewards, unrelated wallet functionality, or a broad UI build before the local runtime execution foundation is proven.

## 7. Verification and reporting

Verification is specific to each issue and states what was actually run and observed. Mocked unit tests, host builds, Android cross-compilation, emulator runs, and real-device runs must be clearly distinguished. Protocol integration is claimed only when actual Logos runtime and module behavior has been verified. Android lifecycle, process recovery, resource adaptation, and network behavior are tested as their relevant components are implemented.

No phase is marked complete solely because code compiles when its acceptance criteria require runtime or protocol evidence.

## 8. Security and privacy constraints

- Preserve existing Logos protocol and cryptographic implementations; do not invent replacements.
- Treat Android lifecycle and process-management behavior as platform-specific constraints requiring evidence.
- Never commit credentials, private keys, seed material, wallet data, or local secrets.
- Do not claim reliable uninterrupted background operation on Android without device and OS evidence.
- Defer any contribution or reward accounting until the underlying service and independent work-verification mechanism are understood.

## 9. Scope boundaries

This design covers the repository's initial governance, planning, research, and PR workflow, plus the handoff from the feasibility audit to implementation planning. It does not prescribe unverified Rust APIs, select a specific Android UI framework, promise runtime compatibility, or authorize a broad implementation before the audit. Technical component boundaries and detailed interfaces remain decisions for the evidence-backed audit and subsequent approved implementation plan.
