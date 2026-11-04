# Android Qt and Logos Cross-Build Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and stage the pinned Qt and Logos runtime closure for Android `arm64-v8a` with reproducible Nix evidence.

**Architecture:** Add a dedicated Android Nix entry point built on `x86_64-linux`. Pin Qt 6.9.2, API 28, and NDK r27c; use Android-only Nixpkgs overrides and a locked local patch set for the pinned Logos flakes that do not expose Android targets. Build and verify each artifact stage before documenting the runtime closure result.

**Tech Stack:** Nix flakes, Nixpkgs cross compilation, Android NDK, Qt 6.9.2, CMake/Ninja, Python 3 standard library, `llvm-readelf`.

**Spec:** [Android Qt and Logos cross-build design](../specs/2026-11-04-android-qt-crossbuild-design.md)

## Global Constraints

- Target `aarch64-linux-android`, ABI `arm64-v8a`, build host `x86_64-linux`, API 28, NDK r27c `27.2.12479018`, and Qt 6.9.2.
- Use pinned Nixpkgs revision `e9f00bd893984bc8ce46c895c3bf7cac95331127` and the Logos source revisions recorded in `docs/research/logos-repository-map.md` and `docs/research/dependencies-and-build.md`.
- Disable PCRE2 JIT and Qt SCTP only for the Android target; preserve desktop configurations.
- Do not replace a real dependency with a mock or unpinned package.
- Keep host build tools separate from Android target libraries and executables.
- Cross-compilation evidence does not imply APK, emulator, or physical-device evidence.
- APK/application packaging, emulator or physical-device execution, Rust supervisor work, broad UI, Logos protocol implementation, and uninterrupted Android background operation are out of scope.

## Review Focus

- **Android versus Linux AArch64:** assert the cross target config is `aarch64-unknown-linux-android` and inspect target ELF machine IDs; Task 1 and Task 6 own these checks.
- **NDK/API skew:** assert the compiler uses NDK `27.2.12479018` and API 28; Task 1 owns this check.
- **PCRE2 JIT scope:** assert JIT is disabled in the Android derivation and unchanged in the host derivation; Task 2 owns this check.
- **Qt SCTP scope:** assert Android Qt has `QT_FEATURE_sctp=OFF` and no `lksctp-tools` input while host Qt remains unchanged; Task 2 owns this check.
- **Patch/source drift:** assert each patch applies to its recorded upstream commit and rejects a different commit; Task 3 owns this check.

---

### Task 1: Add the Android cross-build entry point and target assertions

**Files:**
- Create: `nix/android/flake.nix`
- Create: `nix/android/target.nix`
- Create: `nix/android/tests/target-config.sh`
- Create: `nix/android/flake.lock`

**Interfaces:**
- Consumes: the pinned Nixpkgs revision and toolchain values in the approved spec.
- Produces: `lib.androidTarget` with `buildSystem`, `targetSystem`, `targetTriple`, `abi`, `apiLevel`, `ndkRevision`, `qtVersion`, `buildPkgs`, and `crossPkgs`.
- Produces: `packages.x86_64-linux.android-target-report`, a JSON report generated from the evaluated cross package set.

- [ ] **Step 1: Write the failing target-config test**

```bash
#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/nix/android"

test "$(nix eval --raw .#lib.androidTarget.buildSystem)" = "x86_64-linux"
test "$(nix eval --raw .#lib.androidTarget.targetSystem)" = "aarch64-linux-android"
test "$(nix eval --raw .#lib.androidTarget.targetTriple)" = "aarch64-unknown-linux-android"
test "$(nix eval --raw .#lib.androidTarget.abi)" = "arm64-v8a"
test "$(nix eval --raw .#lib.androidTarget.apiLevel)" = "28"
test "$(nix eval --raw .#lib.androidTarget.ndkRevision)" = "27.2.12479018"
test "$(nix eval --raw .#lib.androidTarget.qtVersion)" = "6.9.2"
```

- [ ] **Step 2: Run the test and confirm it fails because the Android flake is absent**

Run: `bash nix/android/tests/target-config.sh` from the repository root.
Expected: FAIL because `nix/android/flake.nix` and `lib.androidTarget` do not exist.

- [ ] **Step 3: Implement the cross package set and metadata**

