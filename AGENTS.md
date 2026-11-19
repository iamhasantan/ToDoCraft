# ToDoCraft contributor guidance

This repository follows the principles of [Storytold Craft](https://github.com/storytold) as an independent project.

- Pure Rust native app. No Tauri, Electron, webviews, handwritten JavaScript or TypeScript.
- Clean-room Microsoft To Do compatibility. Observe public behaviour and public documentation, never copy proprietary source/assets or the Microsoft To Do identity.
- The task engine lives in crates/core, disk access in crates/storage, UI in apps/todocraft. No domain logic in widgets.
- Never crash on malformed input or failed storage. No panics, unsafe, unwrap, expect, todo, unreachable or unimplemented in production code. Handle errors visibly.
- Use semantic colour and typography roles in the theme adapter. Avoid decorative hard-coded palette values inside business logic.
- Save state transactionally, surface failures, and keep a test for every correctness fix.
- Build is not proof of feature parity. Update docs/parity.md and docs/gaps.md with implementation evidence.
- No external assets or fonts without a documented redistribution licence. Prefer simple shapes rendered in egui.
- Change main through a PR after initial empty-repository bootstrapping. Do not silently merge, release, or publish.
- Review the whole diff before claiming tests or opening PRs; report only checks that actually ran.
