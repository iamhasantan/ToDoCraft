# ToDoCraft

**ToDoCraft** is an independent, clean-room Rust task manager targeting Microsoft To Do feature parity. It follows the architecture and design principles of the [Storytold Craft suite](https://github.com/storytold), without copying another project's branding, artwork, or source.

## Current stage: pre-alpha

The first implementation provides a native `egui` desktop UI, a pure Rust task engine, and local SQLite persistence. It is not yet a complete Microsoft To Do replacement. See [feature parity](docs/parity.md) and [known gaps](docs/gaps.md).

### Build

Install Rust 1.95 or later and the platform dependencies for eframe. Then run:

```sh
cargo run -p todocraft
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

A Rust-only native desktop build is the first target. Microsoft Graph sync, shared lists, account sign-in, attachments and system notifications are future work.

## Principles

- **Clean room**. Only public documentation and observable behaviour inform the implementation. No proprietary source, icons, sounds or assets.
- **Engine first**. Tasks and recurrence live in `todocraft-core`, persistence in `todocraft-storage`, desktop UI in `todocraft`.
- **Rust only**. No Electron, Tauri, webviews or JavaScript.
- **Never lose data silently**. Storage errors appear in-app. SQLite saves are transactional.
- **Theme by meaning**. The UI uses semantic theme tokens, never fixed colours in workflow logic.
- **Measure parity**. Each feature has a verified status in `docs/parity.md`. Missing features remain visible.
- **PR-first development**. Future changes are proposed on branches, tested and reviewed.

## Independent project

Not affiliated with Microsoft or Storytold. Microsoft To Do is a trademark of Microsoft. ArtCraft and Craft suite names belong to their respective owners.

Licensed MIT OR Apache-2.0 (licence texts to be added before a public release).
