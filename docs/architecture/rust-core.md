# Rust mobile core

The `logos-mobile-core` crate contains deterministic policy primitives shared
by future platform adapters. It currently has no external Rust dependencies and
does not link Android APIs or Logos runtime libraries.

## Capability profiles

Platform adapters implement `Profiler` and return a `CapabilityProfile` whose
fields are typed `Measurement<T>` values. A measurement records an optional
value, sample time, and confidence. Missing data is distinct from an observed
zero. Consumers supply `now`, a maximum age, and a minimum confidence; the
profile layer reports whether a value is fresh, stale, future-dated, missing,
or below the confidence floor. The profile API does not choose node roles.

## Resource budgets

`calculate_budget` is a pure calculation over one profile snapshot and a
caller-selected `BudgetPolicy`. The policy contains caps, reserves, freshness
requirements, battery thresholds, and thermal/battery derating factors. Results
retain per-resource limitation reasons. An unavailable amount (`None`) is not a
measured zero (`Some(0)`). This module does not start, stop, or configure any
service. The policy values are supplied by its caller; they are not Logos
protocol requirements or device-specific recommendations.

## Role assessment

`assess_roles` consumes caller-supplied `RoleRequirement` values and a resource
budget. Requirements carry a reference marked verified by the caller; the core
does not fetch or validate external documentation. It sums the resource minima
for roles in stable enum order, reports shortfalls, and recommends only
requirements with verified module evidence that fit the remaining combined
budget. The core defines no module defaults or protocol thresholds.

`ConsensusEligible` has an additional gate: separate verified protocol
eligibility evidence must be supplied. Hardware fit, full-node role selection,
or module evidence alone cannot satisfy that gate. This software check records
caller-provided provenance; it does not itself establish protocol eligibility.

## Lifecycle

`transition(state, event)` returns a next logical state or a typed
`TransitionError`. It has no process, Android lifecycle, persistence, or
network side effects. A future adapter must perform those operations and report
the resulting events to this state machine. Invalid events leave the caller's
state unchanged.

## Integration boundary

This crate is the platform-independent foundation for the work tracked in
[issue #3](https://github.com/YodHeVauHe/logos-mobile/issues/3). Android app and
process integration is tracked by [issue #4](https://github.com/YodHeVauHe/logos-mobile/issues/4).
The Android ARM64 native dependency build investigation is tracked by
[issue #7](https://github.com/YodHeVauHe/logos-mobile/issues/7). No Android
runtime or Logos process integration is claimed by these Rust APIs.

## Local verification

Run these from the repository root:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