In `target.nix`, import pinned Nixpkgs with build platform `x86_64-linux` and cross system `aarch64-unknown-linux-android`, Android API 28, and the prebuilt Android NDK toolchain. Keep the report fields as literal metadata beside the package set so later tasks can compare the declaration with the selected compiler and Qt derivation. Export the Nixpkgs revision through the flake lock.

Use this target constructor in `target.nix`:

```nix
let
  pinnedNixpkgs = builtins.getFlake "github:NixOS/nixpkgs/e9f00bd893984bc8ce46c895c3bf7cac95331127";
  androidPlatform = pinnedNixpkgs.lib.systems.examples.aarch64-android-prebuilt // {
    androidSdkVersion = "28";
    androidNdkVersion = 27;
  };
  buildPkgs = import pinnedNixpkgs { system = "x86_64-linux"; };
  crossPkgs = import pinnedNixpkgs {
    localSystem = "x86_64-linux";
    crossSystem = androidPlatform;
  };
in {
  buildSystem = buildPkgs.stdenv.buildPlatform.system;
  targetSystem = "aarch64-linux-android";
  targetTriple = crossPkgs.stdenv.hostPlatform.config;
  abi = "arm64-v8a";
  apiLevel = "28";
  ndkRevision = "27.2.12479018";
  qtVersion = crossPkgs.qt6.qtbase.version;
  inherit buildPkgs crossPkgs;
}
```

- [ ] **Step 4: Run target-config and derivation checks**

Run: `bash nix/android/tests/target-config.sh`.
Expected: PASS for all seven target/toolchain values.

Run: `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.hostPlatform.config`.
Expected: `aarch64-unknown-linux-android`.

Run: `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.buildPlatform.system`.
Expected: `x86_64-linux`.

- [ ] **Step 5: Commit the cross-build entry point**

```text
feat: add pinned Android cross-build target (#12)

Changes:
- Add the Android Nix entry point and target metadata.
- Pin the Nixpkgs source and assert host, ABI, API, NDK, and Qt versions.

Verification:
- `bash nix/android/tests/target-config.sh`: passed.
- `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.hostPlatform.config`: reported the Android target.
- `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.buildPlatform.system`: reported x86_64-linux.
```

### Task 2: Add Android-only NDK, PCRE2, and Qt overrides

**Files:**
- Create: `nix/android/overlays/android-cross.nix`
- Create: `nix/android/tests/package-overrides.py`
- Modify: `nix/android/target.nix`

**Interfaces:**
- Consumes: `lib.androidTarget.crossPkgs` from Task 1.
- Produces: Android `pcre2`, `qt6.qtbase`, and `qt6.qtremoteobjects` derivations with PCRE2 JIT and Qt SCTP disabled.
- Produces: `packages.x86_64-linux.android-pcre2`, `android-qtbase`, and `android-qtremoteobjects`.
- Produces: a cross compiler backed by NDK `27.2.12479018`, not the Nixpkgs default `27.0.12077973`.

- [ ] **Step 1: Write the failing override assertions**

```python
import json
import subprocess

def eval_json(expr):
    return json.loads(subprocess.check_output(
        ["nix", "eval", "--json", f"nix/android#{expr}"], text=True
    ))

android = eval_json("lib.androidPackageConfig")
host = eval_json("lib.hostPackageConfig")
assert "--enable-jit=no" in android["pcre2ConfigureFlags"]
assert "-DQT_FEATURE_sctp=OFF" in android["qtbaseCmakeFlags"]
assert "lksctp-tools" not in android["qtbaseBuildInputNames"]
assert "--enable-jit=auto" in host["pcre2ConfigureFlags"]
assert "-DQT_FEATURE_sctp=ON" in host["qtbaseCmakeFlags"]
assert "lksctp-tools" in host["qtbaseBuildInputNames"]
assert android["ndkRevision"] == "27.2.12479018"
```

- [ ] **Step 2: Run the assertions and confirm they fail**

Run: `python3 nix/android/tests/package-overrides.py`.
Expected: FAIL because `androidPackageConfig` and `hostPackageConfig` are not exported and the Android derivations are not overridden.

- [ ] **Step 3: Implement target-specific package overrides**

Override PCRE2 with `--enable-jit=no` only when `hostPlatform.isAndroid`. Override Qt Base with `-DQT_FEATURE_sctp=OFF` and filter `lksctp-tools` only for Android. Keep the original host package values for non-Android targets. Rebuild the Nixpkgs `androidndkPkgs_27` package set from the pinned `androidndk-pkgs.nix` expression with `androidenv.composeAndroidPackages { includeNDK = true; ndkVersion = "27.2.12479018"; }` for the build and target SDK inputs; make the cross compiler use that package set.

