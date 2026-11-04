# Logos Mobile Foundation and Phase 0 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish the Logos Mobile repository workflow and complete an evidence-backed feasibility audit for running a real Logos module locally on Android.

**Architecture:** First create the governance and tracking documents. Then research official Logos sources and Android constraints, record the integration decision and proof-of-concept path, and stop at the Phase 0 boundary. Select implementation interfaces and crate boundaries only after the audit has been reviewed.

**Tech Stack:** Rust is the planned mobile infrastructure management layer; Android is the target platform; existing Logos runtimes and modules supply protocol functionality. The audit must establish the actual build systems, versions, and integration mechanism before selecting implementation dependencies.

**Spec:** [Repository foundation and delivery design](../specs/2026-11-04-logos-mobile-foundation-design.md), guided by [the engineering specification](../../../logos-mobile-engineering-specification.md).

## Global Constraints

- “Integrate Logos; do not rebuild Logos.”
- “Rust should own the mobile infrastructure management layer, while existing Logos runtimes and modules remain the source of protocol functionality.”
- “It must not duplicate blockchain, storage, delivery, networking, or consensus implementations already provided by Logos.”
- “Do not assume that a Linux process-management model works unchanged on Android.”
- “Do not infer Android ARM64 support from Linux aarch64 support.”
- “Compilation alone is not sufficient to mark a feature complete.”
- “Tests should distinguish mocked unit tests from real-device integration tests.”
- “Reports must not present simulated results as proof of protocol integration.”
- “Android background execution limits and manufacturer-specific process management mean uninterrupted 24/7 availability cannot be promised.”
- Do not start work without an open GitHub issue. Every issue is addressed through one or more PRs linked to that issue; do not leave work outside an issue checklist.
- Do not add runtime crates or Android app code until Phase 0 identifies a verified integration boundary and the user approves the next plan.

## Review Focus

- Linux AArch64 build evidence can be mistaken for Android ARM64 support. The audit must name the exact Android target, toolchain, and result, or mark it unverified.
- A Logos process-management assumption can fail under Android's process and linker model. The compatibility report must describe the actual launch and module-loading model and its platform constraints.
- “Module runs” can mean a mock, a host process, or an Android-local runtime. The proof-of-concept plan must identify the exact artifact, execution location, and observable evidence.
- Background service behavior can vary by Android version and device manufacturer. The report must separate platform guarantees from device-specific observations and must not promise uninterrupted availability.
- Mocked protocol behavior can be misreported as integration. Every result must label whether it is documentation-only, mocked, host-built, Android-built, emulator-run, or verified on a real device.

---

## File and artifact map

| Path | Responsibility |
|---|---|
| `README.md` | Project purpose, truthful status, scope, operating modes, and documentation entry points. |
| `CONTRIBUTING.md` | Issue-first contributions, branch and PR workflow, verification evidence, and contributor expectations. |
| `AGENTS.md` | Repository-specific agent rules, commit format, security rules, and scope discipline. |
| `LICENSE`, `LICENSE-APACHE` | MIT or Apache-2.0 licensing text. The MIT notice names `YodHeVauHe`. |
| `.gitignore` | Generated Rust/Android build files, local tooling state, and secrets for the toolchains actually selected. |
| `.github/ISSUE_TEMPLATE/phase.yml` | Structured issue form for phase goals, checklist, dependencies, acceptance, and evidence. |
| `.github/pull_request_template.md` | PR scope, linked issue, checklist, verification, and evidence fields. |
| `docs/roadmap.md` | Phase order, status, dependencies, issue links, and exit criteria. |
| `docs/decisions.md` | Decision index and unresolved question register. |
| `docs/adr/README.md` | ADR format and status conventions. |
| `docs/research/logos-repository-map.md` | Official Logos repository and runtime/module map with source revisions and links. |
| `docs/research/dependencies-and-build.md` | Dependency, native library, build system, and architecture inventory. |
| `docs/research/android-compatibility.md` | Android ARM64, process, linker, filesystem, packaging, lifecycle, and background constraints with evidence. |
| `docs/research/android-build-experiment.md` | Reproducible Android build attempt, exact inputs, output, and blockers. |
| `docs/research/android-module-poc-plan.md` | Minimal reproducible proof-of-concept steps for one real Logos module. |
| `docs/adr/0001-logos-mobile-integration-boundary.md` | Evidence-based Rust-to-Logos boundary recommendation and unresolved risks. |

