# Rust Mobile Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a deterministic, platform-independent Rust core for device profiles, resource budgets, role assessments, and node lifecycle transitions.

**Architecture:** Start with one `logos-mobile-core` crate in a Cargo workspace; keep profiler, budget, role, and lifecycle APIs as focused modules until cross-crate interfaces are justified. Platform data collection and Logos runtime control are injected through traits/data and remain outside this issue. Policy functions are pure and consume explicit snapshots, time, constraints, and evidence.

**Tech Stack:** Rust stable, standard library only for the initial core, Cargo workspace, unit tests.

**Spec:** [Engineering specification](../../../logos-mobile-engineering-specification.md), [accepted integration-boundary ADR](../adr/0001-logos-mobile-integration-boundary.md), and [issue #3](https://github.com/YodHeVauHe/logos-mobile/issues/3).

## Global Constraints

- “Integrate Logos; do not rebuild Logos.”
- “Keep the core Rust engine independent of Android UI code.”
- “Android-specific integration should be a narrow platform boundary.”
- “The profiler should collect measurements and expose their freshness and reliability. It should not make role decisions.”
- “Role assessment must use verified module requirements and actual protocol rules. It should explain why a role is unavailable or not recommended.”
- “Consensus eligibility must never be inferred merely from running a full node.”
- “Compilation alone is not sufficient to mark a feature complete.”

## Review Focus

- Stale or low-confidence measurements must reduce what the engine recommends; all freshness decisions receive an explicit `now` value for determinism.
- Missing and zero-valued measurements must remain distinguishable; zero cores or zero bytes are not “unknown.”
- Arithmetic must not overflow when calculating resource budgets or sums of workload requirements.
- Caller-provided role requirements without verified provenance must not become recommendations.
- Lifecycle events that are invalid for the current state must return an explicit error and leave state unchanged.

---

## Task 1: Create the Cargo workspace and capability profile API

**Files:**
- Create: `Cargo.toml`
- Create: `Cargo.lock`
- Create: `crates/logos-mobile-core/Cargo.toml`
- Create: `crates/logos-mobile-core/src/lib.rs`
- Create: `crates/logos-mobile-core/src/profile.rs`

**Interfaces:**
- Consumes: verified Phase 0 constraints; no Android or Logos runtime API.
- Produces: `CapabilityProfile`, `Measurement<T>`, `Confidence`, typed thermal/network/architecture data, and `Profiler` collection boundary.

- [ ] Write tests for fresh, stale, missing, low-confidence, and zero-valued measurements; verify current-age results use an explicit clock input.
- [ ] Run the profile tests and confirm they fail because the API is not implemented.
- [ ] Implement validated typed profile measurements and a `Profiler` trait returning a complete snapshot or a typed collection error.
- [ ] Run the profile tests and confirm they pass.
- [ ] Run `cargo fmt --check` and `cargo test -p logos-mobile-core`; commit with issue #3 in the required commit format.

## Task 2: Implement deterministic resource-budget calculation

**Files:**
- Create: `crates/logos-mobile-core/src/budget.rs`
- Modify: `crates/logos-mobile-core/src/lib.rs`
- Test: module unit tests in `budget.rs`

**Interfaces:**
- Consumes: Task 1 `CapabilityProfile` and measurement freshness/confidence API.
- Produces: `BudgetPolicy`, `ResourceBudget`, per-resource limitation reasons, and pure `calculate_budget(profile, policy, now)`.

- [ ] Write tests for reserve subtraction, policy caps, stale/low-confidence inputs, thermal/battery derating, zero resources, and overflow-safe calculations.
- [ ] Run the budget tests and confirm they fail before implementation.
- [ ] Implement a pure calculation with explicit defaults and reasons; do not start or stop modules.
- [ ] Run the budget tests and confirm they pass, then run the crate test suite.
- [ ] Format and commit the budget module with issue #3 in the required commit format.

## Task 3: Implement evidence-gated role assessment

**Files:**
- Create: `crates/logos-mobile-core/src/roles.rs`
- Modify: `crates/logos-mobile-core/src/lib.rs`
- Test: module unit tests in `roles.rs`

**Interfaces:**
- Consumes: Task 1 profiles and Task 2 budgets.
- Produces: role identifiers from the specification, explicit requirement provenance, deterministic `RoleAssessment`, and reasons for unavailable/not-recommended roles.

- [ ] Write tests for verified versus unverified requirements, missing requirements, budget shortfalls, deterministic ordering, and combinations of multiple roles.
- [ ] Add a specific test proving `ConsensusEligible` is never recommended from hardware/resource fit alone without independent verified protocol-eligibility evidence.
- [ ] Run role tests and confirm they fail before implementation.
- [ ] Implement assessment from supplied requirements and provenance only; include no invented Logos protocol thresholds or default role requirements.
- [ ] Run role tests and confirm they pass; run all crate tests.
- [ ] Format and commit role assessment with issue #3 in the required commit format.

## Task 4: Implement the node lifecycle state machine

**Files:**
- Create: `crates/logos-mobile-core/src/lifecycle.rs`
- Modify: `crates/logos-mobile-core/src/lib.rs`
- Test: module unit tests in `lifecycle.rs`

**Interfaces:**
- Consumes: no process or runtime integration; state is updated only by explicit events.
- Produces: `NodeState`, `LifecycleEvent`, `TransitionError`, and pure `transition(state, event)`.

- [ ] Write transition tests for start, successful startup, health degradation/recovery, stop, startup failure, runtime failure, and explicit recovery/restart.
- [ ] Test invalid events from each state and verify they do not mutate the input state.
- [ ] Run lifecycle tests and confirm they fail before implementation.
- [ ] Implement a deterministic transition table with typed invalid-transition errors and no side effects.
- [ ] Run lifecycle tests and confirm they pass; run all crate tests.
- [ ] Format and commit lifecycle state machine with issue #3 in the required commit format.

## Task 5: Document public contracts and verify the workspace

**Files:**
- Modify: `README.md`
- Create: `docs/architecture/rust-core.md`
- Modify: issue #3 checklist/status through its linked PR review.

**Interfaces:**
- Consumes: completed public APIs from Tasks 1–4.
- Produces: documented engine limits, evidence requirements, and review-ready issue #3 deliverables.

- [ ] Document workspace layout, measurement freshness, deterministic budgeting, evidence-gated roles, lifecycle transitions, and platform/runtime boundaries.
- [ ] Confirm docs do not claim actual device profiling, Android execution, Logos runtime control, or protocol eligibility.
- [ ] Run `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and local Markdown link/whitespace checks.
- [ ] Update issue #3 checklist items with only the work in this PR and record exact command output.
- [ ] Commit final docs and verification record with issue #3 in the required commit format.

## Handoff boundary

This plan completes only the platform-independent Rust foundation. It does not add Android UI/lifecycle adapters or control Logos processes. Android execution remains in issue #4, and the Android native dependency cross-build remains in issue #7.