Use these target-only transformations in `overlays/android-cross.nix`:

```nix
final: prev:
let
  lib = prev.lib;
  android = prev.stdenv.hostPlatform.isAndroid;
in {
  pcre2 = prev.pcre2.overrideAttrs (old: {
    configureFlags = if android then
      lib.filter (flag: !(lib.hasPrefix "--enable-jit=" flag)) old.configureFlags
      ++ [ "--enable-jit=no" ]
    else old.configureFlags;
  });

  qt6 = prev.qt6.overrideScope' (qtFinal: qtPrev: {
    qtbase = qtPrev.qtbase.overrideAttrs (old: {
      buildInputs = if android then
        lib.filter (input: (input.pname or "") != "lksctp-tools") old.buildInputs
      else old.buildInputs;
      cmakeFlags = if android then
        lib.filter (flag: flag != "-DQT_FEATURE_sctp=ON") old.cmakeFlags
        ++ [ "-DQT_FEATURE_sctp=OFF" ]
      else old.cmakeFlags;
    });
  });
}
```

- [ ] **Step 4: Run override assertions and inspect the selected compiler**

Run: `python3 nix/android/tests/package-overrides.py`.
Expected: PASS for Android-only feature values, unchanged host values, and exact NDK revision.

Run: `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.cc.version`.
Expected: `27.2.12479018`.

Run: `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.hostPlatform.androidSdkVersion`.
Expected: `28`.

- [ ] **Step 5: Commit the Android overrides**

```text
fix: configure Android Qt and PCRE2 cross builds (#12)

Changes:
- Pin the Android NDK cross package to r27c and API 28.
- Disable PCRE2 JIT and Qt SCTP for Android while preserving host settings.

Verification:
- `python3 nix/android/tests/package-overrides.py`: passed.
- `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.cc.version`: reported 27.2.12479018.
- `nix eval --raw nix/android#lib.androidTarget.crossPkgs.stdenv.hostPlatform.androidSdkVersion`: reported 28.
```

### Task 3: Patch the pinned Logos target matrices reproducibly

**Files:**
- Create: `nix/android/sources.json`
- Create: `nix/android/.gitignore`
- Create: `nix/android/patches/logos-nix-f55bf91.patch`
- Create: `nix/android/patches/logos-nix-6e0f4a7.patch`
- Create: `nix/android/patches/logos-liblogos-af19e51.patch`
- Create: `nix/android/patches/logos-container-subprocess-df1fbec.patch`
- Create: `nix/android/patches/logos-module-loader-qt-3eee855.patch`
- Create: `nix/android/patches/logos-module-builder-de169fd.patch`
- Create: `nix/android/prepare-sources.sh`
- Create: `nix/android/source_patches.py`
- Create: `nix/android/tests/test_source_patches.py`
- Modify: `nix/android/flake.nix`

**Interfaces:**
- Consumes: exact source revisions from the root liblogos lockfile and the capability/state module lockfiles.
- Produces: patched source snapshots with a shared Android target helper; each snapshot remains traceable to its upstream commit and patch file.
- Produces: local flake input overrides for the patched target-bearing sources. Unpatched inputs continue to use their pinned upstream source.

- [ ] **Step 1: Write the failing source-patch verification test**

```python
import unittest
from source_patches import RevisionMismatch, require_exact_revision

class SourceRevisionTests(unittest.TestCase):
    def test_accepts_recorded_commit(self):
        self.assertEqual(require_exact_revision("logos-nix", "a" * 40, "a" * 40), "a" * 40)

    def test_rejects_unexpected_commit(self):
        with self.assertRaises(RevisionMismatch):
            require_exact_revision("logos-nix", "a" * 40, "b" * 40)