## Task 1: Establish repository contribution and review rules

**Files:**
- Create: `README.md`
- Create: `CONTRIBUTING.md`
- Create: `AGENTS.md`
- Create: `LICENSE`
- Create: `LICENSE-APACHE`
- Create: `.gitignore`
- Create: `.github/ISSUE_TEMPLATE/phase.yml`
- Create: `.github/pull_request_template.md`

**Interfaces:**
- Consumes: Approved repository foundation design and the supplied engineering specification.
- Produces: The repository's contributor-facing scope, dual-license choice, issue/PR forms, and safety rules used by every later task.

- [ ] **Step 1: Write the README with honest project status.** State that the project is in research and foundation setup, describe local Android Logos execution as the goal, link the roadmap and engineering specification, and say compatibility is not established yet.
- [ ] **Step 2: Write contribution rules.** Require an issue checklist, branch `issue-<number>-<slug>`, focused PRs, verification evidence, and review before integration. Explain that research findings must label facts, assumptions, and unknowns.
- [ ] **Step 3: Write agent rules.** Include the same issue scope and review requirements, conventional commit subjects with `Changes:` and `Verification:` bullet sections and no trailers, and prohibitions on committing keys, seeds, wallet data, credentials, or secrets.
- [ ] **Step 4: Add full MIT and Apache-2.0 license texts.** Set the MIT notice to `Copyright (c) 2026 YodHeVauHe`; do not copy the LEZ Wallet copyright holder.
- [ ] **Step 5: Add only confirmed ignore patterns.** Ignore Rust `target/`, Android/Gradle build output when applicable, IDE state, and `.env` or local secrets; do not ignore source or reproducibility artifacts.
- [ ] **Step 6: Add a phase issue form.** Include fields for goal, in-scope work, out-of-scope work, dependencies, checklist, acceptance checks, and evidence. Mark each section required except dependencies.
- [ ] **Step 7: Add a PR template.** Include linked issue, scope summary, completed checklist, verification commands/results, and evidence type (mock, host build, Android build, emulator, or real device).
- [ ] **Step 8: Check links and whitespace.** Run `git diff --check`; open each relative README link and confirm it resolves to a file created in this plan.
- [ ] **Step 9: Commit the foundation rules.** Use a conventional `docs:` subject and the required `Changes:` and `Verification:` body sections.

**Acceptance:** A new contributor can determine the project status, how to propose work, what evidence a PR requires, and what must never be committed. The license texts are complete and name the selected holder consistently.

## Task 2: Define roadmap and decision record structure

**Files:**
- Create: `docs/roadmap.md`
- Create: `docs/decisions.md`
- Create: `docs/adr/README.md`

**Interfaces:**
- Consumes: Task 1's contribution rules and the approved Phase 0 scope.
- Produces: The shared progress record and durable decision format used by later tasks.

- [ ] **Step 1: Create the roadmap with Phase 0 as the active phase.** Include the engineering specification's discovery and feasibility work as a checklist; list later Rust foundation, Android execution, and real Logos integration phases as provisional, each with its stated exit criterion.
- [ ] **Step 2: Add dependencies and status fields.** For each phase, record status, prerequisite, issue link (initially “not created”), and evidence required to mark it complete. Do not assign calendar dates without user-approved scheduling.
- [ ] **Step 3: Create the decision index.** Include a short entry for the Android feasibility decision as pending evidence and link to `docs/adr/0001-logos-mobile-integration-boundary.md` once Task 6 creates it.
- [ ] **Step 4: Define ADR structure.** Specify title, status, context, evidence, decision, alternatives, consequences, and open risks. State that superseded decisions remain linked rather than deleted.
- [ ] **Step 5: Check that roadmap scope matches the source spec.** Confirm no roadmap entry claims that a runtime, Android target, or module is already compatible.
- [ ] **Step 6: Commit the roadmap and decision index.** Use a conventional `docs:` subject and required commit body sections.

**Acceptance:** Phase order and completion evidence are visible in one roadmap, and the decision index distinguishes pending research from accepted decisions.

## Task 3: Open and maintain the Phase 0 tracking issue

