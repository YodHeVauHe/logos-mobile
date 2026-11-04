# Instructions for coding agents

Read [CONTRIBUTING.md](CONTRIBUTING.md), the [roadmap](docs/roadmap.md), the relevant GitHub issue, and its acceptance criteria before changing files.

## Issue and pull request rules

- Do not start work without an open issue. Every pull request must link to the issue it addresses and stay within that issue's checklist.
- Work one phase at a time. Do not start an issue while its dependencies remain open or blocked.
- Use branches named `issue-<number>-<slug>`. Never work directly on `main`, force-push, rewrite pushed history, or merge your own pull request.
- Update issue checklists only for work included in a pull request, and report verification evidence in that PR.
- Ask for clarification in the issue or PR when acceptance criteria conflict or evidence is insufficient. Do not invent upstream APIs or compatibility claims.

## Commit messages

Use this exact structure with real newlines:

```text
Concise conventional subject (include the related issue number when known)

Changes:
- Describe concrete changes.

Verification:
- List commands and results, or explain why verification was not run.
```

Do not use literal `\\n` text or add trailers. Do not invent an author or email.

## Verification and reporting

- Record actual command output as verification. If a command cannot run, state why and identify what remains unverified.
- Distinguish mocked unit tests, host builds, Android-target builds, emulator runs, and real-device integration tests.
- Never claim an actual Logos runtime, module, protocol, or node is working without evidence of that local behavior.
- Do not silently change scope when blocked.

## Security

Never commit seeds, private keys, wallet storage, credentials, tokens, local environment files, or other secrets. Do not send messages or post publicly on anyone's behalf.