```

- [ ] **Step 2: Run the test and confirm it fails because the patch manifest is absent**

Run from `nix/android`: `python3 -m unittest discover -s tests -p 'test_source_patches.py' -v`.
Expected: FAIL because `source_patches.py` does not exist.

- [ ] **Step 3: Implement the source manifest, patch set, and source preparation command**

Record source repository, full commit SHA, patch path, and dependency relationships in `sources.json`. Include both `logos-nix` revisions required by the pinned graph (`f55bf91b8a723ee5c27c39c773b034255538d280` and `6e0f4a7120fced10829b0b3a698ff619a11d4605`), root `logos-liblogos` (`af19e519c9b579a1dab86e8dc3960925fc28b526`), default subprocess container (`df1fbecf2f35910e7efe14d7a8208959f2f20ae1`), default Qt loader (`3eee855362c238052eb66833ec14782c9ab2d07e`), and `logos-module-builder` (`de169fd45d7bdf6e28ce1f151b4c34880523ace9`) used by the selected modules. Add the Android cross-target branch to each required `logos-nix` target helper, then patch only package flakes whose target outputs are required by `logos-liblogos` and the selected modules. `prepare-sources.sh --check` must verify each locked commit and run `git apply --check`; `prepare-sources.sh --apply` must apply the committed patch to a clean checkout at that exact commit and fail on revision or patch drift. Point the Android flake's affected inputs at those prepared source snapshots.

Each `logos-nix` helper patch must add the Android output and use the cross package set with the shared overlay. The patched helper must preserve the current desktop and Windows branch:

```nix
androidSystem = "aarch64-linux-android";
forAllTargets = f:
  nixpkgs.lib.genAttrs (supportedSystems ++ [ androidSystem ] ++ [ "x86_64-windows" ]) (system:
    if system == androidSystem then
      f {
        inherit system;
        pkgs = import nixpkgs {
          localSystem = "x86_64-linux";
          crossSystem = nixpkgs.lib.systems.examples.aarch64-android-prebuilt // {
            androidSdkVersion = "28";
            androidNdkVersion = 27;
          };
          overlays = [ (import ./nix/android-cross-overlay.nix) ];
        };
      }
    else if system == "x86_64-windows" then
      f {
        inherit system;
        pkgs = mkWindowsPkgs { buildSystem = "x86_64-linux"; };
      }
    else
      f {
        inherit system;
        pkgs = import nixpkgs { inherit system; };
      }
  );
```

`prepare-sources.sh --apply` copies `nix/android/overlays/android-cross.nix` to `nix/android-cross-overlay.nix` in each prepared `logos-nix` snapshot before applying the helper patch. Prepared source snapshots live under `nix/android/.sources/`, which is ignored by `nix/android/.gitignore`.

- [ ] **Step 4: Verify every patch and evaluate every Android package output**

Run from `nix/android`: `python3 -m unittest discover -s tests -p 'test_source_patches.py' -v`.
Expected: PASS for the recorded revision and reject a deliberately mismatched revision.

Run: `bash nix/android/prepare-sources.sh --check`.
Expected: PASS with the upstream commit and patch filename printed for every patched source.

Run: `nix eval --json nix/android#packages.x86_64-linux`.
Expected: JSON contains Android PCRE2, Qt Base, Qt RemoteObjects, `android-logos-runtime`, `android-modules-state-module`, and `android-capability-module` outputs; no output resolves to a desktop package set.

- [ ] **Step 5: Commit the target-support patch set**

```text
build: add Android targets to pinned Logos flakes (#12)

Changes:
- Pin and verify the source patch set for target-bearing Logos flakes.
- Expose the Android cross target through the selected runtime dependency graph.

Verification:
- `python3 -m unittest discover -s tests -p 'test_source_patches.py' -v`: passed.
- `bash nix/android/prepare-sources.sh --check`: passed for every patch.
- `nix eval --json nix/android#packages.x86_64-linux`: listed the Android package outputs.
```

### Task 4: Cross-build PCRE2 and Qt

**Files:**
- Create: `nix/android/build-qt.sh`
- Create: `nix/android/tests/qt_outputs.py`
- Modify: `nix/android/flake.nix`

**Interfaces:**
- Consumes: Android package outputs from Tasks 1–3.
- Produces: `android-pcre2`, `android-qtbase`, and `android-qtremoteobjects` artifacts.
- Produces: full build logs and exit statuses for the Qt stage under `docs/research/logs/android-cross-build/`.

- [ ] **Step 1: Write the failing Qt-output assertions**

```python
import json
import subprocess