**Files:**
- Create: GitHub issue `Phase 0: Logos Android feasibility audit`
- Modify: `docs/roadmap.md` with the created issue URL.

**Interfaces:**
- Consumes: Merged Task 1 issue template and Task 2 roadmap, plus the approved Phase 0 exit criteria.
- Produces: The GitHub issue that owns the audit checklist and records its PR evidence.

- [ ] **Step 1: Create the Phase 0 issue from the repository issue form.** Title it `Phase 0: Logos Android feasibility audit`; include scope, dependencies, every deliverable from Tasks 4–6, acceptance criteria, and the rule that no broad implementation starts before review.
- [ ] **Step 2: Link the issue in the roadmap.** Replace the initial issue-not-created status for Phase 0 with the live issue URL; leave future phase issues uncreated until their scope is ready. Submit this edit in a focused PR linked to the new Phase 0 issue.
- [ ] **Step 3: Use focused PRs against the issue.** Keep governance setup separate from research outputs where useful; link every audit PR to the Phase 0 issue and preserve the evidence checklist in the PR.
- [ ] **Step 4: Close Phase 0 only after review.** Verify each acceptance criterion against merged documentation and recorded experiments, then update the roadmap and issue with the final feasibility decision and the approved next planning action.

**Acceptance:** Phase 0 has one authoritative issue with a complete checklist, the roadmap links to it, and completion requires reviewed evidence rather than a status-only update.

## Task 4: Map official Logos repositories and build dependencies

**Files:**
- Create: `docs/research/logos-repository-map.md`
- Create: `docs/research/dependencies-and-build.md`

**Interfaces:**
- Consumes: The engineering specification's official references, Task 2's roadmap/decision format, and Task 3's tracking issue.
- Produces: A versioned source map and build inventory that Task 5 uses to assess Android compatibility.

- [ ] **Step 1: Start from the official Logos documentation index.** Read `https://docs.logos.co/llms.txt` and the relevant official sections named in the engineering specification: Logos overview, running a node, building/running a Logos Core module, and coding with agents.
- [ ] **Step 2: Fetch and inspect the official `logos-co/logos-liblogos` source.** Use `npx opensrc logos-co/logos-liblogos` as directed by the repository's source-reference instructions, then record its source revision, purpose, runtime entry points, module model, build files, and stable source links. If it is already present under `opensrc/`, confirm its revision in `opensrc/sources.json` and inspect that copy.
- [ ] **Step 3: Follow official source links to relevant runtimes and modules.** Add a repository only when official documentation or source establishes its relationship to the required runtime/module path. For each, record owner, URL, revision/tag, role, and evidence.
- [ ] **Step 4: Inventory build inputs.** Record Rust toolchain files, Cargo workspaces and features, Nix/Gradle/CMake/other build definitions, native libraries, generated code, and required external toolchains found in the inspected repositories.
- [ ] **Step 5: Inventory supported architectures and operating systems.** Quote or link the authoritative build configuration or CI matrix for each claimed target. Mark targets absent from source/config as unknown rather than inferred.
- [ ] **Step 6: Record dependency provenance.** For every proposed dependency in the real execution path, record whether it is an upstream crate, system library, bundled library, or generated component, and the exact version/revision where pinned.
- [ ] **Step 7: Cross-check source claims.** Verify every repository map entry against its upstream README/build configuration and every version against a tag, lockfile, or manifest.
- [ ] **Step 8: Commit the source and build inventory.** Use a conventional `docs:` subject and required commit body sections.

**Acceptance:** Another engineer can follow the source links and identify the exact runtime/module repositories, build entry points, native dependencies, pinned revisions, and known target support without relying on unsupported assumptions.

## Task 5: Assess Android compatibility and define the smallest experiment

**Files:**
- Create: `docs/research/android-compatibility.md`
- Create: `docs/research/android-build-experiment.md`
- Create: `docs/research/android-module-poc-plan.md`

**Interfaces:**
- Consumes: Task 4's versioned source and build inventory.
- Produces: An evidence-based compatibility assessment and reproducible experiment plan for Task 6.

