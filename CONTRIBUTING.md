# Contributing

This repository is developed through GitHub issues and reviewed pull requests. Every unit of work must belong to an open issue, and every pull request must link to the issue whose checklist it addresses.

## Work order

1. Read the [roadmap](docs/roadmap.md), the relevant issue, and any linked architecture decision records.
2. Confirm the issue's dependencies are complete before starting. Do not work on a blocked phase.
3. Keep changes within the issue's scope. Add newly discovered work as a separate issue before starting it.
4. Use a branch named `issue-<number>-<slug>` and open a focused pull request that links the issue with `Refs #<number>`.
5. Update the issue checklist as items land. Close an issue only after all acceptance criteria have been reviewed against merged pull requests and evidence.

## Pull requests and verification

- Each pull request addresses an existing issue and includes its scope, completed checklist items, verification commands/results, and evidence type.
- Keep one concern per pull request. Documentation-only changes should be separate when they can be reviewed independently.
- State whether evidence comes from source inspection, a mock, a host build, Android cross-compilation, an emulator, or a real device.
- Do not report compilation as proof of protocol integration. Do not report simulated behavior as real Logos behavior.
- Record failures and blockers clearly. Do not silently widen the issue to work around them.

## Commit messages

Use a concise conventional subject, including the issue number when known, followed by `Changes:` bullets and `Verification:` bullets. Use real newlines and do not add trailers. If verification was not run, state why.

## Project constraints

- Integrate existing Logos runtimes and modules; do not reimplement their protocol functionality.
- Do not infer Android support from Linux AArch64 support.
- Do not promise uninterrupted Android background operation.
- Do not commit credentials, private keys, seeds, wallet data, tokens, or other local secrets.
- Do not post to external forums or social accounts on anyone's behalf.

## License

Contributions are offered under the repository's dual MIT or Apache-2.0 license.