outputs = json.loads(subprocess.check_output(["nix", "eval", "--json", "nix/android#packages.x86_64-linux"], text=True))
required = {"android-pcre2", "android-qtbase", "android-qtremoteobjects"}
assert required <= outputs.keys()
```

- [ ] **Step 2: Run the test and confirm Qt outputs are not exported**

Run: `python3 nix/android/tests/qt_outputs.py`.
Expected: FAIL because the Android Qt outputs are not exported.

- [ ] **Step 3: Build the Android Qt stage in dependency order**

Add the package outputs using the pinned Nixpkgs inputs and Android cross package set. `build-qt.sh` must capture each command's output and exit status without hiding a failure:

```bash
set -euo pipefail
mkdir -p docs/research/logs/android-cross-build
for package in android-pcre2 android-qtbase android-qtremoteobjects; do
  log="docs/research/logs/android-cross-build/${package}.log"
  NIXPKGS_ALLOW_UNFREE=1 NIXPKGS_ACCEPT_ANDROID_SDK_LICENSE=1 \
    nix build -L --no-link --print-out-paths "nix/android#packages.x86_64-linux.${package}" 2>&1 | tee "$log"
done
```

- [ ] **Step 4: Run Qt builds and output checks**

Run: `bash nix/android/build-qt.sh`.
Expected: exit 0 only when all three Android Qt outputs build; each log records the actual Nix output path and command result.

Run: `python3 nix/android/tests/qt_outputs.py`.
Expected: PASS for all three Android Qt package keys and their target metadata.

- [ ] **Step 5: Commit the Qt cross-build stage**

```text
build: cross compile Qt dependencies for Android (#12)

Changes:
- Add ordered Android builds for PCRE2, Qt Base, and Qt RemoteObjects.
- Preserve the full Qt build logs, Nix output paths, and exit statuses.

Verification:
- `bash nix/android/build-qt.sh`: passed for all three Android outputs.
- `python3 nix/android/tests/qt_outputs.py`: passed.
```

### Task 5: Cross-build the pinned Logos runtime closure

**Files:**
- Create: `nix/android/build-logos.sh`
- Create: `nix/android/tests/logos_outputs.py`
- Modify: `nix/android/flake.nix`

**Interfaces:**
- Consumes: Android Qt outputs from Task 4 and patched source inputs from Task 3.
- Produces: `android-logos-sdk`, `android-logos-protocol`, `android-logos-container`, `android-logos-loader`, `android-logos-package-manager`, `android-logos-runtime`, `android-modules-state-module`, and `android-capability-module`.
- Produces: full build logs and exit statuses for each Logos stage under `docs/research/logs/android-cross-build/`.

- [ ] **Step 1: Write the failing Logos-output assertions**

```python
import json
import subprocess

outputs = json.loads(subprocess.check_output(["nix", "eval", "--json", "nix/android#packages.x86_64-linux"], text=True))
required = {
    "android-logos-sdk", "android-logos-protocol", "android-logos-container",
    "android-logos-loader", "android-logos-package-manager", "android-logos-runtime",
    "android-modules-state-module", "android-capability-module",
}
assert required <= outputs.keys()
```

- [ ] **Step 2: Run the test and confirm the complete Logos outputs are not exported**

Run: `python3 nix/android/tests/logos_outputs.py`.
Expected: FAIL because the Android Logos package outputs are not exported.

- [ ] **Step 3: Build the pinned Logos packages in dependency order**

Implement `build-logos.sh` to build the eight package outputs listed above, in dependency order, using `NIXPKGS_ALLOW_UNFREE=1` and `NIXPKGS_ACCEPT_ANDROID_SDK_LICENSE=1`. Capture a separate log and Nix output path for each package with `nix build -L --no-link --print-out-paths`; stop immediately on a failed package.

Use this loop after the dependency order is declared in the script:

```bash
set -euo pipefail
mkdir -p docs/research/logs/android-cross-build
for package in android-logos-sdk android-logos-protocol android-logos-container android-logos-loader android-logos-package-manager android-logos-runtime android-modules-state-module android-capability-module; do
  log="docs/research/logs/android-cross-build/${package}.log"
  NIXPKGS_ALLOW_UNFREE=1 NIXPKGS_ACCEPT_ANDROID_SDK_LICENSE=1 \
    nix build -L --no-link --print-out-paths "nix/android#packages.x86_64-linux.${package}" 2>&1 | tee "$log"
done
```

- [ ] **Step 4: Run the Logos build and verify the output list**

Run: `bash nix/android/build-logos.sh`.
Expected: exit 0 only after all eight pinned Android package outputs build; logs preserve each output path and exit status.

Run: `python3 nix/android/tests/logos_outputs.py`.
Expected: PASS for every required runtime and module package.

- [ ] **Step 5: Commit the Logos closure build**

```text
build: cross compile Logos runtime closure for Android (#12)

Changes:
- Add Android builds for the pinned SDK, protocol, container, loader, package manager, runtime, and bundled modules.
- Preserve the full build logs and output paths for each package.

Verification:
- `bash nix/android/build-logos.sh`: passed for all eight Android outputs.
- `python3 nix/android/tests/logos_outputs.py`: passed.
```

### Task 6: Stage and validate Android artifacts

**Files:**
- Create: `nix/android/stage-runtime.nix`
- Create: `nix/android/verify_staging.py`
- Create: `nix/android/tests/test_verify_staging.py`
- Create: `nix/android/tests/__init__.py`
- Modify: `nix/android/flake.nix`

**Interfaces:**
- Consumes: successful Nix outputs from Task 5.
- Produces: `packages.x86_64-linux.android-runtime-staging` with deterministic library, executable, Qt plugin, and module paths.
- Produces: `share/android-runtime-manifest.json` containing source pin, derivation, relative path, ELF machine, SONAME, and dynamic dependencies for each staged ELF.
- Produces: `verify_staging.validate_manifest(manifest, root)` returning an ordered list of validation errors.

- [ ] **Step 1: Write the failing manifest-validation test**

```python
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from verify_staging import validate_manifest

class StagingValidationTests(unittest.TestCase):
    def manifest(self, path="lib/liblogos_core.so", machine="AArch64"):
        return {
            "target": "aarch64-unknown-linux-android",
            "abi": "arm64-v8a",
            "artifacts": [{"path": path, "machine": machine, "soname": "liblogos_core.so", "needed": []}],
        }

    def test_rejects_host_elf_machine(self):
        with TemporaryDirectory() as tmp:
            errors = validate_manifest(self.manifest(machine="Advanced Micro Devices X86-64"), Path(tmp))
            self.assertIn("lib/liblogos_core.so: unexpected ELF machine", errors)

    def test_rejects_missing_artifact(self):
        with TemporaryDirectory() as tmp:
            errors = validate_manifest(self.manifest(), Path(tmp))
            self.assertIn("lib/liblogos_core.so: file is missing", errors)

    def test_rejects_duplicate_destination(self):
        with TemporaryDirectory() as tmp:
            manifest = self.manifest()
            manifest["artifacts"].append(manifest["artifacts"][0].copy())
            errors = validate_manifest(manifest, Path(tmp))
            self.assertIn("lib/liblogos_core.so: duplicate destination", errors)

    def test_rejects_absolute_destination(self):
        with TemporaryDirectory() as tmp:
            errors = validate_manifest(self.manifest(path="/tmp/liblogos_core.so"), Path(tmp))
            self.assertIn("/tmp/liblogos_core.so: destination must be relative", errors)
```

- [ ] **Step 2: Run the test and confirm the validator is missing**

Run from `nix/android`: `python3 -m unittest discover -s tests -p 'test_verify_staging.py' -v`.
Expected: FAIL because `verify_staging.py` does not exist.

- [ ] **Step 3: Implement deterministic staging and ELF inspection**

Stage only outputs from the Android derivations. Use `llvm-readelf -h -d` on every ELF and require `Machine: AArch64`, root target `aarch64-unknown-linux-android`, and the expected SONAME and dependency fields. `verify_staging.py` must reject absolute paths, missing files, duplicate destinations, non-Android ELF machine IDs, and host Linux artifacts. Do not report the synthetic validator fixture as runtime evidence.

- [ ] **Step 4: Run validator tests and inspect the actual staged tree**

Run from `nix/android`: `python3 -m unittest discover -s tests -p 'test_verify_staging.py' -v`.
Expected: PASS for failure cases covering x86-64, missing artifact, duplicate path, and absolute path.

Run: `nix build -L --no-link --print-out-paths nix/android#packages.x86_64-linux.android-runtime-staging > /tmp/logos-android-staging-path`.
Expected: PASS and write one Nix store path to `/tmp/logos-android-staging-path`.

Run: `python3 nix/android/verify_staging.py --root "$(cat /tmp/logos-android-staging-path)"`.
Expected: exit 0 after validating the staged `android-runtime-manifest.json`; every ELF is AArch64 and every destination is relative and unique.

- [ ] **Step 5: Commit artifact staging and verification**

```text
build: stage and verify Android Logos artifacts (#12)

Changes:
- Stage Qt, Logos runtime, and capability module outputs at deterministic paths.
- Add ELF and manifest checks that reject host artifacts.

Verification:
- `python3 -m unittest discover -s tests -p 'test_verify_staging.py' -v`: passed.
- `nix build -L --no-link --print-out-paths nix/android#packages.x86_64-linux.android-runtime-staging`: passed.
- `python3 nix/android/verify_staging.py --root "$(cat /tmp/logos-android-staging-path)"`: passed for all staged ELFs.
```

### Task 7: Record cross-build evidence and update issues #7 and #12

**Files:**
- Modify: `docs/research/android-build-experiment.md`
- Modify: `docs/research/dependencies-and-build.md`
- Modify: `docs/roadmap.md`
- Create: `nix/android/tests/evidence_complete.py`
- Create: `nix/android/tests/check_markdown_links.py`
- Modify: issues #7 and #12 progress through the issue-linked PR review process.

**Interfaces:**
- Consumes: toolchain report, pinned source patch manifest, build logs, staged artifact manifest, and ELF inspection from Tasks 1–6.
- Produces: a reviewed record that distinguishes evaluation, Qt cross-build, full Logos closure cross-build, and staging evidence.
- Produces: issue #7's verified Android runtime artifact handoff and an updated issue #12 checklist.

- [ ] **Step 1: Write a failing evidence completeness check**

```python
from pathlib import Path

report = Path("docs/research/android-build-experiment.md").read_text()
required = [
    "27.2.12479018", "API 28", "Qt 6.9.2", "arm64-v8a",
    "android-runtime-manifest.json", "cross-compilation evidence",
]
for value in required:
    assert value in report, f"missing evidence field: {value}"
assert Path("docs/research/logs/android-cross-build").is_dir()
```

- [ ] **Step 2: Run the check and confirm the evidence report is incomplete**

Run: `python3 nix/android/tests/evidence_complete.py`.
Expected: FAIL because the final Android build evidence has not been added.

- [ ] **Step 3: Record exact versions, commands, logs, outputs, and limitations**

Update the Android build experiment with the target configuration, source patch inventory, build commands and exit statuses, log paths, Nix store outputs, ELF results, and staging manifest. Update the dependency inventory with the Android cross-target boundary and keep the roadmap's issue #4/#5 status blocked until #7 completes its own artifact-staging acceptance criteria. Update #7 with the verified artifacts and #12 with only the checklist items supported by this PR's evidence.

- [ ] **Step 4: Run documentation and evidence checks**

Run: `python3 nix/android/tests/evidence_complete.py`.
Expected: PASS for every required field and build log directory.

Run: `git diff --check`.
Expected: no whitespace errors.

Run: `python3 nix/android/tests/check_markdown_links.py`.
Expected: all repository-relative Markdown links resolve.

- [ ] **Step 5: Commit the evidence report**

```text
docs: record Android Logos cross-build evidence (#12)

Changes:
- Record pinned toolchain, source patches, build outputs, logs, and ELF staging evidence.
- Update the roadmap and issue #7 handoff with the verified result.

Verification:
- `python3 nix/android/tests/evidence_complete.py`: passed.
- `git diff --check`: passed.
- `python3 nix/android/tests/check_markdown_links.py`: passed.
```

## Final Verification

- `NIXPKGS_ALLOW_UNFREE=1 NIXPKGS_ACCEPT_ANDROID_SDK_LICENSE=1 nix flake check ./nix/android` — all Nix checks pass.
- `bash nix/android/tests/target-config.sh` — build host, Android target, ABI, API, NDK, and Qt assertions pass.
- `python3 nix/android/tests/package-overrides.py` — Android overrides apply only to Android.
- `cd nix/android && python3 -m unittest discover -s tests -p 'test_source_patches.py' -v` and `bash nix/android/prepare-sources.sh --check` — every patch applies to its pinned source.
- `bash nix/android/build-qt.sh` and `bash nix/android/build-logos.sh` — all required Android outputs build and logs record exit status.
- `cd nix/android && python3 -m unittest discover -s tests -p 'test_verify_staging.py' -v` — staging verifier tests pass.
- `python3 nix/android/verify_staging.py --root "$(cat /tmp/logos-android-staging-path)"` — all target ELFs and manifest entries pass.
- `python3 nix/android/tests/evidence_complete.py`, `git diff --check`, and `python3 nix/android/tests/check_markdown_links.py` — evidence and documentation checks pass.