- [ ] **Step 1: Inspect official Android build guidance for each dependency.** Record NDK/Clang requirements, Rust Android targets, minimum Android API level, ABI, linker mode, native library constraints, and any dependency without Android guidance.
- [ ] **Step 2: Compare Android ARM64 requirements to actual build targets.** For each runtime/module dependency, cite the exact configuration or CI result. Do not treat Linux `aarch64-unknown-linux-gnu` as Android support.
- [ ] **Step 3: Trace the runtime's process and module model.** Determine from source whether modules run as child processes, shared libraries, in-process components, or another mechanism. Record required executable permissions, dynamic loading, linker behavior, IPC, and filesystem assumptions.
- [ ] **Step 4: Assess Android app packaging and lifecycle boundaries.** Map required native artifacts into APK/AAB packaging, app-private storage needs, permissions, foreground service needs, process death, restart, and background limits. Separate documented platform behavior from tested device behavior.
- [ ] **Step 5: Run the smallest reproducible Android build experiment supported by the evidence.** Use the exact upstream build path and selected Android ABI/API. Capture the source revision, tool versions, command, exit result, and any blocker in `docs/research/android-build-experiment.md`. If an upstream dependency cannot be built for Android, record the exact failing dependency and build output rather than substituting a mock.
- [ ] **Step 6: Define the minimum real-module execution experiment.** Name the selected upstream module and pinned source revision; specify build commands, Android ABI/API, app packaging steps, launch procedure, expected local observable behavior, logs/artifacts to retain, and cleanup. If no module is viable based on evidence, define the experiment that would resolve the blocking uncertainty.
- [ ] **Step 7: Label experiment evidence.** For each required observation, state whether it is source inspection, host build, Android cross-build, emulator execution, or real-device execution. Explicitly state what a mock can and cannot establish.
- [ ] **Step 8: Record compatibility gaps and risks.** Include manufacturer-specific background behavior, native dependency blockers, linker/process restrictions, storage assumptions, and missing official support evidence.
- [ ] **Step 9: Commit the compatibility report and experiment plan.** Use a conventional `docs:` subject and required commit body sections.

**Acceptance:** The report supports a go/no-go recommendation for the smallest Android execution experiment and provides enough pinned information for another engineer to reproduce it. No Android compatibility claim exceeds the available evidence.

## Task 6: Record the integration decision and Phase 0 exit recommendation

**Files:**
- Create: `docs/adr/0001-logos-mobile-integration-boundary.md`
- Modify: `docs/decisions.md`
- Modify: `docs/roadmap.md`

**Interfaces:**
- Consumes: Tasks 4 and 5's verified evidence and documented unknowns.
- Produces: A reviewed integration boundary recommendation and the smallest proposed post-audit milestone.

- [ ] **Step 1: Write the ADR context and evidence.** Link every compatibility statement to the source map, Android report, official docs, source revision, or recorded experiment.
- [ ] **Step 2: Compare feasible integration boundaries.** Include only alternatives supported by inspected source, such as the actual Logos process model or an Android-compatible library/module boundary. For each, state tradeoffs and blockers; do not invent an upstream API.
- [ ] **Step 3: State the recommendation and confidence.** Mark the ADR proposed until the Phase 0 review accepts it. If a required fact is missing, state the unresolved question and the experiment needed to answer it.
- [ ] **Step 4: Propose one smallest next milestone.** Define its goal, in-scope behavior, acceptance evidence, and dependencies. Exclude broad UI work, contribution rewards, and unrelated services until local runtime execution is demonstrated.
- [ ] **Step 5: Update the decision index and roadmap.** Link the ADR and reports, record evidence status, and leave Phase 0 incomplete until all exit criteria are satisfied and reviewed.
- [ ] **Step 6: Review Phase 0 coverage.** Confirm repository map, dependency/build inventory, Android ARM64 assessment, process/module assessment, integration boundary, proof-of-concept plan, and unresolved-risk list are all present.
- [ ] **Step 7: Commit the ADR and Phase 0 handoff.** Use a conventional `docs:` subject and required commit body sections.

**Acceptance:** The ADR's recommendation follows from linked evidence, unknowns are visible, and the proposed next milestone is small enough for a separate approved implementation plan.

## Handoff boundary

This plan ends after the Phase 0 audit and the proposed next milestone are reviewed. It does not authorize implementing the Rust supervisor, Android app, or Logos runtime adapter. Those changes require a follow-up implementation plan based on the accepted ADR and Phase 0 evidence.
